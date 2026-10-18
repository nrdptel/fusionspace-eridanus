"""Checks that a Windows program needs no Visual C++ runtime, which a clean Windows may lack.

Release 0.1.0's `hpr.exe` imported VCRUNTIME140.dll, so on a Windows without the Visual C++
Redistributable it wouldn't start. From release 0.2 scripts/release/archive.sh links the C runtime
in (`-C target-feature=+crt-static`), and this check, run by scripts/release/smoke-cli.sh on each
Windows archive, fails if the program still imports a DLL that only the Redistributable (or a
Visual Studio install) provides: VCRUNTIME*, MSVCP* (the C++ library), MSVCR* (the C runtime
before 2015), CONCRT*, VCCORLIB*, VCOMP* (OpenMP), VCAMP*, MFC*, or the debug UCRTBASED.dll.
Windows' own msvcrt.dll and the Universal C Runtime's `api-ms-win-crt-*` names are allowed:
Windows 10 and later carry them.

It reads the program's import table and its delay-load import table as Microsoft's PE format page
lays them out (https://learn.microsoft.com/windows/win32/debug/pe-format), prints each DLL named,
and fails closed: on a file it can't read as a 64-bit (PE32+) program, since no 32-bit Windows
target ships; on the old delay-load form, which holds addresses rather than RVAs; and on a file it
reads no imports from.

Python 3 standard library only.

Usage: python check_windows_runtime.py <program.exe>
"""

import re
import struct
import sys
from pathlib import Path

# The DLLs only the Visual C++ Redistributable, or Visual Studio, installs. `msvcr` needs a digit
# after it, so Windows' own msvcrt.dll isn't one.
REDISTRIBUTABLE = re.compile(
    r"(vcruntime|msvcp|msvcr|concrt|vccorlib|vcomp|vcamp|mfc)\d.*\.dll|ucrtbased\.dll",
    re.IGNORECASE,
)


class Unreadable(Exception):
    pass


def unpack(fmt, data, offset):
    size = struct.calcsize(fmt)
    if offset < 0 or offset + size > len(data):
        raise Unreadable(f"{size} bytes at {offset:#x} run past the file's end")
    return struct.unpack_from(fmt, data, offset)


def imports(data):
    """The DLL names in the program's import and delay-load import tables, in table order."""
    if data[:2] != b"MZ":
        raise Unreadable("no MZ header")
    (pe,) = unpack("<I", data, 0x3C)
    if data[pe : pe + 4] != b"PE\0\0":
        raise Unreadable("no PE signature")
    sections, optional_size = unpack("<H12xH", data, pe + 6)
    optional = pe + 24
    (magic,) = unpack("<H", data, optional)
    if magic != 0x20B:  # PE32+; 0x10B is 32-bit PE32
        raise Unreadable(f"not a 64-bit PE: optional header magic {magic:#x}")
    if optional_size < 112:
        raise Unreadable(f"an optional header of {optional_size} bytes holds no directories")
    (count,) = unpack("<I", data, optional + 108)
    directories = optional + 112
    # Only the directories the header has room for; past it the section table begins.
    count = min(count, (optional_size - 112) // 8)
    table = optional + optional_size
    spans = []
    for i in range(sections):
        virtual_size, rva, raw_size, raw = unpack("<4I", data, table + 40 * i + 8)
        spans.append((rva, max(virtual_size, raw_size), raw, raw_size))

    def offset(rva):
        for start, size, raw, raw_size in spans:
            if start <= rva < start + size:
                if rva - start >= raw_size:
                    raise Unreadable(f"RVA {rva:#x} has no bytes in the file")
                return raw + rva - start
        raise Unreadable(f"RVA {rva:#x} is in no section")

    def name(rva):
        at = offset(rva)
        end = data.find(b"\0", at)
        if end < 0:
            raise Unreadable(f"the name at RVA {rva:#x} has no end")
        return data[at:end].decode("ascii", errors="replace")

    def directory(index):
        return unpack("<2I", data, directories + 8 * index) if index < count else (0, 0)

    found = []
    rva, _ = directory(1)
    if rva:
        at = offset(rva)
        while True:
            descriptor = unpack("<5I", data, at)
            if not any(descriptor):
                break
            found.append(name(descriptor[3]))
            at += 20
    rva, _ = directory(13)
    if rva:
        at = offset(rva)
        while True:
            descriptor = unpack("<8I", data, at)
            if not any(descriptor):
                break
            attributes, dll = descriptor[0], descriptor[1]
            # Attribute bit 0 clear is the old form, which holds addresses, not RVAs; a 64-bit
            # linker doesn't write it.
            if not attributes & 1:
                raise Unreadable("the old delay-load form, which holds addresses")
            found.append(name(dll))
            at += 32
    return found


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__.rstrip().rsplit("\n", 1)[-1])
    path = Path(sys.argv[1])
    try:
        names = imports(path.read_bytes())
    except Unreadable as error:
        sys.exit(f"check_windows_runtime: {path}: not a Windows program it can read ({error})")
    if not names:
        sys.exit(f"check_windows_runtime: {path}: no imports read, so nothing was checked")
    print(f"check_windows_runtime: {path.name} imports {', '.join(names)}")
    needed = [n for n in names if REDISTRIBUTABLE.fullmatch(n)]
    if needed:
        for n in needed:
            print(
                f"check_windows_runtime: {path.name} needs {n}, from the Visual C++ "
                "Redistributable; build it with -C target-feature=+crt-static",
                file=sys.stderr,
            )
        sys.exit(1)
    print(f"check_windows_runtime: {path.name} needs no Visual C++ runtime")


if __name__ == "__main__":
    main()
