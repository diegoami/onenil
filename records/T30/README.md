# T30 original movement and native kernel evidence

[confirmed] `capture-r0001.json` producer e4e5814 records64 original6F98
calls during kickoff positioning:41 bounds rejections,23 allowed moves,no
occupancy reads. `capture-r0002.json` producer25c1e16 records a separate64
active calls through watched tick50/draw1029:all64 allowed,33 actual occupancy
reads with tags7/0. Both join the exact T39 first watched-tick state and ordered
RNG prefix. All kernel calls preserve the seed and draw no RNG. The separate
fixtures and provenance files remain immutable. These are typed logical
observations,not physical register/timer/render-buffer equivalence.

[confirmed] Actual missing-PyO3 API,original-Rust-stub and six static branch/
ownership failures are retained before implementation. Native replay checks all
128 actual inputs and scene/result/seed outputs. Static/synthetic cases cover
signed-byte wrap,bounds,stay outside,guard ordering,tag5 and membership-byte
edges,missing/wrong indexed witnesses and whole-image ownership. No captured
active call rejects due to occupied terrain;do not claim dynamic coverage of
that branch.

[designed] Mid-call observation supplies only the raw readonly cell selected
by the original lookup. Incoming scene state stays at root entry. Rust must
compute the candidate independently and address the admitted cell;changing the
witness coordinate makes a reached lookup fail rather than steering movement.
Decoded logical pitch cells/presence metadata occupy host image pools41000/
42000,disjoint from prior pools. This represents gameplay occupancy,without
copying original screen/font/palette assets.

[designed] A release timing receipt measures only this candidate/legality
kernel,including admission parsing/image setup and serialization. It is not a
watched tick or complete match,and supplies no emulator/full-game speed claim.
70D5 direction/retries,72BB action dispatch,ball work and the watched clock
remain before detailed player commentary. The existing browser still uses the
quick-result summary backend. Native62/35 remain open.

[confirmed] Final release receipt `T30-r0001.json` uses clean producer
`d15456b8adb37687a9ec4d17b3088c2b88f03845`,freshly compiled/installed release
binary`54a988d2734114accfc3f03f4a63431f3d6f21b67a3cc2d06c531d6491d893fa`.
The exact receipt records12,800 standalone calls and93,156,936ns. All27 prior
bridge/setup checks,the new128-case replay and five controller tests passed
against that binary;416 Rust tests pass. This is not full watched-match timing.
