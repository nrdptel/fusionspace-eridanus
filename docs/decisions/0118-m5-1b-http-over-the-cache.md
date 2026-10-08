# ADR-118: M5.1b, HTTP over the cache (2026-09-30)

- **Status:** accepted; its cache folders renamed by [ADR-199, FusionSpace HPR](0199-the-2026-10-07-fusionspace-hpr.md)
- **Summary:** M5.1b: `ureq` 3 over rustls behind feature `http`, the body limit on unpacked bytes, the platform cache folder by hand

**Context.** M5.1b's *done when*: a loopback server replaying a recorded fixture fills the cache,
and `cargo deny` passes. The architecture names `ureq` or `reqwest` with rustls only, and
`directories` for platform paths.

**Decision.**

1. `Http`, a `Transport` over `ureq` 3.4 with its `rustls` and `gzip` features and no others,
   behind `hpr-net`'s `http` feature (off by default; the facade's `net` turns it on). `ureq` is
   blocking, which `hpr-net` allows, and small beside `reqwest`'s tokio stack. rustls uses the
   ring provider and Mozilla's roots compiled in (`webpki-roots`, CDLA-Permissive-2.0, already
   allowed by ADR-001's list), so no OpenSSL and no platform certificate store; ring compiles its
   own C and assembly with the platform's C compiler, which CI's three runners have. `ureq`'s defaults
   stay for redirects (ten) and the proxy (the first of `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`,
   for every URL, bar `NO_PROXY`). Three are tightened, each pinned by a test: any status but 2xx
   fails (`ureq` fails only 4xx and 5xx, so a 304 read as an empty body that the cache would keep
   for a TTL); a SOCKS proxy from the environment is refused, where `ureq` without its
   `socks-proxy` feature warns and connects directly (hosts `NO_PROXY` exempts are fetched with no
   redirect followed, since the next host might not be exempt, and the error names only the
   proxy's protocol: its address may carry a password, and a malformed one puts the user name
   where the host should be); and the timeout is capped at 30 days, since
   `Duration::MAX`, the usual "no limit", overflowed `Instant` and panicked on the first request.
   A 60 s timeout covers the whole request. The `User-Agent` names hpr-sim, its version and the
   repository, so providers can tell who calls. Settings live in a `#[non_exhaustive]`
   `HttpConfig` (timeout, body limit, whether to read the proxy from the environment), so later
   knobs add fields, not constructors; the tests turn the environment's proxy off.
2. The body limit (64 MiB by default) counts **unpacked** bytes and is inclusive. `ureq`'s own
   limit sits inside its gzip decoder, so it counts bytes on the wire, and a kilobyte of gzip can
   unpack to gigabytes; it also refuses a body of exactly the limit. `Http` reads the unpacked
   stream through `take(limit + 1)` and refuses more than `limit`. `tests/http.rs` pins both: a
   body one byte over is refused and one at the limit passes, and a megabyte of gzipped zeros,
   under 64 KiB on the wire, is refused at 64 KiB.
3. `Cache::platform_dir()` is written by hand, not with `directories`: its `dirs-sys` 0.5
   depends on `option-ext` (MPL-2.0) on every target, which `deny.toml` rejects. The rules are
   those crates' for caches, read from the environment: `$HOME/Library/Caches/hpr-sim` on macOS,
   `%LOCALAPPDATA%\hpr-sim\cache` on Windows, `$XDG_CACHE_HOME/hpr-sim` (absolute only, per the
   XDG Base Directory Specification) or `$HOME/.cache/hpr-sim` elsewhere. `HPR_CACHE_DIR`
   overrides all three. `None` when nothing is set; the caller then names a folder.
4. The loopback test serves `tests/fixtures/replay`'s recording at its path and query, from a
   thread on `127.0.0.1` in the test itself: no fixture server dependency, no network. A dropped
   connection is a route that hangs up, not a port freed and reused, which another test could take.
5. `Transport::get` keeps its `String` error. Telling a 404 (don't retry) from a 503 or a timeout
   (retry later) matters once a source retries, and nothing retries yet; the trait can change then,
   before any release.

**Consequences.** The first network dependency: `ureq` brings 26 crates on macOS, all permissive;
`cargo deny` passes, with a second `base64` (0.23 beside our 0.22) as a duplicate warning only.
TLS is compiled and linked but no test performs a handshake, since CI has no network and a local
TLS server would need certificates made for the test. A one-off manual run on macOS on 2026-09-30,
from a scratch program outside the repository, fetched an Open-Meteo forecast over HTTPS (328
bytes, then `Cached` on the second call) and refused `expired.badssl.com`'s expired certificate;
repeatable checks come with M5.2's recorded responses. Windows reads `LOCALAPPDATA` from the environment rather
than asking the shell for the known folder; the two agree unless a user unsets the variable.
