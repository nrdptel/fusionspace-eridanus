//! The pages' Core Web Vitals (#443; the product system's `web.md`, *Performance*): every page
//! of the built site loaded cold in headless Chrome as a mid-tier phone on a slow mobile
//! network, its Largest Contentful Paint (LCP), Cumulative Layout Shift (CLS) and Interaction
//! to Next Paint (INP) measured, and each held to the bound `web.md` sets for the 75th
//! percentile of real visits: LCP at most 2.5 s, CLS at most 0.1, INP at most 200 ms (the
//! "good" thresholds of <https://web.dev/articles/vitals>).
//!
//! A lab load is not a visit, so this is a stand-in for the field numbers, and it errs slow:
//!
//! - **The phone and the network are Lighthouse's mobile preset**, applied the way Lighthouse
//!   applies it when it throttles through DevTools (`throttlingMethod: 'devtools'`): a Moto G
//!   Power's screen, 412 by 823 CSS pixels at 1.75 device pixels each, and its "slow 4G": every
//!   request waits 562.5 ms (150 ms of round trip, times DevTools' 3.75), 188,743 bytes a second
//!   down and 86,400 up (1.6 Mbit/s and 750 kbit/s, times DevTools' 0.9). Sources: Lighthouse's
//!   `core/config/constants.js` and `core/lib/emulation.js`, and the Lantern constants they import
//!   (`front_end/models/trace/lantern/simulation/Constants.ts` in Chrome's DevTools).
//! - **The CPU is slowed to a phone's whatever the machine.** Before it loads any page, the
//!   check measures the machine's BenchmarkIndex as Lighthouse does (`vitals.js`,
//!   `computeBenchmarkIndex`) and slows the CPU by the factor Lighthouse's throttling guide points
//!   to for it ([`cpu_slowdown`]), so a fast laptop and a CI runner both stand in for the same
//!   phone: about 4 for a desktop of the guide's day, and 16 for an Apple M-series laptop
//!   (BenchmarkIndex about 4,400), where the calculator's last line is carried past the machines
//!   it was fitted to. That is the least certain step here.
//! - **Every load is a first visit**: the cache off and the site's storage cleared before each.
//!   The site is served as GitHub Pages serves it, its text compressed with gzip, but over
//!   HTTP/1.1, where a browser opens at most six connections to one server; GitHub Pages speaks
//!   HTTP/2, which has no such limit, so a page's requests wait longer here than they would.
//! - **The interactions are a reader's first ones on a phone**: Tab, which moves the focus, and,
//!   on a page with the menu button, a tap that opens the menu and one that closes it. INP is the
//!   slowest of them, as it is for a visit with fewer than 50 interactions (`web.dev/articles/inp`).
//!   The taps and keys come through DevTools' input domain, which the browser counts as trusted,
//!   so Event Timing reports them; the page counts each one arriving before it is read.
//! - **A page over a bound is measured three times**, and the median of each vital is held to
//!   the bound, as Lighthouse advises taking the median of several runs.
//!
//! Three canaries, pages served by the check itself, must each fail their own vital and pass
//! the other two, or the check fails: one whose stylesheet arrives late (LCP), one that moves its
//! text after it is drawn (CLS) and one whose menu button keeps the page busy (INP).

use std::collections::{BTreeMap, VecDeque};
use std::path::Path;
use std::process::Child;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::{Value, json};

use super::cdp::Cdp;
use super::pages::{self, CHECK_PREFIX, Part, Scratch};

/// The page's half of the measurement: its observers and Lighthouse's benchmark.
const SCRIPT: &str = include_str!("vitals.js");

/// The Moto G Power's viewport in CSS pixels, and its device pixels per CSS pixel
/// (Lighthouse's `MOTOGPOWER_EMULATION_METRICS`).
pub(super) const WIDTH_PX: u32 = 412;
const HEIGHT_PX: u32 = 823;
const SCALE: f64 = 1.75;
/// Lighthouse's slow 4G as DevTools applies it: each request's added wait, and the bytes a
/// second down and up (`mobileSlow4G`'s `requestLatencyMs`, and its download and upload
/// throughput in kbit/s times 1024 / 8, floored, as `emulation.js` sends them).
const LATENCY_MS: f64 = 562.5;
const DOWNLOAD_BYTES_PER_S: f64 = 188_743.0;
const UPLOAD_BYTES_PER_S: f64 = 86_400.0;

/// The bounds, `web.md`'s *Performance* (the "good" thresholds of web.dev's *Web Vitals*).
const LCP_BOUND_MS: f64 = 2500.0;
const CLS_BOUND: f64 = 0.1;
const INP_BOUND_MS: f64 = 200.0;

