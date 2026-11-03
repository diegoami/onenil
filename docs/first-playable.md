# First playable delivery target

[designed: delivery plan2026-11-02,following the owner's request for something
playable] The next product milestone is one native quick-result exhibition
match with running commentary and a preferred simple pitch view. Track live progress and acceptance in
[issue35](https://github.com/diegoami/onenil/issues/35). Routine/test counts are
regression evidence;they do not establish this milestone's completion.

[designed: owner clarification2026-11-03] Running commentary must describe actual
player actions. Generic "Play continues" messages and a goals/substitutions feed
do not satisfy this requirement. T29 delivered a quick-result summary preview;
detailed pass/tackle/shot events require the original watched/action-producing
engine. The earlier review prompt is superseded until this gap is addressed.

The player chooses a supported lineup and seed,starts the match,and receives
a running match clock, chronological text commentary, and a final score.
[designed: owner clarification,2026-11-02] Text commentary is sufficient for the
first playable. A simple pitch view is a desired stretch goal, not a requirement
for accepting the commentary-first version. Commentary and pitch graphics must follow actual native
match events rather than a scripted playback. One continuing Rust state/RNG stream must
produce setup,steps,halftime/termination and results. Captured per-step inputs,
expected helper results and the original executable cannot serve as the runtime
backend. Supported demo sessions must finish without unresolved requests.

## Delivery gates

| Gate | Exit evidence | Dependencies to close |
| --- | --- | --- |
| 1. Match-state changes | Every state-changing callee reached by the chosen reference match executes natively | Scene/cache codes;remaining164F rearrangement and A94B/AEB8 selection/replacement;availability/substitution paths reached by that match |
| 2. Continuous quick step | A whole DD02 transition and caller clock run from one input state,with no fixture-fed handoffs or unresolved reached requests | Shared canonical pools across existing slices;6B2F/78B7/input adapters and remaining logical effects;clear separation of proven rendering-only work |
| 3. Complete quick match | One initial state reaches a final result and matches a full original reference;repeat/alternate supported configurations finish | Squad/start-data decoding;19CA seed table;86E9 kickoff;actual7C3F/7B7B control/termination;complete event/state continuity |
| 4. Playable shell | A documented launch command and scripted player session choose lineup/seed,kick off,follow the clock/commentary,and show final results | Thin shell,event-feed formatting,optional simple pitch view,usable diagnostics,prototype scope documentation and owner-run milestone review |

[confirmed: nativeT28/T29] One selected-squad admission now computes original
setup and a complete continuing match:258 setup draws,11 setup boundaries and95
DD02/clock/RNG boundaries,actual halftime,two substitutions and final result.
The local browser offers a supported reference/reserve-starting XI and two seed
settings, with native setup for all four combinations. Controller smokes reach
full time for every combination. Chromium verifies reference/alternate matches,
actual chosen player/seed handoff,pause/halftime/restart and desktop/mobile views.
Alternate sessions establish completion support,not additional original parity.
The commentary/formation preview is available with the README launch command.
Owner-run milestone review in issue35 remains required before acceptance.

[derived: archaeology218490d,match-structure/step-tail contracts] Do not turn
one recorded92-step match into an invented fixed-length loop. Match setup,
RNG consumption and termination must follow the original's actual control flow.
The original watched/animated engine is separate and is excluded from this first
target. A simple pitch view may visualize native match data/events; it does not
establish watched-engine parity or authorize invented player movement.
Season fixtures,tables,transfers,finances and the full game shell remain later.

[designed] Close evidence gaps only as needed for the selected full-match path
first. Other23 replacement observations/six-week breadth remain explicitly
unfinished under archaeology120/121;this milestone does not waive those later
claims. Missing data or unsupported branches are blockers to the supported
playable session,never silent no-ops. Preserve previous evidence and tests.

Progress reports identify the gate advanced,the next end-to-end check,and any
new blocker. Only gate3 demonstrates an engine-ready complete match;only gate4
is called first playable. A calendar estimate should be revisited after gate2
has demonstrated the remaining full-match integration cost.
