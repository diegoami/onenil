//! [designed:T72] Display metadata cannot bypass owned worker/campaign admission.
use onenil_desktop::{
    controller::{Controller, Key, Screen},
    timeline,
};
#[test]
fn actual_marker_rows_are_complete_and_malformed_packets_preserve_campaign() {
    let campaign = onenil_campaign::Campaign::initial().unwrap();
    let actual = timeline::compute_campaign(&campaign).unwrap();
    actual.validate_markers().unwrap();
    assert_eq!(actual.marker_rows.len(), 3826);
    assert_eq!(actual.marker_rows[0].len(), 22);
    assert_eq!(actual.frames.len(), 3825);
    for damage in 0..3 {
        let mut packet = actual.clone();
        match damage {
            0 => {
                packet.marker_rows.pop();
            }
            1 => {
                packet.marker_rows[0].pop();
            }
            _ => packet.marker_rows[0][0][0] = 256,
        }
        let mut c = Controller::default();
        c.key(Key::Enter);
        c.key(Key::Enter);
        c.accept_timeline(c.job_id, packet);
        assert_eq!(c.screen, Screen::Preparation);
        assert!(c.message.contains("display data"));
        assert_eq!(c.editor.campaign, campaign);
        assert!(c.playback.is_none() && !c.prepare_requested);
    }
}