/// CLS's session windows (<https://web.dev/articles/cls>): a shift less than a second after the
/// one before joins its window, and a window spans at most five seconds.
const WINDOW_GAP_MS: f64 = 1000.0;
const WINDOW_SPAN_MS: f64 = 5000.0;

/// How long a page must go without a new paint or shift, once loaded, before it is read; and
/// the most the check waits for that.
const QUIET_MS: u64 = 1000;
const SETTLE_MOST_MS: u64 = 15_000;
/// The most the check waits for a page to load, and for an input to finish.
const LOAD_MOST: Duration = Duration::from_secs(90);
const INPUT_MOST_MS: u64 = 5000;
/// How long one DevTools command may take, but for the waits above.
const CALL_TIMEOUT: Duration = Duration::from_secs(30);

/// How many times a page over a bound is measured; the median is held to the bound.
const RUNS: usize = 3;
/// The most browsers that measure at once. Each loads one page at a time, and more than half
/// the machine's cores would have them slow each other down.
const MOST_BROWSERS: usize = 4;
/// The share of [`LATENCY_MS`] a page must have taken to arrive, from the navigation's start to
/// its response's end, or the network throttle didn't hold for it and the load is measured
/// again. (Chrome adds the wait before a response's body, not before its first byte.)
const THROTTLE_HELD: f64 = 0.9;

/// A vital.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Vital {
    Lcp,
    Cls,
    Inp,
}

/// A page the check serves to prove it fails each vital.
struct Canary {
    file: &'static str,
    fails: Vital,
    html: &'static str,
}

/// The stylesheet the slow canary waits for, and how long the server holds it.
const SLOW_CSS: &str = "vitals-slow.css";
const SLOW_CSS_DELAY: Duration = Duration::from_millis(2500);
/// The page each browser loads first, where the machine's CPU is measured.
const BLANK: &str = "vitals-blank.html";

const CANARIES: [Canary; 3] = [
    Canary {
        file: "vitals-canary-lcp.html",
        fails: Vital::Lcp,
        html: concat!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">",
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
            "<title>Canary: a late paint</title><link rel=\"stylesheet\" href=\"vitals-slow.css\">",
            "</head><body><h1>A late paint</h1><p>Nothing here is drawn until a stylesheet the ",
            "server holds back arrives.</p></body></html>"
        ),
    },
    Canary {
        file: "vitals-canary-cls.html",
        fails: Vital::Cls,
        html: concat!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">",
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
            "<title>Canary: a shift</title></head><body><div id=\"above\"></div>",
            "<h1>A shift</h1><p>This text is drawn, then pushed down by a box that grows above ",
            "it once the page has loaded.</p><p>And this paragraph moves with it.</p><script>",
            "addEventListener('load', function () { setTimeout(function () { ",
            "document.getElementById('above').style.height = '300px'; }, 300); });",
            "</script></body></html>"
        ),
    },
    Canary {
        file: "vitals-canary-inp.html",
        fails: Vital::Inp,
        html: concat!(
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">",
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
            "<title>Canary: a slow tap</title></head><body>",
            "<label id=\"mdbook-sidebar-toggle\" style=\"display:block;width:48px;height:48px;",
            "background:#ccc\">Menu</label><h1>A slow tap</h1><p>The menu button keeps the page ",
            "busy for 300 ms each time it is tapped.</p><script>",
            "document.getElementById('mdbook-sidebar-toggle').addEventListener('click', ",
            "function () { const end = performance.now() + 300; while (performance.now() < end) {} });",
            "</script></body></html>"
        ),
    },
];

/// The answer to a request under `/__check/` that names one of the vitals check's own files:
/// its content type, its body, and how long to hold it first.
pub(super) fn served(name: &str) -> Option<(&'static str, Vec<u8>, Duration)> {
    if name == SLOW_CSS {
        return Some((
            "text/css; charset=utf-8",
            b"p { margin: 1em 0; }".to_vec(),
            SLOW_CSS_DELAY,
        ));
    }
    if name == BLANK {
        let html = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">\
                    <title>Blank</title></head><body></body></html>";
        return Some(("text/html; charset=utf-8", html.into(), Duration::ZERO));
    }
    CANARIES
        .iter()
        .find(|canary| canary.file == name)
        .map(|canary| {
            (
                "text/html; charset=utf-8",
                canary.html.into(),
                Duration::ZERO,
            )
        })
}

