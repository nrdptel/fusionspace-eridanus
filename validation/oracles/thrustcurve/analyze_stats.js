// ThrustCurve.org's own statistics for every curve file hpr-motor's catalog stores as passing,
// and a recount of which files pass from the raw snapshot.
//
// What this pins: the total impulse, NFPA 1125 burn time (between the 5%-of-peak crossings),
// average thrust (total impulse over that burn time), peak thrust and burn window that
// ThrustCurve.org computes for each .eng/.rse file catalog.json stores as passing its 1% check
// (ADR-298, M5.7c), using the site's own code:
// simulate/analyze/analyze.js from github.com/JohnCoker/thrustcurve3 at commit
// 577afa62302f70c6b2ba04e97a39240638cd704b (ISC license, copyright John Coker). The file is
// fetched into refs/sources/thrustcurve3/analyze.js (see the `thrustcurve3-analyze` entry in
// validation/refs.lock.toml); this script refuses to run if its sha256 differs.
//
// How ThrustCurve calls it: routes/simfiles.js at the same commit parses a simfile with
// `parsers.parseData(format, data, errs)` and then calls `analyze.stats(parsed, errs)`, where
// `errs` is an error-collector function. With a function as the second argument, `stats` uses
// the default `StdParams` (5% burn cutoff, 50 us time merge, 500 uN leading-zero threshold).
// Before integrating, `stats` normalizes the points: it drops invalid points, sorts by time,
// drops leading points with thrust below 500 uN and merges points closer than 50 us (averaging
// their thrust). This script calls `stats({points}, error)` the same way, and separately calls
// `normalize` with the same defaults to report whether that cleaning changed a curve.
//
// Loading: analyze.js does `require('../../lib/errors')`. The real module
// (lib/errors/errors.js) depends on the npm package `strformat`, so it is not loaded. analyze.js
// only passes its numeric codes to the error callback, which cannot change any statistic, so
// they are supplied in-process with the values from lib/errors/errors.js at the same commit
// (INVALID_INFO 104, INVALID_POINTS 105, DUPLICATE_POINTS 106). The verified bytes are compiled
// with vm.compileFunction and a require that serves only that module, so the pinned file is run
// unmodified and nothing else is resolved.
//
// Point extraction is a minimal reader that follows ThrustCurve's parsers
// (simulate/parsers/rasp.js and rocksim.js):
// - .eng (RASP): lines are trimmed and split on whitespace; leading blank and `;` lines are
//   skipped, the first other line is the seven-field header, and the following two-field lines
//   are (time s, thrust N) points. Anything but blank or `;` lines after the data block (a
//   second motor, a malformed line) is an error.
// - .rse (RockSim): the `t` and `f` attributes of each `<eng-data .../>` element; the file must
//   hold exactly one `<engine ...>` element.
// Numbers must match lib/number's isNumber syntax (plain decimals, no exponent) and be
// non-negative, which is what those parsers accept; each file's sha256 must match catalog.json.
//
// Output: JSON on stdout, 2-space indent, trailing newline, one entry per curve in catalog order.
// Field mapping from the `stats` result: points <- pointCount, total_impulse_ns <- totalImpulse,
// burn_time_s <- burnTime, average_thrust_n <- avgThrust, max_thrust_n <- maxThrust,
// burn_start_s <- burnStart, burn_end_s <- burnEnd. raw_points is the count read from the file;
// merged_or_dropped is true when normalization changed the points. Numbers are printed as
// JavaScript's shortest round-trip form. The output depends only on the inputs and the Node
// version (recorded in "node"), so two runs are byte-identical.
//
// Run from the repository root (Node.js only, no npm packages):
//
//     node validation/oracles/thrustcurve/analyze_stats.js > validation/fixtures/motor/thrustcurve-analyze-stats.json
//
// The recount (ADR-288's consequences): `--recount` reads the raw snapshot answers under
// refs/snapshots/thrustcurve/<captured>/ (gitignored; `snapshot.py` writes them), not the
// committed records, file list or curve files. For every RASP or RockSim file the download answers
// hold the bytes of, it works out the file's figures with analyze.js as above and checks them
// against the raw search answer's record of its motor: the file passes when its total impulse,
// average thrust and burn time are each within 1% of the record's, |file - record| <= 0.01 x
// record, a record figure that isn't positive failing. It then fails unless the files it finds
// passing are exactly those catalog.json stores as passing, and every other file it reads is
// stored as failing or left out, and prints the counts. A file this reader refuses is printed
// with its reason and must be one the index leaves out or stores as failing. Nothing is written.
//
//     node validation/oracles/thrustcurve/analyze_stats.js --recount

