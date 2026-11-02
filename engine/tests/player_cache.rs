use onenil_engine::player_cache::{self, State};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/player_cache/cases.jsonl")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
#[test]
fn actual_original_parent_roots_and_every_continuation_replay() {
    let cases = cases();
    assert_eq!(cases.len(), 2);
    for c in cases {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(player_cache::refresh(&s).unwrap()).unwrap(),
            c["output"]
        );
    }
}
#[test]
fn actual_original_parent_direct_and_nested_census() {
    let (
        mut events,
        mut roles,
        mut visits,
        mut slots,
        mut scores,
        mut tables,
        mut stores,
        mut caller_stores,
    ) = (0, 0, 0, 0, 0, 0, 0, 0);
    for c in cases() {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        let out = player_cache::refresh(&s).unwrap();
        events += out.events.len();
        caller_stores += out.events.iter().filter(|e| e["kind"] == "store").count();
        roles += out.roles.len();
        for r in out.roles {
            for v in r["visits"].as_array().unwrap() {
                visits += 1;
                stores += v["stored"].as_u64().unwrap();
                for h in v["helpers"].as_array().unwrap() {
                    match h["kind"].as_str().unwrap() {
                        "slot" => slots += 1,
                        "old-score" | "new-score" => scores += 1,
                        _ => (),
                    };
                    tables += h["children"].as_array().unwrap().len();
                }
            }
        }
    }
    assert_eq!(
        (
            events,
            roles,
            visits,
            slots,
            scores,
            tables,
            stores,
            caller_stores
        ),
        (106, 8, 82, 74, 148, 148, 27, 44)
    );
}