/// What the check found.
pub(super) struct Report {
    pub pages: usize,
    pub browsers: usize,
    /// The machine's BenchmarkIndex, and the CPU slowdown it gave.
    pub benchmark: f64,
    pub slowdown: f64,
    pub seconds: f64,
    /// The worst page for each vital: LCP in ms, CLS, INP in ms.
    pub worst: [(f64, String); 3],
    pub problems: Vec<String>,
}

/// The CPU slowdown that makes a machine whose BenchmarkIndex is `index` stand in for the
/// mid-tier phone Lighthouse's mobile preset emulates: the piecewise-linear rule of Lighthouse's
/// CPU throttling calculator (P. Hulce, `lighthouse-cpu-throttling-calculator`, linked from
/// Lighthouse's `docs/throttling.md`, which puts desktops at 1,000 to 2,000 and advises about 4
/// for a high-end one): 1 + (i − 150)/650 under 800, 2 + (i − 800)/500 under 1,300, and
/// 3 + (i − 1,300)/233 from there. A machine under 150 is as slow as the phone already and
/// can't be slowed to it, so it is refused: its numbers would flatter the site.
fn cpu_slowdown(index: f64) -> Result<f64, String> {
    if !index.is_finite() || index < 150.0 {
        return Err(format!(
            "this machine's BenchmarkIndex is {index:.0}, under the 150 of the phone the check \
             stands in for, so it can't be slowed to that phone"
        ));
    }
    Ok(if index < 800.0 {
        1.0 + (index - 150.0) / 650.0
    } else if index < 1300.0 {
        2.0 + (index - 800.0) / 500.0
    } else {
        3.0 + (index - 1300.0) / 233.0
    })
}

/// CLS from a page's layout shifts, each `(start time in ms, score)` in the order they came: the
/// largest sum of one session window (<https://web.dev/articles/cls>).
fn cls(shifts: &[(f64, f64)]) -> f64 {
    let mut largest = 0.0_f64;
    let mut window = 0.0;
    let mut first = f64::NEG_INFINITY;
    let mut last = f64::NEG_INFINITY;
    for &(at, score) in shifts {
        if at - last >= WINDOW_GAP_MS || at - first >= WINDOW_SPAN_MS {
            window = 0.0;
            first = at;
        }
        window += score;
        last = at;
        largest = largest.max(window);
    }
    largest
}

/// INP from a page's interaction timings, each `(interaction id, duration in ms)`: the slowest
/// interaction, an interaction lasting as long as its slowest event, as for a visit with fewer
/// than 50 interactions (<https://web.dev/articles/inp>). No timing means none reached 16 ms.
fn inp(events: &[(u64, f64)]) -> f64 {
    let mut slowest: BTreeMap<u64, f64> = BTreeMap::new();
    for &(id, duration) in events {
        let entry = slowest.entry(id).or_insert(0.0);
        *entry = entry.max(duration);
    }
    slowest.values().copied().fold(0.0, f64::max)
}

/// Whether a layout shift at `at` ms counts toward CLS: one Chrome doesn't flag as following an
/// input (`recent`), or one it flags that came before the first input (at `input_at`), as
/// headless Chrome flags shifts in the half second after a page loads when nothing was input.
fn counts(at: f64, recent: bool, input_at: Option<f64>) -> bool {
    !recent || input_at.is_none_or(|input| at < input)
}

/// The median of `values`, which aren't empty.
fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

/// What the page reported (`vitals.js`, `read`).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Seen {
    visible: bool,
    lcp: Option<f64>,
    lcp_what: String,
    shifts: Vec<(f64, f64, String, bool)>,
    events: Vec<(u64, String, f64)>,
    first_input: bool,
    input_at: Option<f64>,
    unsupported: Vec<String>,
    waited_ms: Option<f64>,
}

/// One page's vitals, and what each is about.
#[derive(Clone, Debug)]
struct Vitals {
    lcp_ms: f64,
    lcp_what: String,
    cls: f64,
    cls_what: String,
    inp_ms: f64,
    inp_what: String,
}

impl Vitals {
    fn get(&self, vital: Vital) -> f64 {
        match vital {
            Vital::Lcp => self.lcp_ms,
            Vital::Cls => self.cls,
            Vital::Inp => self.inp_ms,
        }
    }

    /// The vitals over their bounds.
    fn over(&self) -> Vec<Vital> {
        [Vital::Lcp, Vital::Cls, Vital::Inp]
            .into_iter()
            .filter(|&vital| self.get(vital) > bound(vital))
            .collect()
    }