'use strict';

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const vm = require('vm');

const ROOT = path.resolve(__dirname, '..', '..', '..');
const ANALYZE_PATH = path.join(ROOT, 'refs', 'sources', 'thrustcurve3', 'analyze.js');
const ANALYZE_SHA256 = '267eb5364a7ca13b4ec32246c97ee0194a4ee6fd5489765ff178fddf666c637a';
const DATA_DIR = path.join(ROOT, 'crates', 'hpr-motor', 'data', 'thrustcurve');
const CATALOG_PATH = path.join(DATA_DIR, 'catalog.json');
const SNAPSHOTS_DIR = path.join(ROOT, 'refs', 'snapshots', 'thrustcurve');

// Error codes from lib/errors/errors.js at commit 577afa6 (the only ones analyze.js uses).
const ERRORS_STUB = Object.freeze({
  INVALID_INFO: 104,
  INVALID_POINTS: 105,
  DUPLICATE_POINTS: 106,
});

function fail(message) {
  process.stderr.write('analyze_stats.js: ' + message + '\n');
  process.exit(1);
}

// A file the readers below refuse. The fixture fails on it; the recount reports it.
class Refusal extends Error {}

function refuse(message) {
  throw new Refusal(message);
}

function sha256(bytes) {
  return crypto.createHash('sha256').update(bytes).digest('hex');
}

function loadAnalyze() {
  let bytes;
  try {
    bytes = fs.readFileSync(ANALYZE_PATH);
  } catch (e) {
    fail('cannot read ' + ANALYZE_PATH + ' (' + e.message + '); fetch the thrustcurve3-analyze ' +
         'entry of validation/refs.lock.toml');
  }
  const digest = sha256(bytes);
  if (digest !== ANALYZE_SHA256) {
    fail('sha256 of ' + ANALYZE_PATH + ' is ' + digest + ', expected ' + ANALYZE_SHA256);
  }
  const params = ['exports', 'require', 'module', '__filename', '__dirname'];
  const factory = vm.compileFunction(bytes.toString('utf8'), params, { filename: ANALYZE_PATH });
  const mod = { exports: {} };
  const requireStub = function(id) {
    if (id === '../../lib/errors') return ERRORS_STUB;
    fail('analyze.js required unexpected module ' + JSON.stringify(id));
  };
  factory.call(mod.exports, mod.exports, requireStub, mod, ANALYZE_PATH,
               path.dirname(ANALYZE_PATH));
  const analyze = mod.exports;
  if (typeof analyze.stats !== 'function' || typeof analyze.normalize !== 'function') {
    fail('analyze.js does not export stats and normalize');
  }
  return analyze;
}

// lib/number isNumber at commit 577afa6: what ThrustCurve's parsers accept before parseFloat.
function parseNumber(text, where) {
  if (!(/^-?([0-9]+)(\.[0-9]*)?$/.test(text) || /^\.[0-9]+$/.test(text))) {
    refuse(where + ': "' + text + '" is not a plain decimal number');
  }
  const n = parseFloat(text);
  if (!Number.isFinite(n) || n < 0) refuse(where + ': "' + text + '" is not a non-negative number');
  return n;
}

