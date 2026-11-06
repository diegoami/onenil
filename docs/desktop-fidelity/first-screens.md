# First desktop screens: One-Nil reference study

[designed] These are evidence-linked design requirements, not new native screenshots
or completed UI. All research links below are pinned to accepted manual commit
`6aca98caeebc1b9b5410852139fa305462224a05`. Captured values identify the supplied
save scenario; they are not new-career defaults. The next visual mockups must be
read beside these references and distinguish implementation gaps from departures.

## D01 Weekly main menu

![Original weekly main menu](https://raw.githubusercontent.com/diegoami/onenil-archaeology/6aca98caeebc1b9b5410852139fa305462224a05/runs/T41/T41-r0002/S002.png)

[confirmed] [T41 record/S002](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/runs/T41/T41-r0002/record.json),
[startup W001](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/startup.md),
and [squad W011/W012](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/squad.md)
establish the composition and later exercised shortcuts. The reference has a
dark green ground and thin grey frames; bright cyan group labels/values, green
menu options, contrasting opponent information and a grey selected menu row.

| Region | Preserve in the proposed desktop screen |
| --- | --- |
| Left | Club/manager/money/league position/season above next opponent/manager/league and competition/venue. These remain visible while choosing an action. |
| Centre | Formation, first-team descriptive averages, separate rating bars, team value and the eight tactic rows. Insert cycles this region to club or staff information while the action menu stays visible. |
| Right | Game name above the original ordered main menu; PLAY NEXT GAME highlighted in this supplied-save arrival. Conditional Previous player follows the audited original boundary, not a permanently invented entry. |
| Footer | Contextual arrows/Enter hints, F7 Save/F10 Quit and Insert/Space shortcut information. Preserve screen-specific hints and actual meanings. |

[designed acceptance] Show the complete three-region hierarchy and readable
original data/labels at the selected desktop sizes. Arrows/Enter activate the
actual highlighted original action. Space opens the remembered Players view;
Insert changes information without navigating away. F7/F10 use the observed
save/quit procedures rather than treating every prompt as a standard OS dialog.
Verify persistent effects separately from visual matching. T48's tab bar,
synthetic club header and generic cash card do not supply this design.

## D02 Players: skills, details and carried-player move

![Original Players with move active](https://raw.githubusercontent.com/diegoami/onenil-archaeology/6aca98caeebc1b9b5410852139fa305462224a05/runs/T46/session-r0001/step-0003.png)

[confirmed] [T45 skills/details W010–W013](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/squad.md)
and [T46 move/instructions W014–W017](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/selection.md)
are the procedure references. Exact move-active record:
[T46/session-r0001/step-0003.json](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/runs/T46/session-r0001/step-0003.json).

| Region/variant | Preserve in the proposed desktop screen |
| --- | --- |
| Left skills | The22 named skill bars in original order, their framed shared scale and original bar appearance. Captures show no numeric skill values; T48's single Skill column and percentages do not replace this view. |
| Left details | Insert changes to the original personal, contract, instructions, appearance/goal/average and rating fields, preserving selected identity. Do not discard categories to fit a small card. |
| Right roster | FIRST TEAM/SUBS and the lower first-team players, selection bar, original role colours/glyph column and separate reserves navigation. A player below SUBS is not automatically a reserve. |
| Move active | Enter changes the contextual action to PUT HERE; the left panel remains the carried player's information while the right highlight chooses the destination. Escape cancels this pre-match move; Enter commits the observed swap. |
| Options | Space opens the original options overlay; unavailable entries stay visibly unavailable. The original individual instruction cycles remain separate from a generic select box. |

[designed acceptance] Compare browse, details, move-active, cancelled and committed
variants beside their actual reference images. Verify arrows, Left/Right,
Insert, Enter, Space and Escape against the selected procedures, preserving
remembered selection/view and carried-player identity. Do not replace PUT HERE
with T48's destination dropdown/modal unless a separately explained departure
is accepted. Role-font glyphs do not reliably map to decoded CP437; resolve them
from rendered references and known textual positions, using independent product
glyphs. No goal2 role table is imported by analogy.

## D03 Watched pitch, short commentary, history and bench

![Original watched kickoff](https://raw.githubusercontent.com/diegoami/onenil-archaeology/6aca98caeebc1b9b5410852139fa305462224a05/runs/T48/session-r0001/step-0003.png)

[confirmed] [Original watched W024–W029](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md)
and [T48/session-r0001/step-0003.json](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/runs/T48/session-r0001/step-0003.json)
show the supplied-save Blackburn/P St Germain friendly. This is a layout/procedure
reference, not a parity claim for the different accepted native exhibition fixture.

| Region/state | Preserve in the proposed desktop screen |
| --- | --- |
| Header | Teams/score and original clock/indicators at the top, brief recent-action commentary across the upper-right region. Preserve clock meaning from engine evidence; unread decoded counters are not invented minutes. |
| Main | Wide pitch filling most of the screen below the header, original orientation/proportions/areas and display variants. No generic card dashboard or permanently competing full-history column. |
| Footer | Up/Down display, Left/Right speed and their shown value; R result, Space commentary, F9 sound, Escape/Enter bench as supported. Captured setting changes are not proof of every range or timing formula. |
| History | Space opens the longer action history; its Up/Down/Page Up/Page Down/Home/End controls and Escape return are evidenced separately. The short recent-action region remains distinct. |
| Bench/interval | Escape reaches the live bench; Space reaches Players. At halftime the original statistics screen precedes bench access and PLAY GAME resumes. Fulltime follows its observed result/next-week route. |

[designed acceptance] Render actual ordered Rust actions and player/ball state
in this composition, with named passes/tackles/shots rather than generic filler.
Test history/bench overlays, visible current key meaning and return context.
Preserve the important difference: Escape leaves a live Players view **without**
clearing a pending substitution request; Enter cancels when CANCEL SUB REQUESTED
is offered. In contrast, Escape cancels the halftime PUT HERE replacement.
Do not infer a universal substitution budget from one displayed count.

## Departures, unknowns and next design slice

[designed] No changed layout, menu navigation or game key is approved by this
study. Scaling/window chrome and a licensed replacement font are necessary desktop
presentation work; show their actual consequences in mockups rather than treating
them as permission to rearrange screens. Mouse equivalents, any alternate glyph
or extra accessibility hint must preserve the original decision and be recorded
explicitly if it changes the visible composition.

[confirmed/derived] These three families are not the full UI map. Start/setup,
configuration, preparation, contracts/market/scout, administration, competition,
history and rollover still require designed counterparts using W001–W083 and
the18 audited menu slots. G01–G14 remain explicit unknown branches. The study
does not implement sound, result switching, substitutions, campaign publication
or arbitrary edited-XI admission merely by listing their original controls.

[designed next task] Build faithful main-menu, Players skills/details/PUT HERE,
and pitch/bench/history mockups with an original-to-proposed mapping and departure
ledger. Then demonstrate those selected compositions in a desktop feasibility
slice with the Rust lifecycle and real save boundary. Choose the toolkit from
that evidence; do not ask the owner to approve browser-first delivery again.
