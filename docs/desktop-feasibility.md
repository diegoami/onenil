# Desktop feasibility

[designed] T52–T54 are experiments for #118 and the owner's toolkit decision #103.
Desktop priority and original-led UI fidelity are settled. Toolkit selection is pending.

T52 is a shared Rust controller, captured example squad editor, versioned experiment save
and owned child-process match preparation. The captured eighteen rows have experiment IDs,
not engine player IDs; one reserve slot bounds this test. Uncaptured cards/actions remain
unavailable rather than borrowing another player's values. F9 is an explicit experiment
reload shortcut. This is not a saved management campaign.

[confirmed] Research T46 steps2–8 and T59 steps150–153, corrected by T64/PR169,
show the card follows the highlighted destination during PUT HERE. Carried source is separate;
Escape cancels the move and keeps destination selection. Earlier T50/T51 records are unchanged.

[derived] Native computation uses the existing prepared reference64 admission, unchanged Rust
Session::new/snapshot/finish and the accepted tools/play_watched.py presentation mapping at
native a1599a4. Messages are the same newly written English templates (messages.json), not
original text assets. All22 positions, clock, live goals and actual player-action identities
come from computed output. Team order follows the existing adapter's club IDs130/134; it is
not inferred from a fixture title. Detailed action output is precomputed before playback;
managed half-time changes to the running engine and arbitrary edited XI admission remain
later work. Final whistle can precede final caller frames; playback consumes those frames
before completing, as the accepted adapter does. No campaign result is posted.

The worker has a job identity and bounded protocol, nonblocking try_wait/try_recv polling,
kill/wait on cancel and Drop, and a separate reader. Invalid data cannot replace editor state.
Saves validate the whole schema/version and exactly one of every owned player ID before
replacement, then write a new temporary file, sync and rename. Pending moves cannot be saved.
Platform atomic replacement and cancellation are measured on Linux; Windows remains unproved.

Run shared core checks with `cargo test --manifest-path desktop/faithful/Cargo.toml`.
With the existing Python engine extension installed, retain full independent presentation
comparison using `python3 desktop/faithful/verify.py --record records/T52/<new-name>.json`.
T53 adds the actual software window; T54 adds the second native candidate and cost comparison.
Hosted existing engine CI does not currently include this standalone experimental crate;
its local acceptance is retained separately. No workflow policy change is implied.
