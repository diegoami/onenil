# onenil

[designed: owner, 2026-10-29] A faithful rebuild of *One-Nil* (Wizard, 1-0; the 1995 and 1996
editions), the club-management successor in spirit of *goal*, built from the evidence of
[onenil-archaeology](https://github.com/diegoami/onenil-archaeology) (its findings `F<nn>`, run
records and tasks). The model is goal2 / goal2-archaeology, with one lesson applied from the start:
**the engine is Rust**, checked against the original through a Python harness.

## The original game

The binaries are in the owner's private Google Drive folder **`1-0`** (My Drive; on the owner's
Windows machine `G:\Meine Ablage\1-0`;
[link](https://drive.google.com/drive/folders/1IyIUQBE_Clkv6aBizULKIymRzHBLd4U8)): `One-Nil95/`
(`1-095.EXE`) and `One-Nil96/` (`1-0.EXE`), with their league data, palette, font and saves.
`python3 tools/artifacts.py pull` copies them into the git-ignored `.cache/` through the rclone
remote `onenildrive` ([docs/artifacts.md](docs/artifacts.md)). They never enter git.

## Architecture

[designed: owner, 2026-10-29, after goal2's port measured about 14 times slower than its emulator]
- **Engine**: a Rust crate (`engine/`), translated from onenil-archaeology's decompiled code for its
  structure and from its findings for its semantics. Guest-faithful state is one memory image with
  typed accessors (`u16`/`i16`, explicit wrapping), not idiomatic ownership: faithful first, then
  refactor behind passing tests.
- **Harness**: the engine is exposed to Python (PyO3, built with maturin in CI), so the Python tools
  can run the original and the engine on the same inputs and compare them record by record.
- **Game shell and UI**: full-game design remains a decision for the owner when the engine runs its first season.
  [designed: delivery plan,2026-11-02,following the owner's playable-first request] A minimal
  text quick-result prototype may precede that season;the next product target is
  [one complete playable match](docs/first-playable.md),tracked in issue35.
  [designed: owner clarification,2026-11-02] A match clock and running text commentary
  are the minimum for this first playable; a simple pitch view is the preferred
  stretch goal. Report progress
  against its end-to-end gates,not routine/test counts alone.
  [designed: owner clarification,2026-11-03] Commentary means detailed player actions
  (passes,tackles,shots and their outcomes),not repeated "Play continues" messages.
  The quick-result summary preview does not satisfy that requirement;extend the
  original watched/action-producing path rather than inventing actions in prose.
  [designed: owner decision,2026-11-03] Port the original watched engine;the
  owner explicitly chose fidelity over a newly designed native action simulation.
- Speed is measured from the first runnable slice: a simulated week, a season, a matchday's results.

[designed: owner clarification, 2026-11-06] **Desktop is the primary product; browser delivery is
secondary**, potentially a later published version. Reproduce One-Nil's original UI and workflow
as faithfully as possible: information, layout, colour/typography character, decisions, keys,
defaults, navigation order and per-screen confirmation/cancellation semantics. Follow goal2's
original-led screen design and evidence mapping method, not its implementation stack or its
game-specific UI choices. Design the faithful screens before selecting the toolkit. T48's
browser-first recommendation is superseded; its generic sample editor is a technical experiment,
not an approved visual design. Desktop priority does not select Electron, Qt or another toolkit.
See [the desktop fidelity contract](docs/desktop-fidelity/README.md). The Rust engine remains.

## Working rules

1. [designed: owner, 2026-10-29] No model is fixed: the main session plans, implements, verifies
   and merges on whichever model the owner runs it on, and spawns helpers on its own judgment.
2. Work on a branch in the checkout at hand, one task at a time per checkout. Never `git stash`,
   never force-push, never rewrite `main`.
3. One task, one PR, squash-merged once its Done-when lines and CI pass:
   `gh pr merge <pr> --squash --match-head-commit <sha>`.
4. [designed: owner, 2026-10-29, as in malpaco] Only milestones mandate a review. No session starts
   a review: the main session writes the prompt and the owner runs it on a model of their choice.
   Relay review findings in full.
5. The Done-when is not negotiable by the implementer: it stops and reports, never weakens an
   assertion, skips a test or edits its task file. The main session amends a Done-when on `main`
   with the reason in the commit.
6. A question is not a request to edit files. Design decisions go to the owner; nothing else waits.
7. A lesson or decision of the owner goes into this file or `docs/` in a commit, never only into a
   model's memory. Status lives in GitHub labels and issues.
8. [designed: owner, 2026-10-29] Every task is resumable and cut into milestones
   ([docs/process.md](docs/process.md) §4): outputs pushed at each, private bytes to the owner's
   Drive, a checkpoint comment; long runs append-only with a resume command.
9. Wait on a background job by its PID, never with `pgrep -f` (it matches the waiting shell).

## Project rules

10. **Evidence tags.** Every behaviour claim in code or docs is `[confirmed]` (cites the
    onenil-archaeology finding(s) and commit), `[derived]` (follows from confirmed claims) or
    `[designed]` (a choice of this rebuild). A `[designed]` choice that later contradicts a finding
    is a bug.
11. **The original is the arbiter.** Where the engine and a run of the original disagree on a case
    the evidence covers, the engine is wrong until proven otherwise.
12. **Engine truth comes from onenil-archaeology.** A rule, constant or routine enters the engine
    only with the finding it rests on; a defect in a finding goes back there.
13. **Tests fail before and pass after.** Every new behaviour arrives with the test that visits it,
    and the PR says how the failure was shown.
14. **Nothing of the original game in git.** No executable, data, palette, font, save or savestate
    byte, and nothing that embeds them, in this repository or its releases
    (`tools/check_originals.py`).

[designed: owner clarification, 2026-11-08] Original screens are design references
and mockups, not a directive for total visual conversion. Preserve the recognisable
look and gameplay/workflow semantics, while allowing visual departures that improve
readability, sharp typography and polish. Do not reproduce rendering glitches or
obscure basic actions in pursuit of pixel similarity. Make starting a game clear.
