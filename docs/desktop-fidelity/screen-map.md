# Original-to-desktop screen map

[designed] Proposed screen coverage, not implemented features or approved departures.
Generated from [screen-map.json](screen-map.json) by [check_map.py](check_map.py).
The original manual remains the exact procedure/key/record authority. Families below
are compositions and variants, not interchangeable controls or ordered state machines.

[confirmed] Research source: `6aca98caeebc1b9b5410852139fa305462224a05`;83 workflows,18 menu slots and14 explicit gap groups.

## Screen families

### D01 · Weekly main menu

[designed] Original three-region club/opponent, information and ordered action-menu composition; game name and contextual footer remain visible.

**Variants:** team-information; club-information; staff-information; single-manager; two-manager-handover; no-managed-opponent.

**Input/return contract:** Arrows/Enter select the original menu; Space opens remembered Players; Insert cycles information; F7/F10 retain their own save/quit procedures. Default highlight is route-specific, not always reset.

**State boundary:** Browsing information is not a change of club, a match result or a posted week. Previous player activation remains unexercised.

**Unknown groups:** G01, G02, G03.

### D02 · Players and player options

[designed] Full original skill-bar/details panel beside FIRST TEAM/SUBS/lower first-team rows or separate RESERVES; role column, selection and contextual footer.

**Variants:** skills; details; empty-reserves; player-options; put-here; sub-request-active; halftime-replacement; injured-or-ineligible.

**Input/return contract:** Up/Down changes selection; Left/Right changes first-team/reserves; Insert changes panel; Space options; Enter follows current footer. Return context is main menu, preparation or live bench.

**State boundary:** Pre-match PUT HERE Escape cancels a pending move; Escape from live Players does not clear a substitution request. Actual replacement and its allowance are separate transactions.

**Unknown groups:** G04, G05, G06.

### D03 · Watched pitch, action history and match boards

[designed] Original top score/indicator and short-commentary areas, large pitch and bottom key line; full history and statistics are distinct views.

**Variants:** live-pitch; numbered-display; commentary-history; halftime-board; fulltime-board; pending-stop; result-exit.

**Input/return contract:** Display/speed/history/bench keys follow W024–W029. Space history has its own arrows/paging/Home/End and Escape return. Result and sound hints do not prove every branch.

**State boundary:** Rust supplies actions, clock and outcomes. Playback redraw and window navigation cannot advance a campaign or change RNG. The observed fulltime exit path differs from quick mode.

**Unknown groups:** G05, G10, G14.

### D04 · Startup and saved-game selection

[designed] Original startup load question followed by the observed save selector/name route; preserve highlighted save and prompt context.

**Variants:** startup-question; save-name-selection; fresh-process-load; missing-save-unknown.

**Input/return contract:** Y/N and the recorded selector/name keys follow W001/W006. Escape has the observed selector behavior rather than a universal application-back action.

**State boundary:** Load must replace state only after successful validation. Emulator RAM restoration is not game-save persistence; invalid-save original UI remains G01.

**Unknown groups:** G01, G02, G13.

### D05 · New-game setup and manager handover

[designed] Original filename/season/manager count, league/club and name/nationality sequence with browsable club information and squad.

**Variants:** filename; starting-season; manager-count; club-browser; name; nationality; duplicate-club-refusal; second-manager.

**Input/return contract:** Exact setup keys and defaults come from W002–W009. Escape at filename, season and club selection has different observed consequences.

**State boundary:** Keep browsing, club acceptance, identity editing and duplicate refusal distinct. No fabricated new-game defaults or full two-manager weekly lifecycle.

**Unknown groups:** G01, G02, G03.

### D06 · Game save and quit confirmation

[designed] Original management F7 save acknowledgement and F10 quit question, with identity and current committed state kept visible as observed.

**Variants:** normal-save; quit-no; fresh-reload-comparison; progressed-week-save; next-season-save.

**Input/return contract:** F7 and F10/N follow W005. F10/Y and save-error/overwrite routes remain unvisited; no OS-dialog replacement is assumed.

**State boundary:** Persist confirmed lineup/instructions/week state. Normal .1-0 save and seasonal .95 archive are different purposes; do not claim deeper archive reload.

**Unknown groups:** G01, G13.

### D07 · Game configuration

[designed] Original five-option panel and Continue, preserving check/x indicators and the entry highlight.

**Variants:** visible-five-options; watch-on; watch-off; continue.

**Input/return contract:** Navigate and toggle only as the original named option procedure establishes. Return to the calling management/preparation context.

**State boundary:** Only Watch-off completion is exercised. Four other toggles and persistence/effects are gaps; labels are not implementations.

**Unknown groups:** G03, G14.

