# Post-match translation boundary

[confirmed: archaeology ba499b9a678254d6a14497153581f85f93ee4e18,
T86, docs/postmatch-tail.md] Original E679 encloses the post-match work and
returns to EBE6. The accepted watched league specimen observes 35 condition
calls, 103 total RNG draws and 439 writes. Its scheduled clubs are 134/130;
the result, live-score and club-pair globals have distinct orders.

[designed:T82] `tools/generate_post_match.py` compiles this static direct-call
closure into the private `watched_compiled::post_match_graph` module. The
record `records/T82/graph.json` binds seven listing digests, reused action and
match graph digests, generated Rust, source addresses and the research commit.
There are 21 translated procedures (1,639 instructions) and one explicitly
unported procedure, 1998:14A5. The latter immediately returns an error. The
existing T81 condition implementation remains the authority for that routine;
this graph does not duplicate it. Unexpected unsupported instructions fail
at generation rather than becoming silent runtime placeholders.

[derived: same static source] The newly translated procedures cover:

| Entries | Scope |
| --- | --- |
| 0AB9:E4CA/E532/E679 | Club context and enclosing caller |
| 1998:0DD5/0E25/0F06/0F7E/10CA/1126 | Form/performance or random branch helpers |
| 1998:12BE/154E/1897/1B99 | Player/club processing and branch predicates |
| 1B55:1798/20B1/210F/223F/2301/24C3/4A8F | Shared bookkeeping, table and staff helpers |
| 232A:01E3 | Numeric helper |

[designed:T82] Compile checks establish syntax, the Cpu interface and exact
reproducibility. They do **not** establish original-record execution parity,
new branch support, input admission or campaign behavior. There is no public
entry into this module. Existing desktop and weekly transitions do not call it.

[designed:next slice] Before exposing an execution entry, add an adapter from
the actual Cpu image/pointers and current seed to the proven T81 kernel.
Expected outputs and recorded exit seeds must never enter execution. Respect
ABI returns, shared-image aliasing and store order. Capture a genuine failing
original-reference replay before implementing the adapter. Isolated tests may
initialize the complete original caller-entry contract, explicitly separating
read-before-write inputs, preserved context and scratch/pointer bindings.
Match both T86 repeats' final fields, all 103 ordered RNG requests/values/seeds
and all 439 observable writes, including repeated stores and their widths.
Reject missing/unsupported inputs without committing partial live state.
Document unexercised draw, injury and competition branches.

[designed:subsequent admission/integration] A continuing native match must
receive any newly needed incoming fields from authenticated pre-match data.
A caller-entry replay fixture is not a way to inject late snapshots into a
continuing engine. Campaign schema, atomic save/reload and exactly-once week
integration need separate checks before enabling player updates. The T85
quick-friendly handoff differs from this T86 league specimen; its 44 prefix
RNG draws cannot be transplanted into the performance branch, which has zero.

Run the public static check without private game bytes:

```sh
python3 tools/test_post_match_graph.py --research-root ../onenil-archaeology
cargo test --manifest-path engine/Cargo.toml --locked
```