function readEng(text, file) {
  const lines = text.trim().split(/\r?\n/).map((l) => l.trim());
  let i = 0;
  while (i < lines.length && (lines[i] === '' || lines[i].charAt(0) === ';')) i++;
  if (i >= lines.length) refuse(file + ': no header line');
  if (lines[i].split(/\s+/).length < 7) refuse(file + ': header line has fewer than seven fields');
  const points = [];
  for (i++; i < lines.length; i++) {
    const line = lines[i];
    if (line === '' || line.charAt(0) === ';') break;
    const fields = line.split(/\s+/);
    if (fields.length !== 2) {
      refuse(file + ':' + (i + 1) + ': expected two fields (a second motor or a malformed line)');
    }
    points.push({
      time: parseNumber(fields[0], file + ':' + (i + 1)),
      thrust: parseNumber(fields[1], file + ':' + (i + 1)),
    });
  }
  for (; i < lines.length; i++) {
    if (lines[i] !== '' && lines[i].charAt(0) !== ';') {
      refuse(file + ':' + (i + 1) + ': content after the data block (expected exactly one motor)');
    }
  }
  return points;
}

function readRse(text, file) {
  const engines = text.match(/<engine[\s>]/g) || [];
  if (engines.length !== 1) refuse(file + ': ' + engines.length + ' <engine> elements, expected 1');
  const opened = (text.match(/<eng-data[\s/>]/g) || []).length;
  const points = [];
  const re = /<eng-data\s([^>]*?)\/>/g;
  let m;
  while ((m = re.exec(text)) !== null) {
    const attrs = m[1];
    const t = /(?:^|\s)t="([^"]*)"/.exec(attrs);
    const f = /(?:^|\s)f="([^"]*)"/.exec(attrs);
    if (!t || !f) refuse(file + ': <eng-data> element #' + (points.length + 1) + ' lacks t or f');
    points.push({
      time: parseNumber(t[1], file + ' eng-data #' + (points.length + 1) + ' t'),
      thrust: parseNumber(f[1], file + ' eng-data #' + (points.length + 1) + ' f'),
    });
  }
  if (points.length !== opened) {
    refuse(file + ': ' + opened + ' <eng-data elements but ' + points.length + ' parsed');
  }
  return points;
}

function copyPoints(points) {
  return points.map((p) => ({ time: p.time, thrust: p.thrust }));
}

function finite(value, name, file) {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    fail(file + ': stats.' + name + ' is not a finite number (' + value + ')');
  }
  return value;
}

// The parameters that keep every sample: StdParams with a time merge of 0 s, so `normalize`
// merges no two points (its test is `time < last + timeEpsilon`) but still sorts them and drops
// leading points below 500 uN. Two samples at one time are then a step, as hpr-motor reads them
// (crates/hpr-motor/src/curve.rs), where the standard 50 us merge averages their thrust.
function unmergedParams(analyze) {
  return Object.assign({}, analyze.StdParams, { timeEpsilon: 0 });
}

// The points of one file's text. Throws a Refusal for a file the readers refuse.
function pointsOf(text, format, file) {
  let points;
  if (format === 'RASP') {
    points = readEng(text, file);
  } else if (format === 'RockSim') {
    points = readRse(text, file);
  } else {
    refuse(file + ': unsupported format ' + format);
  }
  if (points.length < 2) refuse(file + ': fewer than two data points');
  return points;
}

// analyze.stats on `points` with `params`, and the points it integrated.
function statsWith(analyze, points, params, file) {
  const reports = [];
  const error = function(code) {
    reports.push(code);
  };
  const stats = analyze.stats({ points: copyPoints(points) }, params, error);
  if (stats == null) refuse(file + ': analyze.stats returned nothing');
  if (reports.includes(ERRORS_STUB.INVALID_POINTS)) {
    refuse(file + ': analyze.js reported invalid points');
  }
  const cleaned = analyze.normalize(copyPoints(points), params, function() {});
  if (cleaned == null || cleaned.length !== stats.pointCount) {
    fail(file + ': normalize and stats disagree on the point count');
  }
  return {
    cleaned: cleaned,
    figures: {
      total_impulse_ns: finite(stats.totalImpulse, 'totalImpulse', file),
      burn_time_s: finite(stats.burnTime, 'burnTime', file),
      average_thrust_n: finite(stats.avgThrust, 'avgThrust', file),
      max_thrust_n: finite(stats.maxThrust, 'maxThrust', file),
      burn_start_s: finite(stats.burnStart, 'burnStart', file),
      burn_end_s: finite(stats.burnEnd, 'burnEnd', file),
    },
  };
}

