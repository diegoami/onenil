# T78: actual-window squad selection and the persisted exchange

Two isolated real-window runs (software/minifb backend, XTEST keyboard input,
one fresh window per case) drive the live halftime squad selection end to end.
`window-r0003.json` is the witnessed case: SPACE opens the squad view over the
owned interval snapshot (Pearce 2582 in first-team slot 2, Gale 2592 in bench
slot 12, remaining substitutions 7 displayed), an unwitnessed pair (Flowers→
Gale) refuses with the readable witnessed-exchange message, the witnessed pair
confirms, the worker executes the original CBF0/C866/AEB8 transaction on the
retained image, and the continuation's pitch/commentary follow it; full time
posts the result and commits the roster exchange in one campaign save
(`first_team[2]=2592`, `first_team[12]=2582`, one-based), F7 saves it and a
fresh process reloads it; the replay attempt refuses (exactly-once).
`window-r0004.json` cancels the selection after the pick, continues the same
match, and records the result without any roster change.

Raster bindings tie the squad view, the refusal, the second half and full time
to the diagnostic state at both supported window sizes; save proofs bind the
pre-match and completed native saves by digest. The keyboard census covers
every step; all owned windows and worker processes are closed and reaped.

Checks: `python3 desktop/faithful/check_live_squad_records.py
records/T78/window-r0003.json records/T78/window-r0004.json`.
