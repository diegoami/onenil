#[cfg(feature = "raster")]
#[test]
fn resized_surface_preserves_edges_and_rejects_wrong_font() {
    use onenil_desktop::{controller::Controller, render::Renderer};
    let font = std::env::var_os("ONENIL_DESKTOP_FONT")
        .expect("set ONENIL_DESKTOP_FONT to independently verified replacement font");
    let mut r = Renderer::new(std::path::Path::new(&font)).unwrap();
    r.draw(&Controller::default());
    for (w, h) in [(1440, 900), (960, 640), (720, 450), (360, 200)] {
        let scaled = r.scaled(w, h);
        assert_eq!(scaled.len(), w * h);
        let top = (h - w * 400 / 720) / 2;
        assert_eq!(scaled[top * w], r.pixels[0]);
        assert_eq!(
            scaled[(top + w * 400 / 720 - 1) * w + w - 1],
            r.pixels[399 * 720 + 719]
        );
    }
    let wrong = std::env::temp_dir().join(format!("onenil-wrong-font-{}", std::process::id()));
    std::fs::write(&wrong, b"incorrect font").unwrap();
    assert!(Renderer::new(&wrong).is_err());
    std::fs::remove_file(wrong).unwrap();
}

#[cfg(feature = "raster")]
#[test]
fn chooser_preserves_decoded_border_and_unselected_background() {
    use onenil_desktop::{
        controller::{Controller, Screen},
        render::Renderer,
    };
    let font = std::env::var_os("ONENIL_DESKTOP_FONT").unwrap();
    let mut r = Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c = Controller::default();
    c.screen = Screen::SavedGames;
    c.load_entries = vec!["MANUAL".into(), "SAVE1".into()];
    r.draw(&c);
    // T43 step13 decoded top border foreground and list background.
    let cells: serde_json::Value =
        serde_json::from_str(include_str!("../assets/cells.json")).unwrap();
    let sheet = &cells["screens"]["saved-games"];
    assert_eq!(
        r.pixels[(13 * 16 + 8) * 720 + 26 * 9 + 4],
        sheet[13 * 80 + 26][1].as_u64().unwrap() as u32
    );
    assert_eq!(
        r.pixels[(16 * 16) * 720 + 26 * 9],
        sheet[16 * 80 + 26][2].as_u64().unwrap() as u32
    );
}

#[cfg(feature = "raster")]
#[test]
fn instruction_labels_follow_owned_state_and_options_keep_original_border() {
    use onenil_desktop::{
        controller::{Controller, Key, Screen},
        editor::Location,
        render::Renderer,
    };
    let font = std::env::var_os("ONENIL_DESKTOP_FONT").unwrap();
    let mut r = Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c = Controller::default();
    c.key(Key::Space);
    c.editor.select(Location::First(17)).unwrap();
    c.key(Key::Space);
    assert_eq!(c.screen, Screen::Options);
    r.draw(&c);
    let cells: serde_json::Value =
        serde_json::from_str(include_str!("../assets/cells.json")).unwrap();
    assert_eq!(
        r.pixels[(5 * 16 + 8) * 720 + 77 * 9 + 4],
        cells["screens"]["player-options"][5 * 80 + 77][1]
            .as_u64()
            .unwrap() as u32
    );
    for text in [
        "come out of box P",
        "come out of box",
        "stay in box P",
        "come out of box P",
    ] {
        assert_eq!(
            r.instruction_text(&c, c.editor.player(2598).unwrap(), 0),
            text
        );
        c.editor
            .campaign
            .cycle_goalkeeper_positioning(2598)
            .unwrap();
    }
}

#[cfg(feature = "raster")]
#[test]
fn changed_card_retains_original_detail_panel_dividers() {
    use onenil_desktop::{
        controller::{Controller, Key},
        editor::Location,
        render::Renderer,
    };
    let font = std::env::var_os("ONENIL_DESKTOP_FONT").unwrap();
    let mut r = Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c = Controller::default();
    c.key(Key::Space);
    c.editor.select(Location::First(17)).unwrap();
    c.key(Key::Space);
    c.editor
        .campaign
        .cycle_goalkeeper_positioning(2598)
        .unwrap();
    r.draw(&c);
    let cells: serde_json::Value =
        serde_json::from_str(include_str!("../assets/cells.json")).unwrap();
    for (x, y) in [(36, 12), (53, 8), (36, 18)] {
        assert_eq!(
            r.pixels[(y * 16 + 8) * 720 + x * 9 + 4],
            cells["screens"]["details"][y * 80 + x][1].as_u64().unwrap() as u32
        );
    }
}

#[cfg(feature = "raster")]
#[test]
fn cached_glyphs_and_repeated_redraws_produce_identical_owned_card() {
    use onenil_desktop::{
        controller::{Controller, Key},
        editor::Location,
        render::Renderer,
    };
    let font = std::env::var_os("ONENIL_DESKTOP_FONT").unwrap();
    let mut r = Renderer::new(std::path::Path::new(&font)).unwrap();
    let mut c = Controller::default();
    r.draw(&c);
    c.key(Key::Space);
    r.draw(&c);
    c.editor.select(Location::First(17)).unwrap();
    c.key(Key::Space);
    r.draw(&c);
    for _ in 0..4 {
        c.key(Key::Down);
    }
    c.key(Key::Enter);
    r.draw(&c);
    c.key(Key::Enter);
    r.draw(&c);
    c.key(Key::Enter);
    r.draw(&c);
    c.key(Key::Escape);
    c.key(Key::Escape);
    c.editor.select(Location::First(1)).unwrap();
    c.editor.campaign.set_position(2582, 1).unwrap();
    c.key(Key::Space);
    r.draw(&c);
    let mut fresh = Renderer::new(std::path::Path::new(&font)).unwrap();
    fresh.draw(&c);
    assert_eq!(r.pixels, fresh.pixels);
    assert_eq!(
        r.observed(serde_json::json!({}))["raster_sha256"],
        fresh.observed(serde_json::json!({}))["raster_sha256"]
    );
}
