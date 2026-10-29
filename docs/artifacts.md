# Where artifacts live

[designed: owner, 2026-10-29, as in goal2] Everything the project needs is in the repository, so a
fresh clone (a cloud session included) has it, except the original game.

| What | Where |
| --- | --- |
| The project's own outputs: findings, run records, measurements, reports, fixtures holding no original byte | **Committed** |
| The same, over ~50 MB | A **GitHub release**, linked from the document that uses it |
| The original game's bytes: One-Nil 95 and 96 (`1-095.EXE`, `1-0.EXE`, `ALTER.EXE`, `INSTALL.EXE`, `HELP.EXE`, the league data `*.DAT`, `1-0.PAL`, `1-0.TFN`, the saves `SAVE1.1-0`), emulator savestates and anything embedding them | The owner's **private Google Drive folder** `1-0`, pulled into the git-ignored `.cache/` |

The repositories are public and the game is copyrighted: its bytes never enter git or a release
(`tools/check_originals.py` refuses tracked files of its types).

## The Google Drive folder

[The folder](https://drive.google.com/drive/folders/1IyIUQBE_Clkv6aBizULKIymRzHBLd4U8) (`My Drive/1-0`,
on the owner's Windows machine `G:\Meine Ablage\1-0`) holds `One-Nil95/` and `One-Nil96/`. Its
location is `artifacts.json`.

- With [rclone](https://rclone.org), once per machine: a remote `onenildrive` of type drive whose
  root is that folder (`rclone config create onenildrive drive scope=drive
  root_folder_id=1IyIUQBE_Clkv6aBizULKIymRzHBLd4U8`, a browser login). Then
  `python3 tools/artifacts.py pull` copies the folder into `.cache/`, `pull One-Nil96/1-0.EXE` one
  file, `push .cache/<file>` uploads one, `where` prints the location.
- By hand: copy the folder from Google Drive into `.cache/`.
- In a cloud session: create the remote with a token from a configured machine, kept in a secret,
  never committed.

**What the login can reach.** A remote created with `scope=drive` holds a login to the owner's
whole Google Drive: `root_folder_id` only sets where rclone starts, it is no security boundary. Keep
such a token on the owner's machine. For CI, use a Google service account that the folder alone is
shared with (it sees nothing else), never the owner's own token.

rclone's shared Google Drive client_id is being retired during 2026: give the remote its own
client_id (`client_id`/`client_secret`, https://rclone.org/drive/#making-your-own-client-id) before then.
