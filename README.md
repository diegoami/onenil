# onenil

A faithful rebuild of *One-Nil* (Wizard, 1995/1996), a football club manager, from the evidence of
[onenil-archaeology](https://github.com/diegoami/onenil-archaeology). The engine is Rust,
checked against the original through a Python harness. No byte of the original game is in this
repository; see [docs/artifacts.md](docs/artifacts.md). Rules: [CLAUDE.md](CLAUDE.md). Process:
[docs/process.md](docs/process.md).

A native desktop feasibility prototype now reproduces the original-style main menu and
Players interaction, backed by the original saved Blackburn club and complete native save/load.
The unchanged supplied campaign and exactly its witnessed first-team Pearce/Hendry
exchange can play the PSG friendly with detailed
player commentary, clock, score and pitch positions. It runs locally without a browser/server:

```sh
python3 desktop/faithful/fetch_font.py
cargo run --release --manifest-path desktop/faithful/Cargo.toml --features software --locked
```

Space opens Players; Insert switches skills/details; Enter picks/places a player; Escape
cancels/returns. F7 saves the complete campaign; F9 loads a native save, showing a chooser when several exist. Main-menu Enter opens the actual PSG
fixture preparation; Enter on the highlighted PLAY GAME computes the supported match.
Both the unchanged initial campaign and exactly one first-team Pearce/Hendry exchange
are admitted, including native save/reload of that state. Every other campaign edit
refuses before worker launch. To perform the supported exchange from the initial
main menu, press **Space, Down, Enter, Down, Enter, Esc, Up**; then **Enter twice**
starts its match. The two Enter presses also start the unchanged case from the
initial main menu. Press E on preparation for the separate fixed native exhibition.
See [saved-club state, launch and limits](docs/campaign-state.md).
Player options now implement Reposition, Club position and the observed goalkeeper
Positioning cycle under Change tactics. Save/reload retains those edits. Other
instruction rows and management actions remain unavailable.
Acknowledge full time to record the computed fixture result, then use F7 to save it.
Reload retains the recorded score and blocks replay of that completed fixture.
Week progression is not available yet.
The pitch uses the original numbered variant (outfield0–9), with an independent
G keeper-glyph replacement and computed team colours. See the
[numbered-player proof and limited display-mode scope](records/T72/README.md).
Arbitrary edited XIs and instruction combinations remain unsupported. This is a tested Linux desktop
candidate, with several UI/statistics/management gaps; no final toolkit or Windows package is claimed.

From Windows PowerShell with WSLg, launch the already-built demo with:

```powershell
wsl --cd /home/diego/projects/onenil-archaeology/.cache/onenil-native --exec ./desktop/faithful/target/release/onenil-desktop --save /tmp/onenil-demo.json
```

Use your own native checkout path if it differs. The local build/font must exist;
this launches the Linux desktop through WSLg. See the [current demo controls and
readability proof](records/T70/README.md).

The earlier technical watched preview remains available for engine evidence. The next product target is [one playable watched match](docs/first-playable.md),
tracked in [issue35](https://github.com/diegoami/onenil/issues/35):choose a supported
lineup/seed,kick off,follow a match clock and text commentary,and reach a final
score through one continuing native state. Commentary is the minimum; a simple
pitch view is the preferred stretch goal.
The watched preview now provides detailed named player actions,a running clock,
live scores and actual player/ball positions. Build the release extension below,
then run `.cache/venv/bin/python tools/play_watched.py` and open
http://127.0.0.1:8041. Allow time for the initial native match computation.
Choose the reference XI (Newell starts) or reserve XI (Warhurst starts),and
seed64 or122,then click Prepare match. The preparation indicator remains visible
while Rust computes the match;when ready,kick off,pause,continue at halftime
and restart from the browser. All four choices execute native selected-squad
setup through the actual full-match return. Only reference64 has complete
original-match parity evidence;the alternatives establish supported completion.
The earlier quick-result preview and its
lineup/seed controls remain at `tools/play_match.py` on port8040.

The original-game illustrated manual is accepted in research milestone135.
The product is a **desktop game that reproduces the original UI and workflow**.
Browser delivery is a secondary, later publishing option. Start with the
[desktop fidelity contract and original-screen study](docs/desktop-fidelity/README.md).
The [T48 delivery experiments](docs/ui-delivery.md) retain useful file/interaction
measurements; their generic sample editor and browser-first recommendation are
superseded as product direction. No desktop toolkit has been selected.

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

T19 composes164F rearrangement for the fixed first-match reference. `[confirmed]`
F28–F35 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide two parent observations and164 direct caller events:44 availability
sum calls,44 role calls,four signed divisions,44 assignments,fifteen typed
exchanges,three selector marks and ten tail boundaries. Their five native
callees execute in order on the parent's live image:10B9 pairing,03BB scene
codes,1539 player cache,3155 positions and21D7 proximity. The parent then clears
word12 of physical keeper scenes1 and12. No tail input state is reloaded.

```sh
python3 tools/replay_rearrangement.py --check
python3 tools/replay_rearrangement.py --benchmark --out records/T19
```

`[confirmed]` Availability gates unsigned P8+4 additions and maps unavailable
players to local role14. The signed32 quotient of sum/22 plus one updates the
opposite club total. Strict unsigned role comparisons drive ordered exchanges
of roster words,local roles,complete25-byte scenes,event words and cache-selector
bytes;only after all five does the caller reindex actor/secondary. Supplied
physical indices and stored selectors retain their separate meanings. CPU-class
instruction selection is normalized to logical signed division;zero/overflow
diagnose. The actual specimen uses CPU2 and positive small operands.

`[designed]` Raw supplemental inputs join exact T28 tail boundaries against
T29/T30/T31/T33/T34/T35. Earlier independent source/projection pins are verified
before the join. Redundant typed component inputs reconcile through raw-byte
alias checks before allocating a single image. Canonical player sets must agree;
full P8/P15 records,twenty roster words,two event/cache-selector arrays and
reached tables travel in the parent core. Expected results,division outputs,
sorted roles and per-tail observations are not runtime inputs. Named caller
projections and full final component state are compared;unneeded T28 selection/UI
fields are outside the projected input contract. Computed nested diagnostics do
not add original observations. Both parents failed against69b08f5's stub with
retained exit101 proof. Original draw4025 normalizes to0.

`[designed]` Synthetic tests cover equal/mixed/unavailable ordering,non-stable
strict exchanges,unsigned sums,signed division exceptions,both offsets,selector
exchange timing,whole-scene transfers,canonical/table aliases,unsupported/extra
inputs,live-image mutations and final keeper clears. Whole-image checks preserve
seed,all canonical player bytes,unowned roster/event/cache entries and other
pools. Earlier public APIs retain their contracts;terminal execution can borrow
the caller image. These tests add no original dynamic breadth.

`[designed]` This closes the fixed-reference rearrangement dependency in
first-playable gate1. Next are reached A94B/AEB8 selection/replacement paths,
then continuous DD02/clock and setup→final-match integration. It is not a complete
playable match. All nineteen release tools run locally;CI retains all Rust tests,
release build and its first five bridges under22. Earlier evidence,lock and
workflow remain unchanged. The immutable committed-source release receipt covers
parsing/setup/native computation/serialization and makes no emulator/full-game
speed claim. Other23 replacements/six-week breadth remain under archaeology120/121.

T20 composes logicalAEB8 replacement for the two fixedr0017 substitutions.
`[confirmed]` F24–F28 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide their26 direct helper observations andT27's25 compact53FB observations.
Both original tests failed againstc74719b's stubs(exit101,retained proof).

```sh
python3 tools/replay_replacement.py --check
python3 tools/replay_replacement.py --benchmark --out records/T20
```

`[confirmed]` AEB8 distinguishes the bench slot,physical scene,outgoing scene
player and incoming first-eleven roster player after its caller's swap. It sets
the outgoing flag bit,tests managed clubs,saves word2 with its sentinel rule,
rebuilds through8663,applies67D0's signed clamp/cache store,conditionally clears
and recomputes incoming53FB flags,updates event/season counters,accounts for
reserves/substitution flags and restores word2. Managed cached players that are
unavailable are temporarily replaced by their club's first roster player during
native164F,then cleared. Both scene assignment and the complete rearrangement
run on the same live image;no observed helper output initializes a later call.

`[confirmed]`53FB computes4E0C from P8+1/+3/+5:the younger path retains a signed
word intermediate and signed32 maximum with zero;the adult path returns raw AL
with byte wrapping. A result below4 sets flag bit8 and clears bit4. Roles0/1/3/4
add bits5/1;role2 adds5/6/2;other bits are preserved.67D0 reuses the existing
signed byte6 clamp and signed cache reload.186C increments byte4/word0 with
wrapping only for nonzero allowance. T27's25 observations verify this helper
only;they do not expand the two-replacement full-context coverage.

`[designed]` Inputs add explicit replacement arguments and allowance,reserve,
substitution,mode,sentinel and speaker context to the existing shared-state
read set. Exact T25/T27 source/index pins and the supplemental rearrangement
projection are checked. All canonical aliases validate before allocation;
unsupported/mismatched identities and extra expected/helper fields diagnose.
Snapshots reread canonical player/cache values after replacement mutations.
Named helper projections and full supported final state match the observations.

`[designed]` Reached port61 speaker,ADD0 message and0F94 log boundaries emit
structured presentation events in observed order. These preserve logical reach
and semantic arguments for a future commentary adapter;they do not reproduce
original hardware,display/log scratch or timer state. They are separate from
engine requests,with no opaque reached engine callee. The optional message
branch is synthetic only. No original hardware/display parity is claimed.

`[designed]` Synthetic tests cover young/adult thresholds,preset role flags,
signed motion/cache routing,counter wrapping,managed replacement,saved-word2
sentinel,substitution/reserve choices,temporary cache handoff,presentation
branches,malformed/extra inputs and whole-image write ownership. These add no
original dynamic observations. Original draws4025 normalize to0. All twenty
release tools run locally;CI retains all Rust tests/release build and its first
five bridges under22. Earlier evidence,lock and workflow remain unchanged;
the immutable committed-source receipt makes no emulator/full-game speed claim.

`[designed]` First-playable gate1 next needsA94B selection connected to this
replacement. Continuous DD02/clock,setup→complete match and the commentary shell
remain later gates. This is not a complete playable match;pitch graphics remain
the preferred stretch goal. Other23 full replacements/six-week breadth remain
under archaeology120/121.

T21 connectsA94B selection to nativeAEB8 replacement. `[confirmed]` F23–F28 at
archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide37 calls in the r0017 week and33 observed helper boundaries:five candidate
scans,five limits,fifteen availability calls,five preferred selections,one hand
lookup and two replacements. The two replacements occur in the chosen first
match;other early-return calls are inexpensive regression evidence,not a larger
playable scope. Each zero-draw interval normalizes its own original start to0.
Both replacements join T25 exactly and execute on the selection caller's image.
The original test failed atf72a28c's stub(exit101,retained proof).

```sh
python3 tools/replay_ai_selection.py --check
python3 tools/replay_ai_selection.py --benchmark --out records/T21
```

`[confirmed]` Capacity includes the conditional keeper allowance. Its exhausted
branch tests signed roster P15+8 alone. Candidate scans overwrite by raw style
in increasing reserve order and skip unavailable players. Preferred selection
retains the last nonzero match;Pascal-character search returns a one-based index,
including NUL,and handedness must equal the index-above-three predicate as a
raw byte. Weighted fallback retains the first strict weight improvement and
halves on hand mismatch. A chosen player's role is copied before roster,event
and cache-selector swaps and the native replacement call.

`[confirmed: listing0AB9:A8CF–A8DD]` The scan's exclusion lookup subtracts the
side offset before reading the scene player. It therefore reads the normalized
scene index,which differs from the physical caller scene when the offset is11.
The retained five scans use offset0;the offset11 behavior has static/synthetic
coverage only. Do not simplify that instruction sequence to a physical-scene
lookup or claim a new dynamic observation.

`[designed]` The compact T23 context admits complete canonical references,
ordered preference rows,Latin1 Pascal text,raw hand cells,reserve limits and
candidate cells. Optional supplemental replacement read sets supply full scene,
player,club and downstream tables;they contain no chosen slot or expected
helper result. Earlier raw T23 values reconcile their aliases before a single
image allocation. Missing reached replacement inputs and unreachedA837 paths
diagnose explicitly. Unsupported indexed domains/extra-result fields reject.
Earlier public APIs and evidence remain unchanged.

`[designed]` Synthetic tests separately cover candidate overwrite/exclusion,
raw handedness and NUL search,preferred-last versus weighted-first ties,
halving,signed exhausted-capacity and keeper gates,both offsets,missing/invalid
read sets,A837 diagnostics,canonical/table aliases,live counter mutation across
replacement and whole-image write ownership. Original weighted/A837 branches
remain unreached. The implemented weighted path has synthetic coverage only;
A837 remains unsupported. Presentation events follow the T20 adapter contract,
excluding original hardware/display/log-scratch/timer parity.

`[designed]` All twenty-one release tools run locally;CI retains all Rust tests,
release build and its first five bridges under22. Earlier evidence,lock and
workflow remain unchanged. The immutable committed-source receipt measures
JSON/setup/native computation/observations,not emulator/full-game speed.
Next is auditing reached state changes and connecting the substitution/input
callers to a continuous DD02+clock transition. This is not a complete playable
match. Commentary+clock remain the minimum,pitch graphics the preferred stretch;
other-week/full-game parity remains separate under archaeology120/121.

T22 connects6B2F substitution dispatch to the native selection/replacement chain.
`[confirmed]` F22/F23 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
provide91 dispatchers at first-match steps0–94(excluding four goal-reset steps),
401 direct observations and37 nested selections. Direct observations include182
managed checks,91 keeper checks,91 capacity checks and37 AI boundaries. Exact
T22/T23 joins also compare each nested full selection continuation and its33
observed helpers. The original test failed at58bf69b's stub(exit101,retained proof).

```sh
python3 tools/replay_substitution.py --check
python3 tools/replay_substitution.py --benchmark --out records/T22
```

`[confirmed]` The dispatcher visits sides0 then1. Capacity adds zero-extended
reserves/allowance and subtracts substitution flags with word wrapping;unsigned
nonzero tests retain underflow. Managed sides check keeper availability before
the pending gate. Available keepers additionally test wrapping remaining+added
and capacity before prompt work. AI sides scan physical slots11 down to1,skip
mask20/nonpositive signed byte8,and require remaining2–34,slot>1 and a signed
event-total word below zero-extended90-minus-remaining. A94B receives the scene's
stored selector and roster identity. Zero reserves after the call end the scan.

`[designed]` Each dispatcher initializes one image from its named context and
supplemental raw read set. Selection tables and fields join the adjacent T23
observations;T22 supplies every dispatcher field. Unread supplementary fields
carried outside those boundaries do not establish new original observations.
The one step57 parent supplies full replacement inputs once and computes five
selection calls including both substitutions without child-state reloads.
Expected decisions,slots and helper outputs are never inputs. Raw aliases and
canonical/indexed domains validate before allocation;bad reached stored-selector
side mappings diagnose rather than underflow.

`[designed]` No original prompt/editor is reached in this reference. Their static
pending/keeper/time/capacity gates are retained;an eligible6A7A/D5D6 UI path
diagnoses explicitly instead of fabricating a response. This is the supported
fixed-reference closure,not arbitrary managed-UI parity. Twelve synthetic tests
cover capacity wrapping,managed gates,side/reverse-scan order,precise mask20 and
signed byte thresholds,time/event boundaries,stored selector identity,reserve
break,live counter mutations through both replacements,missing/aliased inputs
and whole-image ownership. Seed and unrelated pools are preserved.

`[designed]` All twenty-two release tools run locally;CI retains all Rust tests,
release build and its first five bridges under issue22. Earlier evidence,lock
and workflow remain unchanged. The immutable committed-source receipt includes
JSON/setup/native computation/observations,without an emulator/full-game speed
claim. Neutral78B7 input handling,its conditionalDB68 auto-switch and continuous
DD02/clock remain next;they are not assumed no-ops. This is not a continuous
complete match or playable shell. Commentary+clock remain the minimum,pitch the
preferred stretch,and broader parity remains under archaeology120/121.

T23 implements the explicit no-pending-key branch of78B7 and conditionalDB68.
`[confirmed static]` Listings78B7–79EC andDB68–DBC5 at archaeology
[`218490d`](https://github.com/diegoami/onenil-archaeology/tree/218490d846e365babdd158d09c5ab865124e30ed)
show that no key does not imply no state change:nonzero D1CC requestsDB68,
which can set quick/mirror modes and clear or normalize the auto-switch flag.
Quick mode,remaining zero or nonzero D0B9 block that transition. Raw nonboolean
bytes retain their original tests and writes.

```sh
python3 tools/replay_neutral_input.py --check
python3 tools/replay_neutral_input.py --benchmark --out records/T23
```

`[designed]` Fourteen immutable hand-specified cases pin this static contract,
with source listing hashes and explicit **zero new original observations**.
They first failed atdd9886a's stub(exit101,retained proof). Six additional tests
cover current-image reads,no-request behavior,DB68's direct hint path,guard/word
endpoints,extra/keyboard-field rejection and whole-image ownership. The adapter
writes only D074,D075 andD1CC;seed,clock fields,scene-code mode,other pools and
raw configuration remain unchanged. The pointed speed byte shares the canonical configuration object at
config+2 afterT24 integration,disjoint from scene-code mode and pairing/table pools.

`[designed]` Speed status,speaker calls and quick-screen buffer refresh emit
structured presentation events in source order. Buffer-refresh arguments are
semantic(1,0),not push order. Original formatted text,hardware,display/log scratch
and timer parity remain outside the adapter. Keyboard acquisition is an explicit
caller responsibility:this API represents no pending key and rejects keyboard
fields;it does not silently accept keyboard-present paths. A borrowed-image
entrypoint allows the future whole-step caller to retain its current state.

`[designed]` All twenty-three local release tools pass;this count includes the
static-only adapter check,not23 original corpora. CI retains all Rust tests,
release build and its first five bridges under issue22. Prior evidence,lock and
workflow remain unchanged. The immutable committed-source measurement identifies
its static-only limits and makes no original-call or emulator/full-game claim.
Next is a complete initial read set and continuous DD02/clock acceptance from
one state,then native kickoff/setup and a commentary shell. No per-step fixture
backend or hardcoded match score is permitted. A complete playable match remains
unproven;commentary+clock minimum,pitch graphics preferred stretch.

T24 gives all native slices one canonical logical pool layout before continuous
DD02 integration. `[designed]` Earlier standalone slices used different player/
club offsets,and prefix history overlapped the newer player pool. Six retained
cross-slice failures at63ba06f expose those mismatches. Three additional failures
at1a0d646 expose staff/count aliases. These are native integration tests,reusing
existing decoded observations;they add no original dynamic census.

| Logical storage | Canonical offset |
| --- | --- |
| DS named fields/tables | 00000 |
| P8 players | 10000 |
| P15 players | 20000 |
| Clubs | 30000 |
| Pairing locals / scene-code mode | 40000–4002A / 40080 |
| Configuration / speed | 49000 / config+2 |
| Shared staff/count table | 50000 |
| History | 60000 |
| Image end | 70000 |

`[confirmed]` Ratings and assignment both call original4A37 and read DCC8's
same staff entries. Their raw club indices shift as words;high index bits alias
within the shared64K pool. The main task definition was corrected at1b44470
from the checked ratings/rebuild contracts. Public bounded staff-input validation
is unchanged. Canonical player/club views now agree in both directions,and
history writes preserve player bytes. Synthetic far descriptors use the same
layout;original far metadata returned by existing contracts remains unchanged.
Central range/end-point tests check disjoint pools while intentionally sharing
the staff/count views. Full-image ownership still checks every allocated byte.

```sh
cargo test --manifest-path engine/Cargo.toml shared_image_ --locked
python3 tools/benchmark_shared_layout.py --check
python3 tools/benchmark_shared_layout.py --benchmark --out records/T24
```

`[designed]` All23 existing release replays must retain exact decoded results;
the mixed smoke reuses original prefix/extra-event/substitution and static-neutral
cases. Its receipt measures independent public calls,not a continuous match;
actual same-image agreement is tested in Rust. Earlier fixtures,provenance,
receipts,lock and workflow remain unchanged. No new original observation or
emulator/full-game speed claim follows from this storage migration.

`[designed]` Next is the first complete initial read set and continuousDD02/clock
acceptance from one state,then native setup/kickoff and the commentary shell.
Per-step captured states cannot serve as runtime inputs. First playable remains
unfinished;clock/commentary minimum,pitch graphics preferred stretch.

### Continuous first-half debug session (T25)

[confirmed: archaeologyT36 at1415668,nativeT25] One `NativeQuickSession`
admits a named original initial state once;successive `step()` calls take no
inputs. The Rust image/RNG executes ratings,contest,placement/goal effects,
prefix,extra-event handlers,substitution dispatch or goal reset,neutral input
and caller clock. The fixed reference matches46 complete transitions through
the actual halftime return,including three goals/resets and every RNG draw.
No intermediate expected state enters the session.

```sh
.cache/venv/bin/python tools/replay_continuous_step.py --check
```

[designed] This is a debug admission after setup,not the playable shell. The
session stops when the clock returns its first real boundary;it does not use a
46-step runtime loop. Native258-draw setup,halftime/second-half control and the
later actual AI substitutions still need end-to-end integration before a
complete native match can be accepted. Running commentary and a preferred
simple pitch follow that complete match under issue35.

The admission joinsT36 with the first T12/T15 contest/rating inputs for the
selected chance8 and club-byte55 read sets,not later fixture states. Remaining
numeric queue rows use the original86E9 FF initialization,checked in
`seg_0AB9_14.asm:8700–870E` at archaeology218490d and separately hash-pinned.
Earlier source fixtures,receipts,lock and workflow remain unchanged.

### Continuous full-match debug session (T26)

[confirmed: archaeologyT37 at461ceb7,nativeT26] `NativeMatchSession` now
continues the same image through halftime and full time. One admission,then
input-free steps and an explicit `continue_without_changes()` action match all95
original DD02/clock/RNG boundaries,the five caller projections,both actual
substitutions and the final1–3 result. The expected result is verification only.

```sh
.cache/venv/bin/python tools/replay_continuous_match.py --check
.cache/venv/bin/python tools/replay_continuous_match.py --benchmark --out records/T26
```

[confirmed:T37/F14/F25] Halftime restores unsigned condition1..30 by2,
runs both managed164F calls and actual50EB/quarter/cap writes,then resolves
kickoff/physical availability from current state. The explicit presentation
adapter acknowledges the board with Space. Full replacements also call0F94
before rearranging the roster,so numeric queue/count changes reach the caller
clock. Code0's fixed-profile weight is0,derived from its unchanged weighted
actor total. Standalone bounded replacement APIs retain their old presentation
projection. Later snapshots never enter the engine.

[designed] This remains a fixed-reference debug session after setup. Native
258-draw setup,supported-session breadth,and the commentary/pitch shell and
owner review remain first-playable acceptance work. A local viewer of this
prepared session is the next visible deliverable. Receipts measure release
admission/native progression/serialization only,with no full-game speed claim.

### Run the local match viewer (T27/T29)

After building the Python extension with the installation steps above:

```sh
.cache/venv/bin/python tools/play_match.py
# Open http://127.0.0.1:8040 ; Ctrl-C stops the server.
```

[designed] Kick off,pause/resume,change pace,restart,or explicitly continue
at halftime. The running clock,score,goals/scorers,actual substitutions and
phase commentary follow one continuing Rust session. The pitch reads actual
signed formation coordinates and current player identities;the shell mirrors
the two teams for display. A gold ring identifies the engine's match focus.
There is no invented ball tracking or watched-engine animation. Names are
narrowly decoded facts with source digests in `demo/metadata.json`;all graphics
are newly drawn SVG/CSS. No original game assets are required.

[designed] Choose Blackburn's reference XI (Newell starts) or alternate XI
(Warhurst starts), and seed64 or122. Click **Prepare match**,then **Kick off**.
Pause/resume and choose the pace; explicitly continue at halftime. Restart
preserves your choices and runs fresh native preparation. Controls lock after
kickoff; restart to change them. The pitch and lineup show the actual chosen
players before kickoff.

The runtime admits only selected-squad `demo/exhibition.json` and factual names;
Rust computes setup and the match on one continuing image. Python structurally
links roster aliases and raw rating witnesses for the chosen XI. It loads no
replay/expected stream or original game asset. The fixed Sheffield Wed–Blackburn
fixture supports four XI/seed combinations, all smoke-checked through full time.
The original reference retains exact setup/match replay; alternate sessions are
completion smokes, without additional original parity claims. Watched-engine
animation and career/season systems are outside this preview. Owner-run review
under issue35 remains the final acceptance step.

Checks:

```sh
.cache/venv/bin/python tools/test_play_match.py
# Optional real-browser smoke (separate test dependency):
.cache/venv/bin/python -m pip install playwright==1.58.0
.cache/venv/bin/python -m playwright install chromium
.cache/venv/bin/python tools/smoke_match_viewer.py
```

### Native selected-squad setup (T28)

[confirmed: archaeologyT38 at d9e20a4 and nativeT28] `NativeExhibitionSession`
admits `demo/exhibition.json` once, computes the 258 setup draws and continues
through the same Rust image to full time. It reads no expected boundary or
original executable at runtime. The input contains selected squads, raw readonly
player/table facts, configuration and explicit incoming legacy state. In
particular, old seed/selectors remain inputs to native reseeding; the initial
clock, scenes and caches have not been replaced with prepared kickoff values.
Career automatic squad/colour selection is supplied as the chosen squads and
presentation attributes. Match income/RNG is computed; career ledger persistence
is outside this exhibition.

```sh
.cache/venv/bin/python tools/replay_match_setup.py --check
.cache/venv/bin/python tools/replay_match_setup.py --benchmark --out records/T28
```

The check compares all 258 setup draws and 11 setup boundaries (the first two
compare seeding-owned fields), then all 95 original DD02/clock/RNG boundaries,
actual halftime, both replacements and the original final result from that same
session. Missing/inconsistent data and reached unsupported branches diagnose.
The immutable settings observations under `records/T28` read original config+F6
and untouched queue-zero cells at the exact T38 admission/255-draw prefix.

The T29 browser viewer uses this API with two supported starting XIs and seeds.
Controller checks cover all four combinations; Chromium checks reference and
alternate full matches, actual player/seed handoff and desktop/mobile controls.
Owner-run milestone35 review remains before formal first-playable acceptance.

### Watched-engine port: movement legality (T30)

[designed: owner decision2026-11-03] Detailed player-action commentary requires
porting the original watched engine. The current browser uses the quick-result
backend and remains a summary preview. Generic clock messages do not meet the
owner's requirement for passes,tackles and shots.

[confirmed: archaeologyF39/T39 at c7637bd and T30 captures] The first native
watched component computes6F98 candidate movement and legality:reset from the
previous position,wrapped coordinate-byte movement,bounds/guard order and
actual occupancy-tag/membership checks. Its bridge replays128 real original
calls:64 kickoff-positioning cases and64 active cases,including33 actual
occupancy reads. Tag5 and blocked-occupancy branches have static/synthetic
coverage;the captured active cases all allow movement. No RNG is drawn.

```sh
.cache/venv/bin/python tools/replay_watched_movement.py --check
.cache/venv/bin/python tools/replay_watched_movement.py --benchmark --out records/T30
```

Inputs contain complete raw scene/global/role/blocking facts and an optional
indexed occupancy witness. Rust computes the new coordinate before looking up
that witness;the witness is not an expected position. Wrong/missing reached
lookups diagnose. Expected scene/result snapshots are assertions only. Whole
native-image checks permit only the scene's position word to change.

This kernel does not yet run the watched match. Original70D5 direction/retry
selection,72BB player/action dispatch,ball work and the watched clock remain
before a continuous player-action feed can replace the summary viewer. Captured
original action sequences are test references,never the runtime backend.

### Watched direction selection and retries (T31)

[confirmed: archaeologyF39 at c7637bd,70D5 static at d9e20a4,and T31 original
captures] The next native parent adjusts the target's high coordinate byte,
chooses an axis using the original coin draw,and retries6F98 on the same image
until movement is allowed. Root inputs contain raw occupancy cells;directions,
candidates and helper outcomes are computed in Rust. All128 original parent
calls match full scene,seed,ordered random draws and movement attempts.
The warm64 contain110 attempts/46 bounds rejections;the active64 reach33
occupancy reads and all accept their first attempt. Neither captured stream
changes the target clamp;clamp guards and blocked occupancy retries have
synthetic/static coverage. These observations do not establish dynamic shot
or goal behavior.

```sh
.cache/venv/bin/python tools/replay_watched_direction.py --check
.cache/venv/bin/python tools/replay_watched_direction.py --benchmark --out records/T31
```

[designed] This is a standalone movement parent. Player/action dispatch72BB,
ball state and the watched clock still need porting and integration before the
browser can produce the requested detailed commentary. The current preview
remains the quick-result summary.

### Watched action-one target preparation (T32)

[confirmed: archaeologyF39 at c7637bd and561E/helper listings at d9e20a4]
Rust now computes561E's role-indexed target offset,byte wrapping,1F5E
orientation and selector/actor overrides on one image. Arrival preserves the
original hold/actor guards and can change the action and call1DB7(49).
The existing native0F94 numeric routine owns player identities,weighted totals,
queue acceptance/deduplication/counts and clock-pair copies. The input carries
raw table/identity/roster facts;no target or commentary outcome is supplied.

```sh
.cache/venv/bin/python tools/replay_watched_target.py --check
.cache/venv/bin/python tools/replay_watched_target.py --benchmark --out records/T32
```

[designed] This is one preparation action within72BB. Other actions,ball effects
and the watched clock/tick driver remain before detailed player-action
commentary can run in the browser. Numeric code49 alone does not deliver that
experience;the summary preview remains unchanged.

[confirmed: T32 capture-r0003] The complete admission stream contains592561E
calls through the first active tick48/draw943,including all22 arrival
transitions and one accepted code49 event at tick12(primary slot10,secondary0,
side0). All full projected states and numeric effects match Rust. The separate
64-call warm fixture overlaps this sequence;656 replay cases represent592
unique original calls. Original deduplication,queue wrapping and nonzero
weights are not exercised by this stream;synthetic/static tests cover them.
The aborted active-only run is retained and excluded from accepted evidence.

### Original kickoff readiness and first ball launch (T33)

[confirmed: archaeologyF39/c7637bd,56EE/2625/helper listings at d9e20a4,
T33 capture-r0001] Rust computes the initial action-two kickoff on one image:
22-scene readiness with real availability checks,target orientation,wrapped
ball direction/distance/endpoint,mode selection,native1BD6/1AE2 skill and RNG,
activation and all22 action5 stores. No readiness,skill result,vector or random
value is supplied. Every35 original56EE calls through first active tick48
matches:34 wait returns and one successful launch,using one draw at1BF1
(bound84,value19). The observed launch is mode6,direction(-1,0),distance2,
endpoint(-2,0). Mode7/8 and failed-skill RNG branches have synthetic/static
coverage only.

```sh
.cache/venv/bin/python tools/replay_watched_kickoff.py --check
.cache/venv/bin/python tools/replay_watched_kickoff.py --benchmark --out records/T33
```

[designed] This is the initial inactive-entry gameplay projection. Active-play
launches diagnose until their additional effects are ported. Original sound7
is retained as a raw fact;PC-speaker I/O and presentation countdown/timer
fields D088/D5A/DE26 remain outside this projection. Matching these gameplay
fields does not claim full physical sound/timer execution. Other watched
actions,ball flight and clock/tick/viewer integration remain before detailed
browser commentary is available.

### Original in-bounds ball flight and pitch transport (T34)

[confirmed: archaeologyF39/c7637bd and7423/1998:081B/084B/0A16/0B3F at
d9e20a4] Rust computes independent wrapped coordinate movement,distance decay,
the original low-word flight metric and air/glyph guards. Logical pitch
transport restores an old7/9 marker from the indexed background and stamps
the selected glyph word where foreground equals background. Player movement
legality and ball transport use the same canonical main occupancy image.
Raw indexed main/background cells are inputs;no bounds,metric,coordinate or
helper result is injected. Missing reached cells diagnose.

```sh
.cache/venv/bin/python tools/replay_watched_ball.py --check
.cache/venv/bin/python tools/replay_watched_ball.py --benchmark --out records/T34
```

[designed] This covers the in-bounds gameplay/occupancy branch. Outside-pitch
restarts diagnose pending their original routines. Pixel/audio/timer transport
is external;the logical occupancy writes used by gameplay stay inside this
projection. Other watched actions,restarts and clock/tick/viewer integration
remain before detailed browser play-by-play is available.

[confirmed: T34 capture-r0002] All32 original7423 roots through tick107/draw3666
match full ball/main/background/metric/bounds/seed/draw observations. They
include14 air-flag entries,28 metric calls,64 main/58 background indexed
witnesses and56 changed main cells;no RNG is drawn. All32 remain inside the
playing pitch. Signed metric extremes,bounds rejection and other guard
endpoints have synthetic/static coverage. Aborted r0001 is retained/excluded:
the observer initially failed to wrap effective pitch-buffer offsets to16 bits;
accepted r0002 uses corrected segmented addresses and verifies every indexed
read against the original ES:DI address.

### Compiled watched target-adjustment graph (T35)

[confirmed: archaeologyF39/c7637bd and pinned static listings at d9e20a4]
The complete35F8 graph is compiled into native Rust basic blocks/direct calls:
15 procedures,584 original instructions,including tactic membership,directional
comparisons,offset limits and actual0CDC RNG. Temporary Pascal frames are native
scratch storage;gameplay data stays in the existing Engine image. The runtime
loads no original executable and has no opcode decoder or instruction stream.
Python only translates listing structure and observes original executed operand
addresses;it computes no player decision or helper outcome.

```sh
python3 tools/compile_watched.py --research-root ../onenil-archaeology --check
.cache/venv/bin/python tools/replay_watched_adjustment.py --check
.cache/venv/bin/python tools/replay_watched_adjustment.py --benchmark --out records/T35
```

[confirmed: T35 capture-r0004] All64 original35F8 roots through tick50/draw1032
match full scenes,seeds and ordered draws. Both positive and negative offset
changes occur;no RNG is drawn by these64 roots. Synthetic/static tests cover the
conditional coin,tactic overrides,wrapping and arithmetic/frame boundaries.
Earlier aborted memory-hook captures are retained/excluded;accepted observation
uses stable code hooks and decoded effective addresses. Only root-entry readonly
data facts are supplied;missing reads or uninitialized scratch diagnose.

[designed] This mechanism supports the remaining native action ports. It is one
shared helper graph;full action5,watched ticks,clock and viewer integration still
remain before detailed browser play-by-play is ready.

### Watched target selection (T36)

[confirmed: F39/c7637bd and pinned static d9e20a4] The complete3846
selection graph compiles24 procedures/977 instructions into direct Rust blocks
and calls. All64 original roots match full scenes,seeds and72 ordered draws;
whole-image checks permit only root8/9,12,20/21 and the seed to change.

[confirmed: T36 capture-r0001 and frames-capture-r0001] The inactive goalkeeper
path reads BP-5 without initializing it. Standalone selection therefore requires
one raw entry ABI byte (`frame_byte5`),captured at SS:(entrySP-7). A second
original run joins every boundary,full scene,seed and ordered draw to the first;
its raw byte supplies no helper outcome. Values0 and nonzero have separate
native tests. Other uninitialized scratch reads still diagnose. The old
adjustment schema remains strict and unchanged. Full native callers must carry
their own continuing frames;these kernel facts are never per-step match inputs.

```sh
python3 tools/compile_watched.py --research-root ../onenil-archaeology --root 0AB9:3846 --out engine/src/watched_compiled/selection.rs --check
.cache/venv/bin/python tools/replay_watched_selection.py --check
.cache/venv/bin/python tools/replay_watched_selection.py --benchmark --out records/T36
```

[designed] This closes target selection only. Action dispatch,continuous watched
clock/phase flow and detailed browser commentary remain under milestone35.

### Watched passing and contact actions (T37)

[confirmed: F39/c7637bd,static d9e20a4 and T37 original oracle] Native448C
and4DFF match all3768 original calls during watched ticks47–128,including2307
ordered draws and25 accepted commentary entries. The events include actual
passes,receptions,runs,tackles,ball wins,evasion,interception and collection.
Scenes,seeds,all declared gameplay changes and logical pitch occupancy compare
against independent original entry/exit observations. This closes these action
kernels;it does not establish a continuous watched match or update the browser.

[designed] Inputs are strict named decoded entry facts,original register/stack
context and typed pointers in one native data image. Missing reads/first-write
baselines diagnose;the exporter may select facts only from original ENTRY RAM.
Expected EXIT values and helper outcomes never enter the engine. Public fixtures
need no original bytes. Private original snapshots are archived in the owner's
`1-0` Drive folder as `T37-oracle-r0001.tar.gz`;their digests/provenance are in
`records/T37`. The runtime loads no original executable or opcode stream.

```sh
.cache/venv/bin/python tools/replay_watched_action.py --check --replay
.cache/venv/bin/python tools/replay_watched_action.py --benchmark
python3 records/T37/validate_provenance.py
```

[designed: main amendment8fdce1d] Numeric0F94/0E45/0F42 remains native.
09BD text drawing and1998:00D6/01E0 sound waits are presentation boundaries;
waits emit native duration requests. D088 countdown and ISR phase0D5A cannot be
action inputs or outputs. Other reached unsupported instructions fail explicitly.
Clock/ISR composition,remaining action/shot branches,continuous watched ticks
and detailed browser commentary remain required by milestone35.

[designed] With the private archive downloaded to `.cache/T37/T37-oracle-r0001.tar.gz`,
restore its `oracle-r0001` directory and repeat the independent entry/exit fact
and whole-gameplay ownership checks:

```sh
tar -xzf .cache/T37/T37-oracle-r0001.tar.gz -C .cache/T37
.cache/venv/bin/python tools/replay_watched_action.py --check-private
```

## Native watched actor dispatch

[confirmed: T38 original-parent corpus and margin-r0001] All50 isolated72BB
parent transitions match the original,including all11 dispatched action modes,
computed shots and headers. The32 ordinary admissions are derived from retained
448C entries and source-proven parent register/stack liveness;they are not
captured live parent entries. Other cases use explicit mode or original tick-entry
player/ball overlays. Each expected result comes from original execution.

[confirmed: original1998:087B and margin-r0001] Set-piece occupancy reaches the
logical pitch's outer rows. Actor inputs admit19rows;older action/contact
inputs retain their17row bounds. The style pointer supplies one logical glyph
mode byte,which affects occupancy;it supplies no font data. Unknown reads and
writes without original-entry baselines still fail. Direct Rust procedures
execute arithmetic,branches and numeric commentary without an opcode runtime.

```sh
.cache/venv/bin/python tools/replay_watched_actor.py --check --replay
.cache/venv/bin/python tools/replay_watched_actor.py --check-private
.cache/venv/bin/python tools/replay_watched_actor.py --benchmark
python3 records/T38/validate_provenance.py
```

[designed] Private validation needs the earlier original entry cache;public
checks and native fixtures do not. Original admission proof validation can be
repeated with the research virtualenv's Python and
`tools/watched_parent.py --prove --research-root ../onenil-archaeology`.
The microkernel benchmark includes parsing,image construction and serialization;
it does not measure a continuous tick or playable match. Continuous7A02 state,
logical7B7B clock progression and detailed viewer commentary remain required.

## Persistent watched ticks

[confirmed: T39 original tick captures and repeated-root corpus] A native
`TickSession` keeps one admitted memory image across successive original7A02
calls. Eight captured original tick transitions and an independent eight-call
sequence match all declared gameplay,22scenes,pointers,seeds,ordered draws and
accepted events. Only the initial ENTRY fact union enters the native session;
`advance()` accepts no fixture state or outcomes. A missing input closes the
session rather than allowing a partial tick to be retried.

[confirmed: T39 neutral-input proof] All176 observed player visits take the
original no-pending-key branch. This explicit external input preserves the
existing native mode-switch writes while status rendering/speaker requests
remain presentation events. Strict unknown reads,first-write baselines,typed
pointer aliases and raw stack guards remain enforced.

```sh
.cache/venv/bin/python tools/replay_watched_tick.py --check --replay
.cache/venv/bin/python tools/replay_watched_tick.py --benchmark
.cache/venv/bin/python tools/replay_watched_tick.py --check-private
python3 records/T39/validate_provenance.py
```

[designed] Private validation uses restored original capture pages from the
owner's private project artifacts;public validation/replay needs no original
bytes. The repeated-root sequence omits the outer caller and clock. Logical7B7B
clock progression,full caller/match composition and the detailed commentary
viewer remain required by milestone35;these kernel checks do not establish a
complete playable watched match.

## Continuous watched caller and clock

[confirmed: T40 original caller corpus and clock cases] `CallerSession` composes
171 actual original caller iterations,including watched actions,occupancy,
offside work and the original logical clock. One initial admission supplies all
state; `advance()` accepts no intermediate inputs. The sequence reaches four
elapsed game minutes and33 accepted actions,including the retained32-action
prefix. Eight independently derived original clock cases cover halftime,
fulltime,stoppage,saturation and16-bit wrap. These are boundary tests;the full
match lifecycle and detailed commentary/pitch viewer remain milestone35 work.

```sh
.cache/venv/bin/python tools/replay_watched_caller.py --check --replay
.cache/venv/bin/python tools/replay_watched_caller.py --benchmark
.cache/venv/bin/python tools/replay_watched_caller.py --check-private
.cache/venv/bin/python tools/replay_watched_caller.py --check-research ../onenil-archaeology
python3 records/T40/validate_provenance.py
python3 tools/watched_caller_oracle.py --research-root ../onenil-archaeology --capture --clock-cases
```

[designed] Public fixtures store one ENTRY fact union and pinned original sparse
changes. The checker reconstructs every declared gameplay cell for each step;
these expectations enter only assertions. Public replay needs no original bytes.
Private validation additionally compares the original RAM exits. Completed
capture commands validate retained records without rerunning the original;
incomplete captures require their recorded observer version and private inputs.
The release benchmark includes admission,all computed steps and serialization,
with one persistent native image per trajectory;it measures this caller slice.

## Complete reference watched match

[confirmed: T41 full original reference,archaeology F40/60179ee] The native
watched lifecycle now matches all4,199 original entry/return snapshots,182,553
computed random draws and831 accepted actions through halftime,fulltime and
the actual0AB9:EBE2 return. One frozen post-setup ENTRY admission supplies all
initial facts;no intermediate state or helper outcomes enter the runtime.
The258 earlier setup draws remain admitted context and are not computed by
this lifecycle. Native selected-lineup/seed setup,additional complete supported
sessions and the detailed commentary/pitch viewer remain milestone35 gates.

```sh
python3 tools/replay_watched_match.py --check
.cache/venv/bin/python tools/replay_watched_match.py --replay
.cache/venv/bin/python tools/replay_watched_match.py --benchmark
python3 records/T41/validate_provenance.py
python3 records/T41/validate_presentation_provenance.py
.cache/venv/bin/python records/T41/validate_native_guards.py
cargo test --manifest-path engine/Cargo.toml --test watched_match --locked
```

[designed] Public checks and native replay require no original game bytes.
Private provenance validation uses the research virtualenv's Python with
`tools/replay_watched_match.py --private` after restoring private capture pages.
The release benchmark measures parsing,the persistent native lifecycle and
output serialization from the pre-serialized admission. It makes no emulator
speed comparison. The reference's legacy `scores` field is scratch during play;
running scores use nativeD0C4/C5,with the source-defined final side mapping.

## Watched commentary and pitch preview (T42 baseline)

[confirmed: T42 browser/controller checks] The local watched viewer presents
all831 accepted native actions over4,199 ticks,including named passes,tackles,
shots,save/block/deflection outcomes,goals and substitutions. Its22 players and
ball use the shared native packed-coordinate pitch;it applies no team mirroring
or invented movement. Live scores are observed directly atD0C4/C5 after each
watched tick. A caller event's newer timestamp advances the displayed clock
when emitted between the tick return and the next entry. Half-time stops
playback;fulltime consumes the complete native post-whistle run.

[designed: historical T42 delivery] Each server startup computed one Rust match from the pinned initial
admission and caches only that fresh native result for playback/restart.
The runtime never reads original reference outcomes,executes the original game
or models its actions in Python. It reads the initial numeric facts and the
existing narrowly decoded factual name map. T46 below replaces this fixed
post-setup admission with selected-squad native preparation.

```sh
.cache/venv/bin/python tools/play_watched.py --port 8041
.cache/venv/bin/python tools/test_play_watched.py
.cache/venv/bin/python tools/smoke_watched_viewer.py --out records/T46
python3 records/T42/validate_audits.py
```

[derived: original09E8/09EB and static data table atd9e20a4] Codes28–31 mean
possession,save,block,deflection respectively. This direct indexing corrects
the earlier prose summary,tracked in archaeology issue134. Markers1/2 select
primary/secondary identities;secondary is not automatically a pass receiver.
Browser acceptance checks detailed actions,pitch movement,clock/score,pause,
halftime/continue,fulltime,restart,desktop/mobile and local request guards.

## Native selected-squad watched preparation

[confirmed: T43 original preparation and complete first-entry join] The
`NativeWatchedPreparation` API computes258 ordered setup draws and11 watched
preparation boundaries from selected-squad/readonly/legacy inputs. Every field
of the first watched entry's22scenes,ball,clock,actor and seed matches the
complete original reference. The owned prepared Engine remains in the object;
inspection neither advances it nor reloads an expected snapshot. The input
contains T28's initial selected data and a typed pre-seeding supplement for
ball fields preserved by1998:09E9 and incoming scratch-score aliases.
It contains no captured setup/helper outcomes or T41 post-setup admission.

```sh
.cache/venv/bin/python tools/replay_watched_setup.py --check --replay
.cache/venv/bin/python tools/replay_watched_setup.py --benchmark
python3 records/T43/validate_provenance.py
python3 records/T43/validate_audits.py
```

[designed] Private validation reruns the original only through the first
watched entry and compares the decoded inputs/assertions,without persisting
new RAM archives. Use the research virtualenv's Python with
`tools/watched_setup_oracle.py --research-root ../onenil-archaeology --private records/T43/original-r0001/result.json`.
The release receipt measures preparation,observation serialization and drop;
it is not a full-match benchmark. T44/T45 below continue this prepared image
through the watched lifecycle. Independent full-field/clock proof,additional
complete lineup/seed sessions and viewer input controls remain required before
milestone35 acceptance. The reference commentary/pitch preview can be launched
on port8041 with the command above.

## Native prepared watched caller

[confirmed: T44 original support and headless-original-r0001 prefix]
`NativePreparedWatchedSession` moves the selected-squad preparation's tracked
native image into compiled E008/7D86/7C3F. It computes the first watched entry,
then advances171 original caller iterations without further inputs. The22 player
scenes,ball,7280 ordered setup-and-caller draws,33 watched actions and every
per-iteration caller clock field match the original four-minute passage.
The source coin-toss event59 is also computed during setup and remains tick0.
No captured first-entry image or intermediate outcome is an input.

```sh
.cache/venv/bin/python tools/replay_prepared_watched.py --check --replay --python
python3 records/T44/validate_provenance.py
```

[designed] Its frozen initial admission contains the existing selected-squad
inputs and209 decoded pre19CA support facts. Source-owned initialization leaves
other memory unknown;reads are checked throughout,with first-write baselines
checked during play. The pitch background is generated from numeric cells;
the original field-initializer experiment preserves all171 measured iterations.
Pointer aliases,return frames,no-pending-key admission and unsupported quick or
phase transitions are checked. `snapshot()` and `observation()` inspect the
retained session;`step()` advances it. At171 steps,the API reports the prefix
boundary and retains its native continuation. The T45 `finish()` API below adds
fulltime continuation;verified lineup/seed choices and viewer controls remain
required for milestone35.


## Native prepared full-match completion

[confirmed: T45 complete-return-parity-r0001] The prepared session's `finish()`
continues the same owned guest frames from fresh setup or the retained171-step
boundary to the actual `0AB9:EBE2` return. Both paths produce identical output,
matching all4199 original ticks,182811 computed setup-and-match draws and831
watched actions. It accepts only initial selected-squad inputs and1770 decoded
support facts checked against their original pre19CA values. Current actions,
RNG outcomes and intermediate images remain computed.

```sh
python3 tools/replay_prepared_match.py --check
python3 tools/replay_prepared_match.py --selftest
.cache/venv/bin/python tools/replay_prepared_match.py --python
```

[confirmed: T45 original-r0003,full-original-proof-r0001] The independent original
full-field preservation run completes all4199 ticks,182811 draws and831 actions
through the actual outer return. All4101 actual caller-clock rows match the native
result in every field,including the counter and halftime/fulltime tick identities.
Its immutable manifest is pinned by the public checker;`--check` and normal replay
require the complete proof. The explicit `--development-reference-only` diagnostic
compares the existing match reference and known171-clock prefix;it is not the
strict acceptance command.
Exact measured observer sources are frozen under `records/T45/preservation-observer-r*.py`
so evidence checks survive squash merges and branch deletion. Those copies verify
source identity;launch captures through `tools/prepared_match_oracle.py`.
Supported additional lineup/seed sessions and viewer controls remain the next
acceptance task for milestone35.


### Stopping and recovering the original verifier

[designed] Run original observations with the research virtualenv's Python.
`tools/prepared_match_oracle.py --checkpoint-seconds 60 --stop-after-seconds 600`
saves a private machine snapshot at safe guest boundaries and pauses after the
time limit. `--stop-file` instead pauses when its specified file exists. A pause
has no completed result/manifest and cannot satisfy full-match acceptance.

The snapshot contains original-game bytes and stays outside git. Before shutdown,
copy receipt-referenced snapshots to the research repository's ignored persistent
`.cache/native-recovery/T45/`, preserving their native-root-relative paths and
verifying each receipt's SHA256. This avoids relying on `/tmp` surviving reboot.
After recreating the native checkout,restore that cache with:

```sh
mkdir -p .cache
cp -a /home/diego/projects/onenil-archaeology/.cache/native-recovery/T45/.cache/prepared-match-checkpoints .cache/
```

Read the latest [checkpoint](https://github.com/diegoami/onenil/issues/1) for the
exact unfinished run and latest receipt. Pass that receipt to `--resume`,with
`--research-root /home/diego/projects/onenil-archaeology`;the observer verifies
source/reference/input identity,private checksum,cursors and restored RNG prefix.
A normal resume leaves prior records intact and continues the same machine.
`--fork-checkpoint` starts a separately recorded attempt from an authenticated
checkpoint in an already finalized attempt;it does not accept an aborted result.
The older schema1 observer has decoded progress only,so its progress journal
cannot restore the original machine. Use the checkpointed attempt identified
in the live issue,not that older progress journal.


## Selectable prepared watched viewer

[confirmed: T46 `records/T46/native-completions-r0004.json`,
`four-native-verification-r0001.json`, `T46-r0002.json`] The current watched
viewer starts with selected initial data,computes the original setup,and retains
one Rust image/RNG through the actual outer `0AB9:EBE2` return. Choose Newell or
Warhurst starting,and seed64 or122. Preparation runs in the background with a
visible indicator;the local HTTP server remains responsive. Kick off then plays
chronological named actions alongside actual native player/ball coordinates,
caller-clock minutes and live goals. Pause,halftime/continue,fulltime and restart
are checked on desktop and mobile for every choice. Restart replays only the
fresh native presentation computed by this running application.

[designed] Use one controlling browser tab per local server. Match state is
shared across tabs and API clients. Pause stops only this viewer's playback
requests; another tab or API client can still advance the shared match. Run
separate servers on different ports for independent viewers.

```sh
.cache/venv/bin/python tools/play_watched.py --port 8041
# Open http://127.0.0.1:8041; choose lineup/seed, Prepare match, then Kick off.
.cache/venv/bin/python tools/replay_prepared_choices.py --check --replay
.cache/venv/bin/python tools/test_play_watched.py
.cache/venv/bin/python tools/smoke_watched_viewer.py --out records/T46
python3 records/T46/validate_audits.py
```

[confirmed: T45 original-r0003 and T46 release-r0001] Reference64 matches
all4,199 original ticks,182,811 ordered draws,831 watched actions,4,101 actual
caller-clock rows and final state. Reference122,reserve64 and reserve122 finish
natively with3:0,0:0 and4:2 respectively;these establish completion support,
not additional original parity. Full-time whistle and post-whistle walkoff
follow native control flow rather than a fixed tick count.

[designed] Runtime reads only public initial numeric facts and decoded identity
metadata. It uses neither original executables nor captured post-setup states,
per-step inputs or expected outcomes. The additional source-owned initial facts
are readonly numeric descriptor/character-membership tables,not game assets.
Headless startup owns the original mouse-driver availability byte;compiled
input polling still executes. Native errors remain explicit.

[confirmed: `records/T46/release-r0001.json`] Release computation took
7.73–10.02seconds per complete supported match on the recorded host. This
measurement includes input preparation,native setup/full match,serialization
and decoding;it excludes GUI presentation conversion and does not compare
against emulator speed. Native match time is displayed during paced playback;
wall-clock playback speed is a prototype control. This is a four-choice
exhibition,with season/management systems and broader replacement evidence
still deferred. Milestone35 remains open for owner-run review and fixes.


The faithful desktop prototype also accepts `--choose-save`: choose native campaign
JSON files beside `--save`, using Up/Down and Enter. Escape keeps the original-style
list open; F9 cancels this native invocation. F7 saves to the successfully loaded
file. `--load` remains direct restore. See [campaign state](docs/campaign-state.md)
for supported routes and the remaining original startup/new-game work.
