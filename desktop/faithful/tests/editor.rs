use onenil_desktop::editor::{Editor, Location, Panel, Saved};
use std::fs;

#[test]
fn carried_source_and_destination_card_are_separate() {
    let mut e = Editor::default();
    e.panel = Panel::Details;
    e.pick().unwrap();
    e.select(Location::First(17)).unwrap();
    assert_eq!(e.card(), Some(17));
    assert_eq!(e.carried_player(), Some(0));
    e.cancel();
    assert_eq!(e.card(), Some(17));
    assert_eq!(e.first[0], Some(0));
    assert_eq!(e.first[17], Some(17));
    e.pick().unwrap();
    e.select(Location::First(0)).unwrap();
    e.place().unwrap();
    assert_eq!(e.card(), Some(17));
    assert_eq!(e.first[17], Some(0));
    assert_eq!(e.panel, Panel::Details);
    e.select(Location::Reserve).unwrap();
    assert_eq!(e.card(), None);
    e.select(Location::First(0)).unwrap();
    assert_eq!(e.card(), Some(17));
}

#[test]
fn invalid_save_is_atomic_in_memory_and_on_disk() {
    let dir = std::env::temp_dir().join(format!("onenil-editor-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("squad.json");
    let mut e = Editor::default();
    e.pick().unwrap();
    e.select(Location::Reserve).unwrap();
    e.place().unwrap();
    e.save(&path).unwrap();
    let good = fs::read(&path).unwrap();
    let mut loaded = Editor::default();
    loaded.load(&path).unwrap();
    assert_eq!(loaded.first, e.first);
    assert_eq!(loaded.reserve, Some(0));
    let unchanged = loaded.clone();
    for malformed in [
        b"{".to_vec(),
        good.replace_bytes(b"\"version\":1", b"\"version\":2"),
    ] {
        assert!(loaded.load_bytes(&malformed).is_err());
        assert_eq!(loaded, unchanged);
    }
    let mut duplicate: Saved = serde_json::from_slice(&good).unwrap();
    duplicate.first[1] = Some(0);
    assert!(loaded
        .load_bytes(&serde_json::to_vec(&duplicate).unwrap())
        .is_err());
    assert_eq!(loaded, unchanged);
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, b"occupied temporary file").unwrap();
    assert!(loaded.save(&path).is_err());
    assert_eq!(fs::read(&path).unwrap(), good);
    fs::remove_file(temporary).unwrap();
    for bad in [
        b"{\"schema\":\"onenil-captured-editor\",\"version\":1,\"first\":[]}".as_slice(),
        b"{\"unexpected\":true}".as_slice(),
    ] {
        fs::write(&path, bad).unwrap();
        assert!(loaded.load(&path).is_err());
        assert_eq!(loaded, unchanged);
    }
    fs::write(&path, &good).unwrap();
    loaded.select(Location::First(1)).unwrap();
    loaded.pick().unwrap();
    assert!(loaded.save(&path).is_err());
    assert_eq!(fs::read(&path).unwrap(), good);
    fs::remove_dir_all(dir).unwrap();
}

trait ReplaceBytes {
    fn replace_bytes(&self, a: &[u8], b: &[u8]) -> Vec<u8>;
}
impl ReplaceBytes for Vec<u8> {
    fn replace_bytes(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        let s = std::str::from_utf8(self).unwrap();
        s.replace(
            std::str::from_utf8(a).unwrap(),
            std::str::from_utf8(b).unwrap(),
        )
        .into_bytes()
    }
}
