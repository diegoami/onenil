//! [designed] Public campaign result/storage contract; no invented engine behavior.
use onenil_campaign::{storage::PendingSave, Campaign, Location};
use std::{fs, process::Command};

#[test]
fn result_is_owned_once_and_failed_completions_are_atomic() {
    let initial = Campaign::initial().unwrap();
    let ticket = initial.match_ticket().unwrap();
    let mut campaign = initial.clone();
    campaign.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    let result = campaign.played_fixture.as_ref().unwrap();
    assert_eq!(result.fixture, initial.next_fixture);
    assert_eq!(result.season, initial.season);
    assert_eq!(result.week_index, initial.week_index);
    assert_eq!(result.score, [1, 4]);
    assert_eq!(result.request_revision, 0);
    assert_eq!(result.request_sha256.len(), 64);
    assert_eq!(result.native_return, "0AB9:EBE2");
    assert_eq!(campaign.revision, 1);
    let mut projection = campaign.clone();
    projection.played_fixture = None;
    projection.revision = initial.revision;
    assert_eq!(projection, initial);
    let posted = campaign.clone();
    assert!(campaign.record_match(&ticket, [9, 9], "0AB9:EBE2").is_err());
    assert!(campaign.match_ticket().is_err());
    assert_eq!(campaign, posted);
    for return_address in ["", "0AB9:0000", "0ab9:ebe2"] {
        let mut live = initial.clone();
        assert!(live.record_match(&ticket, [1, 4], return_address).is_err());
        assert_eq!(live, initial);
    }
    let mut changed = initial.clone();
    changed
        .exchange(Location::First(0), Location::First(1))
        .unwrap();
    let changed_ticket = changed.match_ticket().unwrap();
    let before = changed.clone();
    assert!(changed.record_match(&ticket, [1, 4], "0AB9:EBE2").is_err());
    assert_eq!(changed, before);
    let mut live = initial.clone();
    assert!(live
        .record_match(&changed_ticket, [1, 4], "0AB9:EBE2")
        .is_err());
    assert_eq!(live, initial);
    let mut foreign = initial.clone();
    foreign.next_fixture.opponent = 134;
    assert!(foreign.match_ticket().is_err());
    let mut exhausted = initial.clone();
    exhausted.revision = u64::MAX;
    let ticket = exhausted.match_ticket().unwrap();
    let before = exhausted.clone();
    assert!(exhausted
        .record_match(&ticket, [1, 4], "0AB9:EBE2")
        .is_err());
    assert_eq!(exhausted, before);
}

#[test]
fn stored_result_context_is_validated_without_replacing_live_state() {
    let mut live = Campaign::initial().unwrap();
    let ticket = live.match_ticket().unwrap();
    live.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    let before = live.clone();
    for damage in [
        "fixture", "season", "week", "hash", "revision", "return", "extra",
    ] {
        let mut value = serde_json::to_value(&live).unwrap();
        let result = &mut value["played_fixture"];
        match damage {
            "fixture" => result["fixture"]["opponent"] = 134.into(),
            "season" => result["season"]["counter"] = 9.into(),
            "week" => result["week_index"] = 3.into(),
            "hash" => result["request_sha256"] = "foreign".into(),
            "revision" => result["request_revision"] = 1.into(),
            "return" => result["native_return"] = "unfinished".into(),
            _ => result["unknown"] = true.into(),
        }
        assert!(
            live.replace_from_bytes(&serde_json::to_vec(&value).unwrap())
                .is_err(),
            "{damage}"
        );
        assert_eq!(live, before);
    }
}

#[test]
fn result_survives_atomic_storage_and_a_fresh_process() {
    let directory = std::env::temp_dir().join(format!("onenil-result-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("campaign.json");
    let mut campaign = Campaign::initial().unwrap();
    let old = campaign.encode().unwrap();
    assert!(!serde_json::from_slice::<serde_json::Value>(&old)
        .unwrap()
        .as_object()
        .unwrap()
        .contains_key("played_fixture"));
    assert_eq!(Campaign::decode(&old).unwrap(), campaign);
    let ticket = campaign.match_ticket().unwrap();
    campaign.record_match(&ticket, [1, 4], "0AB9:EBE2").unwrap();
    campaign.save(&path).unwrap();
    let bytes = fs::read(&path).unwrap();
    let mut changed = campaign.clone();
    changed.set_instructions(2598, 16).unwrap();
    let pending = PendingSave::prepare(&changed, &path).unwrap();
    fs::remove_file(pending.pending_path()).unwrap();
    assert!(pending.commit().is_err());
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert!(Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "fresh_process_result_reader", "--nocapture"])
        .env("ONENIL_RESULT_TEST_PATH", &path)
        .status()
        .unwrap()
        .success());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn fresh_process_result_reader() {
    let Some(path) = std::env::var_os("ONENIL_RESULT_TEST_PATH") else {
        return;
    };
    let mut loaded = Campaign::load(std::path::Path::new(&path)).unwrap();
    assert_eq!(loaded.played_fixture.as_ref().unwrap().score, [1, 4]);
    assert_eq!(loaded.revision, 1);
    let before = loaded.clone();
    let ticket = Campaign::initial().unwrap().match_ticket().unwrap();
    assert!(loaded.record_match(&ticket, [0, 0], "0AB9:EBE2").is_err());
    assert_eq!(loaded, before);
}
