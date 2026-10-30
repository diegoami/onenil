# onenil

A faithful rebuild of *One-Nil* (Wizard, 1995/1996), a football club manager, from the evidence of
[onenil-archaeology](https://github.com/diegoami/onenil-archaeology). The engine will be Rust,
checked against the original through a Python harness. No byte of the original game is in this
repository; see [docs/artifacts.md](docs/artifacts.md). Rules: [CLAUDE.md](CLAUDE.md). Process:
[docs/process.md](docs/process.md).

The first slice is `0AB9:D983`, the quick result's three-contest routine. Its scope is the input
ratings, random draw sequence, final seed and goal bytes; it is not a complete match/week or a
timer/CPU-register emulator. Evidence is pinned under `engine/tests/fixtures/`: decoded original
routine cases from onenil-archaeology T11, linked to the 15 T08 traces (six distinct weeks).
The [F11 contract and source fixtures](https://github.com/diegoami/onenil-archaeology/tree/78bdebc10d0051fa52b080a72312d894bfab5560/traces/T11)
are pinned to archaeology commit `78bdebc10d0051fa52b080a72312d894bfab5560`; the local provenance
file also pins the copied index's SHA-256.

The engine uses a DS memory image with explicit little-endian word access and byte wrapping.
`[confirmed]` D983's chance loop is unsigned, its three draw comparisons are signed 16-bit and ties
lose, while the RNG's product is unsigned (archaeology F02/F07/F09 at `7859d7d`; T11/F11 records
the routine inputs and outputs). Timer writes and scratch/stack state are outside this slice.

Build and check from a Python 3.12-or-newer virtual environment with Rust installed:

```sh
python3 -m venv .cache/venv
.cache/venv/bin/python -m pip install maturin==1.16.0
cargo test --manifest-path engine/Cargo.toml --locked
. .cache/venv/bin/activate
maturin develop --manifest-path engine/Cargo.toml --features python --release --locked
python3 tools/replay_contests.py --check
cargo fmt --manifest-path engine/Cargo.toml -- --check
python3 tools/check_originals.py
```

The Rust tests and Python-to-Rust replay check every recorded draw, including its site, argument,
seed transition and returned value; mismatches report their first call and draw. The Python tool
contains no engine formulas. Fixture file hashes and the pinned archaeology index/commit are checked
before the bridge replay.

Measure this routine slice after committing its code and fixtures:

```sh
python3 tools/replay_contests.py --benchmark --out records/T01
```

`[designed]` Each benchmark writes a new immutable receipt, including code/platform, fixture pin,
repetitions and elapsed time. It includes DS image setup, Python/Rust calls, trace collection and
output serialization. It has no speed threshold and says nothing about full-match or week speed.
The receipt identifies the loaded extension by SHA-256 and records its compiled debug-assertion
setting; the binary itself is never committed.

The next slice is goal placement (`0AB9:DD7C..DF35`), including native availability and
first-roster-match helpers. It consumes complete rosters and eligibility fields for every
recorded entry, including entries that produced no goal. The Rust API receives inputs only;
expected minute/scorer decisions, draws and goal outcomes never steer the algorithm.
`[confirmed]` F12 at `77841e9` specifies repeated fills of both minute tables, residual generated
counts, the cap of 15, and the at-most-one-goal limit. T13/F13 supplies compact original cases.

`[designed]` The `5083` projection computes its verified single `random(2)` draw and emits a
goal-effect request with the raw draw and the wrapper's boolean coin. Statistics, scorer logs
and display effects remain external. This slice does not claim a complete DD02 implementation,
whole-memory/timer equality or whole-match/week parity.

After building the same release bridge:

```sh
python3 tools/replay_placement.py --check
python3 tools/replay_placement.py --benchmark --out records/T02
```

The placement fixtures and separate original helper checks live under
`engine/tests/fixtures/placement/`; their index and provenance pin the archaeology source.
The [F13 input contract](https://github.com/diegoami/onenil-archaeology/blob/a39ac703c09f0981e073d471d61f7bce0f3642e7/findings/F13-goal-placement-inputs.md)
and [source fixtures](https://github.com/diegoami/onenil-archaeology/tree/a39ac703c09f0981e073d471d61f7bce0f3642e7/traces/T13)
are pinned to merge `a39ac703c09f0981e073d471d61f7bce0f3642e7` and the copied index's SHA-256.
The timing receipt includes bridge input parsing, image setup, native placement/helpers/RNG,
trace/effect collection and serialization; input JSON preparation and output decoding are excluded.

`[designed]` Placement accepts generated counts at most 11, a byte-sized minute bound, valid
roster slots and complete, unambiguous player fields. Unsupported/incomplete inputs and a reached
scorer query with no eligible candidate return replay-domain diagnostics. The incoming seed and
generated counts must match the DS image when composing D983 with placement; the method does not
reset that handoff to fixture values. Saved score and minute arrays represent temporary frame
fields. The returned generated bytes are their residual values before DD02 restores global scores.

The 5083 effect requests contain computed event arguments, its raw random draw and the wrapper's
0/1 coin (`raw value == 1`). They report pending external effects; player statistics, scorer logs
and display state are not updated by this projection. The request's coin has already advanced the
shared seed; applying the remaining external effects must not draw that coin again. Expected callbacks are compared after the
native computation and are never supplied to the input-only Rust/Python API.

The logical goal-effect slice applies the remaining `5083` updates using the goal request
already emitted by placement. Its coin draw has already advanced the shared seed. The handler
must reuse that coin, and the composed replay verifies that it makes no random request and does
not reset the seed or actor handoff to expected values.

`[confirmed]` The static contract includes shared-player counters/raw byte6 updates, signed
clamping and cache reloads, event totals and side counters, scorer-log caps/minute encoding, and
numeric commentary queue bookkeeping. Those caches and counters remain logical state even when
the original updates them through functions that also draw text. Formatted commentary, render
buffers, pixels, timer and scratch effects are external. Complete original state is supplied by
[archaeology T14/F14](https://github.com/diegoami/onenil-archaeology/blob/da4f9affc0f147c32ba5db132c6cfd5e7db731b4/findings/F14-logical-goal-effects.md); its source commit and copied index are pinned under
`engine/tests/fixtures/goal_effects/provenance.json`.

After building the release bridge, check or measure this logical slice:

```sh
python3 tools/replay_goal_effects.py --check
python3 tools/replay_goal_effects.py --benchmark --out records/T03
```

`[designed]` The timing receipt identifies the actual loaded release binary and measures the
bridge, memory-image setup, native logical updates and output serialization. Input preparation
and output decoding are excluded. It is a logical-handler baseline, not full-DD02/match/week or
renderer parity and not an emulator speed comparison. T01/T02 evidence remains immutable.

`[designed]` The effect request's coin remains consumed. The standalone logical API receives a
before-state plus that request; a composed API executes D983 and placement first, then applies
the same computed request without reseeding the image. Player records share canonical memory
by ID, so scorer/keeper aliases observe sequential mutations. Only named logical fields are
loaded; unrelated bytes are preserved. Typed scene/player/club bounds identify this projection's
supported input domain, and invalid or incomplete inputs return diagnostics.

The two table rows follow the original helper selectors, not a uniform scoring-side convention.
Event-total slot `s=1..20` is word `D146+40*side+2*s`; cache slot is byte
`D197+20*side+s`. Numeric queue rows preserve the entry-selected indices while the head can
advance or wrap. A duplicate row suppresses queue/count acceptance and the opaque paired-word
copy, while the weighted event total still changes. The scorer log uses a separate side selector
and marks the minute with byte addition, preserving the original wrap behavior.
