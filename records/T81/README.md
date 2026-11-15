# Native 1998:14A5 condition-refresh slice

[confirmed: archaeology 83b7ef7c27eac15bcc5be1c8173e669d198cda24, T83/T84]
The original routine's contract and controlled cases are in research
docs/condition-refresh.md and records/T84. Native fixtures preserve
calls-r0007.json and calls-r0008.json byte for byte; source.json pins their
SHA256 digests and the accepted research commit. Each repeat contains 21
controlled original calls and 47 actual RNG draws. They are isolated calls,
not naturally reached match/week composition.

[designed:T81] Fail-before is retained at **6ec7154d**. The original-case
integration test compiled and failed on the explicit error "original condition
refresh is not implemented" (exit101; source-identity test passed). The same
test then passed after translation. It compares every output byte/seed,
ordered draw bound/value/seed pair and numeric memory write, including writes
that preserve a value and intermediate recovery-byte wrap/floor writes.

[confirmed: same research source, listing 1998:14A5/11EC/13F9] The native
routine updates p15+7, truncates its candidate to a signed low byte before
clamping, applies the original cap table and conditional age/facility cap,
increments unused-player values after clamping, and conditionally copies
p15+7 into p15+8. The nonpositive branch updates p15+14 and copies only when
its resulting byte is zero. The original shared RNG implementation is reused.

[designed:T81] The isolated typed adapter admits complete explicit inputs;
it accepts no expected result. Session retains the admitted Engine image and
refresh accepts only the live caller flag. Its crate-visible refresh_image
entry reads the live player fields, original cap table, actual club staff
word and staff pool, and seed. It never reloads a fixture or resets the seed.
The isolated adapter maps its declared facility to staff slot zero; the
image kernel resolves arbitrary admitted staff identities through the existing
canonical pools. Consecutive-call and preceding-native-draw tests protect
the live handoff; domain, cap and tracked missing-input checks precede any
player/seed write. Address-resolution controls exercise player3920, club176
and staff1000, and reject staff1001.

```sh
cargo test --manifest-path engine/Cargo.toml --locked --test condition_refresh
cargo test --manifest-path engine/Cargo.toml --locked --lib condition_refresh
cargo test --manifest-path engine/Cargo.toml --locked
cargo test --manifest-path campaign/Cargo.toml --locked
cargo fmt --manifest-path engine/Cargo.toml -- --check
python3 tools/check_originals.py
```

[designed:T81 scope] Campaign and desktop week behavior are unchanged. The
campaign still lacks p15+7, clamp dependencies and the complete post-match
caller RNG handoff. Full 1998:1897, manager-roster processing, persistence,
whole-week parity and timing remain separate work. This routine port does
not enable a weekly refresh by copying T83 endpoints or choosing a fresh seed.