### D08 · Match preparation and live bench

[designed] Original opponent/statistics/PLAY GAME/tactics menu, central team data and named squad, including remaining-substitutions display.

**Variants:** pre-match; opponent; match-stats; live-bench; halftime-bench; incomplete-refusal; ineligible-refusal; observer-match.

**Input/return contract:** PLAY GAME starts/resumes as the current phase allows; Space Players and Escape returns follow the exact procedure. Initial PLAY GAME highlight is evidenced for the supplied route.

**State boundary:** Admission refusal preserves edit/phase. Substitution requests, halftime swaps, injuries and observer continuation must not be conflated; displayed remaining counts do not define a universal budget.

**Unknown groups:** G03, G04, G05, G10.

### D09 · Opponent and other-club browser

[designed] Original club/division selector, information modes and selected-club squad; inspection does not replace active managed club.

**Variants:** next-opponent; other-teams; club-info; staff-info; squad-details.

**Input/return contract:** Arrows/division navigation, Insert and Space follow W018/W046; Escape returns to caller. No-fixture Next opponents refusal is not an exercised route.

**State boundary:** Browse identity is separate from campaign manager/club identity. Empty and unmanaged-club bounds stay explicit.

**Unknown groups:** G03, G12.

### D10 · Team and individual instructions

[designed] Original instruction rows and markers in the original card/overlay grouping; no invented numeric sliders or percentages.

**Variants:** team-eight-rows; personal-goalkeeper; personal-outfield; current-position; club-position.

**Input/return contract:** Arrows/Enter cycle the observed row values. Escape exits editors while retained changes may remain; it is not assumed rollback. Reposition and Club position remain distinct.

**State boundary:** Preserve team versus personal source markers and unknown P meaning. Displayed formation changes from actual current positions, not a free unverified formation dropdown.

**Unknown groups:** G04, G05.

### D11 · League tables and league results

[designed] Original populated/empty result and standings columns, competition/division context and page controls.

**Variants:** empty-after-friendly; early-league; midseason; final-table; new-season-zero-table; match-results.

**Input/return contract:** Navigate and acknowledge according to the current original page. Results/table ordering is phase-specific; return to weekly flow or caller.

**State boundary:** An observed two-points result is not authority for unknown tie-break or qualification algorithms. Persist only supported original-derived standings changes.

**Unknown groups:** G11, G12, G14.

### D12 · Cup results and fixture schedules

[designed] Original cup pairing/leg/aggregate pages, weekly fixtures and team fixture list; keep competition/club/date context.

**Variants:** cup-pairing; two-leg-result; group-table; draw; neutral-final; extra-time; weekly-fixtures; team-fixtures; new-season-schedule.

**Input/return contract:** Original paging/club/division and acknowledgement keys are page-specific. Chosen-viewer Enter-watch and print remain distinct from automatically reached observer finals.

**State boundary:** Future announcement/pairing is not a played fixture. Preserve actual leg/aggregate and extra-time displays without inventing tie-resolution or scheduling rules.

**Unknown groups:** G10, G11, G14.

### D13 · Transfer market, bids and incoming offers

[designed] Original player market/list and skill/details pane, bid editor, listing confirmation, incoming offer and departure displays.

**Variants:** domestic-market; foreign-market; bid-editor; refused-bid; accepted-bid; listing; unlist; incoming-offer; tribunal-departure.

**Input/return contract:** Market navigation/Insert and bid keys follow W036–W038; incoming Y/N and automatic tribunal displays use their own original prompts.

**State boundary:** Listing is not a completed sale. Cancelled bid, accepted fee, contract acceptance, departure and later cash posting are separate commits; no generalized price thresholds.

**Unknown groups:** G06, G07.

### D14 · Player contract, dismissal and youth trial

[designed] Original contract categories/offer action, confirmation/compensation dialogs, position selector and trial-period selector.

**Variants:** renewal; offer-refused; offer-accepted; fire-first-question; compensation-question; youth-position; youth-trial; expiry-warning; automatic-expiry.

**Input/return contract:** Use each original category/edit key and Y/N/Enter/Escape context. Selecting OFFER CONTRACT and submitting it are distinct; Escape behavior depends on the automatic/editor route.

**State boundary:** A cancelled expiry may leave a listed player; listed-zero status does not always prevent admission. Renewal does not prove retirement cancellation. Quote/dismissal/posting and trial creation/expiry remain separate.

**Unknown groups:** G06, G07, G13.

### D15 · Scout criteria and results

[designed] Original criteria categories, chosen values and results list/player card, preserving search context.

**Variants:** criteria; criteria-return; search-results; player-inspection; empty-result-unknown.

