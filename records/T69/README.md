# Edited campaign desktop proof

[confirmed: archaeology T69 at a440c8a, T71 at 011a341, T73 at 230f8f1;
native T66–T68] The supported edit exchanges I Pearce and C Hendry, first-team
slots 1 and 2. The exact original keyboard sequence is Space, Down, Return,
Down, Return, Escape, Up. Escape leaves Players selected on the main menu.

[designed:T69] Admission compares the complete campaign with either the initial
campaign or exactly one such exchange. Native save/reload preserves this edited
state. Additional edits, revision changes, foreign provenance and completed
fixtures refuse before a worker. This is a bounded second playable case, not
support for arbitrary edited lineups or week progression.

[confirmed: native T68 full parity and T69 computed worker] The edited match has
3,970 ticks, 170,212 ordered RNG draws, 869 running actions plus one setup action,
and actual return `0AB9:EBE2`. Both scoring actions belong to PSG (club 17):
Blackburn's displayed result is **0–2 PSG**. The legacy scratch `scores` array
is not the live score ABI; the existing adapter maps native live counters using
computed action side/club identities. No expected score is injected.

[designed:T69] `before-r0001.json` retains the pre-change admission failure;
the exact original test is available at c0b6cc4. `window-r0001.json` and its
screenshots/native saves record actual software-window keyboard input: edit,
F7 save, fresh-process reload, cancelled/reaped worker, pitch, named commentary,
bench, half/full time, result acknowledgement, F7 save, another fresh process
and duplicate refusal. The original display was unavailable; an isolated Xvfb
X11 server supplied the real native window. The independently licensed
replacement font is the same as earlier desktop proofs. These JSON saves are
the rebuild's format and contain no original DOS saves.

[designed:T69] Result acknowledgement alone increments revision 1 to 2 and adds
the fixture receipt. Week, cash, players and the edited roster stay unchanged.
The receipt owns the exact pre-match save digest. Saves remain explicit (F7).

Run the public artifact checker without the game or a display:

```sh
python3 desktop/faithful/check_edited_result_records.py records/T69/window-r0001.json
```

Reproduce with a new record basename (never overwrite measurements):

```sh
cargo build --manifest-path desktop/faithful/Cargo.toml --release --locked --features software
DISPLAY=:69 .cache/venv/bin/python desktop/faithful/edited_campaign_window_check.py \
  --binary desktop/faithful/target/release/onenil-desktop \
  --font .cache/T51-font/PxPlus_IBM_VGA_8x16.ttf \
  --record records/T69/window-r0002.json
```

[designed] The public checker pins this recorded run; a new run requires a new
reviewable checker binding. The software backend remains a feasibility vehicle;
this task does not select the final desktop toolkit or expand season support.
