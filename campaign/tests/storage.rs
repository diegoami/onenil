use onenil_campaign::{storage::PendingSave, Campaign, Location};
use std::{fs, path::PathBuf};
fn directory() -> PathBuf {
    let p = std::env::temp_dir().join(format!("onenil-native-save-{}", std::process::id()));
    fs::create_dir_all(&p).unwrap();
    p
}
#[test]
fn prepared_and_failed_replacements_preserve_the_committed_file() {
    let directory = directory();
    let path = directory.join("campaign.json");
    let old = Campaign::initial().unwrap();
    let outcome = old.save(&path).unwrap();
    assert!(outcome.committed);
    let committed = fs::read(&path).unwrap();
    let mut changed = old.clone();
    changed
        .exchange(Location::First(0), Location::First(17))
        .unwrap();
    let staged = PendingSave::prepare(&changed, &path).unwrap();
    let staged_path = staged.pending_path().to_owned();
    assert_eq!(fs::read(&path).unwrap(), committed);
    drop(staged);
    assert!(!staged_path.exists());
    let staged = PendingSave::prepare(&changed, &path).unwrap();
    fs::remove_file(staged.pending_path()).unwrap();
    assert!(staged.commit().is_err());
    assert_eq!(fs::read(&path).unwrap(), committed);
    let mut invalid = changed.clone();
    invalid.club.first_team[1] = invalid.club.first_team[0];
    assert!(invalid.save(&path).is_err());
    assert_eq!(fs::read(&path).unwrap(), committed);
    changed.save(&path).unwrap();
    assert_eq!(Campaign::load(&path).unwrap(), changed);
    fs::remove_dir_all(directory).unwrap();
}
