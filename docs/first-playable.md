# First playable delivery target

[designed: delivery plan2026-11-02,following the owner's request for something
playable] The next product milestone is one native exhibition driven by the original
watched engine,with detailed player-action commentary and a pitch view. The owner
explicitly chose that port over a newly designed action simulation on2026-11-03. Track live progress and acceptance in
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

## Updated delivery gates after the owner’s clarification

| Gate | Exit evidence | Dependencies to close |
| --- | --- | --- |
| 1. Watched state changes | Every reached movement,ball,collision,action/shot and commentary callee executes natively | Read the watched7A02/72BB path;preserve gameplay occupancy dependencies and raw read sets |
| 2. Continuous watched tick | One admitted image/RNG advances actual watched ticks and clock with no expected-state handoff | Compose real action dispatch,movement,ball work and input/timer boundary;retain explicit unsupported paths until closed |
| 3. Complete watched match | One initial state reaches full time and matches an original watched reference;additional supported sessions finish | Native watched setup,halftime/termination,substitutions and all reached events;quick-result parity is separate regression evidence |
| 4. Detailed playable shell | A documented command selects supported input and shows chronological player actions and outcomes | Actual player names/passes,tackles,shots,saves/goals;controls/pitch/rendering checks and owner-run milestone review |

[confirmed: nativeT28/T29,delivered quick-result baseline] One selected-squad admission computes original
setup and a complete continuing match:258 setup draws,11 setup boundaries and95
DD02/clock/RNG boundaries,actual halftime,two substitutions and final result.
The local browser offers a supported reference/reserve-starting XI and two seed
settings, with native setup for all four combinations. Controller smokes reach
full time for every combination. Chromium verifies reference/alternate matches,
actual chosen player/seed handoff,pause/halftime/restart and desktop/mobile views.
Alternate sessions establish completion support,not additional original parity.
The commentary/formation preview is available with the README launch command.
This quick-result baseline does not satisfy detailed commentary. Watched action
production is now required before a new owner-run milestone review.

[derived: archaeology218490d,match-structure/step-tail contracts] Do not turn
one recorded92-step match into an invented fixed-length loop. Match setup,
RNG consumption and termination must follow the original's actual control flow.
The original watched engine is now required for the requested player actions.
Its gameplay computations must be native;the existing formation view does not
establish movement parity. A simple pitch view may visualize native match data/events; it does not
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
