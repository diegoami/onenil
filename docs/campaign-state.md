# Native campaign state

[designed: owner continuation after desktop inspection,2026-11-06] T55 implements
the toolkit-independent B1 save core. T56 connects it to the existing prototype.
Continuing that prototype does not close owner toolkit decision#103 or waive the
milestone118 review. No production toolkit choice is inferred from "looks good".

[confirmed] Source is research T65 at39e289a393d12ce562ee8b9fba16b413fa17647e,
`runs/T65/seed-r0001.json`, producer4189661. `campaign/fixtures/source.json` pins
save/league/record/projection identities. `campaign/import_seed.py` compares the
tracked decoded source with the exact upstream object; no private bytes are
required for native checks. Record SHA256 ebafc57db43544adcf13d3c002f372e587d8cc201e4a1ae8aa1a74dd37e31483.

The initial state is the actual supplied96 SAVE1 Blackburn130, Dalglish130,
season1994/95, current week0, cash452000 and its next home friendly against
P St Germain17. Twenty first-team and twenty reserve slots retain18 real player
IDs, names/current and club positions/packed instructions/observed availability.
This namespace is ENGL-95; the later Sheffield exhibition must not borrow these
identities/names merely because numeric IDs coincide. The source changed-save
projection is a test assertion only; initialization reads the initial projection.

[designed] Native JSON schema `onenil-campaign`, version1, is independent of both
DOS `.1-0` saves and the old `onenil-captured-editor` experiment. All nested objects
reject unknown fields. Source/namespace/club/manager/season/week/pending fixture
are validated against this bounded profile. Player records and first/reserve
slots must contain exactly the source player set, without duplicate/foreign IDs;
canonical names/club positions remain identified. Current positions use the14
observed role codes. Packed instructions/availability are persisted observations,
not a newly simulated tactics or eligibility model. Incomplete first elevens can
be saved; match admission is a later engine guard, not an invented save refusal.

Exchanges/position/instruction changes validate a candidate before commit and
advance the native revision once per actual change. Cancellation belongs to the
UI/controller; a pending carried player is not part of saved campaign state.
Decoding validates everything before replacing live state. State/result/week
progression outside the one pending fixture is unsupported rather than fabricated.

Saving validates first, writes a new exclusive file in the target directory,
flushes/syncs, then atomically renames it over the target. Pre-rename failure leaves
the committed save unchanged. RAII cancels an abandoned staged write. A killed
process may leave a staged file: loading never promotes that orphan automatically.
A post-rename directory-sync failure is reported as committed with a durability
warning; it is not reported as preserving the old file. Linux interruption and
failure controls are checked; other platform/power-loss guarantees remain unproved.

Commands (Rust required):

```sh
cargo test --manifest-path campaign/Cargo.toml --locked
cargo run --manifest-path campaign/Cargo.toml -- new /tmp/blackburn.json
cargo run --manifest-path campaign/Cargo.toml -- inspect /tmp/blackburn.json
cargo run --manifest-path campaign/Cargo.toml -- swap /tmp/blackburn.json first:0 first:17
```

The headless app also has `position FILE ID VALUE`, `instruction FILE ID VALUE`,
`project FILE` and validated `load FILE CANDIDATE`. `stage FILE SOURCE DEST` is an
explicit persistence diagnostic, waiting for `commit` on stdin after sync; normal
saves do not pause. Fresh-process proof: `python3 campaign/verify.py --record
records/T55/<new-name>.json`. It checks complete original initial/changed projections,
invalid-file preservation and a killed process before replacement.

T55 does not yet change the displayed desktop editor, admit an edited campaign XI,
play the PSG fixture, update finances/week/table, migrate old files or provide DOS
save compatibility. T56's bounded UI bridge comes next; B4/B5 remain engine and
campaign transitions requiring their own original evidence.
