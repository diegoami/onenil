# T74 Owned native halftime boundary

[confirmed: archaeology011a341,7DD3/7E4A and E061/E064] The native prepared
session can now pause before the original halftime bench call, retaining the
same guest memory, stack and caller frames. `pause_at_interval()` reaches that
boundary; `interval_observation()` reads it without advancing; `finish()` resumes
the owned frames and reaches the actual outer return. The existing monolithic
finish path is retained. No original observation is accepted as continuation state.

[designed] Duplicate/out-of-phase pause, prefix advance while paused and further
execution after completion/failure refuse. A genuine source fault remains a
failure, not a successful yield. New CPU yield guards require the original
interval control, positive remaining time, no pending tick and retained caller
chain. Two generated continuation roots reuse the existing native procedures;
there is no opcode runtime or manually reconstructed halftime output.

`before-r0001.json` retains the missing-API compilation regression. The focused
Rust test exercises both original and edited campaign profiles (including an
already advanced171-tick prefix), read-only/phase guards and missing inputs.
It compares the complete output with each existing monolithic session. A separate
release probe compares paused continuation directly with every tick/draw/action,
final field and actual return in the accepted original reference streams.

```sh
cargo test --manifest-path engine/Cargo.toml --release --locked --test live_interval
python3 tools/generate_interval.py --research-root ../onenil-archaeology --check
cargo build --manifest-path engine/Cargo.toml --release --locked --example interval_probe
python3 tools/check_interval.py --probe engine/target/release/examples/interval_probe
```

`generation-r0001.json` binds152 source instructions in the two roots, the
original listing hashes and reused native modules. CI fetches that accepted
public listing revision, regenerates deterministically and runs direct original
parity in addition to the existing engine/source/desktop gates.

[designed] This is an engine prerequisite. The desktop still computes a complete
timeline before playback. No substitution controls or new lineup/mutation
admissions are enabled here. Worker streaming/ownership and the source-witnessed
replacement transaction require separately bounded follow-on tasks.
