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

T53 candidate: native software window (minifb0.29.0/X11 on this machine), fontdue0.9.5,
shared Rust controller and direct local child. The cell export contains13 authenticated
public decoded text/style sheets; bitmap screenshots are acceptance evidence only and are
never loaded as UI backgrounds. Runtime rows, card selection, move state, menus, saved state,
clock, score,22 players and detailed commentary follow owned state. Only Flowers/Pearce cards
and Mimms details have captured data; other cards show an explicit unavailable statement.
The independent VGA font remains an ignored local dependency with T51's attribution/licence.

Build: `cargo build --manifest-path desktop/faithful/Cargo.toml --features software --locked`.
Run the resulting `desktop/faithful/target/debug/onenil-desktop --font <verified-T51-ttf>`.
Use `--root <native-repository>` when launching from another directory, `--save <path>` for
an experiment save, and `--load` to restore it on startup. F7 saves from the main menu;
F9 reloads as a visibly labelled experiment shortcut. Space opens Players, Insert changes
skills/details, arrows select, Enter picks/places, Escape cancels a move or returns to caller.
Main-menu Enter enters preparation; PLAY GAME computes the fixed exhibition. Escape cancels
preparation. In the match, Space opens commentary history and Escape/Enter opens the bench;
PLAY GAME resumes. At half-time any key opens the bench; at full-time any key returns to main
without posting a result. F10 then Y quits. Unsupported options diagnose their limited scope.

[designed] Match pitch uses a new schematic field and figures with the accepted adapter's
coordinate projection, scaled into the original composition. Team colors are schematic;
custom font/clock, pitch figures and letterboxing remain proposed differences requiring owner
inspection. Native half/full-time boards currently display actual score/events and explicitly
unavailable statistics; they do not borrow the captured PSG match's figures. Bench is a bounded
resume view, not an implemented original team-management panel. These are comparison gaps,
not claims of completed B1–B6 fidelity. Keyboard only; no screen-reader semantics, IME or
Windows packaging demonstrated. Linux X11 at tested sizes does not establish those platforms.

The optional `--observe <path>` emits a read-only diagnostic snapshot for graphical acceptance;
it accepts no commands or state. `graphical_check.py` sends actual X11 key/resize/WM-close events,
records source/binary/font identity and native screenshots, and checks worker PID reaping.
The first run exposed lost short taps in sampled-key handling; the adapter now queues native
key edges through InputCallback. It does not rely on stretching test key duration.
