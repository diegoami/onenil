# Process

[designed: owner, 2026-10-29, from goal2 and malpaco] How work runs. The rules are in CLAUDE.md.

## 1. Tasks

`docs/tasks/T<nn>.md` from [TEMPLATE.md](tasks/TEMPLATE.md), 150-400 words, indexed in
[tasks/README.md](tasks/README.md), each with its GitHub issue. Each **Done when** line is one check a
command can run: it fails before the task and passes after. A task likely to outlast about an hour
of implementation is split. The main session edits task files on `main`, with the reason.

## 2. The loop

`ready → implement → checks → merge → follow-ups → report`

1. On a branch `task/T<nn>-<slug>`; commit and push after each step; a draft PR after the first push.
2. **Checks**: every Done-when line on the PR head, the repository's checks and green CI.
3. **No per-PR review**: the PR merges on its checks. A review only at a milestone, or when the
   owner asks for one: then the main session writes the prompt and the owner runs it on a model of
   their choice. No session starts a review.
4. Merge pinned to the checked head (`gh pr merge <pr> --match-head-commit <sha>`).

## 3. Milestones and reviews

A milestone is a coherent set of merged work. Its issue lists the PRs merged since the last one;
the main session gives the owner the review prompt, the reviewer posts findings, the main session
relays them in full and handles them.

## 4. Stop and resume

Every task is resumable and cut into milestones of its own, so a crash or a stop loses at most the
unit in progress. Reaching one means its outputs are committed and pushed (anything holding
original bytes to the owner's Drive, never to git) and a checkpoint comment says which is reached
and what is next. Long runs write append-only records, have a resume command that skips what is
recorded, and push their records while they run. A session resumes from the latest checkpoint
comment and rechecks live heads and checks first; a run found stopped is resumed, never assumed to
have passed.

## 5. Bugs and triage

Bugs and non-blocking findings are issues labelled `triage:needed`; triage decides fix, task,
fold or close. Status lives in GitHub labels and issues, not in documents.
