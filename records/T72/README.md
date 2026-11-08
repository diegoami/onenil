# Original numbered pitch players

[designed: owner, 2026-11-08] Match players should retain the original game's
letter/number appearance. The invented prototype stick figures are superseded;
T70's sharp typography and start guidance stay.

[confirmed: archaeology011a341, W026 and T48/session-r0001/step-0010.png]
The original has a numbered display variant. [confirmed: same accepted tree,
static/1-0-exe/scene-codes.md F30,0AB9:0207/02D7; native scene_codes.rs]
Its outfield label uses the scene's stored selector, wrapping unsigned
selector-minus-one modulo11 plus47. Selectors2–11 and13–22 display0–9.
Selectors1/12 use custom keeper glyph1. Physical scene index and player ID are
separate inputs. Native tests already cover these distinctions and wrap cases.

[designed:T72] **G** is an independently rendered replacement for private keeper
glyph1. The same independently licensed font is used; no original font/sprite or
palette asset is shipped. Computed scene display attributes select independent
standard text colours. Other original figure/role/team/orientation variants and
Up/Down display cycling are not implemented by this task. This is specifically
the numbered variant, not invented shirt numbers or a claim of every original
marker shape. Existing pitch geometry is unchanged.

[derived:T72] Timeline.marker_rows carries the actual computed stored selector
and display attribute for initial state and every frame, alongside existing
View/Frame fields. Rendering does not mutate native display mode, RNG, snapshots,
positions, focus, scores or events. Label checks compare against the existing
native F30 refresh function. This projects the accepted running match into the
numbered appearance; it does not claim a separately captured full native mode4
match. Malformed marker metadata refuses before playback without campaign mutation.

[confirmed:T72] Before45869fc fails on the stick figure for physical scene1 with
stored selector2; after it draws digit0. Software/raster/metadata/native-rule
tests pass. The full existing exhibition adapter comparison passes all4,199
frames, every initial/frame player/position/focus/ball/clock/score and every
setup/running event; only parallel presentation metadata is added.

[confirmed:T72 window-r0001 and window-r0002] Two immutable actual-window runs
show22 numbered/keeper labels and preserved named commentary at1440×900 and
1280×800. The first records kickoff; the second waits for open play (at least
500 native frames) before slowing visual playback for captures. Each run has
six keyboard stages and four screenshots. Every image is bound to its observed
native raster, exact computed labels/attributes/positions, producer, binary,
font and runtime/source-contract digests. Campaign state stays unchanged and
workers are reaped. Owner's existing windows were left untouched.

```sh
python3 desktop/faithful/check_numbered_pitch_records.py records/T72/window-r0001.json
python3 desktop/faithful/check_numbered_pitch_records.py records/T72/window-r0002.json
```

Reproduce with a new filename, never overwrite either measurement:

```sh
DISPLAY=:72 .cache/venv/bin/python desktop/faithful/numbered_pitch_window_check.py \
  --binary desktop/faithful/target/release/onenil-desktop \
  --font .cache/T51-font/PxPlus_IBM_VGA_8x16.ttf \
  --record records/T72/window-r0003.json
```

[designed] These are independent native UI screenshots, not original assets.
The original reference and native-rule links establish marker semantics;
computed-frame comparison establishes unchanged match presentation data.
For Windows WSLg launch/start controls see the primary README. Press Enter on
PLAY NEXT GAME, then Enter on PLAY GAME. The newer demo must be opened to see
new markers; an already-running older process retains its previous renderer.
