# T28 native preparation evidence

[confirmed] `T28-r0001.json` measures the release built after committing producer
`d4b0b8927e6e193d8a3643a5b29009a40d67ec0e`. It includes native preparation
and all 95 successive steps for each of ten exhibitions on one image per match.
Its binary SHA and exact elapsed time are authoritative. Verification runs before
measurement; expected observations are not runtime inputs. This is a fixed
exhibition measurement, without an emulator or full-game speed comparison.

[confirmed] `settings-r0001.json` reads config+F6; `settings-r0002.json` additionally
reads legacy queue-zero cells. Both stop at the exact T38 pre86E9 state and
255-draw prefix, without modifying guest configuration. The second observation
supplies those legacy cells; the first stays immutable. T38 source/index files
are unchanged. Additional static source hashes cover B6BD and1998:186C.

[confirmed] Missing-session, current-cache-gate, managed-fallback, automatic
selection and setup-projection red proofs retain actual failures before their
fixes. The original setup prototype's final-score test passed before the full
boundary checker found omitted legacy income/scratch cells and starter counters.
The complete replay now compares all 11 setup boundaries (first two seeding
fields only), 258 draws and the same continuing session through 95 original
DD02/clock/RNG boundaries, all caller points, 37 AI calls and two replacements.
No source streams or assertions were weakened to obtain this equality.

[designed] Career automatic squad/colour selection is represented by supplied
squads/presentation attributes and its incoming candidate scratch vector.
Career financial persistence is excluded; actual match income/caps and RNG are
native. The viewer still uses T27 prepared input until the next task wires this
API to supported lineup/seed choices. First-playable acceptance requires those
smokes and owner review.
