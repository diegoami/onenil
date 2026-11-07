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
Hosted CI checks the standalone desktop crate in release mode, including the
owned campaign worker and its detailed timeline. Actual graphical acceptance
remains separately retained with source/binary/font identities and screenshots.

T53 candidate: native software window (minifb0.29.0/X11 on this machine), fontdue0.9.5,
shared Rust controller and direct local child. The cell export contains13 authenticated
public decoded text/style sheets; bitmap screenshots are acceptance evidence only and are
never loaded as UI backgrounds. Runtime rows, card selection, move state, menus, saved state,
clock, score,22 players and detailed commentary follow owned state. Only Flowers/Pearce cards
and Mimms details have captured data; other cards show an explicit unavailable statement.
The independent VGA font remains an ignored local dependency with T51's attribution/licence.

Build: `cargo build --release --manifest-path desktop/faithful/Cargo.toml --features software --locked`.
Run the resulting `desktop/faithful/target/release/onenil-desktop --font <verified-T51-ttf>`.
Use `--root <native-repository>` when launching from another directory, `--save <path>` for
an experiment save, and `--load` to restore it on startup. F7 saves from the main menu;
F9 reloads as a visibly labelled experiment shortcut. Space opens Players, Insert changes
skills/details, arrows select, Enter picks/places, Escape cancels a move or returns to caller.
Main-menu Enter enters the supplied campaign's preparation; PLAY GAME computes its
fresh PSG match only for the unchanged supported profile. E selects the separate
fixed exhibition. Escape cancels
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

## Candidate comparison (T54)

[designed] Both are Rust desktop transports of the same controller and software-drawn surface.
`--features software` chooses minifb/X11; `--features egui` chooses eframe0.37.0 with glow/X11.
With both features enabled, `--egui` selects the second candidate. Neither uses HTTP. The second
candidate's startup needed libxkbcommon-x11 on this minimal machine; packages were extracted
into ignored cache, not installed system-wide. It also required the graphical driver to wait
for a viewable mapped window before setting focus. These attempts remain recorded.

[confirmed: records/T54/cost-r0002.json] Linux/Xvfb,1440×900, debug profile, shared warm compile
cache, two-second idle main-menu sample. These numbers describe these adapters and this machine,
not release builds or general toolkit performance. The normal graph includes the app and engine.

| Measured property | Software window | egui/glow |
| --- | ---: | ---: |
| Normal dependency graph packages | 35 | 128 |
| Debug executable bytes | 64,946,192 | 249,501,128 |
| Main-menu RSS KiB | 23,396 | 139,680 |
| Idle CPU, percent of one core | 13.0% | 0.0% in two-second sample |
| Warm debug build seconds | 4.23 | 7.37 |

The first cost run retained continuous-redraw baseline (99%/129% of a core). T54 caches the
unchanged surface in both candidates and stops idle egui repaint requests. Software still blits
its cached scaled buffer to service expose events at60Hz; its remaining idle cost needs attention
before shipping. The egui route uploads a changed720×400 texture and lets its event loop sleep
when idle. Xvfb/software GL costs do not predict a real GPU. No cold/release/installer measurements.

[designed recommendation] Continue the faithful prototype with the software-window candidate:
it has the smaller dependency/binary/memory footprint and passed the same interaction and owned
Rust lifecycle cases. Keep egui as a measured alternative, especially if native controls or its
idle event loop become decisive. The owner still selects the production toolkit in #103.
No decision is implied by the default Cargo feature (there is none).

Both preserve contextual keys through actual native event queues; focus/resize and job cleanup
are acceptance cases, not a screenshot-only demonstration. Window resize at1440×900,960×640 and
720×450 is tested. DPI-scale changes, multi-monitor focus, IME and Windows/macOS delivery remain
unknown. Both are custom raster surfaces: neither has screen-reader semantics, editable native
text fields, native file pickers or touch affordances merely because a toolkit supports them.
Pixels remain the same original-led study across transports; font/clock/role aliases/figures,
limited cards, bench and missing statistics still need the owner's fidelity inspection.

Next gates are owner inspection/toolkit choice; a Windows build and distributable resource layout;
idle repaint cost; then B1–B6 (#107–#112) for actual saved management state, linked squad admission,
original management screens, result transaction and the next week. No browser publishing work
precedes desktop delivery. An end-to-end campaign and interactive managed substitutions are
separate production work, not completed by the exhibition replay.
