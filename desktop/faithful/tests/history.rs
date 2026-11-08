//! [designed:T73] Synthetic presentation fixtures, never engine observations.
use onenil_desktop::{controller::{Controller, Key, Screen}, timeline::{Playback, Timeline}};
fn history(count: usize) -> Controller {
    let view=serde_json::json!({"clock":12,"score":[0,0],"ball":{"x":0,"y":0},"players":[]});
    let events:Vec<_>=(0..count).map(|i|serde_json::json!({"id":i,"native":{"minute":i},"kind":"test","text":format!("Test player action {i}")})).collect();
    let timeline:Timeline=serde_json::from_value(serde_json::json!({
        "club_names":["Test A","Test B"],"initial":view,"frames":[{"view":view,"events":[{"id":999,"native":{"minute":99},"kind":"test","text":"FUTURE UNREACHED"}]}],
        "setup_events":events,"native_return":"0AB9:EBE2","draws":0,"clock_rows":0,"seed":0,"clubs":[130,17],"marker_rows":[[],[]]
    })).unwrap();
    let mut p=Playback::new(timeline); p.start_or_continue().unwrap();
    let mut c=Controller::default(); c.screen=Screen::Pitch; c.playback=Some(p); c.key(Key::Space); c
}
#[cfg(feature="raster")]
#[test]
fn history_up_changes_visible_native_action_text() {
    let font=std::env::var_os("ONENIL_DESKTOP_FONT").expect("verified replacement font");
    let mut r=onenil_desktop::render::Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c=history(45); r.draw(&c); let latest=r.pixels.clone();
    c.key(Key::Up); r.draw(&c);
    assert_ne!(&r.pixels[5*16*720..23*16*720],&latest[5*16*720..23*16*720],"Up must reveal earlier history text");
}
