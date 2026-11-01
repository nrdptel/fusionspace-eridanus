// Reads ghost files back with readers this project didn't write, and holds each format to the
// others (M7.2g, ADR-313):
//
// - JSON: Ajv, against schema/cli/ghost.schema.json.
// - CZML: CesiumJS's own CzmlDataSource. Each position it interpolates, at each moment, is taken
//   into the pad's frame by Cesium's Transforms and held to the JSON's east, north and up.
// - glTF: Khronos's glTF-Validator (no error), then glTF-Transform's reader. Each animation's
//   times and places are held to the JSON's.
//
// Usage: node read_back.mjs SCHEMA FOLDER. Each NAME.json in FOLDER has a NAME.czml and NAME.gltf.
// Prints one JSON line a ghost; exits 1 on any failure.
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { NodeIO } from "@gltf-transform/core";
import { Ajv2020 } from "ajv/dist/2020.js";
import {
  Cartesian3,
  CzmlDataSource,
  JulianDate,
  Matrix4,
  Transforms,
} from "cesium";
import validator from "gltf-validator";

// Cesium's scene code names the browser's Image when it builds a path's material; nothing here
// draws, so an empty stand-in lets the data source load in Node.
globalThis.Image ??= class {};

const [schemaPath, folder] = process.argv.slice(2);
const validate = new Ajv2020({ strict: false, allErrors: true }).compile(
  JSON.parse(readFileSync(schemaPath, "utf8")),
);

// Far under any log's resolution. Cesium's clock and its interpolation came within about 1 mm on
// these logs, where a track moves at 300 m/s; a 32-bit float holds about 2 mm at 30 km, and about
// 0.06 ms at 1,000 s.
const CZML_M = 0.01;
const GLTF_M = 0.01;
const GLTF_S = 0.001;

// What the writers draw where the JSON holds a null: the nearest value before it, or else the
// first there is, or else 0. This is the format's rule, restated here so a change has to be made
// on both sides.
function filled(values) {
  let held = values.find((v) => v !== null) ?? 0;
  return values.map((v) => (held = v ?? held));
}

function drawn(track) {
  const east = filled(track.east_m);
  const north = filled(track.north_m);
  return track.time_s.map((t, i) => [t, east[i], north[i], track.up_m[i] ?? 0]);
}

async function readCzml(ghost, text) {
  const data = await CzmlDataSource.load(JSON.parse(text));
  const { latitude_deg: lat, longitude_deg: lon, height_m: h } = ghost.pad;
  const origin = Cartesian3.fromDegrees(lon ?? 0, lat ?? 0, h ?? 0);
  const toPad = Matrix4.inverse(
    Transforms.eastNorthUpToFixedFrame(origin),
    new Matrix4(),
  );
  const epoch = JulianDate.fromIso8601("2000-01-01T12:00:00Z");
  let moments = 0;
  ghost.tracks.forEach((track, index) => {
    const entity = data.entities.getById(`track-${index + 1}`);
    if (!entity) throw new Error(`CZML: no track-${index + 1}`);
    if (entity.name !== track.name) throw new Error(`CZML: its name is ${entity.name}`);
    const points = drawn(track);
    // Every moment on a short track; on a long one, a stride that still lands on the ends.
    const stride = Math.max(1, Math.floor(points.length / 2000));
    for (let i = 0; i < points.length; i += i + stride < points.length || i === points.length - 1 ? stride : points.length - 1 - i) {
      const [t, east, north, up] = points[i];
      const at = JulianDate.addSeconds(epoch, t, new JulianDate());
      if (!entity.isAvailable(at)) throw new Error(`CZML: ${track.name} not available at ${t} s`);
      const place = entity.position.getValue(at);
      if (!place) throw new Error(`CZML: ${track.name} has no position at ${t} s`);
      const enu = Matrix4.multiplyByPoint(toPad, place, new Cartesian3());
      const off = Math.hypot(enu.x - east, enu.y - north, enu.z - up);
      if (!(off <= CZML_M)) {
        throw new Error(`CZML: ${track.name} at ${t} s is ${off} m from the JSON's`);
      }
      moments += 1;
      if (i === points.length - 1) break;
    }
  });
  return moments;
}

async function readGltf(ghost, text) {
  const bytes = new TextEncoder().encode(text);
  const report = await validator.validateBytes(bytes, { uri: "ghost.gltf" });
  if (report.issues.numErrors > 0) {
    const first = report.issues.messages.find((m) => m.severity === 0);
    throw new Error(`glTF-Validator: ${report.issues.numErrors} errors, first ${JSON.stringify(first)}`);
  }
  const document = await new NodeIO().readJSON({
    json: JSON.parse(text),
    resources: {},
  });
  const [animation] = document.getRoot().listAnimations();
  const channels = animation.listChannels();
  if (channels.length !== ghost.tracks.length) {
    throw new Error(`glTF: ${channels.length} channels for ${ghost.tracks.length} tracks`);
  }
  const zero = Math.min(...ghost.tracks.map((t) => t.time_s[0]));
  let moments = 0;
  ghost.tracks.forEach((track, index) => {
    const sampler = channels[index].getSampler();
    const times = sampler.getInput().getArray();
    const places = sampler.getOutput().getArray();
    const points = drawn(track);
    if (channels[index].getTargetPath() !== "translation") throw new Error("glTF: not a translation");
    let row = 0;
    for (let i = 0; i < times.length; i += 1) {
      const t = times[i] + zero;
      while (row < points.length - 1 && Math.abs(points[row][0] - t) > GLTF_S) row += 1;
      const [time, east, north, up] = points[row];
      if (Math.abs(time - t) > GLTF_S) throw new Error(`glTF: ${track.name} has a moment ${t} s the JSON hasn't`);
      const off = Math.hypot(places[3 * i] - east, places[3 * i + 1] - up, places[3 * i + 2] + north);
      if (!(off <= GLTF_M)) throw new Error(`glTF: ${track.name} at ${t} s is ${off} m from the JSON's`);
      moments += 1;
    }
    if (times.length < points.length * 0.99) {
      throw new Error(`glTF: ${track.name} kept ${times.length} of ${points.length} moments`);
    }
  });
  return { moments, warnings: report.issues.numWarnings };
}

let failed = 0;
const names = readdirSync(folder).filter((f) => f.endsWith(".json")).sort();
if (names.length === 0) {
  console.error(`no ghost files in ${folder}`);
  process.exit(1);
}
for (const file of names) {
  const stem = file.slice(0, -".json".length);
  try {
    const ghost = JSON.parse(readFileSync(join(folder, file), "utf8"));
    if (!validate(ghost)) throw new Error(`Ajv: ${JSON.stringify(validate.errors[0])}`);
    const czml = await readCzml(ghost, readFileSync(join(folder, `${stem}.czml`), "utf8"));
    const gltf = await readGltf(ghost, readFileSync(join(folder, `${stem}.gltf`), "utf8"));
    console.log(JSON.stringify({ file: stem, czml_moments: czml, gltf_moments: gltf.moments, gltf_warnings: gltf.warnings }));
  } catch (error) {
    failed += 1;
    console.log(JSON.stringify({ file: stem, error: String(error.message ?? error) }));
  }
}
process.exit(failed === 0 ? 0 : 1);