// ThrustCurve's statistics for one file's points, as the fixture records them: the standard
// figures, how many points the 50 us merge folded into another, and, where it folded any, the
// figures with every sample kept (`unmerged`). Throws a Refusal for a file the readers refuse.
function statsOf(analyze, points, file) {
  const standard = statsWith(analyze, points, analyze.StdParams, file);
  const kept = statsWith(analyze, points, unmergedParams(analyze), file);
  const cleaned = standard.cleaned;
  const changed = cleaned.length !== points.length ||
    cleaned.some((p, k) => p.time !== points[k].time || p.thrust !== points[k].thrust);
  const merged = kept.cleaned.length - cleaned.length;
  const entry = Object.assign({
    raw_points: points.length,
    points: cleaned.length,
    merged_or_dropped: changed,
    merged_points: merged,
  }, standard.figures);
  if (merged > 0) entry.unmerged = kept.figures;
  return entry;
}

// The fixture: ThrustCurve's statistics for every file catalog.json stores as passing.
function fixture(analyze, catalog) {
  const curves = [];
  for (const motor of catalog.motors) {
    for (const curve of motor.curves) {
      if (curve.check.passes !== true) continue;
      const file = curve.file;
      const bytes = fs.readFileSync(path.join(DATA_DIR, file));
      if (sha256(bytes) !== curve.sha256) fail(file + ': sha256 does not match catalog.json');
      const extension = { RASP: '.eng', RockSim: '.rse' }[curve.format];
      if (extension === undefined || !file.endsWith(extension)) {
        fail(file + ': unsupported format ' + curve.format);
      }
      let stats;
      try {
        stats = statsOf(analyze, pointsOf(bytes.toString('utf8'), curve.format, file), file);
      } catch (e) {
        if (e instanceof Refusal) fail(e.message);
        throw e;
      }
      curves.push(Object.assign({ file: file, designation: motor.designation }, stats));
    }
  }

  const out = {
    oracle: 'ThrustCurve.org simulate/analyze/analyze.js at commit 577afa6 (ISC)',
    generator: 'validation/oracles/thrustcurve/analyze_stats.js',
    selection: 'every file crates/hpr-motor/data/thrustcurve/catalog.json stores as passing',
    node: process.version,
    params: {
      burn_time_cutoff: analyze.StdParams.burnTimeCutoff,
      time_epsilon_s: analyze.StdParams.timeEpsilon,
      thrust_epsilon_n: analyze.StdParams.thrustEpsilon,
    },
    curves: curves,
  };
  process.stdout.write(JSON.stringify(out, null, 2) + '\n');
}

function readJson(file) {
  try {
    return JSON.parse(fs.readFileSync(file, 'utf8'));
  } catch (e) {
    fail('cannot read ' + file + ' (' + e.message + ')');
  }
}

// Whether a file's figures pass the 1% check against its record: |file - record| <= 0.01 x
// record for total impulse, average thrust and burn time, a record figure that isn't a positive
// number failing.
function passes(figures, record) {
  const stats = figures;
  return [
    [stats.total_impulse_ns, record.totImpulseNs],
    [stats.average_thrust_n, record.avgThrustN],
    [stats.burn_time_s, record.burnTimeS],
  ].every(([file, stated]) =>
    typeof stated === 'number' && stated > 0 && Math.abs(file - stated) <= 0.01 * stated);
}

