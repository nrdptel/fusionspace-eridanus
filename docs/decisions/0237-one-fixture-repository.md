# ADR-237: One fixture repository: the pin, the labels and a file two packages install (2026-10-11)

- **Status:** accepted; closes M2.3c4
- **Summary:** The reference lock pins only the private flight collection, at its `master` merge commit `7268bcd0`, with a second manifest pin for the design library copied inside it. The two older private repositories are read from its byte-exact `imported/` copies, and the survey counts the library under that path. The OpenRocket round-trip report was flown again because its record predated a writer change. The Python environment check now accepts a file two packages install when it holds either package's copy.

**Context.** M2.3c4's *done when*: the lock pins only `hpr-sim-fixtures`, at its catalog's merge
commit, the other two read from its `imported/` copies; `cargo xtask ork` finds the same files;
`cargo xtask refs verify` passes. Moves: nothing. The collection's catalog arrived in two merged
pull requests: the catalog (`3c555683`) and its pairing pass (`7268bcd0`, `master`'s head). Its
handoff names what moves: the pin, two manifest hashes, three path constants and two reports.

**Decisions.**

1. **The pin is `7268bcd0`,** the merge of the catalog's last pull request and `master`'s head.
   It holds everything the first merge did plus the pairing pass, and the handoff checks HPR's
   reads against `master`. The root manifest's pin moves to the one `master` has carried since the
   collection's first pull request (9,557 entries); the files HPR reads are unchanged.
2. **The copies are checked as copies.** Each `imported/` directory has the same git tree id as
   its original's pinned commit (`4fbf2e48` and `3d137c6f`), so every byte matches. The design
   library keeps its own manifest, pinned at the hash Loft used. A lock entry now takes a list
   of manifests, and each manifest's paths run from its own directory, so a repository copied
   whole into a subdirectory verifies with its manifest unchanged.
3. **Labels name where the files are.** The survey and the flight reports count the library
   under `refs/hpr-sim-fixtures/imported/loft-fixtures`, not the retired `refs/loft-fixtures`.
   The default survey reads the two `imported/` copies and still leaves out the rest of the
   collection. It also skips any leftover checkout of the retired repositories, so an old clone
   can't count a file twice. The gitignored OpenRocket records name each design by path and
   match it by sha256; since the copies are byte-exact, their paths were relabelled rather than
   flown again.
4. **The round-trip report was flown again.** `openrocket-export-flights` compares OpenRocket's
   flight of each design with its flight of the file HPR writes back out. Its record of the
   exports dated from 2026-09-27, before release 0.1's rename changed the `creator` attribute HPR
   writes, so its own check refused it and it could not be relabelled. Written again and flown
   again, every export still opens and flies within the 0.5% bar. The same apogee to the last bit
   now holds for 143 configurations of 151, not 142, and the largest difference is 0.000191%, not
   0.000162%. That is a fresh measurement of today's writer, not a change this milestone made;
   no physics moved.
5. **A file two packages install.** `refs verify` re-hashes each installed Python file against
   its package's `RECORD`. Two packages in the oracle environment both install a top-level
   `LICENSE`, so the file holds the copy installed last and could never match both. The check
   now passes such a file when it matches one of the packages that list it, and still fails a
   file that matches none. A test runs the check's own Python code on stand-in packages, and a
   mutation back to the old rule fails it.

**Consequences.** Commands that run the OpenRocket oracle scripts over the default corpus name the
two `imported/` directories after `refs`, because the scripts don't follow the symlink a local
setup makes of `refs/hpr-sim-fixtures`. The older repositories can be archived (a "Needs Neer"
item). Nothing in the simulator changed.
