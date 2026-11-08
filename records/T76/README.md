# T76 — live halftime worker transport

[derived:research88102d2/native15423ea] The accepted engine retains an actual
7E4A interval and performs the verified fresh replacement. This task transports
its already computed history while retaining that same engine in a live child.

[designed:T76] `LiveJob::spawn_campaign` returns an interval before child exit.
`IntervalTimeline` has no completed-return field; a separate `Completion` holds
the actual finished timeline and computed native-result digest. The child waits
for one Continue or ReplaceVerified decision on its local command pipe. Continue
supports both admitted campaign profiles; replacement keeps T75's fresh-only
admission. No expected game output enters a request.

Packets use a bounded length-prefixed JSON protocol, version, job identity,
canonical campaign digest and ordered phase. Parent validation binds the fixture,
markers and complete first-half prefix. Extra/truncated/oversized/unknown frames,
foreign identities, early/duplicate decisions and an incomplete completed result
refuse. A completed packet is delivered only after EOF verification and child
reaping. Cancellation/drop also reaps an interval-waiting child and joins the
reader. The bounded reader cannot deadlock on a full sender queue.

Original timeline arithmetic and narration remain shared with the old complete
adapter. Engine export is read-only and does not claim an outer return. Tests
compare complete original/edited continuations with their existing timelines,
replacement with T75's independently retained native digest, and reached
history/markers before and after decisions. Real-child raw-protocol tests cover
foreign commands and queued duplicates, with bounded waits and RAII cleanup.

Frozen main dc696f3; missing-API regression846e0e4 failed with E0432. The before
receipt is immutable. Full local checks pass:549engine results and52desktop/
raster results, including the new lifecycle, protocol and history checks. The
real-process probe additionally verifies three completions and cancel/drop at
interval. Later proof/validation receipts bind a clean producer and binaries.
Exact-head CI remains required before pinned squash.

```sh
cargo test --manifest-path desktop/faithful/Cargo.toml --release --locked --test live_worker --test live_protocol
cargo test --manifest-path engine/Cargo.toml --release --locked --test interval_history
cargo build --manifest-path desktop/faithful/Cargo.toml --release --locked --example live_worker_probe --bin onenil-desktop
python3 tools/check_live_worker.py --probe desktop/faithful/target/release/examples/live_worker_probe --worker desktop/faithful/target/release/onenil-desktop
# A clean producer can retain a fresh receipt with --record NEW_PATH.
```

**Limits:** backend transport only. The application/controller still use the old
complete worker; no desktop substitution controls or saved roster changes are
enabled. Subsequent integration must retain the child across pitch/bench/history,
consume the reached tail between halftime whistle and actual interval, and keep
existing exactly-once score tickets. Original post-match roster persistence needs
source authentication before saved substitutions are enabled. No original
substituted full-match parity or Windows-specific runtime proof is claimed.
Owner demo windows and unsaved state are preserved. No original capture or
private artifact upload is needed by this task.
