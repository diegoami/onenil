use onenil_desktop::{controller::{Controller,Key,Screen},editor::Location};
#[test]
fn real_campaign_identity_and_exhibition_boundary() {
    let mut c=Controller::default();assert_eq!(c.editor.campaign.club.id,130);assert_eq!(c.editor.card(),Some(2581));
    let initial=c.editor.campaign.clone();
    c.key(Key::Space);c.key(Key::Enter);c.key(Key::Down);c.key(Key::Escape);
    assert_eq!(c.editor.campaign,initial);assert_eq!(c.editor.card(),Some(2582));
    c.key(Key::Enter);c.key(Key::Up);c.key(Key::Enter);assert_eq!(c.editor.campaign.club.first_team[..2],[Some(2582),Some(2581)]);assert_eq!(c.editor.campaign.revision,1);
    c.key(Key::Escape);c.key(Key::Enter);c.key(Key::Enter);assert_eq!(c.screen,Screen::Preparation);assert!(!c.prepare_requested);assert!(!c.exhibition);
    c.key(Key::Exhibition);assert!(c.exhibition);c.key(Key::Enter);assert_eq!(c.screen,Screen::Preparing);
    assert_eq!(c.editor.campaign.week_index,0);assert_eq!(c.editor.campaign.next_fixture.opponent,17);
    c.key(Key::Escape);assert_eq!(c.screen,Screen::Preparation);assert!(c.exhibition);
    c.editor.select(Location::First(0)).unwrap();
}
