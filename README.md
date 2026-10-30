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

The DBC6 slice translates the quick-step extra-event decision and its direct availability,
scene-rating, predicate and threshold helpers. It checks 184 original calls, 499 decision
draws, 909 helpers and 22 computed requests to the still external `69B8` handler. Source pins
are in `engine/tests/fixtures/extra_event/`: archaeology T17/F17 at
`321b49556c0acc10576057bc820ff8ecee2d15f7`, supplemented by T18/F18 at
`59eb0c485d65956cb77283d7061542c20a75c624`. Both findings remain proposed.
T17's original array contains D01..D0A, while DBC6 actually reads D02..D0B; the reached
last cell comes from T18's separately captured raw input. All copied exports are byte-identical.

```sh
python3 tools/replay_extra_event.py --check
python3 tools/replay_extra_event.py --benchmark --out records/T06
```

`[confirmed]` The decision preserves wrapper booleans, short circuit draw order, unsigned
rating comparison, signed final threshold comparison and one-based predicate bits. The raw
scene-rating helper preserves signed bytes and low-word products. Its stack-dependent branch
can use an explicitly supplied preexisting word in an isolated helper call; composed DBC6
returns a diagnostic when that branch is reached because scratch propagation is not yet modeled.
All original composed cases avoid it. Expected helpers/draws/results never enter the decision API.

`[designed]` The decoded domain supports ordered scenes 1..22, canonical player IDs 0..3920,
clubs 1..176, style indices 0..13, level indices 0..20 and modes 6/14. Lookup cells sharing an
address and aliased club/player records must agree. Named pools share the existing engine image;
arbitrary physical far-pointer/scratch aliases are outside this projection. Composition runs
native placement and prefix first, then guards actual seed, actor, scene/player and club fields
before loading additional inputs. Failed handoffs and unsupported scratch paths preserve the
caller's image. `69B8` is returned as a request at its computed pre-handler state; its observed
return snapshot is not claimed as native output. Remaining event/substitution/reset/input effects,
full DD02/match/week, watched-engine and timer/render parity are pending. Timing includes bridge
parsing, image setup, decisions/helpers/RNG and serialization; preparation/decoding and external
handler execution are excluded. It is routine timing with no emulator speed claim.

The logical `69B8` slice applies the effect request computed by DBC6. It checks all
22 retained handler calls, 44 cyclic selectors, 17 LCG draws, six numeric events and
23 ordered availability/byte6 helpers. The separately generated cyclic stream is a
raw input, distinct from the LCG seed. Raw weights, queue acceptance, paired-word copies,
player flags, counters, added time, cache and scene clearing are logical engine state.
Source exports and their index are pinned under `engine/tests/fixtures/event_effects/`
to archaeology T19/F19 merge `6804fbb3d5985a45f0553fb1afb5ddcd37c08add`.
F19 remains proposed.

```sh
python3 tools/replay_event_effects.py --check
python3 tools/replay_event_effects.py --benchmark --out records/T07
```

`[confirmed]` Numeric helper `1DE2` reverses actor-slot arguments relative to `1DB7`.
Weighted writes occur even on rejected duplicates. Ordered `67D0` updates read shared
player state after earlier writes, including signed byte reloads. The native projection
supports observed `63E3`, managed `6589` and statically read managed `6849` behavior.
Reached AI roster-changing `A837/A94B` branches return transactional diagnostics;
ordinary AI caution-flag updates are implemented. Original cases contain no injury or
AI rearrangement; those branches have static/synthetic coverage or diagnostics as stated.

`[designed]` Inputs require quick mode, valid scene/slot and canonical player/club domains,
two selector cells at the entry-selected indices (head 1..255), and four numeric queue rows
(head 0..1700). Return/intermediate snapshots retain those indices as heads advance.
Composition runs actual placement, prefix and DBC6 first, then checks the computed request
and all shared handoff fields before loading additional raw context. Bench-only fields are
new context, rather than values already computed by DBC6. AI/invalid-handoff errors preserve
the caller image. Expected selectors/events/helpers/after-states never steer computation.
Full stream setup, remaining late callees, complete DD02/match/week, physical/scratch,
timer and renderer parity remain pending. Routine timing includes bridge parsing, image
setup, logical effects/selectors/RNG and serialization, with preparation/decoding excluded;
it makes no whole-match or emulator speed claim.

