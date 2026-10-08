# ADR-117: M5.1 split; the cache and offline mode (2026-09-29)

- **Status:** accepted
- **Summary:** M5.1 split a and b; M5.1a: the cache, TTL freshness and an offline mode that never calls the transport

**Context.** M5.1's *done when*: tests run against recorded fixtures, and offline mode never touches
the network, asserted by a test. The cache and the offline rule don't depend on HTTP, and the HTTP
transport brings the first network dependency with its `cargo deny` review, so the two are
split to keep each pull request small.

**Decision.**

1. Split M5.1 into M5.1a (the cache, TTLs, the offline mode and attribution over a `Transport`
   trait) and M5.1b (a `ureq` transport with rustls behind a feature, and the platform cache
   directory).
2. `Client::fetch(source, url, now_s)` takes the time as an argument, so tests are deterministic
   and the crate reads no clock. A copy is fresh while `now_s - fetched_at_s < ttl_s`; one exactly
   a TTL old is refetched online.
3. Offline returns any cached copy, marked `Stale` past its TTL, and never calls the transport;
   with no copy it is `NotCached`. Online, a failed fetch with an old copy returns the copy as
   `Stale` with the transport's reason in `stale_reason`, since stale weather with a label beats
   no flight; with no copy the error comes through. A copy dated after `now_s` is never fresh.
   Online, an unreadable entry is a miss and the fetch overwrites it; offline it is the error.
4. The cache stores `<key>.bin` and `<key>.json` per URL, the key being 64-bit FNV-1a of the URL.
   The metadata keeps the URL, so a hash collision reads as a miss. Each file is written to a
   temporary name unique to the process and the call, then renamed, the metadata last.
5. A `Replay` transport reads recorded responses from a folder's `index.json` and counts its calls.
   Its committed fixture is a hand-written body: the cache doesn't read content, and real recorded
   responses come with each source from M5.2, with their licences in `THIRD-PARTY-NOTICES.md`.

**Consequences.** `tests/offline.rs` asserts offline mode with a transport that panics if called,
before and after the cache is filled, fresh and 30 days stale. No pruning and no locking yet:
two writers of one URL leave whole files, but possibly one's body beside the other's fetch time.
