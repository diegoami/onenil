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
The timing receipt includes bridge input parsing, image setup, native placement/helpers/RNG,
trace/effect collection and serialization; input JSON preparation and output decoding are excluded.
