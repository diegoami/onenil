# T77 — live desktop halftime flow

[derived:research88102d2/nativeae80836] Campaign matches now use the retained
live worker. The first-half pitch, numbered markers and named actions are
rendered from a typed incomplete playback. The child remains owned while the
user inspects pitch, bench or reached commentary. Playback consumes the reached
walk-off tail and pauses at the actual native interval rather than dropping it.

PLAY GAME at that interval sends Continue to the same child and shows a waiting
state. The completed continuation must preserve the request, fixture, markers,
first-half frames and reached events. Attaching it preserves cursor and history;
no setup or first-half actions replay. Full-time acknowledgment records the
computed score once through the existing ticket; ordinary native save/reload
retains that receipt. Incomplete/stale/changed/invalid output cannot post it.
Cancellation, reload, failure and close reap the owned worker. Exhibitions keep
the existing complete-worker path.

Frozen main6c392cd; fail-before3669b07 failed with missing desktop interval/view
methods. Implementation ce25fca. Four new regression tests cover both admitted
campaigns, malformed and changed contexts, stable paused views, real subprocess
ownership, command dispatch, cancel/reload/failure/drop and saved completion.
The full local desktop/raster suite passes56 results. Engine algorithms, source
fixtures, campaign schema and prior evidence are unchanged.

Actual isolated software-window captures:

- r0001/r0002 retain the first successful original/edited runs,23/26 recorded
  stages,7 raster-bound screenshots each, saves and fresh-process recovery.
- r0003/r0004 add actual incomplete pitch rasters,24/27 stages and9 screenshots
  each across1440x900/1280x800. Each runs three fresh windows, checks start
  cancellation, retained worker identity during inspection/halftime, waiting
  state, continuation, full-time receipt, save/reload, replay refusal and close.
- All capture records are immutable. Public verification pins record/producer/
  binary/font/source identities, image hashes and actual native raster bindings;
  ten corruption controls per record cover ownership, phase/history, result,
  namespace, lifecycle and integrity failures. Only read-only observations and
  ordinary keyboard/window events drive the captures.

```sh
cargo test --manifest-path desktop/faithful/Cargo.toml --release --locked --features raster
python3 desktop/faithful/check_live_desktop_records.py records/T77/window-r0003.json
python3 desktop/faithful/check_live_desktop_records.py records/T77/window-r0004.json
```

Both software and egui adapters compile. The egui adapter also polls while it
owns a worker on paused screens, allowing failure/reaping without advancing
playback. Current owner windows and unsaved state
are preserved; isolated capture windows/displays are closed. No original game
capture or private upload is needed. Full exact-head CI and checked-tree equality
remain mandatory before squash.

**Limits:** this enables actual live Continue in the desktop. Managed player
selection and saved substitution roster mutation remain gated on research#193.
There is no new arbitrary-XI admission or week progression. The result ticket is
a native fixture receipt, not an implementation of the original week effects.