**Input/return contract:** Use the W039 criteria/edit/search/result and Escape controls; cancel returns to management, with observed selections retained.

**State boundary:** A search result is not an offer or purchase. Other criteria bounds and no-result outcomes remain unvisited.

**Unknown groups:** G06, G12.

### D16 · Staff, managerial offers and stadium order

[designed] Original candidate/category lists and detail/cost fields; separately observed hire/sack confirmations, stadium quote/order and club approach.

**Variants:** assistant; physio; scout; not-interested; hire-confirm; sack-no; stadium-quote; order-confirm; job-approach.

**Input/return contract:** Staff Insert/Delete category direction and Y/N prompts follow W049; stadium Escape abort returns to main menu. Manager approach Escape can defer rather than refuse.

**State boundary:** Hire and next-week charge are separate. Ordered stadium capacity is not completed capacity. Staff sack Y and job acceptance remain unknown, not designed into evidence.

**Unknown groups:** G07, G08, G09.

### D17 · Cash flow and weekly financial statement

[designed] Original income/outgoing categories, totals, club funds/overdraft and contextual continue/stadium action line.

**Variants:** initial-zero; weekly-posting; contract-charge; sale-posting; tribunal-posting; prize-money.

**Input/return contract:** Any-key continue and Insert quote action follow the actual statement context. The following page may be contract, transfer news or another conditional event.

**State boundary:** Displayed totals and known postings are observations, not general finance formulas. UI redraw cannot post a charge twice or treat a quote as payment.

**Unknown groups:** G07, G09.

### D18 · Statistics and player/team/season history

[designed] Original unavailable messages and populated category/list/summary views, retaining player/team/year context and complete available fields.

**Variants:** before-league-four; populated-statistics; transfer-history; team-history; no-season-history; season-summary; full-season-info-unverified.

**Input/return contract:** Acknowledge unavailable notices and navigate original history/category controls. Full seasonal-info deeper view and .95 load are not established by Enter intent.

**State boundary:** Empty, zero and unavailable are distinct. Save archive presence does not prove full seasonal view persistence; no invented statistics categories or meanings.

**Unknown groups:** G12, G13, G14.

### D19 · Weekly notices, interruptions and acknowledgement

[designed] Original transfer news, injury/international/suspension, ten-week warning, approach, arranged-friendly and award/celebration screens in their actual order.

**Variants:** transfer-news; availability; renewal-warning; approach-deferred; join-question; friendly-announcement; award; injury-opportunity.

**Input/return contract:** Keep each original any-key/Space/Escape/Y/N boundary. Escape acknowledges some announcements and defers some approaches; it does not cancel all events.

**State boundary:** These notices consume actual campaign events. No invented football news, hidden cancellation or blanket dismissal/expiry consequence; restore caller/weekly continuation after the supported response.

**Unknown groups:** G05, G06, G07, G08, G10.

### D20 · Season finish and new-season transition

[designed] Original End of season entry, accounts/news, archive question, promotion/relegation/retirement pages and advanced management state.

**Variants:** end-season; seasonal-info-yes; promoted; relegated; retired; next-season-main; reset-counters.

**Input/return contract:** Follow W080–W083 order and the actual Y/acknowledgement keys. Seasonal-info N is unvisited, not a safe inferred cancel path.

**State boundary:** Summary/retirement observations do not establish universal rules. Normal save reload is proven only for .1-0 visible state; full .95 archive/deeper-view and other outcomes remain gaps.

**Unknown groups:** G11, G13.

## Every documented workflow

[derived] Each row preserves the catalog title and complete original screenshot/record
set in the JSON. The source chapter supplies exact keys and order; a family is reused
only for its presentation, never to generalize cancellation or persistent effects.

