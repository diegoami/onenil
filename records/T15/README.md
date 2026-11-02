# T15 timing receipt

[designed] `T15-r0001.json` times40 parent API calls (20 repetitions of the two
recorded parents),including full direct-event/state and nested-role observation
serialization. Its75,356,780 output bytes are included in the timing. This is a
trace-producing bridge baseline,not an engine-only or emulator comparison.

[confirmed: T31/T33 at218490d] Per original two-parent corpus,the receipt's
`recorded_direct_events` includes106 T31 events (including two577F calls,
44 direct4EBF ratings and44 caller stores). `recorded_slot_calls` counts the74
nested8973 calls; `recorded_stores` counts27 nested role stores;
`recorded_caller_stores` separately counts44 parent byte18 updates. The82 visits
and148 scene/table calls are nested role observations. These counts describe
source coverage before the20 timing repetitions,not additional original runs.
