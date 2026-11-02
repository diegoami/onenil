# T13 timing receipts

[designed] `T13-r0002.json` is the final clean-source release receipt. The bridge
was freshly rebuilt after all formatting and commits, all thirteen replay tools
passed against that installed binary, and its SHA is recorded in the receipt.
Its producer3a876e4 has the same engine/tools tree as the final implementation.

[confirmed] `T13-r0001.json` retains an earlier measurement. A fresh rebuild
produced a different binary SHA after final formatting; the earlier build had
overlapped formatting. Its logged clean benchmark-time commit does not prove
that earlier binary was built from exactly that source tree. Retain the record
unchanged, but use r0002 for the final source/binary provenance and timing.
Neither receipt is an emulator comparison or a full-game performance claim.
