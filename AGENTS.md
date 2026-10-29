# onenil — agent entrypoint

Read [CLAUDE.md](CLAUDE.md): its rules apply to every client (Claude Code, Codex, OpenCode), and
[docs/process.md](docs/process.md) for how a task runs. The original game's binaries are in the
owner's private Google Drive folder `1-0` (`G:\Meine Ablage\1-0`;
https://drive.google.com/drive/folders/1IyIUQBE_Clkv6aBizULKIymRzHBLd4U8), pulled into `.cache/`
with `python3 tools/artifacts.py pull` (rclone remote `onenildrive`, [docs/artifacts.md](docs/artifacts.md)).
They never enter git. Resume from the latest checkpoint comment on the open issue labelled
`checkpoint`, then recheck live heads, labels and checks.
