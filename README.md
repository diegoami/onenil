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

The ratings slice computes `1B55:50EB` and its player, staff and predicate helpers from
complete raw roster/player/staff fields and recorded tables. Predicate return values and
expected ratings enter the tests only as assertions. The source pin and fixture hashes live in
`engine/tests/fixtures/ratings/provenance.json`, pinned to archaeology merge
`4f89d245b90053355c6d768bf3367181447920da`. The
[F15 contract](https://github.com/diegoami/onenil-archaeology/blob/4f89d245b90053355c6d768bf3367181447920da/findings/F15-native-ratings-inputs.md)
and supplemental T15 corpus contain
1,128 original side calls, 12,614 player helpers and 2,495 staff-bonus helpers across six weeks.
F15 remains proposed. The native tests and release bridge also compose computed side ratings
into all 558 retained D983 calls, preserving their incoming seed and goal bytes. The composed
request contains only `k`, `seed` and `goals`; D983 reads the actual computed image words.

```sh
python3 tools/replay_ratings.py --check
python3 tools/replay_ratings.py --benchmark --out records/T04
```

`[confirmed]` Player bytes +6 and +8 of the fifteen-byte record are sign-extended; the player
rating clamps to 0..256, then its predicate may halve it. Side calls always use player mode zero,
while separate original helper calls cover mode one. The staff bonus is an unsigned word,
16..38. Split classes discard an odd remainder. A nonzero side flag adds the unsigned quotient
selected by the club byte. Shared IDs use canonical image records; conflicting shared inputs
across a composed pair are rejected, and the second side cannot reset the first side's ratings.

`[designed]` The supported decoded domain uses player IDs 0..3920, clubs 1..176, staff IDs
0..1000, position/style indices 0..13, class values 0..5, selector rows 0..1 and nonzero divisor
bytes. Roster ID zero is skipped; scalar player ID zero is rejected by the original predicate.
Raw fields and tables are loaded into the same memory image as ratings; unrelated bytes are
preserved. No rating helper draws RNG. With eleven bounded player contributions, the raw domain
cannot overflow a side accumulator: at most 3,234 before the largest possible flag bonus.
Wrapping instructions remain explicit. The original corpus does not exercise the +16 branch,
clamps, halving, divisor 64, unusual tables or aliased roster IDs; these have static/synthetic
coverage. Timing includes bridge parsing, image setup, native ratings/helpers and serialization,
with input preparation/output decoding excluded. This is routine timing only, not full
DD02/match/week, renderer/timer parity or an emulator speed comparison. T01–T03 evidence stays
immutable.

The deterministic tail prefix covers `0AB9:DF35..DFDB`: it restores the cumulative score,
computes the packed history byte and visits all 22 physical scene records in order. Shared
player IDs use one record, so repeated scene references observe sequential signed-byte updates.
The separate `7B7B` caller-clock API includes its numeric termination commentary and `CF51`
clear. Public fixtures and their archaeology commit/index pin are in
`engine/tests/fixtures/step_tail/`, pinned to archaeology merge `f6841e011485aa84040d48b1e028ebe330c9406c`.
T16/F16 remains proposed.

After cloning this repository and building the release bridge with the commands above:

```sh
python3 tools/replay_step_tail.py --check
python3 tools/replay_step_tail.py --benchmark --out records/T05
```

The public replay uses only committed decoded fixtures in this repository. It checks 558 tail
prefixes, 558 separate clock calls, 12,276 scene visits and 12 numeric termination events, with
literal T13 placement cases and all 15 T08 references. The composed placement-to-prefix API uses
actual native saved scores, goal flag, actor, generated bytes and seed. An inconsistent recorded
handoff returns a diagnostic before loading fields; expected outputs never reset that image.

`[confirmed]` History packing uses wrapping addition, rather than OR, and its far offset wraps
at 16 bits without carrying into the segment. Clock-kind equality compares a sign-extended byte
with a zero-extended word as long values: raw kinds 128..255 cannot equal a remaining word.
Numeric clock events have actor and secondary slots zero, so their weighted event word is
`D16E`, which aliases side 0's slot 20 in the earlier goal-effect projection. That weight changes
even when queue deduplication rejects insertion. Accepted events alone increment the side-1
count, advance the cyclic queue and copy the opaque paired words; clock termination clears
`CF51` after that numeric call on either acceptance result.

`[designed]` The prefix supports quick mode, clubs 1..176, player IDs 0..3920 and complete ordered
scenes with unique shared-player definitions. A canonical 64 KiB history region preserves the
original offset arithmetic; original segment values are pointer metadata. Arbitrary physical
far-pointer aliasing with DS or other named pools is outside this projection. Clock queue
context contains the entry-selected head and next two rows, retained at those indices in the
output even when the head advances. Formatted text, rendered buffers and timer effects remain
external.

The returned `pending_calls` explicitly identify untranslated `DBC6`, `6B2F`, `52B6`, `78B7`
and the goal-reset call as selected by the actual frame goal flag and D6DD. These requests are not
executed by the prefix. The separate clock replay starts from the original clock entry after
those intervening effects; this is not a direct prefix-to-clock or complete DD02/match/week
replay. The timing receipt measures independent prefix and clock routine calls, including
bridge parsing, image setup, native updates and serialization, with input preparation/output
decoding excluded. It identifies the actual release binary and makes no emulator speed claim.