// The recount: which files pass, from the raw snapshot answers and ThrustCurve's code alone,
// against what catalog.json stores. The index's check reads two samples at one time as a step,
// so the recount's verdict is the one with every sample kept; where ThrustCurve's standard 50 us
// merge gives the other verdict, the file is printed with both.
function recount(analyze, catalog) {
  const captured = catalog.snapshot && catalog.snapshot.captured;
  if (typeof captured !== 'string' || !/^\d{4}-\d{2}-\d{2}$/.test(captured)) {
    fail('catalog.json names no snapshot date');
  }
  const dir = path.join(SNAPSHOTS_DIR, captured);
  const search = readJson(path.join(dir, 'search.json'));
  if (!Array.isArray(search.results) || search.matches !== search.results.length) {
    fail(dir + '/search.json does not hold every match');
  }
  const records = new Map();
  for (const record of search.results) {
    if (record.type === 'SU' || record.type === 'reload') records.set(record.motorId, record);
  }

  // What the index stores for each file: 'passing', 'failing' or 'left out'.
  const stored = new Map();
  for (const motor of catalog.motors) {
    for (const curve of motor.curves) {
      stored.set(curve.simfile_id, curve.check.passes === true ? 'passing' : 'failing');
    }
  }
  for (const entry of catalog.left_out) stored.set(entry.simfile_id, 'left out');

  let answers = 0;
  let listed = 0;
  let read = 0;
  let passing = 0;
  const motorsPassing = new Set();
  const notes = [];
  const disagree = [];
  const seen = new Set();
  for (;;) {
    const name = 'download-' + String(answers).padStart(2, '0') + '.json';
    if (!fs.existsSync(path.join(dir, name))) break;
    answers++;
    for (const entry of readJson(path.join(dir, name)).results || []) {
      const id = entry.simfileId;
      const record = records.get(entry.motorId);
      if (record === undefined) continue;
      listed++;
      seen.add(id);
      const index = stored.get(id) || 'missing';
      if (typeof entry.data !== 'string' || entry.license === 'other' ||
          (entry.format !== 'RASP' && entry.format !== 'RockSim')) {
        if (index !== 'left out') {
          disagree.push(id + ': no bytes the index could hold, ' + index + ' in the index');
        }
        continue;
      }
      let stats;
      try {
        const text = Buffer.from(entry.data, 'base64').toString('utf8');
        stats = statsOf(analyze, pointsOf(text, entry.format, id), id);
      } catch (e) {
        if (!(e instanceof Refusal)) throw e;
        notes.push(id + ' (' + index + ' in the index): refused by the recount: ' + e.message);
        if (index === 'passing' || index === 'missing') {
          disagree.push(id + ': refused by the recount, ' + index + ' in the index');
        }
        continue;
      }
      read++;
      const ours = passes(stats.unmerged || stats, record) ? 'passing' : 'failing';
      const site = passes(stats, record) ? 'passing' : 'failing';
      if (ours === 'passing' && index !== 'left out') {
        passing++;
        motorsPassing.add(entry.motorId);
      }
      if (index === 'left out') {
        notes.push(id + ' (left out in the index): ' + ours + ' by the recount');
      } else if (index !== ours) {
        disagree.push(id + ': ' + ours + ' by the recount, ' + index + ' in the index');
      }
      if (site !== ours) {
        notes.push(id + ' (' + index + ' in the index): ' + ours + ' with every sample kept, ' +
                   site + ' with ThrustCurve\'s 50 us merge of ' + stats.merged_points +
                   (stats.merged_points === 1 ? ' point' : ' points'));
      }
    }
  }
  if (answers === 0) fail('no download answers under ' + dir);
  for (const id of stored.keys()) {
    if (!seen.has(id)) disagree.push(id + ': in the index, not in the download answers');
  }
  const storedPassing = [...stored.values()].filter((v) => v === 'passing').length;
  for (const line of notes) process.stdout.write('  ' + line + '\n');
  process.stdout.write(
    'recount of the raw snapshot ' + captured + ': ' + records.size + ' solid motors, ' + listed +
    ' files in ' + answers + ' download answers; ThrustCurve\'s code reads ' + read + '; of the ' +
    'files the index holds, ' + passing + ' pass the 1% check, and ' + motorsPassing.size +
    ' motors have one that does; catalog.json stores ' + storedPassing + ' as passing\n');
  if (disagree.length > 0) {
    for (const line of disagree) process.stderr.write('disagrees: ' + line + '\n');
    fail(disagree.length + (disagree.length === 1 ? ' file disagrees' : ' files disagree') + ' with catalog.json');
  }
  process.stdout.write('the recount agrees with catalog.json on every file\n');
}

function main() {
  const args = process.argv.slice(2);
  if (args.length > 1 || (args.length === 1 && args[0] !== '--recount')) {
    fail('usage: analyze_stats.js [--recount]');
  }
  const analyze = loadAnalyze();
  const catalog = readJson(CATALOG_PATH);
  if (args.length === 1) {
    recount(analyze, catalog);
  } else {
    fixture(analyze, catalog);
  }
}

main();
