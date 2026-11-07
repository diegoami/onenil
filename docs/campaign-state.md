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

## T56 desktop bridge

[designed] The existing software-window prototype now owns T55 state directly.
Its first/reserve lists contain20slots each and the actual original player IDs;
there is no parallel synthetic roster. F7 saves the complete campaign; F9 opens
the native load route (T57 adds a chooser when multiple saves exist). Default native path is `.cache/desktop-campaign/SAVE1.json`. Old experiment
saves remain separate and are refused by the campaign loader. `--save <path>` and
`--load` select/reopen a native file. Startup load refusal shows the initial state
with an explicit error; active F9 refusal preserves the complete active state.

[confirmed UI read map: research static004fb87b] Position strings at8724+19×role,
role glyph table8716, instruction alternatives85E8+32×row+16×choice and goalkeeper
alternatives0B70/0B90 supply bounded decoded labels. The lower instruction bits
and upper override markers follow0AB9:8B0B. Their public listing hashes and pinned
listing comparison are retained in assets/campaign-labels.json. Independent font
and custom role appearance remain proposed departures. No original font/template
buffers are embedded.

Cards follow the highlighted real ID. Captured extra detail/skill values are shown
only for the same source player/state; changed or unobserved cards show owned
name/age/current and club position/instructions and explicitly unavailable remaining
data. Changes invalidate captured team averages rather than displaying stale ones.
The application still lacks complete player models and management calculations.

PLAY NEXT GAME now opens the saved PSG preparation. PLAY GAME there reports that
campaign engine admission is pending and does not launch the Sheffield match.
The additional prototype key **E** opens a clearly separate reference64 exhibition;
it leaves every campaign field unchanged, including after full-time. This is a
working saved-club editor plus the accepted exhibition, not a played campaign week.
Original contextual move/cancel/card/return controls stay in place. The tactics
and reposition editors remain subsequent B3 work; the graphical proof explicitly
uses headless native commands to establish the recorded instruction/position values
before reopening the desktop, rather than claiming those UI editors exist.

Actual native-input/restart proof uses `desktop/faithful/campaign_window_check.py`;
`check_campaign_records.py <record>` verifies real IDs, complete original initial/
changed projections, invalid loads, distinct desktop process IDs, images and
unchanged campaign through exhibition, rejecting four corrupted records. Earlier
T52–T54 graphical records/checkers remain historical evidence for their producers.


## T57 saved-game chooser

[confirmed] Research W001/W006 at 68cd3396: the one-save case loads directly;
multiple saves start at the first filename. Up/Down moves the highlight; Escape
keeps the list and selected row; Enter loads. The decoded list/title layout comes
from authenticated T43/session-r0003 step13 at the T51 source pin6aca98ca.

[designed] F9 now discovers regular `.json` files beside the configured `--save`
path. With multiple files it opens that original-led list. Enter validates the
complete candidate before replacing the campaign, game name and F7 target together.
Invalid files preserve all three and keep the list open. F9 again returns from
this native shortcut; Escape follows the original observed self-loop. One file
loads directly, no files reports a message. Native discovery sorts case-insensitive
stems then paths, refuses more than1024 files, ignores symlinks, and scrolls nine
visible rows. Up at the first row and Down at the last row clamp in place: this
is also native design; W006 does not establish the original list-edge behaviour.
Those filesystem/navigation choices are not claimed as DOS behaviour. The list
shows18 filename characters and the original game-name box shows8; diagnostics
retain the full native target. Use a dedicated directory for native campaign JSON.

`--choose-save` explicitly enters this chooser on startup. `--load` still restores
the exact configured file directly; combining both options is refused. The default
prototype entry still opens the supplied saved-club seed. The original Y/N startup,
N/new-game filename/league/manager/club setup, club creation and DOS-save loading
remain B2 work. No final toolkit choice or campaign progression is implied.

## T58 original player controls

[confirmed] Research W015–W017 at68cd3396 and the original handlers/listings
identified by `records/T58/source-r0001.json` govern the bounded controls. From
Players, Space opens player options in Details, remembering the option highlight.
Reposition immediately cycles current role through all14 codes; a club goalkeeper
cannot use it. The guard checks club role, so a repositioned outfield player can
cycle through goalkeeper and onward to sweeper. Club position restores the stored
club role for age18 and above, leaving every other field unchanged. An already
restored position is a no-op, including the native revision.

Change tactics displays five instruction rows in Details, including Mark. The
selection among the first four rows is remembered; Up/Down selects, Enter changes the
supported goalkeeper Positioning row. W015's displayed cycle is `come out of box P`
→ `come out of box` → `stay in box P` → `come out of box P`. The original B271
handler changes only choice bit0/override bit4; on returning to inherited it also
copies choice bit0 from club instructions. Other packed bits are preserved.
Escape returns to options, then Players. Insert from instructions returns to
options in Skills; Insert within options toggles the card without changing state.
The P marker is presentation evidence, not a claim about match effects.

[designed] Only this exercised goalkeeper instruction action is enabled; other
instruction rows and Sell/Take off T.M/New contract/Fire report unavailable. The
availability column shows supported prototype actions, so those original actions
have x marks here. The fifth Mark row always shows `unavailable`, including on an
unchanged captured card; this explicit marker is native design. It remains outside
the four selectable management rows (original B271 disables Mark when0D91=0).
Mark targets, complete skills/details, team instructions and
management calculations remain missing. F7 saves changed positions/instructions;
F9 or startup --load/--choose-save restores them. The keyboard proof performs all
edits in the window; no headless campaign editing is used. Original changed-save
projection equality and a fresh desktop restore are checked.

