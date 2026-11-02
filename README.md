# onenil

A faithful rebuild of *One-Nil* (Wizard, 1995/1996), a football club manager, from the evidence of
[onenil-archaeology](https://github.com/diegoami/onenil-archaeology). The engine will be Rust,
checked against the original through a Python harness. No byte of the original game is in this
repository; see [docs/artifacts.md](docs/artifacts.md). Rules: [CLAUDE.md](CLAUDE.md). Process:
[docs/process.md](docs/process.md).

The next product target is [one playable quick-result match](docs/first-playable.md),
tracked in [issue35](https://github.com/diegoami/onenil/issues/35):choose a supported
lineup/seed,kick off,follow a match clock and text commentary,and reach a final
score through one continuing native state. Commentary is the minimum; a simple
pitch view is the preferred stretch goal.
The current routine harness is not yet playable. Progress is tracked through
match-state closure,a continuous quick step,a complete match,and text controls.

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

The T10 terminal rearrangement slice computes `0AB9:3155` scene positions and
`0AB9:21D7` proximity selectors, including `1CA1/4F89/1ECC/1CBB`, on the same DS
memory image. This is the first split of the reviewed r0017 substitution target;
A94B, AEB8 and the earlier rearrangement/cache/scoring helpers are still outside
this slice. It does not replay substitutions, DD02, a match or a week.

`[confirmed]` Archaeology F34/F35 at
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
supply two original calls per helper:44 position visits,40 selector candidates,
224 direct/nested helper observations,four selector initializations,four nearest
stores and zero draws. Byte-identical source fixtures/indexes and their pins
live under `engine/tests/fixtures/terminal/`. Python reconstructs the compact
projection from those raw inputs/observations and compares results; it contains
no position,distance,direction or winner arithmetic. Expected outputs are never
arguments to Rust. The actual-case failure at the initial stub is retained in
`red-proof.txt` and commit `4e2a5eb`.

```sh
python3 tools/replay_terminal.py --check
python3 tools/replay_terminal.py --benchmark --out records/T10
```

`[confirmed]`3155 assigns each indexed position word,then adjusts adjacent
duplicates using separate low/high byte operations. Its initial scratch-derived
previous-scene local is dead before any output-relevant use; no extra scratch
input or fabricated zero is supplied.21D7 uses signed coordinate/direction bytes,
unsigned distance thresholds and signed low-word product comparisons. Defaults
are offset+1,while stores copy scene.byte7. Equal outer input/output states do
not conceal native intermediate stores:all visits,paired rows,helper returns and
selector stores are compared with the original. The composed API runs both
helpers on one image without reloading expected intermediate fields.

`[designed]` Supported inputs have all22 ordered full25-byte scene rows,physical
offsets[11,0] or[0,11],actor and scene.byte7 values1–22,raw initial selector bytes,
unique player IDs0–3920,complete
canonical position/availability bytes and unique position cells for every
canonical position code. Raw sign/coordinate/phase bytes and time words retain
their widths. Missing,ambiguous,out-of-domain or extra expected-result fields
return diagnostics before image setup. Invalid input leaves the caller's input
unchanged;unowned image bytes and seed are preserved. Synthetic tests cover wide
and near duplicate adjustments,byte wraps,shared IDs,ahead stores,actor/byte19/
availability skips,ties,signed extremes,direction boundaries and actual handoff;
these are static/synthetic checks,not new original coverage.

`[designed]` The original specimens cover only r0017's two enclosing replacements,
not the other23 replacements across six weeks. The wide duplicate/sign branch,
ahead stores and byte19==13 skip have synthetic rather than original coverage.
Review requirements
[#120](https://github.com/diegoami/onenil-archaeology/issues/120) and
[#121](https://github.com/diegoami/onenil-archaeology/issues/121) remain open for
broader native composition. Undefined 0207/13B8/1AE2 domains are not translated
here and must be diagnosed when those routines are added. The release receipt
measures bridge parsing,image setup,computed helper observations and serialization;
input preparation/output decoding are excluded. It is routine timing only,with
no speed threshold or emulator comparison. Previous native evidence stays immutable.

T11 adds the read-only cached-player selector `0AB9:13B8` and its
`1B55:4EBF` remainder rating, reusing the native availability predicate.
`[confirmed]` Archaeology F32 at
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provides two selector results and44 standalone rating words in r0017.
Their byte-identical T32 exports/index and source pin are under
`engine/tests/fixtures/cache_selection/`. Two actual-original tests failed at
the initial stubs in commit `0c6ac1a`; `red-proof.txt` retains that evidence.

```sh
python3 tools/replay_cache_selection.py --check
python3 tools/replay_cache_selection.py --benchmark --out records/T11
```

`[confirmed]` The rating returns signed division's remainder, not quotient.
Below21 it logically shifts the complete16-bit word. Negative remainders retain
their bit patterns. Selection compares the sign-extended current score against
a zero-extended candidate, so a negative current word can be replaced even by a
lower unsigned value. Ordinary nonnegative ties retain the first player; equal
raw high-bit words need not. Native computed visits preserve unavailable skips
and the repeated rating call on improvement. Those visits are diagnostic output,
not an independently recorded original nested-call census. Neither helper writes
logical game state or advances RNG; a full-image preservation test checks this.

`[designed]` Inputs require side0/1,club1–176,eleven roster words,IDs0–3920 and
exactly one complete canonical record per referenced ID. Duplicate roster IDs
share that record. Missing,duplicate,unreferenced or out-of-range inputs and
extra expected-result fields diagnose explicitly. With no eligible candidate,
13B8's original result local is undefined; native code returns a diagnostic
instead of player0 or invented scratch. Rating inputs retain their three raw
unsigned byte widths. Python validates source hashes/order and compares only;
expected results never enter the native API.

`[designed]` Original selector specimens choose2661 for side1/club134;the44
rating calls contain only the original returns3/7. Negative/shift/high-bit,
zero/blocked/shared/no-eligible and alternate-side/club cases have separately
labelled synthetic/static tests,not new original coverage.1455/1AE2/9F6E,
1539/164F,role pairing/scene codes,A94B/AEB8,the other23 replacements and full
AI/match/week/render/timer/scratch parity remain later slices. Review notes
archaeology #120/#121 still constrain that composition. The timing receipt
includes parsing,memory setup,native computation and observation serialization;
input preparation/output decoding are excluded. It is a routine baseline only,
not an emulator comparison or full-game performance claim. Prior fixtures,
provenance,receipts and Cargo.lock remain immutable. All eleven release replays
are checked locally;native #22 tracks the CI workflow permission gap.


T12 adds read-only role scoring `0AB9:9F6E` and `0AB9:1AE2`.
`[confirmed]` F33 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provides148 scene-score observations and148 nested table calls from eight role
boundaries in r0017. Byte-identical T33 source/index, source pin and independently
regenerated raw-field projection live in `engine/tests/fixtures/role_scores/`.
Both original-case tests failed at stubs in `3389175`; `red-proof.txt` retains
the failure. These are helper observations, not complete role-cache replay.

```sh
python3 tools/replay_role_scores.py --check
python3 tools/replay_role_scores.py --benchmark --out records/T12
```

`[confirmed]` Native9F6E computes wrapped16-bit indices and DS addresses from
canonical P8 bytes and the criterion, then reads the two actual raw table cells.
Native1AE2 calls it on the same image; expected scores and intermediate results
never enter the API. Signed-byte baseline addition, low-word product truncation,
logical halving/shifts, unsigned scene/side bonuses and minimum1 retain guest
widths. Both helpers preserve the entire image and consume no RNG draws.

`[designed]` Scene inputs admit raw P15+6 only0–31 and criteria0–255; all other
P15+6 values diagnose explicitly. Signed32–127 can reach an uninitialized local
in the original; negative signed bytes are defined on another branch but remain
outside this capture's admitted domain. No zero scratch is assumed. Standalone
table criteria permit all16-bit words. Raw table-index witnesses are validated,
canonical player IDs stay0–3920, scene indices1–22 and scene.word14 must match
the player. Conflicting byte aliases reject before image setup; consistent
aliases share the actual image byte. Extra expected/intermediate fields reject.

`[designed]` The recorded scores/table words span5–131/17–175; all observed
scene.byte22 values are0. Synthetic tests separately cover halving, signed bytes,
word truncation, bonuses, minimum, wrapped indices/addresses, aliases and domain
errors. They add no original dynamic coverage. Full1455/8973 role selection,
1539/164F, A94B/AEB8 and the other23 replacements remain later work under
archaeology review requirements #120/#121. Timing includes bridge parsing,
validated image setup, nested native calculation and serialization; preparation
and output decoding are excluded. This is a routine baseline without emulator
comparison or full-game speed claims. Prior evidence and Cargo.lock remain
immutable. All twelve release replays run locally; native #22 tracks the CI
workflow permission gap (CI currently runs only the first five bridge tools,
alongside all Rust tests and a release bridge build).

T13 adds `0AB9:1455` role-cache selection with its reached `0AB9:8973`
player-to-scene lookup. `[confirmed]` Archaeology F33/T33 at
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provides eight role roots from r0017:82 availability visits,74 slot lookups,
148 scene scores with148 table children,and27 stores. Native output matches
all ordered helper arguments/results, scene snapshots, cache clears and store
continuations. The original draw counter4025 is normalized to routine-local0.

```sh
python3 tools/replay_role_cache.py --check
python3 tools/replay_role_cache.py --benchmark --out records/T13
```

`[confirmed]` Selection clears the requested cache word, visits slots after
09F6 (09F7 for kind5), skips unavailable players, and stores the first eligible
player. Later candidates replace only on a strictly greater unsigned score;
ties retain the cached player.8973 scans the first eleven player-club roster
words and uses the last match plus the offset selected by club==D6E6. An absent
match returns0. Native availability, slot lookup,1AE2 and9F6E operate on one
image. Scores and store decisions are computed from raw bytes; no expected
helper result enters the API. Only the selected cache word may change; full-image
checks on all eight originals preserve every other byte and the RNG seed.

`[designed]` The input projects relevant raw fields from the complete T33 state:
22 complete scenes, canonical player IDs/P8/table/P15 fields, player-club rosters,
both five-word caches, six captured criterion bytes, side offsets, match clubs,
seed and score bonus bytes. It omits unrelated outer state. Canonical records
are unique and include every referenced scene/roster/cache player; extra canonical
records from the original snapshot are retained and validated. Side0/1,kind1–5,
criteria0–255,offsets[0,11]/[11,0],distinct clubs1–176 and IDs0–3920 are admitted.
Raw indexed-cell witnesses and shared-byte aliases must agree before image setup.
Reached scores retain T12's P15+6 admission0–31; unavailable or unscored players
do not reach that guard. A reached missing/mismatched cached-player scene returns
a diagnostic rather than scoring row0.8973 is exposed only through this bounded
selection contract, not as an arbitrary-state/zero-player lookup API.

`[designed]` The six-byte criterion shape was corrected after the first tests
stopped at deserialization (`713df7c`). Both actual-root/census tests then failed
at the missing selector in `8e32e6f`; `stub-red-proof.txt` retains the valid red
proof, alongside the initial failure record. The source/index already pinned
by T12 are reused unchanged. T13's raw projection independently regenerates
from those original rows and checks its own digest.

`[designed]` Synthetic tests separately cover blocked/zero/all-blocked candidates,
empty ranges, kind5, first/tie/improvement, repeated roster IDs/last match,
alternate side/offset/club/player limits, absent/mismatched scenes, reached-only
score guards, contradictory aliases, malformed canonical inputs, extra result
fields and actual raw-cell handoff. These do not add original dynamic coverage:
all82 original availability returns were0. Full1539/164F,role pairing/scene
codes,A94B/AEB8,the other23 replacements and full AI/match/week parity remain
later work under archaeology #120/#121. The immutable receipt measures release
bridge parsing, validated setup, actual nested computation and serialization;
it makes no emulator or full-game speed claim. All thirteen replay tools run
locally; CI still runs the first five plus all Rust tests/release build under
permission follow-up #22. Previous evidence and Cargo.lock remain unchanged.

T14 closes the remaining non-selection helpers and caller rating stores needed
by1539. `[confirmed]` Archaeology F31/T31 and F32 at
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide four managed-club0E7A calls,two577F cached roster lookups,and four
side phases with44 actual4EBF calls and44 caller byte18 stores. T31 source/index
are pinned byte-identically in `engine/tests/fixtures/cache_helpers/`;the raw
projection regenerates independently. All three original-case tests failed at
stubs in `12bd6f9`,exit101;`red-proof.txt` retains the evidence.

```sh
python3 tools/replay_cache_helpers.py --check
python3 tools/replay_cache_helpers.py --benchmark --out records/T14
```

`[confirmed]`0E7A compares the full16-bit query against two zero-extended managed
bytes;it now also supplies post-score strategy's existing predicate.577F reads
the selected cache kind/side and match-club roster,returns the first matching
slot among eleven (including zero IDs),or12. This helper has different semantics
from8973's last-match search and zero miss. The side phase repeatedly calls the
existing native4EBF with its kind1 cached player,not the scene's own player.
It doubles the returned word,adds AL to scene.byte18 with byte wrapping,and
records actual arguments/rating/before/after rows in original order. The original
counter4025 is normalized to routine-local0. Each side phase uses one image;
the reusable image-level functions support later parent composition.

`[designed]`Managed queries admit all u16 words and both raw u8 bytes,including
literal zero;no inactive-slot convention changes equality. Cached-slot inputs
require side0/1,kind1–5,club1–176 and raw roster/cache IDs0–3920. No availability
filter is applied to the search. Side-phase inputs require side0/1,offsets
[0,11]/[11,0],22 complete ordered scenes,both caches and unique canonical player
rating bytes;every referenced scene/cache ID must exist. Extra source canonical
records are retained and validated. The phase has no availability filter either:
ID0 has its own raw rating bytes. Expected returns,stores and nested rating
results are rejected as inputs. Unrelated parent state is outside this projection.

`[designed]`Full-image tests preserve everything for the read-only primitives
and permit only the selected side's eleven byte18 stores for each original
phase,preserving seed and caches. Synthetic tests separately cover both managed
bytes/full-word equality,duplicates/zero/last-slot/missing roster IDs,all cache
kinds,alternate sides/offsets and club/player limits,actual cached-player handoff,
negative remainders/logical shifts,byte overflow,malformed/extra fields and two
side phases on one image. Original managed returns are1/0;both slot results are9
for side0/kind1/club130;recorded ratings are3/7. Synthetic cases add no original
dynamic coverage. Full1539 selection composition,B6BD managed/UI fallback,
164F,role pairing,scene codes,A94B/AEB8,the other23 replacements and full AI/
match/week parity remain later under archaeology #120/#121. The immutable timing
receipt follows a committed-source release build and includes parsing,setup,
native computation/observations and serialization;there is no emulator or
full-game speed claim. All fourteen local bridge checks pass;CI runs all Rust
tests/release build and its first five bridge checks under permission follow-up
#22. Earlier evidence and Cargo.lock remain unchanged.

T15 composes the bounded `0AB9:1539` player-cache parent on one guest image.
`[confirmed]` F31–F33 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide two original parents with106 ordered direct events,eight role selections,
82 candidate visits,74 cached-scene lookups,148 scene/table score pairs,27 role
stores and44 caller byte18 updates. The native parent computes every reached
helper result through existing Rust code;helper states are not reloaded from
recorded boundaries. It preserves managed/gate branching,kind1 caller stores,
role criteria/order and cached-player rating/store continuations across both sides.

```sh
python3 tools/replay_player_cache.py --check
python3 tools/replay_player_cache.py --benchmark --out records/T15
```

`[derived]` T31 parent snapshots join all eight exact T33 role entry/exit states.
Raw multi-criterion tables,bonus and criterion cells captured at those boundaries
close the extra read set at parent entry:the preceding closed helpers do not
write those cells. Canonical aliases agree at every projected boundary;no cell
is inferred from a score. The T31/T33 exports/indexes already pinned by T14/T12
are reused unchanged. T15's source/projection digests and exact parent/role joins
are verified independently;Python projects raw inputs and observations only.
Both original parent/continuation and nested-census tests failed at the stub in
`59c7543`,exit101;`engine/tests/fixtures/player_cache/red-proof.txt` retains proof.

`[designed]` Inputs carry both caches,22 complete scenes,unique canonical P8/P15
fields,first-eleven player-club rosters,managed bytes,gate,six criterion bytes,
clubs,side offsets,seed and complete indexed table witnesses for every canonical
player and distinct role criterion. Missing/duplicate/unreferenced read sets,
contradictory canonical/table/global aliases and invalid domains reject before
image allocation. IDs0–3920,distinct match clubs1–176,offsets[0,11]/[11,0] and
byte-width role criteria are admitted. Original extra canonical records remain
validated. Immutable input fields remain fixed through this closed graph;
snapshots reread current caches,scenes and seed. Full-image tests check that
invariant and permit writes only to ten cache words and22 scene.byte18 cells.

`[designed]` Reached B6BD managed/UI fallback returns a diagnostic. Undefined
13B8 selection with no eligible player,and the existing reached P15+6/missing
cached-scene guards,remain explicit;no zero scratch or fallback player is
invented. Expected results/events/nested scores are rejected as inputs.
Synthetic tests separately cover managed/gate branch combinations,caller kind1
and role handoffs,side ordering,empty ranges,byte wrapping,zero cached ratings,
unsupported fallback/noeligible/score domains,canonical/multi-criterion aliases
and memory ownership. These add no original dynamic coverage. The original
parents cover only r0017's two replacements. Full164F,10B9/1000 pairing,scene
codes,A94B/AEB8,the other23 replacements and full AI/match/week parity remain
later under archaeology #120/#121. The release receipt follows a clean committed
build and includes parsing,setup,native calculations/observations and serialization;
there is no emulator/full-game speed claim. All fifteen local replay tools are
checked;CI still runs all Rust tests/release build and its first five bridge tools
under permission follow-up #22. Earlier fixtures/provenance/receipts/lock remain
immutable. This closes the bounded player-cache stage,not complete AI replay.

T16 adds bounded `0AB9:10B9` role pairing and its `0AB9:1000` range helper.
`[confirmed]` F29/T29 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provides two original parents and20 range boundaries from successful T29-r0002.
Those source/index bytes are pinned in `engine/tests/fixtures/role_pairing/`;
aborted r0001 is not used. The projection independently copies raw52 preference
cells,canonical role/availability fields,club flags/rosters,complete scenes and
actual range arrays. Both original-case tests failed at stubs in `070bc9f`,
exit101;`red-proof.txt` retains that evidence.

```sh
python3 tools/replay_role_pairing.py --check
python3 tools/replay_role_pairing.py --benchmark --out records/T16
```

`[confirmed]` Native1000 visits four preference passes and descending opponent
slots11–2,searches the ten Pascal data bytes for the first matching singleton
character,and clears/stores the selected cells. NUL is a valid character;there
is no assigned-slot or zero-byte skip.10B9 generates its own arrays,excludes own
role1/reserved/unavailable players and unavailable opponents,then excludes every
reserved roster-word match. It calls ranges12–13,9–11,5–8,1–1,2–4 in order on
the same image/frame. Early keeper/second-role1 clears,player flag0x40 retention,
club byte57 bit0x80 and actual opposing roster words determine scene.word20.
The existing native availability predicate is reused. No expected local arrays
enter the parent API;standalone range arrays are actual caller-entry inputs.

`[designed]` Caller arrays use normalized image addresses,with explicit length10
and assignment sentinels. Singleton Pascal conversion/search is represented by
exact byte comparisons,including embedded NUL;physical stack/register/scratch
parity is outside this contract. Inputs require roles0–13,all52 raw preference
bytes,unique canonical IDs0–3920,distinct clubs1–176,their eleven roster words,
22 complete ordered scenes and offsets[0,11]/[11,0]. Referenced players must
exist;extra original canonical records are retained/validated. Stored word20
values remain raw words,not additional player lookups. Range bounds require
1<=low<=high<=13;local sentinels and assignment values0/2–11 validate explicitly.
Malformed/extra expected-result or parent-local fields diagnose before setup.

`[designed]` Full-image checks permit only caller-array cells for standalone
ranges,and those arrays plus scene.word20 for the parent,preserving seed,
preferences and all other bytes. Prefilled frame tests verify caller initialization
without assumed zero scratch. Synthetic tests separately cover pass/reverse-slot/
first-match order,endpoints,NUL overwrite/miss,keeper/role1 handling,flag-bit and
signed availability distinctions,reserved duplicate roster exclusions,club gate,
actual roster handoff,offset/club/player limits and malformed/extra inputs.
They add no original dynamic coverage. Original final assignments have9/8 nonzero
entries per side;named word20 differences are2 then1. These are value changes,
not an observed instruction-write or primitive-call census. Original draw4025
normalizes to0. The20 ranges are replayed standalone and composed without becoming
40 new original observations.

`[designed]` This closes the bounded pairing stage for r0017's two replacements;
full164F,scene/cache codes,A94B/AEB8,the other23 replacements and full AI/match/
week parity remain later under archaeology #120/#121. The immutable release
receipt follows a committed-source build and includes parsing,setup,native
computation/observations and serialization;no emulator/full-game speed claim.
All sixteen tools run locally;CI still runs all Rust tests/release build and its
first five bridge checks under permission follow-up #22. Earlier evidence and
Cargo.lock remain unchanged.


T17 advances first-playable gate1 by closing the scene/cache code dependency
`0AB9:03BB`,with0143/0207/02D7/033E on one image.
`[confirmed]` F30/T30 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provides two parents,44 scene boundaries,two cache boundaries and six nested0143
returns. These and the six standalone code replays match complete projected
state. T30 source/index are byte-identical and pinned;Python independently
projects raw data/observations,with clock/role aliases validated. Both actual
original tests failed at stubs in `2fdc167`,exit101;`red-proof.txt` retains proof.

```sh
python3 tools/replay_scene_codes.py --check
python3 tools/replay_scene_codes.py --benchmark --out records/T17
```

`[confirmed]` Native0143 reuses the existing1ECC direction function:unsigned
clock versus signed phase,stored-selector side predicate,raw signed direction
and signed high-byte arguments. Products retain low words before signed
comparison. The helper returns its word using old byte6,then logically negates
byte6 only for unequal packed arguments. Mode0 passes A=scene.word2,B=word0
and truncates only the resulting low code to AL. Modes1–4 preserve selector
indirection,raw role-code lookup,boolean code tables and word-wrapped modulo.
Keeper selectors1/12 use their separate high bytes.033E passes cache.word2
then word0,performs three ordered calls on the same actor-selected keeper
scene,and stores full word returns plus0500 in each cache.word4. Physical scene
indices and stored selectors remain separate inputs. Snapshots read the current
image,including mode,clock and canonical role bytes,for future caller integration.

`[designed]` The pointed mode byte uses canonical image storage,not an asserted
original pointer. Parent inputs require mode0–4,22 ordered complete scenes,
canonical IDs0–3920/roles0–13,offsets[0,11]/[11,0],all14 role-code cells,six
code bytes,three ordered caches and raw actor/clock/phase/direction bytes.
Mode>4 diagnoses the original undefined local;reached mode1 selectors outside
1–22 diagnose. Other modes retain raw u8 selectors,including0. Scalar0143
accepts a complete physical scene and its raw packed/context inputs only.
Malformed/extra expected-result fields reject;no observed scalar return is fed
back into the engine. Whole-image checks allow only scene bytes4/5/6 and three
cache words4;scalar calls may change only their scene.byte6. Seed/unowned bytes
and cache padding are preserved.

`[designed]` Synthetic tests separately cover modes0–4,signed direction/high
bytes/word comparisons,argument order,full-word return versus AL truncation,
logical byte6 toggle/return order,selector identity,keepers,target1/12,all three
cache handoffs,word-wrapped selector0,undefined/malformed domains and actual
current-image reads. Original cases only show mode0/equal position/cache words,
unchanged parent projections,code156/cache1436 and zero draws. Unchanged named
values are not a write-free claim;other branches have synthetic/static coverage.

`[designed]` Gate1 still needs remaining rearrangement and selection/replacement
paths reached by the first whole-match reference. Continuous DD02/clock/setup/
match integration and text controls remain explicit later gates in issue35.
This task is not called playable. Other23 replacements/six-week breadth and
full AI/match/week acceptance remain under archaeology120/121. The committed-
source release receipt includes parsing,setup,native calculation/observations
and serialization,without an emulator/full-game speed claim. All seventeen
replay tools run locally;CI still runs all Rust tests/release build plus its
first five bridge tools under native22. Earlier evidence and Cargo.lock remain
unchanged.

T18 closes scene assignment8599 and its rebuild wrapper8663, a prerequisite
for composing first-playable rearrangement164F. `[confirmed]` F26/T26 and
F28/T28 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide44 assignment boundaries and two rebuilds with20 recorded nested helpers
for the fixed Sheffield Wednesday–Blackburn reference. Source files/indexes are
pinned byte for byte;Python projects raw fields and recorded observations only.
Both original tests failed against the stubs at0793ae3 (retained exit101 proof).

```sh
python3 tools/replay_scene_assignment.py --check
python3 tools/replay_scene_assignment.py --benchmark --out records/T18
```

`[confirmed]`8599 stores the supplied selector, derives slot/side from that
selector independently of the physical scene index, reads the actual roster
player and that player's club-count table, computes a signed32 minimum, applies
unsigned age thresholds and computes the handedness/role predicate.8663 then
sets position words, selects keeper/ordinary direction and resets its specified
fields. Its caller's later word2 restore remains the caller's responsibility.
Nested routines share one live image. Snapshot values are read back from it;
no recorded count, minimum, flag or intermediate state drives computation.

`[designed]` Inputs admit22 ordered scenes, distinct header clubs1–176,
selectors/physical indices1–22, unique canonical players0–3920 with roles/styles
0–13, eleven roster words per header and raw count indices/cells for each
nonzero canonical player's club. All references and overlapping raw bytes
validate before image allocation. Unselected player0 remains a sentinel;
a reached player0 or unsupported club lookup diagnoses. Wrapped count-table
indices share actual bytes in a separate normalized64K pool at50000, disjoint
from pairing locals. Handedness table/selector aliases must also agree. Extra
expected/intermediate fields reject. Whole-image checks limit writes to each
routine's specified scene fields, preserving seed and other pools.

`[designed]` Synthetic coverage separately tests selector/roster handoff,
player-club versus header-club identity, word-index aliases, signed32 high/low
comparisons, age thresholds, raw nonzero hand values, early/second predicate
branches, keeper directions, reset order, malformed domains and live-state
composition. These add no original dynamic coverage. Original draws4025→4025
normalize to0;T28 nested assignment helpers are computed diagnostics, whereas
T26's20 nested observations are compared in full.

`[designed]` Next is parent164F composition using assignment plus the existing
pairing, scene-code, position, proximity and player-cache stages. Replacement/
selection integration, continuous DD02/clock, setup→final-score execution and
controls/commentary remain later first-playable gates. This slice does not run
a complete match. Pitch graphics remain a preferred stretch goal. All eighteen
bridge tools run locally;CI retains all Rust tests/release build and its first
five bridge tools under permission follow-up22. The immutable committed-source
release receipt includes parsing, image setup, native calculation/observations
and serialization;it makes no emulator/full-game speed claim. Earlier fixtures,
receipts and Cargo.lock remain unchanged.
