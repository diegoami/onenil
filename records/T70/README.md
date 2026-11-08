# Desktop readability and start guidance

[designed: owner, 2026-11-08] Original screens are mockups and design references.
Visual departures may improve readability and polish while gameplay and owned
campaign semantics remain faithful. This supersedes literal visual conversion.

[derived:T70 before-r0001, producer c2de9ad] Three regressions failed before the fix:
glyphs were stretched horizontally from native font columns, the preparation
screen drew an additional PLAY GAME over the substitution counter and shifted
its roster one column left, and Main/Preparation omitted explicit Enter hints.

[designed:T70] Render native glyph columns with crisp foreground/background
coverage, clipped inside each cell. Preserve the original independent font hash;
no original game font is used. Preparation keeps its substitution counter, aligns
its roster with Players, and highlights the selected entry in the real menu.
Its title and both start hints are explicit. There are no engine, campaign,
worker-ownership or keyboard-action changes.

[confirmed:T70 window-r0001] A real native software window received keyboard
input through Main, Players, Preparation, the owned worker, pitch and detailed
commentary. Eight screenshots cover Main/Players/Preparation/commentary at
1440×900 and 1280×800. Campaign state stayed unchanged throughout; the worker
was reaped. The check ran on an isolated X11 display so the owner's running
older demo was not interrupted. Font, producer, binary, runtime and image hashes
are recorded. Earlier measurements remain unchanged.

```sh
python3 desktop/faithful/check_readability_records.py records/T70/window-r0001.json
ONENIL_DESKTOP_FONT="$PWD/.cache/T51-font/PxPlus_IBM_VGA_8x16.ttf" \
  cargo test --manifest-path desktop/faithful/Cargo.toml --release --locked --features software
```

[designed] CI now fetches the independently licensed replacement font through
T51's existing checksum-pinned downloader, runs the actual raster unit regressions,
and checks this artifact in addition to the existing engine/source/desktop gates.

Launch from Windows PowerShell (WSL/WSLg):

```powershell
wsl --cd /home/diego/projects/onenil-archaeology/.cache/onenil-native --exec ./desktop/faithful/target/release/onenil-desktop --save /tmp/onenil-demo.json
```

On Main, select **PLAY NEXT GAME** and press **Enter**. On Preparation, select
**PLAY GAME** and press **Enter**. Arrow keys select menu entries. Space during
the match opens detailed commentary; Esc returns. A running older process keeps
its older renderer until closed and relaunched. F7 saves from Main; add `--load`
to restore that native save. These remain bounded demo cases, without next-week
progression. The owner-requested polish is separate from the T66–T69 engine
milestone review in issue154.