| Workflow / original procedure | Desktop families | Variant anchors | Transaction requirement |
| --- | --- | --- | --- |
| [W001 load the supplied SAVE1](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/startup.md) | D04, D01 | startup-question; loaded-main | Supplied-save startup, not a default new career; preserve arrival highlight and save identity. |
| [W002 filename, season and number of managers](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/setup.md) | D05, D01 | filename; season; manager-count | Filename Escape, season Escape and club-browser Escape have different observed outcomes; do not merge them. |
| [W003 browse and choose a club](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/setup.md) | D05, D09, D01 | club-browser; club-detail; accepted-club | Inspect clubs/squads without committing a manager identity until original acceptance. |
| [W004 manager name and nationality](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/setup.md) | D05, D01 | manager-name; nationality; main-arrival | Preserve name/nationality ordering and displayed scenario defaults; no invented nationality/identity rules. |
| [W005 save and cancel quitting](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/saves.md) | D06, D01 | F7-save; quit-question; quit-no | F7 normal save and F10/N are exercised; F10/Y and save errors remain G01. |
| [W006 choose and reload the saved game](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/saves.md) | D04, D06, D01 | save-selector; fresh-load; restored-main | Real fresh-process named save reload; Escape in the selector is not application-wide cancel. |
| [W007 two human managers and duplicate-club refusal](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/setup.md) | D05, D01 | first-manager; second-manager; duplicate-refusal; next-player | Duplicate club refusal and NEXT PLAYER are observed; Previous player and a complete alternating week are G02. |
| [W008 Italy 1993/94 setup](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/setup.md) | D05, D01 | Italy-season; Italian-club; manager-entry | Italy1993/94 setup is an isolated observed route; preserve its club/season/manager context rather than generalizing England defaults. |
| [W009 Scotland 1993/94 setup and another nationality](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/setup.md) | D05, D01 | Scotland-season; Scottish-club; nationality | Scotland1993/94 and changed nationality are a separate original setup scenario, not proof of every country/season combination. |
| [W010 enter Players, compare players and return](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/squad.md) | D02, D01 | skills; details; first-team; empty-reserves; options; main-return | Insert preserves player identity; empty RESERVES clears card; Escape options and Escape Players are different returns. |
| [W011 the main-menu squad shortcut remembers the view](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/squad.md) | D01, D02 | remembered-player; remembered-details | Space reopens remembered player and panel; it is not a new screen or reset selection. |
| [W012 cycle main-menu information](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/squad.md) | D01 | team-info; club-info; staff-info | Insert cycles central team/club/staff information while main menu stays visible. |
| [W013 inspect a player outside the starting eleven](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/squad.md) | D02 | lower-first-team-player; details | Lower first-team players below SUBS are not the separate RESERVES list. |
| [W014 cancel a move, then exchange goalkeepers](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/selection.md) | D02 | carried-player; destination; cancel; confirmed-swap | Enter carries a player; card follows destination highlight; carried source is separate; Escape cancels, Enter swaps occupied rows. |
| [W015 change individual goalkeeper instructions](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/selection.md) | D02, D10 | personal-options; unavailable-reposition; instruction-cycle | Preserve personal instruction cycle and P marker without inventing its engine meaning. |
| [W016 Reposition cycles current position; Club position restores it](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/selection.md) | D02, D10 | current-position; first-position; club-position-restore | Current Pos. and 1st pos remain distinct; Club position restores, Reposition cycles. |
| [W017 save and reload the changed selection](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/selection.md) | D02, D10, D06, D04, D01 | changed-lineup; changed-instructions; saved; fresh-reload | Changed lineup/instructions survive F7 and fresh-process reload; view alone is not persistence. |
| [W018 inspect the opponent and configuration](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/match-preparation.md) | D09, D07, D01 | opponent-info; configuration; five-visible-options | Opponent browsing and five visible configuration labels do not prove all toggle effects. |
| [W019 enter match preparation](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/match-preparation.md) | D08 | pre-match; PLAY-GAME-highlight; remaining-subs | Original supplied-route PLAY GAME default, named squad and Remaining subs display; no universal allowance. |
| [W020 edit team instructions](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/match-preparation.md) | D10, D08 | team-instructions; row-cycle; retained-on-exit | Escape leaves retained team-instruction edits; it is not rollback. |
| [W021 change the displayed formation through player positions](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/match-preparation.md) | D02, D10, D08 | repositioned-player; changed-formation | Formation display follows changed current positions; first-position identity is preserved. |
| [W022 view pre-match statistics and start the watched fixture](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/match-preparation.md) | D08, D03 | pre-match-statistics; return; watched-start | Statistics any-key return precedes PLAY GAME; displayed attendance/zeros are scenario facts. |
| [W023 an incomplete lineup is refused at PLAY GAME](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/match-preparation.md) | D07, D02, D08 | auto-select-off; empty-XI-slot; insufficient-refusal | Insufficient-first-team refusal and disabled Auto player select are distinct from later ineligible-player refusal. |
| [W024 start, open the bench and resume](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D03, D08, D02 | pitch; live-bench; players; resume | Pitch to bench to Players and back follows original menu/phase; not a second match instance. |
| [W025 request and cancel a substitution](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D02, D03, D08 | request-active; bench-exit; request-retained; Enter-cancel | Enter requests/cancels per footer; Escape exits Players while the substitution request remains active. |
| [W026 display and speed controls](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D03 | display-mode; speed23; speed24 | Displayed speed23→24→23 and representation changes are observed; ranges/time formula and sound remain gaps. |
| [W027 read detailed commentary history](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D03 | full-history; Home; End; Page-Up; pitch-return | Full action history overlays short recent commentary; Home/End/Page Up and Escape have their own context. |
| [W028 halftime and a legal replacement](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D03, D08, D02 | halftime-board; bench; replacement-active; replacement-cancel; replacement-commit | Halftime statistics precede bench; Escape cancels pending replacement; actual swap consumes the observed allowance. |
| [W029 fulltime, commentary and final statistics](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D03, D08 | fulltime-pitch; history; final-statistics; man-of-match | Fulltime pitch/history/statistics and man-of-match are original views, not a fabricated summary. |
| [W030 return to the next management week](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/watched-match.md) | D03, D17, D19, D01 | fulltime-exit; accounts; transfer-news; next-main | Observed completed-bench CONTINUE→pitch exit→stats→cash→news→next main sequence; friendly grants no evidenced league points. |
| [W031 Disable Watch match and complete a fixture](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/quick-results.md) | D07, D08, D03, D17, D19, D01 | Watch-off; quick-halftime; bench; quick-fulltime; next-week | Watch-off reaches halftime/bench then fulltime; it is not one immediate whole-fixture result. |
| [W032 Switch the watched pitch to R-result](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/quick-results.md) | D03, D08, D17, D19, D01 | watched-start; R-result; quick-completion | R-result is a separate observed route; do not claim quick/watched outcomes are equivalent. |
| [W033 League tables and results after a friendly](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/quick-results.md) | D11, D01 | initial-table; league-results | Friendly completion leaves early league/results views distinct from populated league state; no friendly points inferred. |
| [W034 Cup views and weekly fixtures](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/quick-results.md) | D12, D01 | cup-pairings; weekly-fixtures | Cup pairings and browsed weekly fixtures are not played outcomes or chosen-viewer Enter-watch evidence. |
| [W035 Team fixture list](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/quick-results.md) | D12, D01 | team-schedule; division-page | Keep team/division/page context and original fixture columns; Enter-watch/printing hints remain G10/G14. |
| [W036 Browse the market and cancel a bid](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/transfers.md) | D13, D01 | domestic-market; foreign-market; bid; cancel | Bid Escape and refused bid preserve distinct market/editor states; no fee threshold inferred. |
| [W037 Refused fee, accepted purchase contract and resulting squad](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/transfers.md) | D13, D14, D02, D01 | bid-refused; fee-accepted; contract; accepted-purchase | Fee acceptance precedes contract acceptance and squad change; rejected offer remains its own state. |
| [W038 Sell confirmation, transfer listing and removal](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/transfers.md) | D02, D13 | sell-no; sell-yes; listed; unlist | Y lists for sale; listing is not completed sale; N cancels; unlisting is a separate action. |
| [W039 Scout criteria, search results and cancellation](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/transfers.md) | D15, D01 | criteria; cancel; criteria-restored; results | Criteria Escape cancels to main while selections remain; results do not buy a player. |
| [W040 Existing-player renewal, editing and cancellation](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/contracts.md) | D02, D14 | contract-edit; cancel; reopened-defaults | Enter advances contract categories before OFFER CONTRACT submission; editing/cancelling is not acceptance. |
| [W041 Fire confirmation, compensation and cancellation](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/contracts.md) | D02, D14 | dismiss-question; compensation-question; cancel; confirmed-dismissal | First dismissal question and compensation confirmation are separate; N is not proof of a Y dismissal. |
| [W042 Hire a youth player on trial](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/contracts.md) | D02, D14 | youth-position; position-cancel; new-trial; trial-period | Youth position and trial period are separate; on-trial creation does not prove trial conversion/expiry. |
| [W043 Statistics availability](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/history.md) | D18, D01 | league-four-prerequisite-message | Unavailable-before-league-four message, not a populated statistics view. |
| [W044 Player transfer history and team history](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/history.md) | D18, D01 | empty-transfer-history; team-history | Transfer history and team history are linked views; empty rows are not absent support. |
| [W045 Seasonal history availability](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/history.md) | D18, D01 | no-season-history-message | No completed season produces an unavailable notice; populated summary is W082. |
| [W046 Other-team information and squad details](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/history.md) | D09, D02, D01 | other-club; other-division; club-info; staff-info; squad | Browse another club without changing the active managed club or losing return context. |
| [W047 Read cash flow and club/staff information](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/administration.md) | D17, D01 | zero-accounts; club-info; staff-info | Any-key returns from accounts; Insert changes main information only after leaving accounts. |
| [W048 Stadium quote, abort and scheduled improvements](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/administration.md) | D17, D16, D01 | stadium-quote; quote-abort; order-confirm; scheduled-improvement | Stadium quote Escape returns directly to main; ordered expansion is not completed capacity. |
| [W049 Staff browser, refusals and a successful hire](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/administration.md) | D16, D01 | staff-categories; refusal; hire-no; hire-yes; sack-no | Staff category Insert/Delete directions and hire/sack N/Y prompts remain distinct; staff sack Y is unvisited. |
| [W050 Weekly account posting after the hire](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/administration.md) | D07, D08, D03, D17, D19, D01 | quick-fixture; posted-accounts; transfer-news | Known first-week cash totals reconcile the hire bonus; no universal posting formula inferred. |
| [W051 Successive weeks and the first league match](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D01, D08, D03, D19, D17, D11 | successive-friendlies; league-one; results; table; next-main | Actual successive friendly/league flow changes next opponent; preserve observed two-points-for-win evidence and unknown tie-breaks. |
| [W052 Expiring contract interrupts weekly processing](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D14, D19 | automatic-expiry; contract-editor; weekly-continuation | Automatic expired-player editor interrupts weekly processing; unrelated F7/Space are not successful save/exit. |
| [W053 Incoming offer and completed sale during the week](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D13, D14, D19, D17 | incoming-offer; accepted-sale; squad-change; later-posting | Incoming offer acceptance, leaving squad and later sale posting are separate original states. |
| [W054 International duty, suspension and an incomplete lineup](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D02, D08, D19 | international; suspension; incomplete-refusal; edited-XI | Availability flags and admission refusal must show actual reason; blank fixture/observer context is separate. |
| [W055 Cup injury interruption, replacement and next-round draw](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D03, D02, D08, D12, D19 | cup-injury; replacement; result; next-draw | Injury opportunity and replacement precede cup continuation/draw; no invented substitution-budget rule. |
| [W056 Statistics after league fixture four](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D18, D11, D01 | league-four-statistics; populated-categories | Populated statistics after league four is distinct from the earlier unavailable message; category/sort/print breadth remains G12/G14. |
| [W057 Save and reload the progressed week in a fresh process](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/league-start.md) | D06, D04, D01 | progressed-main; F7; fresh-process; matching-load | Fresh process reload reproduces actual progressed week; game-save copy is not emulator RAM restore. |
| [W058 Restore and inspect availability](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D04, D02, D01 | restored-save; availability-details | Restore the identified actual save and inspect current availability; flags are scenario observations, not invented eligibility formulas. |
| [W059 Complete a European two-leg tie](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D12, D08, D03, D19, D01 | first-leg; second-leg; aggregate; observer-week | Two actual legs/aggregate display; no managed tied-aggregate/penalty rule established. |
| [W060 Injury replacement and recovery](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D02, D08, D19 | injury-opportunity; replacement; recovered-player | Preserve the named original page/context and its recorded keys, state effects and limits; do not infer unvisited variants. |
| [W061 Ineligible-player refusal and extra time](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D08, D02, D03, D12, D19 | ineligible-refusal; eligible-swap; extra-time | Ineligible-player refusal differs from insufficient players; extra time is conditional, not every draw. |
| [W062 Ten-week contract warnings](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D14, D19 | ten-week-warning; renewal-no; renewal-yes; editor-cancel | Ten-week N continues news; Y opens renewal despite retiring flag; Escape is not retirement cancellation. |
| [W063 Managerial approach, deferred decision and refusal](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D19, D16, D01 | job-approach; Escape-defer; later-join-question; N-refusal | Approach Escape defers; later N explicitly refuses; preserve both contexts. |
| [W064 Scenario changes and league-ten checkpoint](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/autumn.md) | D01, D02, D10, D06, D19 | tactics-change; availability-swap; league-ten-save | Deliberate instruction/lineup scenario changes are not causal injury-probability findings. |
| [W065 Resume the saved season and progress through league sixteen](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D04, D01, D19 | league-ten-restore; league-sixteen-progression | Identified prior save is restored before real progression; not generated campaign state. |
| [W066 European elimination and the following group stage](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D12, D19, D01 | first-leg; return-leg; aggregate-defeat; group-results | Played aggregate elimination then observer group/results flow; displayed draw is not a managed fixture. |
| [W067 Injury replacement, recovery and international absence](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D02, D08, D19 | injured-player; replacement; international-absence; recovery | Actual highlighted player controls replacement; intent labels or navigation bursts do not prove selected identity. |
| [W068 Expiry cancellation leaves a listed squad member](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D14, D02, D19 | automatic-expiry; Escape-cancel; listed-member | Expiry Escape leaves listed squad member; it is not immediate dismissal. |
| [W069 Renew a player still marked as retiring](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D02, D14 | retiring-player; offer-submit; accepted-contract; retiring-flag-retained | Contract acceptance and updated terms coexist with Retiring this year; do not erase that flag. |
| [W070 Domestic cups, extra time and neutral semi-final](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D08, D03, D12, D19 | domestic-rounds; extra-time; neutral-semi-final | Original cup/extra-time/neutral venue states; no new modern competition rule. |
| [W071 Inspect actual selection and cancel before committing a move](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D02, D08 | actual-selected-player; carried-player; destination; cancel | Carried-player/destination identities stay separate; Escape cancels uncommitted replacement. |
| [W072 Weekly announcements and saved continuation](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/midseason.md) | D19, D06, D01 | friendly-announcement; approach-deferred; later-refusal; save | Announcement Escape acknowledges, not friendly cancellation; approach deferral/refusal stay separate. |
| [W073 Expired player leaves at a tribunal-set fee](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/later-season.md) | D13, D14, D17, D19, D02 | expired-player; tribunal-announcement; departure; later-payment | Automatic tribunal departure has no Y/N offer question; later cash posting is separate. |
| [W074 Announced friendlies become actual fixtures](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/later-season.md) | D12, D08, D03, D19, D01 | announced-friendly; actual-friendly; following-results | Announced friendlies are subsequently played; announcement Escape did not cancel them. |
| [W075 Neutral final, injury replacement and winner celebration](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/later-season.md) | D08, D03, D12, D02, D19, D17 | neutral-final; injury-replacement; winner; prize | Neutral final, actual injury replacement and winner/prize display are distinct commits. |
| [W076 Listed expiry and actual eligibility are separate observations](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/later-season.md) | D14, D02, D08, D19 | ten-week-warning; expired-listed-player; actual-admission | Listed zero-games/retiring status does not alone prove ineligibility; preserve actual admission outcome. |
| [W077 Later weekly flow and league progression](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/later-season.md) | D01, D11, D08, D03, D17, D19 | league-progression; accounts; next-opponent | Later actual results/account postings advance the identified scenario; do not infer all scheduling/points/finance rules from its totals. |
| [W078 Save the league-twenty-four endpoint](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/later-season.md) | D06, D01 | league24-results; table; accounts; main; F7 | Save only at identified management endpoint, not from an assumed confirmation screen. |
| [W079 Finish fixtures and handle unavailable players](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/season-rollover.md) | D02, D08, D03, D11, D19 | ineligible-refusal; availability-swap; injury-opportunities; league30-results; final-table | Retain aborted prefixes and completed final-result recovery; repeated injury opportunities do not fabricate a replacement. |
| [W080 European final and the actual End of season entry](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/season-rollover.md) | D08, D03, D12, D19, D20 | observer-final; extra-time; End-of-season | Observer European final/extra time precedes actual End of season; no managed fixture invented. |
| [W081 Save seasonal information, promotion and retirement](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/season-rollover.md) | D20, D17, D19, D18 | final-accounts; seasonal-info-Y; promotion; relegation; retirement | Seasonal-info Y writes separate archive; N unvisited; renewed Gale still retires in this scenario. |
| [W082 Inspect populated history and the new-season state](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/season-rollover.md) | D20, D18, D11, D12, D02, D01 | new-season-main; squad-reset; populated-season-summary; new-fixtures; zero-table | Summary/reset table/fixtures and changed squad are observed; deeper .95 history view is unverified. |
| [W083 Verify next-season persistence with a fresh process](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/season-rollover.md) | D06, D04, D01, D20 | next-season-F7; fresh-process; matching-main-load | Fresh .1-0 reload reproduces visible next-season main state; it does not establish .95 deeper-view reload. |

## All executable main-menu slots

[derived from the accepted audit] Labels and observed/static boundaries retain their source;
the desktop destination is proposed. Previous player is visible/static only.

| Slot | Original action | Desktop destination | Original coverage boundary |
| --- | --- | --- | --- |
| 1 | PLAY NEXT GAME / NEXT PLAYER | D08, D19, D01 | Single-manager play and first-to-second manager handover observed. Full two-manager match/week progression remains G02. |
| 2 | Previous player | D01 | Visible/static only; actual activation is unvisited,G02. |
| 3 | Players | D02 | Observed. Full squads, further role limits and untested option variants remain G04/G05. |
| 4 | Next opponents | D09 | Observed when a fixture exists. Blank/no-fixture or unmanaged-club boundary is static, not an exercised refusal,G03. |
| 5 | Transfer market | D13 | Observed limited purchase route; negotiation extremes and financial/full-squad cases remain G06. |
| 6 | Use scout | D15 | Observed. Empty results, combinations and all criteria bounds remain G06. |
| 7 | New staff | D16 | Observed for one role; actual sack Y and other roles remain G08. |
| 8 | League tables | D11 | Observed before/during/after season. Tie-break and other promotion scenarios remain G11. |
| 9 | League results | D11 | Observed. All league/year navigation combinations are not exhausted,G11. |
| 10 | Cup results | D12 | Observed. Managed tied aggregate/penalties and other seeding rules remain G10. |
| 11 | Weekly fixtures | D12 | Observed browsing. Activating a chosen other club's fixture from this viewer remains G10; automatic observer finals are a separate route. |
| 12 | Fixture list | D12 | Observed. Enter-watch from this viewer and printing remain G10/G14. |
| 13 | Statistics | D18 | Both empty/populated variants observed. Every category, sort boundary and print path remains G12/G14. |
| 14 | Player/team history | D18 | Observed. Long-history paging and all historic categories remain G12. |
| 15 | Seasonal history | D18 | Summary observed. Full seasonal info capture failed; .95 reload and prompt N remain G13. |
| 16 | Other teams | D09 | Observed. Every club and unmanaged/empty edge is not exhausted,G12. |
| 17 | Game configuration | D07 | Only Watch toggle/Continue effects exercised. Other toggles/default combinations/downstream effects remain G03. |
| 18 | Cashflow | D17 | Observed named periods. General financial rules, board/overdraft outcomes and other postings remain G07. |

## Unvisited and conditional branches

[confirmed] These remain evidence gaps, not implemented/observed desktop features.

| Group | Unvisited original boundary | Targeted follow-up |
| --- | --- | --- |
| G01 Saves and input errors | Invalid/empty filename, malformed/unreadable/no-save errors, new-game collision semantics, F10/Y | Isolate a scratch folder; capture no-save/invalid-save UI and F10/Y with preserved originals. Treat overwrite as a distinct scenario. |
| G02 Two managers | Previous player activation; full alternating match/week/persistence | Restore two-manager setup, activate Previous player, then one handover/save/reload; no full extra season needed. |
| G03 Configuration and absent fixture | Four other option toggles/effects; no-fixture Next opponents boundary | Toggle one option per isolated save, record value/persistence and the immediate affected transition. Do not infer Auto save from its label. |
| G04 Squad/instructions | Every role/position overflow, full reserve/squad boundary, all player overrides | Capture one role-cap refusal and one per-player override interaction with pre/post state. |
| G05 Managed match changes | All substitution limits/invalid requests, different competition allowances, managed red-card permutations | Restore before one invalid substitution and capture refusal/cancel without losing the accepted match checkpoint. |
| G06 Market/scout/contract/youth | Financial/full-squad purchase refusals, counteroffer/price bounds, empty scout results, other youth positions and trial conversion/expiry | Follow the preserved youth-trial scenario to its actual expiry, then isolate one financial refusal. Broader economy formulas need static/run tasks. |
| G07 Financial posting | Player-dismissal compensation posting, other income/expenses, debt/board outcomes, universal timing | Continue the isolated player-dismissal save through one accounts period and reconcile the quoted compensation. |
| G08 Staff/job changes | Staff sack Y/cost, other staff roles, accepting a managerial job and club change | Use isolated staff-hire save for sack Y and a week; keep a job-acceptance scenario separate. |
| G09 Stadium | Completion month/capacity/final charge and limits; administration-change fresh reload | Restore the actual ordered-stadium save, advance only the required period and compare completed capacity/charge. Do not use T61 as that save's continuation. |
| G10 Cup and observer variants | Managed tied-aggregate/penalty decision, chosen-other-fixture watch, remaining tie/draw algorithms | Target one pre-tie checkpoint and capture resolution; separate viewer Enter-watch from automatic observer finals. |
| G11 League/competition rules | Tie-break formula, other promoted/relegated managed clubs, full selection/qualification rules | Static-first tie-break/qualification task, then a deliberately chosen original tie/checkpoint; do not substitute modern rules. |
| G12 Statistics/history browsing | All categories/sort/page bounds, long history, unavailable other-club variants | Navigate one populated statistics/history category at a time with explicit entry/exit and limits. |
| G13 Season archive/rollover variants | Seasonal-info N, separate .95 fresh-load/deeper history, other retirement/promotion outcomes, annual awards not shown | Investigate the retained DOSBox-X capture crash before retrying full seasonal info; preserve both .95 and .1-0 files. N is a separate save branch. |
| G14 External output and presentation | Print/F5/P paths, sound behavior, every speed/display boundary | Isolate one print-to-file path if supported; document actual destination/failure rather than assuming a printer workflow. |

## Next

[designed] T51 creates original/proposed visual comparisons for the first families.
Then a representative desktop feasibility task proves rendering, keys, save ownership
and Rust event/decision/result delivery. Owner toolkit selection follows on #103.
The map itself neither runs the game nor approves a visual or control departure.
