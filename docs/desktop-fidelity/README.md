# Desktop One-Nil: original-led UI contract

[designed: owner, 2026-11-06] **Desktop is primary; browser is secondary.**
The objective is a faithful reproduction of One-Nil96's screens and workflows,
with the UI kept as similar as possible. A generic management dashboard, local
browser launcher or reskinned synthetic squad is not the destination. The engine
remains Rust. This direction selects neither a GUI toolkit nor Electron.

## The method to take from goal2

[reference] Inspected goal2 at
[`98860065cf928e52e43453a0c827e7ad2a564688`](https://github.com/diegoami/goal2/tree/98860065cf928e52e43453a0c827e7ad2a564688):
its [original-game manual](https://github.com/diegoami/goal2/blob/98860065cf928e52e43453a0c827e7ad2a564688/docs/original-game/README.md),
[screen-by-screen design and UI rules](https://github.com/diegoami/goal2/blob/98860065cf928e52e43453a0c827e7ad2a564688/docs/design/ui-2026-11/README.md),
and [current UI implementation guide](https://github.com/diegoami/goal2/blob/98860065cf928e52e43453a0c827e7ad2a564688/docs/ui.md).
The design starts with the original player's journey, maps every original screen
to a designed counterpart or an explained departure, and preserves decisions,
information, layout, keys and order. Appearance comes from the game's actual
references. Toolkit selection follows that design; mockups and working code are
identified separately. Its desktop roadmap also separates presentation from
ordered engine work and committed campaign/save ownership.

[designed] Apply that method to **One-Nil**, not goal2's Python/Qt/Pygame stack,
colour values, national-team rules, screen IDs or owner-approved deviations.
For example, One-Nil's captured pitch has a short commentary panel above the
pitch; goal2's later main-window/sidebar decision is not authority to relocate it.

## Source of truth

[confirmed] Accepted research manual/audit reference:
[`6aca98caeebc1b9b5410852139fa305462224a05`](https://github.com/diegoami/onenil-archaeology/tree/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual).
It catalogs W001–W083,18 executable menu slots and explicit G01–G14 limits.
The [first-screen study](first-screens.md) links exact main-menu, squad and match
records/images. A screen's full record path and screenshot identify the evidence;
short S labels alone do not. Original rendered screenshots illustrate design;
they are not shipped product assets or an implementation.

## Fidelity acceptance

| Area | Desktop requirement |
| --- | --- |
| Information and wording | Preserve original visible categories, hierarchy, labels and values. Skill bars, descriptive fitness and team averages retain their original meanings; no generic overall skill or invented percentages replace them. |
| Layout | Preserve the recognisable whole-screen composition, box groupings, roster/card relationship, main-menu position, pitch/score/commentary arrangement and footer key bar. Start from the original, not T48's cards, tabs or table columns. |
| Colour and typography | Use One-Nil's own observed functional colours, contrast, highlight treatment, monospaced/blocky character and bars. Goal2's palette is not One-Nil evidence. Select an independently licensed replacement font/glyph treatment; original game fonts remain private. |
| Controls | Preserve each screen's arrows, Enter, Escape, Space, Insert, function keys and default highlight where observed. Mouse access may mirror the same actions; it cannot silently change transaction boundaries. New shortcuts or changed key meanings require an explained design departure. |
| Navigation and modal state | Reproduce screen order and return destinations. A move's carried-player state, a substitution request and a file prompt are different transactions. Escape follows the named original procedure, not a universal cancel convention. |
| Match | Keep original-style pitch prominence, score and short commentary placement, detailed history access, bench/interval/result flow and supported controls. Every action/outcome comes from Rust, with unsupported original controls named as gaps rather than fabricated. |
| Campaign and saves | Real confirmed edits/results update owned state exactly once and survive fresh-process save/load. A visual mockup or fixed exhibition is not a campaign. UI timing cannot alter RNG, event order or engine inputs. |
| Desktop lifecycle | Launch as an application; own window, saves and engine lifecycle. No separately opened browser or manually started web service as the user's normal play path. Prove engine responsiveness, error/cancel/shutdown and result publication with direct API or owned local IPC as appropriate to the eventual toolkit. |
| Scaling | Preserve information and readable type across the chosen desktop window sizes/scales. Do not replace the screen hierarchy with responsive web cards. Proposed scaling, clipping, scrolling or letterboxing decisions must be shown beside references and tested. |
| Departures and unknowns | Record the exact original evidence, proposed difference, reason and acceptance impact. Do not treat goal2-specific exceptions as approved One-Nil changes. Keep unvisited branches explicit; unknown role glyphs/bar semantics are not invented mappings. |

## Corrected roadmap

[designed] The [visual study](visual-study.md) now pairs ten initial proposed
desktop compositions with their exact original references. It includes thirty
rendered sheets, scaling examples and the explicit font/role/marker differences.
These are static design artifacts for inspection, not an implemented desktop UI
or approval of departures. Representative native feasibility and toolkit choice
remain next.

[designed] The [complete screen map](screen-map.md) assigns all83 documented
workflows and18 executable menu slots to proposed desktop compositions/variants.
Its [manifest](screen-map.json) preserves original references and G01–G14;
[check_map.py](check_map.py) verifies coverage against the pinned public research
tree, including missing/duplicate mappings and static-only boundaries.

```sh
python3 docs/desktop-fidelity/check_map.py --research-root ../onenil-archaeology --negative-checks
```

1. [designed] Complete an original-to-desktop screen map and faithful visual
   mockups from the accepted manual. Start with the weekly main menu, Players
   skills/details/move, preparation and watched pitch/bench/result flow. Extend
   the map to every workflow/menu family before declaring design coverage.
2. [designed] Show original/reference, proposed screen and departures together
   for owner inspection. This is a design presentation, not an agent-run milestone
   review and not permission to invent missing behavior.
3. [designed] Freeze a representative **desktop** feasibility task using that
   design. Test keyboard/focus, dense bars/role glyphs, resizing, local saves and
   Rust event/decision/result delivery without requiring HTTP polling as the
   application architecture. Assess toolkit/platform packaging from actual runs.
4. [designed] Select the toolkit with the owner on #103, then implement the
   existing B1–B6 backlog (#107–#112) toward #104. Desktop direction is already
   decided and must not be asked again as browser versus desktop.
5. [designed] Consider a publishable browser version later, with a separately
   specified hosting/client-engine model. It does not set desktop acceptance.

[designed] T48's immutable measurements and screenshots stay available as
technical history. They show generic interaction/file/packaging feasibility, not
fidelity to the original. The local-browser recommendation is superseded, and
the earlier asynchronous delivery-choice question is obsolete. Electron is not
automatically selected by the desktop priority. No new toolkit choice or campaign
implementation is made by this contract.
