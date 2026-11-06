# UI delivery comparison for owner decision

[designed] **Decision pending — neither candidate is the selected game shell.**
T48/#105 supplies two working, disposable candidates for #103. The recommended
next step is local-browser HTML/CSS/JavaScript around the existing local Rust
bridge for B1–B6, with desktop packaging deferred. The owner can instead select
Electron, or request another desktop candidate before deciding. This proposal
does not authorize campaign implementation or choose a framework on their behalf.

## What actually runs

[confirmed: designed spike measurements] The same 24-player sample squad runs
directly from local HTML in Chromium and inside a **packaged Linux x64 Electron
application**. Both provide a scrolling table, player details, arrow/Home/End
navigation, move preview/commit/cancel, sample instructions, real JSON file
save/load, invalid-file refusal without replacing state, and fixture paging.
The data, positions, skill values and finances are deliberately synthetic;
they are not extracted game assets or a port of management rules. The spike's
versioned JSON is not a proposed production campaign schema.

[confirmed: T48] [ui-r0001](../records/T48/ui-r0001.json) is the pre-implementation
failure (missing UI). [ui-r0002](../records/T48/ui-r0002.json) retains an assertion
that ran before the dialog close handler committed the move; the test now waits
for the visible committed position without removing the assertion.
[ui-r0003](../records/T48/ui-r0003.json) passes both file paths and offline browser
reopening. [ui-r0004](../records/T48/ui-r0004.json) adds live exhibition and package
source verification. [ui-r0005](../records/T48/ui-r0005.json) is the final acceptance
run, including desktop file-dialog cancellation and networking-disabled desktop
save/load and fresh-process reload. Its `sources` map hashes every producer file;
the packaged ASAR UI/main/preload contents are checked against those exact sources.
Original engine sources and earlier records remain unchanged.

| Evidence | Browser | Packaged desktop |
| --- | --- | --- |
| Squad after confirmed move | [Screenshot](../records/T48/ui-r0005-browser-squad.png) | [Screenshot](../records/T48/ui-r0005-desktop-squad.png) |
| Reopened state | [Mobile, offline](../records/T48/ui-r0005-browser-mobile-offline-reopened.png) | [Fresh desktop process, offline](../records/T48/ui-r0005-desktop-reopened.png) |
| Existing native exhibition | [Pitch and commentary](../records/T48/ui-r0005-browser-native-exhibition.png) | [Pitch and commentary](../records/T48/ui-r0005-desktop-native-exhibition.png) |

## Delivery results and costs

[confirmed: ui-r0005] Runtime versions are Node24.22.0 for the harness,
Playwright1.65.0, Chromium145.0.7632.6 for browser acceptance, and Electron45.2.3
(Chromium156.0.8078.80) for desktop acceptance. The shared HTML/CSS/JavaScript
totals **15,452 bytes**. The uncompressed Linux package totals **287,145,955 bytes**
(about274MiB), including Electron and the spike assets. These are file sizes,
not memory, startup or performance measurements. They exclude the existing Rust
extension, Python runtime, engine fixtures and match service. Browser delivery
also needs those engine/runtime dependencies for a live match;15KB is not the
size of a complete browser game.

| Concern | Local browser | Electron desktop |
| --- | --- | --- |
| Current build | Shared files open directly; no frontend build step | Node24+, npm lockfile, Electron download, staging and Linux packaging verified |
| Real file persistence | Download exported JSON; explicitly choose file to reload | Restricted preload bridge opens native save/open dialogs and reads/writes chosen file |
| Offline | New browser context with networking disabled loads exported file | Packaged process with networking disabled saves/loads and fresh process reloads file |
| Existing Rust integration | Current local Python/PyO3 match server reused | Same server is embedded as a view; service is **not bundled or supervised** by this package |
| Distribution | Requires browser plus separately installed local service for matches | Includes browser engine, but still requires separate service; Windows installer/signing/update/sidecar packaging untested |
| Automation | Browser keyboard/file/download cases verified | Same cases plus packaged process and main-process file bridge verified; Playwright Electron support is experimental |
| Accessibility | HTML labels, table headers, focus styles, keyboard navigation and live status provided | Same UI; native menu/shortcut conventions not yet designed |
| Maintenance | Existing simple frontend pattern; supported-browser matrix still needed | Chromium/Node/Electron updates and packaging dependencies added; pinned lockfile with fixed glob/minimatch overrides builds successfully |

[confirmed: test limits] Native Electron file-dialog **results** are supplied by
main-process test doubles selecting real temporary paths. Real disk I/O and
cancellation handlers execute; actual OS dialog appearance, focus, overwrite
confirmation and assistive-technology behavior are not demonstrated. Desktop
automation used Xvfb and `--no-sandbox` because of the development environment;
this is **not a validated production sandbox configuration**. The application
configures context isolation, renderer sandboxing and no Node integration, and
acceptance verifies renderer `require` is absent, but production launch without
that test flag still needs checking. No Windows/macOS build, installer, signing,
update, screen reader, full keyboard audit, startup time or RAM benchmark is
claimed. Browser input-picker cancellation is not automated.