This reaches a saved-squad editing checkpoint, not the full first management week
in #104. B2's original new-game/club selection and B3's unavailable/incomplete XI
admission still need their own accepted source/implementation. B4 requires the
actual PSG campaign's prepared engine state and result transaction; the separate
Sheffield exhibition still consumes its fixed source and posts no campaign result.
Toolkit decision #103 remains open.


## Milestone126 review clarifications (T59)

[confirmed] Research68cd3396 T46/session-r0001 steps44–46 already show the
current-role label edge on Pearce's Details card. His club role remains centre
back throughout: striker displays `stay back P` / `short passes`, goalkeeper
shows `stay in box P` / `throw the ball`, then sweeper returns to `stay back P` /
`short passes`. Original0AB9:8B0B tests player+6 (current position) and substitutes
tables0B70/0B90; the instruction editor calls the same Details presenter. These
existing observations corroborate label selection without claiming unobserved
outfield instruction edits or broadening their admission. T59's source checker
pins the capture/image/listing identities and verifies these named observations.

[designed] Headless `position`/`instruction` commands are diagnostic native
representation edits. The instruction command accepts any u8, including packed
patterns not established as reachable through the original UI. Packed availability
fields likewise are representation data. Neither those commands nor native JSON
validation establish original-UI reachability or DOS-save-format compatibility;
the desktop's bounded controls retain their separately verified transitions.


## Fresh supplied-campaign preparation (T60)

[confirmed] Archaeology T66, source record committed at dc9fa85 and independent
repeat at daf061a, captures the actual supplied SAVE1 preparation through watched
kickoff. `tools/replay_fresh_preparation.py` pins the unchanged original decoded
leaf and incoming supplement; import also authenticates the independent repeat.
The incoming projection uses source-owned scalar values and has no expected
kickoff state. Its adapter quick-contest count is zero because that unrelated API
parameter is unused by watched preparation; zero is not an observed chance count.

[designed] `NativeFreshWatchedPreparation` admits precisely this authenticated
incoming profile. It loads empty incoming scenes separately from the established
completed-match constructor, then computes preparation into a tracked continuing
image. Arbitrary edited XIs or changed source fields refuse. The legacy exhibition
constructor keeps its existing checks. This bounded entry does not expose a
campaign match button, full native PSG completion, result posting or week changes.

[confirmed] Original96 `seg_0AB9_14.asm:86E9..86FA` assigns D0C0 to DCC0+23.
T66 observes a different incoming mode byte before that assignment. Native loading
therefore preserves the incoming view until the native initialization boundary,
then selects the supplied configuration cell. Expected boundary outputs never
supply either value. Run the source/input check and the full preparation comparison:

```sh
python3 tools/replay_fresh_preparation.py --check
python3 tools/replay_fresh_preparation.py --replay
```

[confirmed] Original D5D6 chooses the displayed managed club by membership, while
1B55:321F calls50EB with display column1 and bonus flag0. Selected-side identity
and display column are distinct. Native editor preparation now derives that club
before computing display ratings; the old exhibition's displayed side is retained.
The public replay compares seeding/selectors at the two early career boundaries,
then every named preparation projection from pre86E9 onward, every ordered draw
and every watched first-entry field. Supplied career selection remains input;
this is not a port of the whole management-screen workflow.

## Continuing fresh watched caller (T61)

[confirmed] Accepted archaeology T67 at430e99223341082d550ade2246f050b33affe120
retains two independent original executions through all 3,825 ticks, 158,992
ordered draws, 748 player actions and actual 0AB9:EBE2 return. T68 at9a74b4f
authenticates the incoming support separately. The native reference under
`records/T61/reference-r0001` transports unchanged decoded leaves and all 60
independent chunk receipts; its checker needs no private bytes or game runtime.

[designed] `NativePreparedWatchedSession.fresh(input_json)` retains the exact
T60 preparation profile and moves its tracked image into the compiled caller.
It admits only support matching the unchanged incoming witnesses retained in
`records/T61/incoming-r0001`; a missing subset still refuses at its first unknown
read. `tools/fresh_watched_inputs.py` pins ten original T68 leaves, authenticates
each independent pair, reconciles overlapping requests and derives the incoming
fixture. No expected tick, action, score or final state seeds native memory.

[confirmed: T66 income inputs; original1998:0299] Current fixture kind is a
typed incoming field separate from raw fixture flags. The continuing image loads
both from preparation inputs and rejects conflicting support. [derived: original
1B55:142B..153F] The exact fresh week1/stage1 title path uses the existing
output-only string helpers; week<23 excludes its special1334 branch. The
presentation projection admits this case with the same caller, stack and output
address guards as the independently measured week6/stage0 case.

[designed] `tools/replay_fresh_watched.py` imports T67 only after archaeology's
full public checker accepts the independently repeated original return. Its
public reference checker pins the complete indexed stream, both observer starts,
the repeated completion and every chunk receipt. The replay compares all named
ticks (including phase/clock fields), ordered RNG draws, numeric player actions,
final state and actual return. Python API replay also checks that observations
do not mutate state and that finished sessions reject further execution.

```sh
python3 tools/fresh_watched_inputs.py --check
python3 tools/test_fresh_watched_inputs.py
python3 tools/test_fresh_watched_reference.py
python3 tools/replay_fresh_watched.py --check
python3 tools/replay_fresh_watched.py --replay --python
```

These checks establish the bounded caller contract. Campaign result persistence,
week advancement, arbitrary edited lineups and desktop campaign-match wiring
remain separate work.
