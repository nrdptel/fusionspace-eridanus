# shellcheck shell=bash
# The oldest Linux a release's Linux files run on, and the pinned tools that build and check them
# for it (issue #372). Sourced by archive.sh, python.sh, check-glibc.sh and smoke-manylinux.sh.
#
# The floor is glibc 2.17, the manylinux2014 policy (PEP 599), which is also the oldest glibc Rust's
# x86_64- and aarch64-unknown-linux-gnu targets support. Zig links against its own copy of that glibc's
# symbols, so a build on a new runner needs no newer one: cargo-zigbuild builds the command line
# for the `<target>.2.17` target, and maturin builds the wheel with `--zig --compatibility
# manylinux2014`, refusing it if it calls a newer symbol. Both come from PyPI through uv, which
# installs Zig from PyPI's `ziglang` package beside them.

# shellcheck disable=SC2034 # each script uses some of these
GLIBC_FLOOR="2.17"
MANYLINUX="manylinux2014"
# The wheel's platform tag for the floor, as maturin writes it first in the file name.
MANYLINUX_TAG="manylinux_2_17"
CARGO_ZIGBUILD="0.23.4"
ZIGLANG="0.16.0"
# The image the files are smoke-tested in: pypa's manylinux2014, CentOS 7 with glibc 2.17 and
# CPython 3.9 and later under /opt/python/cp3XX-cp3XX, for the machine's own architecture (x86-64
# or 64-bit ARM, ADR-222). Pinned by digest; the tag names the build.
case "$(uname -m)" in
  aarch64 | arm64)
    MANYLINUX_PLATFORM="linux/arm64"
    MANYLINUX_IMAGE="quay.io/pypa/manylinux2014_aarch64:2026.10.03-1"
    MANYLINUX_IMAGE+="@sha256:9026a55e05e76ad74d9a4e6be814a5abda26cde08b1baa659ea4f7ee16b246e3"
    ;;
  *)
    MANYLINUX_PLATFORM="linux/amd64"
    MANYLINUX_IMAGE="quay.io/pypa/manylinux2014_x86_64:2026.10.03-1"
    MANYLINUX_IMAGE+="@sha256:ffd6d1f11237599748657996b750db6b3a5b724fea5a81cd9ea65add4f45b6c9"
    ;;
esac
