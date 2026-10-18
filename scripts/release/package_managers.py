"""Writes the Homebrew formula and the Scoop manifest for a release's `hpr` archives (issue #434).

Given the version and a folder holding the archives scripts/release/archive.sh packs
(`fusionspace-hpr-<version>-<target>.tar.gz`, or `.zip` on Windows), it writes
`fusionspace-hpr.rb` and `fusionspace-hpr.json` into the output folder. Each archive holds one
folder, `fusionspace-hpr-<version>-<target>/`, with `hpr` (`hpr.exe` on Windows) in it. Only the
targets whose archive is there get an entry; an archive for a target neither file knows stops the
script, so a new target is added here on purpose rather than left out quietly.

The release workflow attaches both files to the release. The tap (nrdptel/homebrew-tap) and the
bucket (nrdptel/scoop-bucket) each copy the latest release's file into themselves on a schedule,
so neither needs a credential for this repository, nor this repository one for them.

Python 3 standard library only.

Usage: python package_managers.py <version> <archives folder> <output folder>
"""

import hashlib
import json
import re
import sys
from pathlib import Path

REPO = "https://github.com/nrdptel/fusionspace-eridanus"
HOMEPAGE = "https://hpr.fusionspace.co"
DESCRIPTION = "Flight simulator for hobby and high-power rockets"

# Each target the release workflow builds, as (operating system, architecture) in Homebrew's
# words, or as the architecture key of Scoop's manifest.
BREW = {
    "aarch64-apple-darwin": ("macos", "arm"),
    "x86_64-apple-darwin": ("macos", "intel"),
    "aarch64-unknown-linux-gnu": ("linux", "arm"),
    "x86_64-unknown-linux-gnu": ("linux", "intel"),
}
SCOOP = {
    "x86_64-pc-windows-msvc": "64bit",
    "aarch64-pc-windows-msvc": "arm64",
}
# What Homebrew's `depends_on arch:` calls each architecture, for an operating system that has
# only one of the two.
BREW_ARCH = {"arm": ":arm64", "intel": ":x86_64"}


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as file:
        for block in iter(lambda: file.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def url(version, name):
    return f"{REPO}/releases/download/v{version}/{name}"


def archives(version, folder):
    """The archives in `folder` for `version`, as {target: (file name, sha256)}."""
    pattern = re.compile(rf"fusionspace-hpr-{re.escape(version)}-(.+?)\.(tar\.gz|zip)")
    found = {}
    for path in sorted(folder.iterdir()):
        match = pattern.fullmatch(path.name)
        if not match:
            continue
        target, kind = match.groups()
        if target not in BREW and target not in SCOOP:
            sys.exit(f"package_managers: {path.name}: no formula or manifest entry for {target}")
        if (kind == "zip") != (target in SCOOP):
            sys.exit(f"package_managers: {path.name}: a {target} archive is a .zip on Windows only")
        found[target] = (path.name, sha256(path))
    if not found:
        sys.exit(f"package_managers: no fusionspace-hpr-{version}-<target> archives in {folder}")
    return found


def formula(version, found):
    by_os = {}
    for target, (os_name, arch) in BREW.items():
        if target in found:
            by_os.setdefault(os_name, {})[arch] = found[target]
    if not by_os:
        return None
    lines = [
        "# Written by scripts/release/package_managers.py in nrdptel/fusionspace-eridanus from the",
        "# release's archives; a release replaces it.",
        "class FusionspaceHpr < Formula",
        f'  desc "{DESCRIPTION}"',
        f'  homepage "{HOMEPAGE}"',
        f'  version "{version}"',
        '  license any_of: ["MIT", "Apache-2.0"]',
    ]
    if len(by_os) == 1:
        lines += ["", f"  depends_on :{next(iter(by_os))}"]
    for os_name in ("macos", "linux"):
        arches = by_os.get(os_name)
        if not arches:
            continue
        lines += ["", f"  on_{os_name} do"]
        if len(arches) == 1:
            arch = next(iter(arches))
            lines.append(f"    depends_on arch: {BREW_ARCH[arch]}")
            lines.append("")
        for i, arch in enumerate(a for a in ("arm", "intel") if a in arches):
            name, digest = arches[arch]
            if i:
                lines.append("")
            lines += [
                f"    on_{arch} do",
                f'      url "{url(version, name)}"',
                f'      sha256 "{digest}"',
                "    end",
            ]
        lines.append("  end")
    lines += [
        "",
        "  def install",
        '    bin.install "hpr"',
        "  end",
        "",
        "  test do",
        '    assert_match version.to_s, shell_output("#{bin}/hpr --version")',
        "  end",
        "end",
    ]
    return "\n".join(lines) + "\n"


def manifest(version, found):
    architecture = {}
    autoupdate = {}
    for target, key in SCOOP.items():
        if target not in found:
            continue
        name, digest = found[target]
        folder = f"fusionspace-hpr-{version}-{target}"
        architecture[key] = {"url": url(version, name), "hash": digest, "extract_dir": folder}
        autoupdate[key] = {
            "url": url("$version", f"fusionspace-hpr-$version-{target}.zip"),
            "extract_dir": f"fusionspace-hpr-$version-{target}",
        }
    if not architecture:
        return None
    document = {
        "version": version,
        "description": f"{DESCRIPTION}: the hpr command line.",
        "homepage": HOMEPAGE,
        "license": "MIT|Apache-2.0",
        "architecture": architecture,
        "bin": "hpr.exe",
        "checkver": {"github": REPO},
        "autoupdate": {"architecture": autoupdate},
    }
    return json.dumps(document, indent=4) + "\n"


def main():
    if len(sys.argv) != 4:
        sys.exit(__doc__.rstrip().rsplit("\n", 1)[-1])
    version, folder, out = sys.argv[1], Path(sys.argv[2]), Path(sys.argv[3])
    if not re.fullmatch(r"\d+\.\d+\.\d+(-[0-9A-Za-z.]+)?", version):
        sys.exit(f"package_managers: {version!r} isn't a release version")
    found = archives(version, folder)
    out.mkdir(parents=True, exist_ok=True)
    for name, text in (
        ("fusionspace-hpr.rb", formula(version, found)),
        ("fusionspace-hpr.json", manifest(version, found)),
    ):
        if text is None:
            sys.exit(f"package_managers: no archive in {folder} for {name}")
        # Unix line ends on every system, so the files are the same bytes wherever they're written.
        (out / name).write_text(text, encoding="utf-8", newline="\n")
        print(out / name)


if __name__ == "__main__":
    main()