    /// What is wrong with the page, one line a vital over its bound.
    fn problems(&self, page: &str, runs: usize) -> Vec<String> {
        let measured = if runs > 1 {
            format!(", the median of {runs} loads")
        } else {
            String::new()
        };
        self.over()
            .into_iter()
            .map(|vital| match vital {
                Vital::Lcp => format!(
                    "{page}: LCP {:.2} s, over 2.5 s{measured} (its largest paint: {})",
                    self.lcp_ms / 1000.0,
                    self.lcp_what
                ),
                Vital::Cls => format!(
                    "{page}: CLS {:.3}, over 0.1{measured} (moved: {})",
                    self.cls, self.cls_what
                ),
                Vital::Inp => format!(
                    "{page}: INP {:.0} ms, over 200 ms{measured} (its slowest input: {})",
                    self.inp_ms, self.inp_what
                ),
            })
            .collect()
    }
}

fn bound(vital: Vital) -> f64 {
    match vital {
        Vital::Lcp => LCP_BOUND_MS,
        Vital::Cls => CLS_BOUND,
        Vital::Inp => INP_BOUND_MS,
    }
}

/// A page to measure: one of the site's, or a canary.
#[derive(Clone)]
enum Job {
    Page(String),
    Canary(usize),
}

/// What one job measured: the vitals and how many loads they took, or why it couldn't.
type Measured = Result<(Vitals, usize), String>;

/// Measures the vitals of the site built in `output`: this part's share of its pages, and the
/// canaries in every part.
pub(super) fn check(output: &Path, part: Part) -> Result<Report, String> {
    let started = Instant::now();
    let mut all = Vec::new();
    super::html_files(output, "", &mut all)?;
    all.retain(|page| !page.starts_with(&format!("{}/", super::API)));
    all.sort();
    let pages: Vec<String> = all
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % part.count == part.index - 1)
        .map(|(_, page)| page)
        .collect();
    let mut queue: VecDeque<Job> = (0..CANARIES.len()).map(Job::Canary).collect();
    queue.extend(pages.iter().cloned().map(Job::Page));

    let chrome = pages::find_chrome()?;
    let server = pages::Server::start_compressed(output.to_path_buf())?;
    let origin = format!("http://{}", server.address);
    let scratch = Scratch::new()?;
    let browsers = thread::available_parallelism()
        .map_or(1, |cores| cores.get() / 2)
        .clamp(1, MOST_BROWSERS)
        .min(queue.len());

    // The CPU is measured in the first browser alone, before any page loads.
    let mut first = Browser::launch(&chrome, &scratch, 0)?;
    let benchmark = first.benchmark(&origin)?;
    let slowdown = cpu_slowdown(benchmark)?;
    let mut tabs = vec![first];
    for index in 1..browsers {
        let mut tab = Browser::launch(&chrome, &scratch, index)?;
        tab.load(&origin, &format!("{origin}/{CHECK_PREFIX}{BLANK}"))?;
        tabs.push(tab);
    }

    let queue = Mutex::new(queue);
    let found: Mutex<Vec<(Job, Measured)>> = Mutex::new(Vec::new());
    let failed: Mutex<Vec<String>> = Mutex::new(Vec::new());
    thread::scope(|scope| {
        for (index, tab) in tabs.iter_mut().enumerate() {
            let (queue, found, failed, origin) = (&queue, &found, &failed, &origin);
            scope.spawn(move || {
                if let Err(err) = tab.throttle(slowdown) {
                    if let Ok(mut failed) = failed.lock() {
                        failed.push(format!("browser {index} could not be throttled: {err}"));
                    }
                    return;
                }
                loop {
                    let Some(job) = queue.lock().ok().and_then(|mut queue| queue.pop_front())
                    else {
                        return;
                    };
                    let url = match &job {
                        Job::Page(page) => format!("{origin}/{page}"),
                        Job::Canary(i) => format!("{origin}/{CHECK_PREFIX}{}", CANARIES[*i].file),
                    };
                    let result = tab.vitals(origin, &url, matches!(job, Job::Page(_)));
                    if let Ok(mut found) = found.lock() {
                        found.push((job, result));
                    }
                }
            });
        }
    });
    drop(tabs);
    drop(server);
    let poisoned = "the vitals check is broken: a browser's thread panicked";
    let found = found.into_inner().map_err(|_| poisoned)?;
    let mut broken = failed.into_inner().map_err(|_| poisoned)?;
    let left = queue.into_inner().map_err(|_| poisoned)?.len();
    if left > 0 {
        broken.push(format!(
            "the vitals check is broken: {left} pages were never measured"
        ));
    }
    let mut problems = Vec::new();
    let mut worst: [(f64, String); 3] = Default::default();
    for (job, result) in found {
        match (job, result) {
            (Job::Canary(i), Ok((vitals, _))) => {
                let canary = &CANARIES[i];
                let over = vitals.over();
                if over != [canary.fails] {
                    broken.push(format!(
                        "the vitals check is broken: the canary {} should fail {:?} alone, and \
                         measured LCP {:.0} ms, CLS {:.3}, INP {:.0} ms",
                        canary.file, canary.fails, vitals.lcp_ms, vitals.cls, vitals.inp_ms
                    ));
                }
            }
            (Job::Canary(i), Err(err)) => broken.push(format!(
                "the vitals check is broken: the canary {} could not be measured: {err}",
                CANARIES[i].file
            )),
            (Job::Page(page), Ok((vitals, runs))) => {
                for (slot, vital) in [Vital::Lcp, Vital::Cls, Vital::Inp].into_iter().enumerate() {
                    if vitals.get(vital) > worst[slot].0 || worst[slot].1.is_empty() {
                        worst[slot] = (vitals.get(vital), page.clone());
                    }
                }
                problems.extend(vitals.problems(&page, runs));
            }
            (Job::Page(page), Err(err)) => {
                problems.push(format!("{page}: could not be measured: {err}"));
            }
        }
    }
    if !broken.is_empty() {
        return Err(broken.join("\n"));
    }
    problems.sort();
    Ok(Report {
        pages: pages.len(),
        browsers,
        benchmark,
        slowdown,
        seconds: started.elapsed().as_secs_f64(),
        worst,
        problems,
    })
}

