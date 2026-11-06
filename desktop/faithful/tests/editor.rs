use onenil_desktop::editor::{Editor, Location, Panel};
use std::fs;
#[test]
fn carried_source_and_destination_card_are_separate() {
    let mut e = Editor::default();
    e.panel = Panel::Details;
    e.pick().unwrap();
    e.select(Location::First(17)).unwrap();
    assert_eq!(e.card(), Some(2598));
    assert_eq!(e.carried_player(), Some(2581));
    let original = e.campaign.clone();
    e.cancel();
    assert_eq!(e.card(), Some(2598));
    assert_eq!(e.campaign, original);
    e.pick().unwrap();
    e.select(Location::First(0)).unwrap();
    e.place().unwrap();
    assert_eq!(e.card(), Some(2598));
    assert_eq!(e.campaign.club.first_team[17], Some(2581));
    assert_eq!(e.panel, Panel::Details);
    e.select(Location::Reserve(0)).unwrap();
    assert_eq!(e.card(), None);
    e.select(Location::First(0)).unwrap();
    assert_eq!(e.card(), Some(2598));
}
#[test]
fn invalid_save_is_atomic_in_memory_and_on_disk() {
    let dir = std::env::temp_dir().join(format!("onenil-editor-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("campaign.json");
    let mut e = Editor::default();
    e.pick().unwrap();
    e.select(Location::Reserve(0)).unwrap();
    e.place().unwrap();
    e.save(&path).unwrap();
    let good = fs::read(&path).unwrap();
    let mut loaded = Editor::default();
    loaded.load(&path).unwrap();
    assert_eq!(loaded.campaign, e.campaign);
    assert_eq!(loaded.campaign.club.reserves[0], Some(2581));
    let unchanged = loaded.clone();
    for bad in [
        b"{".to_vec(),
        br#"{"schema":"onenil-captured-editor","version":1}"#.to_vec(),
    ] {
        assert!(loaded.load_bytes(&bad).is_err());
        assert_eq!(loaded, unchanged);
    }
    let mut duplicate: serde_json::Value = serde_json::from_slice(&good).unwrap();
    duplicate["club"]["first_team"][1] = 2581.into();
    assert!(loaded
        .load_bytes(&serde_json::to_vec(&duplicate).unwrap())
        .is_err());
    assert_eq!(loaded, unchanged);
    loaded.select(Location::First(1)).unwrap();
    loaded.pick().unwrap();
    assert!(loaded.save(&path).is_err());
    assert_eq!(fs::read(&path).unwrap(), good);
    fs::remove_dir_all(dir).unwrap();
}
