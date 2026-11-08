# T73 Commentary history controls

[confirmed: archaeology011a341,manual W027,T48 steps20–24] The original offers
Up/Down, Page Up/Page Down, Home/End and Escape for commentary history.
[designed] This viewport uses77 columns and18 lines, one line/page per key,
clamped boundaries and latest history on entry. Space also retains its native
prototype return shortcut. Only reached Playback.events are projected; engine,
campaign, result posting and precomputed native actions are unchanged.

`before-r0001.json` retains the failed Up raster assertion, before implementation.
`validation-r0001.json` records46 software-feature test results (including8 raster
unit results and4 history tests), both adapter builds and local log digests. The
wrapping sample in the first positive test attempt was too short for its intended
two-page assertion; the final test sample explicitly spans more than two pages.
Synthetic test actions are presentation probes, not original observations. A separate
real native campaign test verifies reached events and cursor/phase preservation.

`window-r0001.json` is the actual software window at producerd61d78b. Thirteen
keyboard stages exercise every scrolling key, opening/latest history, return,
resumed cursor advancement and reopen. Three screenshots at two sizes bind actual
window rasters to observed state. Every stage preserves the campaign; scrolling
preserves the paused cursor/events/phase. The test window ran on an isolated Xvfb,
then closed; owner windows were untouched. Replay on a separate display with:

```sh
python3 desktop/faithful/history_window_check.py --binary desktop/faithful/target/release/onenil-desktop --font .cache/T51-font/PxPlus_IBM_VGA_8x16.ttf --record /tmp/history-new-run.json
python3 desktop/faithful/check_history_records.py records/T73/window-r0001.json
```

The strict checker authenticates the retained producer/binary/font, stage invariants
and screenshot/raster digests. New measurements require fresh output paths.