/// One headless browser and the page it has open, attached as `session`; stopped when dropped.
struct Browser {
    child: Child,
    cdp: Cdp,
    session: String,
}

impl Browser {
    /// Starts browser `index`, attaches to its page and sets the phone's screen, the cold cache
    /// and the page's observers. The CPU and network are throttled later ([`Browser::throttle`]).
    fn launch(chrome: &Path, scratch: &Scratch, index: usize) -> Result<Browser, String> {
        let mut child = pages::launch_debuggable(chrome, scratch, index)?;
        let cdp = Cdp::connect(&scratch.profile(index), || match child.try_wait() {
            Ok(Some(status)) => Some(status.to_string()),
            Ok(None) => None,
            Err(err) => Some(err.to_string()),
        });
        let mut cdp = match cdp {
            Ok(cdp) => cdp,
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(err);
            }
        };
        let attached = (|| {
            let created = cdp.call(
                None,
                "Target.createTarget",
                json!({ "url": "about:blank" }),
                CALL_TIMEOUT,
            )?;
            let target = created["targetId"]
                .as_str()
                .ok_or("the browser opened no page")?
                .to_owned();
            let attached = cdp.call(
                None,
                "Target.attachToTarget",
                json!({ "targetId": target, "flatten": true }),
                CALL_TIMEOUT,
            )?;
            attached["sessionId"]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "the browser gave no session for its page".to_owned())
        })();
        let session = match attached {
            Ok(session) => session,
            Err(err) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(err);
            }
        };
        let mut browser = Browser {
            child,
            cdp,
            session,
        };
        for (method, params) in [
            ("Page.enable", json!({})),
            ("Network.enable", json!({})),
            ("Network.setCacheDisabled", json!({ "cacheDisabled": true })),
            (
                "Emulation.setDeviceMetricsOverride",
                json!({
                    "width": WIDTH_PX, "height": HEIGHT_PX,
                    "deviceScaleFactor": SCALE, "mobile": true,
                }),
            ),
            (
                "Emulation.setTouchEmulationEnabled",
                json!({ "enabled": true }),
            ),
            (
                "Page.addScriptToEvaluateOnNewDocument",
                json!({ "source": SCRIPT }),
            ),
            ("Page.bringToFront", json!({})),
        ] {
            browser.call(method, params)?;
        }
        Ok(browser)
    }

    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.cdp
            .call(Some(&self.session), method, params, CALL_TIMEOUT)
    }

    /// The value of `expression` in the page, awaited when it is a promise.
    fn evaluate(&mut self, expression: &str, timeout: Duration) -> Result<Value, String> {
        let result = self.cdp.call(
            Some(&self.session),
            "Runtime.evaluate",
            json!({ "expression": expression, "returnByValue": true, "awaitPromise": true }),
            timeout,
        )?;
        if let Some(thrown) = result.get("exceptionDetails") {
            let why = thrown["exception"]["description"]
                .as_str()
                .or_else(|| thrown["text"].as_str())
                .unwrap_or("an exception");
            return Err(format!("`{expression}` threw: {why}"));
        }
        Ok(result["result"]["value"].clone())
    }

    /// Loads `url` cold, after clearing what the site stored, and waits for its load event.
    fn load(&mut self, origin: &str, url: &str) -> Result<(), String> {
        self.call(
            "Storage.clearDataForOrigin",
            json!({ "origin": origin, "storageTypes": "all" }),
        )?;
        let navigated = self.call("Page.navigate", json!({ "url": url }))?;
        if let Some(why) = navigated["errorText"]
            .as_str()
            .filter(|why| !why.is_empty())
        {
            return Err(format!("could not load {url}: {why}"));
        }
        let loaded = format!(
            "document.readyState === 'complete' && location.href === {}",
            Value::from(url)
        );
        let started = Instant::now();
        while self.evaluate(&loaded, CALL_TIMEOUT)? != Value::Bool(true) {
            if started.elapsed() > LOAD_MOST {
                return Err(format!(
                    "{url} didn't load within {} s",
                    LOAD_MOST.as_secs()
                ));
            }
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    /// The machine's BenchmarkIndex, measured on the blank page before any throttle is set.
    fn benchmark(&mut self, origin: &str) -> Result<f64, String> {
        self.load(origin, &format!("{origin}/{CHECK_PREFIX}{BLANK}"))?;
        self.evaluate("window.__hprVitals.benchmark()", CALL_TIMEOUT)?
            .as_f64()
            .ok_or_else(|| "the benchmark returned no number".to_owned())
    }

    /// Slows the CPU by `slowdown` and the network to slow 4G.
    fn throttle(&mut self, slowdown: f64) -> Result<(), String> {
        self.call(
            "Emulation.setCPUThrottlingRate",
            json!({ "rate": slowdown }),
        )?;
        self.call(
            "Network.emulateNetworkConditions",
            json!({
                "offline": false,
                "latency": LATENCY_MS,
                "downloadThroughput": DOWNLOAD_BYTES_PER_S,
                "uploadThroughput": UPLOAD_BYTES_PER_S,
            }),
        )?;
        Ok(())
    }

    /// The vitals of `url`, and how many loads they are the median of: one, or [`RUNS`] for a
    /// page of the site over a bound (`retry`).
    fn vitals(&mut self, origin: &str, url: &str, retry: bool) -> Measured {
        let once = self.measure_held(origin, url)?;
        if !retry || once.over().is_empty() {
            return Ok((once, 1));
        }
        let mut runs = vec![once];
        for _ in 1..RUNS {
            runs.push(self.measure_held(origin, url)?);
        }
        let worst_of = |vital: Vital| -> (f64, String) {
            let value = median(runs.iter().map(|run| run.get(vital)).collect());
            let what = runs
                .iter()
                .find(|run| run.get(vital) == value)
                .map(|run| match vital {
                    Vital::Lcp => run.lcp_what.clone(),
                    Vital::Cls => run.cls_what.clone(),
                    Vital::Inp => run.inp_what.clone(),
                })
                .unwrap_or_default();
            (value, what)
        };
        let (lcp_ms, lcp_what) = worst_of(Vital::Lcp);
        let (cls, cls_what) = worst_of(Vital::Cls);
        let (inp_ms, inp_what) = worst_of(Vital::Inp);
        Ok((
            Vitals {
                lcp_ms,
                lcp_what,
                cls,
                cls_what,
                inp_ms,
                inp_what,
            },
            RUNS,
        ))
    }

    /// One measurement of `url`, taken again once if the network throttle didn't hold for it.
    fn measure_held(&mut self, origin: &str, url: &str) -> Result<Vitals, String> {
        match self.measure(origin, url) {
            Err(Unheld(waited)) => match self.measure(origin, url) {
                Err(Unheld(again)) => Err(format!(
                    "the network throttle didn't hold: the page arrived {waited:.0} and then \
                     {again:.0} ms after it was asked for, under the {LATENCY_MS} ms every \
                     request waits"
                )),
                Err(Failed(err)) => Err(err),
                Ok(vitals) => Ok(vitals),
            },
            Err(Failed(err)) => Err(err),
            Ok(vitals) => Ok(vitals),
        }
    }

    /// One cold load of `url`, its LCP read before any input, then its inputs, then its shifts
    /// and their timing.
    fn measure(&mut self, origin: &str, url: &str) -> Result<Vitals, Miss> {
        self.load(origin, url)?;
        self.evaluate(
            &format!("window.__hprVitals.settle({QUIET_MS}, {SETTLE_MOST_MS})"),
            Duration::from_millis(SETTLE_MOST_MS) + CALL_TIMEOUT,
        )?;
        let before = self.seen()?;
        if !before.visible {
            return Err(Failed(
                "the page was hidden, so it reports no paints".into(),
            ));
        }
        if !before.unsupported.is_empty() {
            return Err(Failed(format!(
                "the browser can't observe {}",
                before.unsupported.join(", ")
            )));
        }
        let waited = before.waited_ms.unwrap_or(0.0);
        if waited < THROTTLE_HELD * LATENCY_MS {
            return Err(Unheld(waited));
        }
        let lcp_ms = before
            .lcp
            .ok_or_else(|| Failed("the page reported no largest contentful paint".into()))?;

        // Tab, then the menu button twice where there is one, found again before each tap: the
        // page moves over when the menu opens.
        let mut inputs = 0;
        for kind in ["rawKeyDown", "keyUp"] {
            self.call(
                "Input.dispatchKeyEvent",
                json!({
                    "type": kind, "key": "Tab", "code": "Tab",
                    "windowsVirtualKeyCode": 9, "nativeVirtualKeyCode": 9,
                }),
            )?;
        }
        inputs += 1;
        self.finished(inputs, "Tab")?;
        for _ in 0..2 {
            let Some([x, y]) = self.menu_button()? else {
                break;
            };
            {
                for kind in ["mousePressed", "mouseReleased"] {
                    self.call(
                        "Input.dispatchMouseEvent",
                        json!({ "type": kind, "x": x, "y": y, "button": "left", "clickCount": 1 }),
                    )?;
                }
                inputs += 1;
                self.finished(inputs, "the menu button's tap")?;
            }
        }
        let after = self.seen()?;
        if !after.first_input {
            return Err(Failed("the page timed none of its inputs".into()));
        }
        let counted: Vec<&(f64, f64, String, bool)> = after
            .shifts
            .iter()
            .filter(|shift| counts(shift.0, shift.3, after.input_at))
            .collect();
        let shifts: Vec<(f64, f64)> = counted.iter().map(|s| (s.0, s.1)).collect();
        let cls_what = counted
            .iter()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map_or_else(
                || "nothing".to_owned(),
                |s| format!("{} at {:.1} s", s.2, s.0 / 1000.0),
            );
        let events: Vec<(u64, f64)> = after.events.iter().map(|e| (e.0, e.2)).collect();
        let inp_what = after
            .events
            .iter()
            .max_by(|a, b| a.2.total_cmp(&b.2))
            .map_or_else(|| "none timed".to_owned(), |e| format!("a `{}`", e.1));
        Ok(Vitals {
            lcp_ms,
            lcp_what: before.lcp_what,
            cls: cls(&shifts),
            cls_what,
            inp_ms: inp(&events),
            inp_what,
        })
    }

    /// Where to tap the page's menu button, if it has one on screen.
    fn menu_button(&mut self) -> Result<Option<[f64; 2]>, String> {
        let at = self.evaluate(
            "(function () { const b = document.getElementById('mdbook-sidebar-toggle'); \
             if (!b) return null; const r = b.getBoundingClientRect(); \
             if (r.width <= 0 || r.height <= 0 || r.bottom < 0 || r.top > innerHeight || \
             r.right < 0 || r.left > innerWidth) return null; \
             return [r.left + r.width / 2, r.top + r.height / 2]; })()",
            CALL_TIMEOUT,
        )?;
        Ok(at
            .as_array()
            .and_then(|at| Some([at.first()?.as_f64()?, at.get(1)?.as_f64()?])))
    }

    /// Waits for the `n`-th input, `what`, to finish and be timed.
    fn finished(&mut self, n: u32, what: &str) -> Result<(), String> {
        let arrived = self.evaluate(
            &format!("window.__hprVitals.inputs({n}, {INPUT_MOST_MS})"),
            Duration::from_millis(INPUT_MOST_MS) + CALL_TIMEOUT,
        )?;
        if arrived == Value::Bool(true) {
            Ok(())
        } else {
            Err(format!("{what} didn't reach the page"))
        }
    }

    fn seen(&mut self) -> Result<Seen, String> {
        let text = self.evaluate("window.__hprVitals.read()", CALL_TIMEOUT)?;
        let text = text
            .as_str()
            .ok_or("the page's observers aren't there: no `__hprVitals`")?;
        serde_json::from_str(text).map_err(|err| format!("the page's report: {err}"))
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Why a measurement missed: the network throttle didn't hold (the ms the page took to
/// arrive), or anything else.
enum Miss {
    Unheld(f64),
    Failed(String),
}
use Miss::{Failed, Unheld};

impl From<String> for Miss {
    fn from(err: String) -> Miss {
        Failed(err)
    }
}

impl From<&str> for Miss {
    fn from(err: &str) -> Miss {
        Failed(err.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cpu_slowdown_follows_the_calculator_and_is_continuous() {
        let at = |index: f64| cpu_slowdown(index).unwrap();
        assert_eq!(at(150.0), 1.0);
        assert_eq!(at(800.0), 2.0);
        assert_eq!(at(1300.0), 3.0);
        // A high-end desktop (Lighthouse's docs: about 4).
        assert!((at(1533.0) - 4.0).abs() < 1e-9);
        // No jump where the pieces meet.
        for edge in [800.0, 1300.0] {
            assert!((at(edge) - at(edge - 1e-9)).abs() < 1e-9);
        }
        // Faster machines are slowed more.
        assert!(at(900.0) > at(700.0) && at(3000.0) > at(1500.0));
    }

    #[test]
    fn a_machine_slower_than_the_phone_is_refused() {
        assert!(cpu_slowdown(149.9).is_err());
        assert!(cpu_slowdown(f64::NAN).is_err());
        assert!(cpu_slowdown(f64::INFINITY).is_err());
    }

    #[test]
    fn cls_is_the_largest_session_window() {
        assert_eq!(cls(&[]), 0.0);
        // Two shifts 0.9 s apart share a window; one 1 s after the last starts a new one.
        let shifts = [(100.0, 0.05), (1000.0, 0.04), (2000.0, 0.06)];
        assert!((cls(&shifts) - 0.09).abs() < 1e-12);
        // A window spans at most 5 s, even with shifts every 0.5 s.
        let steady: Vec<(f64, f64)> = (0..20).map(|i| (f64::from(i) * 500.0, 0.01)).collect();
        assert!((cls(&steady) - 0.10).abs() < 1e-12);
        // The later, larger window wins.
        let later = [(0.0, 0.02), (3000.0, 0.08), (3500.0, 0.03)];
        assert!((cls(&later) - 0.11).abs() < 1e-12);
    }

    #[test]
    fn a_shift_flagged_as_following_input_counts_until_the_first_input() {
        assert!(counts(900.0, false, Some(500.0)));
        // Flagged, but nothing had been input yet.
        assert!(counts(900.0, true, None));
        assert!(counts(900.0, true, Some(3000.0)));
        // Flagged after the measurement's first input: the reader's doing.
        assert!(!counts(3200.0, true, Some(3000.0)));
    }

    #[test]
    fn inp_is_the_slowest_interaction_by_its_slowest_event() {
        assert_eq!(inp(&[]), 0.0);
        let events = [(7, 24.0), (7, 40.0), (9, 16.0), (12, 32.0)];
        assert_eq!(inp(&events), 40.0);
    }

    #[test]
    fn a_page_is_over_only_the_bounds_it_exceeds() {
        let vitals = Vitals {
            lcp_ms: 2500.0,
            lcp_what: "p".into(),
            cls: 0.105,
            cls_what: "p".into(),
            inp_ms: 200.0,
            inp_what: "a `click`".into(),
        };
        // Each bound is the most a page may have, as `web.md` writes them (≤).
        assert_eq!(vitals.over(), [Vital::Cls]);
        let problems = vitals.problems("a.html", 3);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].starts_with("a.html: CLS 0.105, over 0.1, the median of 3 loads"));
    }

    #[test]
    fn the_median_of_three_is_the_middle_one() {
        assert_eq!(median(vec![3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(vec![5.0]), 5.0);
    }

    #[test]
    fn each_canary_is_served_and_names_its_vital() {
        let vitals: Vec<Vital> = CANARIES.iter().map(|canary| canary.fails).collect();
        assert_eq!(vitals, [Vital::Lcp, Vital::Cls, Vital::Inp]);
        for canary in &CANARIES {
            let (kind, body, _) = served(canary.file).unwrap();
            assert!(kind.starts_with("text/html"));
            assert_eq!(body, canary.html.as_bytes());
        }
        let (_, _, delay) = served(SLOW_CSS).unwrap();
        assert!(delay.as_secs_f64() * 1000.0 + LATENCY_MS > LCP_BOUND_MS);
        assert!(served("nothing.html").is_none());
    }
}
