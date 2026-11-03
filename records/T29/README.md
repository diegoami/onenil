# T29 configured native viewer verification

[confirmed] `controls-red-proof.txt` retains the actual missing-configure failures
before implementation. The five controller checks now cover the original match,
all four supported XI/seed combinations, actual starting-player and setup-seed
handoff, phase/invalid-input guards and deterministic restart.

[confirmed] `T29-r0001.json` records Chromium acceptance on committed producer
7dd86a8 with the T28 release extension. It retains reference and alternate
screenshots separately. Both reference and reserve-starting seed122 matches
finish through native setup; the reference retains the original result and all
previous controls. The alternate smoke proves actual changed players and seed,
full time, restart preserving choices, locked controls and HTTP request guards.
Desktop/mobile alternate captures were visually inspected. No screenshot or
receipt is overwritten; subsequent runs use a fresh number.

[designed] Alternate XI is Blackburn reserve Warhurst replacing Newell in the
starting eleven, with Newell moving to the bench. Supported seed settings are
64 and122 with the fixture/history configuration unchanged. Python only links
roster aliases/raw canonical rating witnesses; all preparation/match computation
is Rust. Alternate sessions are completion smokes, not additional original
parity claims. Owner-run milestone35 review remains required for acceptance.
