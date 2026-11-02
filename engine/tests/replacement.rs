use onenil_engine::replacement::{self, FlagsInput, State};
use serde_json::Value;
fn cases() -> Vec<Value> {
    include_str!("fixtures/replacement/cases.jsonl")
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
#[test]
fn original_two_replacements_and_twenty_six_helpers() {
    let mut n = 0;
    for c in cases()
        .into_iter()
        .filter(|c| c["stream"] == "replacements")
    {
        let s: State = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(replacement::replace(&s).unwrap()).unwrap(),
            c["output"]
        );
        n += 1;
    }
    assert_eq!(n, 2);
}
#[test]
fn original_twenty_five_compact_rating_flags() {
    let mut n = 0;
    for c in cases().into_iter().filter(|c| c["stream"] == "flags") {
        let s: FlagsInput = serde_json::from_value(c["input"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(replacement::flags(&s).unwrap()).unwrap(),
            c["output"]
        );
        n += 1;
    }
    assert_eq!(n, 25);
}