The post-score strategy slice translates `A4F2` and its baseline, bit-adjustment,
club-getter and player-flag normalizer closure. It replays 17 original calls, 12 draws,
32 setters, 566 ordered normalizers and 34 getters. Source cases and the late-call census
are pinned under `engine/tests/fixtures/post_score/` to archaeology T20/F20 merge
`256659e351567b9fe8d2f174a05008fb91d15756`. F20 remains proposed.

```sh
python3 tools/replay_post_score.py --check
python3 tools/replay_post_score.py --benchmark --out records/T08
```

`[confirmed]` The ignored argument never selects a path. Signed score subtraction narrows
to a byte before absolute value; independent unsigned score-byte comparisons choose bit
direction. Baseline adjustment reads the opposite club getter first and its own club second,
then compares their wrapping signed difference against the limit byte. Setter `0C23`
normalizes every nonzero roster entry, including repeated/shared player IDs in order.
No availability filter applies. Raw club override and first-roster-ID fields supply getter
inputs; expected getter/normalizer/path answers are assertions only.

`[designed]` The named domain supports clubs 1..176, player IDs 0..3920 and complete
20-word rosters with unambiguous aliases/lookup cells. Placement/prefix composition guards
actual shared seed, score, club and roster fields on the quick goal-reset path with a valid
nonkeeper actor. Keeper preludes diagnose. This composes a strategy footprint; the complete
`52B6` scene/reset, `6B2F` substitution and `78B7` keyboard effects remain unexecuted.
The census accounts for all recorded late-gap RNG draws, which does not establish full
DD02/match/week/setup or physical/scratch/timer/render parity. Routine timing includes bridge
parsing, image setup, native strategy/flags/RNG and serialization; preparation/decoding are
excluded, and no full-match or emulator speed comparison is claimed.

The named quick `52B6` reset now composes scene position copies, motion marking,
keeper handling, actual-seed strategy and ordered player updates. It checks 17 original
resets, 374 position copies, 374 motion visits, 374 availability checks, 373 byte6 updates
and 12 draws. Source exports/index are pinned under `engine/tests/fixtures/quick_reset/`
to archaeology T21/F21 merge `3223aa9bf2663d16b436132d1df4bfb7c3e4e9c1`.
F21 stays proposed.

```sh
python3 tools/replay_quick_reset.py --check
python3 tools/replay_quick_reset.py --benchmark --out records/T09
```

`[confirmed]` Active becomes 1 and all word8 copies precede the zero flag test. Motion
uses the old actor's physical side and stored scene slots, excluding physical keepers 1/12.
Keeper handling transforms signed coordinate bytes with the raw direction table, advances
the shared RNG and resolves cyclic eligibility before strategy. Strategy runs on that
computed seed; an expected intermediate state cannot reset it. Available shared-player
byte6 updates reload signed values in order and leave caches unchanged on flag 0.

`[designed]` The projection requires quick mode, physical offsets [11,0], valid
scene/slot/actor and canonical club/player domains. No eligible keeper cycle returns a
transactional diagnostic, with the caller image preserved. Keeper and zero branches have
static/synthetic coverage; all original cases here are nonkeeper with zero flag 0. The new
reset API handles keepers while the earlier T08 prefix-only strategy projection retains
its narrower guard. Placement/prefix composition preserves computed seed/actor/score/
club/scene/eligibility fields; byte6/cache/clock context not computed there is raw input.
Preceding goal-effect integration and 045C buffers, 6B2F substitutions, 78B7 keyboard,
continuous DD02/match/week/setup and arbitrary physical/scratch/timer/render parity remain
separate work. Routine timing includes bridge parsing, image setup, reset/strategy/helpers/
RNG and serialization; preparation/decoding are excluded, with no emulator speed claim.