## Traceability to the accepted original workflow requirements

[derived] Requirements come from research
[`6aca98caeebc1b9b5410852139fa305462224a05`](https://github.com/diegoami/onenil-archaeology/blob/6aca98caeebc1b9b5410852139fa305462224a05/docs/manual/native-gaps.md),
accepted by owner-run milestone135 review. Tests here assess delivery capability,
not original management fidelity.

| Accepted requirement | Observed in both candidates | Remaining production boundary |
| --- | --- | --- |
| Dense squad and linked details, W010–W017 |24 rows; selected player identity follows keyboard and committed move; details survive fixture navigation | Original XI/sub/reserve categories, role restrictions and admission validation must be connected in B3; screen-reader table interaction untested |
| Transactions/cancellation, W014/W025/W036–W042/W048/W049/W068 | Swap is previewed in modal, Escape/cancel preserves state, confirm changes once; desktop file-dialog cancellation preserves state | No contract/dismissal/listing lifecycle implemented; cannot generalize move cancellation to those screens |
| File persistence/offline, W005/W006/W017/W057/W083 | Real save, reopen and load; malformed JSON, unsupported version, duplicate roster IDs and unexpected club value rejected atomically | Synthetic schema only; production state/migrations/atomic writes/backups belong to B1 |
| Match preparation/return, W019/W024–W030/N1/N2 | Existing Rust exhibition runs inside each candidate;22 player circles/named pass, Pause, return preserves sample state | Edited squad does **not** feed the fixed admitted exhibition; results do **not** update campaign state. B4 must implement that bridge; no new responsiveness benchmark claimed |
| Long competition/history views, W033–W035/W044/W056/W082 | Three pages of sample fixture rows navigate without losing selection | Populated league table/history, category/year contexts and original pagination not implemented |
| Conditional availability/accounting, W023/W050/W054/W061/W067/W073 | Sample injury marker and cash are visible; refused file load gives status without mutation | Original first-XI refusal reasons and actual weekly/tribunal postings absent; reuse guard in B3, accounting in B5/B7 |
| Normal save vs seasonal archive, W081–W083/G13 | Spike-only normal save purpose explicitly identified | No seasonal archive implementation; retain original .95 deeper-view unknown; production dual-purpose export remains design work |

## Recommendation and next bounded work

[designed proposal] Keep the shared HTML/CSS/JavaScript interaction layer and
local-browser delivery for the first persisted management week. It reuses the
accepted match service and avoids bundling an extra browser engine while the
campaign bridge is still absent. This recommendation prioritizes getting B1–B6
working; it is not proof that browser delivery best meets a finished game's
installation requirements. Electron is a demonstrated alternative if an installed
desktop shell and native file dialogs take priority, with runtime/service
packaging and Windows validation explicitly added before delivery.

[designed] Tauri/other native webviews were not built in this comparison. Their
Rust integration or package size must not be treated as measured advantages here.
If the owner wants that comparison before choosing, freeze a bounded third spike.
After owner approval, record the selection and its tradeoffs in the architecture
docs, then define B1: a versioned campaign state and real save/load, including
rejection without replacing current state. Do not carry synthetic squad values
or pretend match-result transitions into the real campaign.

## Reproduce

```sh
# Browser UI alone; direct local file navigation also works.
python3 -m http.server 8042 --bind 127.0.0.1 --directory web/spike
# Open http://127.0.0.1:8042

# Existing live Rust match service; build release extension as in README first.
.cache/venv/bin/python tools/play_watched.py --port 8041

# Candidate desktop build (Node24+). Runtime/downloads stay ignored.
npm ci --prefix desktop/spike
node desktop/spike/node_modules/electron/install.js
npm run package:linux --prefix desktop/spike
.cache/T48-packages/OneNilUISpike-linux-x64/OneNilUISpike

# Interactive acceptance: requires a display for Electron and running service8041.
npx --prefix desktop/spike playwright install chromium
node tools/smoke_ui_spike.mjs records/T48
```

[designed] In headless development, provide a display with Xvfb. `CHROMIUM_PATH`
may point to an existing Chromium executable; the recorded browser run did so.
The harness currently passes `--no-sandbox` only to its Electron test launches;
do not add that flag to a release launcher. Each acceptance invocation allocates
a new immutable numbered JSON record and screenshot set, including failures.
Commit producer files first. Native test file paths are injected from the main
process, not exposed as an arbitrary renderer filesystem API.

[reference] Official documentation consulted for delivery/automation boundaries:
[Electron introduction](https://www.electronjs.org/docs/latest/),
[Electron security](https://www.electronjs.org/docs/latest/tutorial/security),
[Playwright Electron automation](https://playwright.dev/docs/api/class-electron).
