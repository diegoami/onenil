//! [designed:T77] Incomplete desktop playback cannot claim or post completion.
use onenil_desktop::{controller::{Controller,Key,Screen},timeline::LiveCampaign};
#[test]
fn desktop_accepts_a_reached_interval_without_claiming_a_completed_match(){
    let mut c=Controller::default();c.key(Key::Enter);c.key(Key::Enter);
    let original=c.editor.campaign.clone();let mut engine=LiveCampaign::new(&original).unwrap();
    let prefix=engine.pause().unwrap();
    assert!(c.accept_interval(c.job_id,prefix));
    assert_eq!(c.screen,Screen::Pitch);assert!(c.playback.is_none());
    assert_eq!(c.match_view().unwrap().cursor,0);
    assert_eq!(c.editor.campaign,original);
}
