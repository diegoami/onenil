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
