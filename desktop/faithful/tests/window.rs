#[test]
fn native_window_adapter_is_present() {
    // A source admission check precedes real X11 input acceptance; it cannot substitute for it.
    let tracked=std::process::Command::new("git").args(["ls-files","--error-unmatch","src/software.rs"]).current_dir(env!("CARGO_MANIFEST_DIR")).output().unwrap();
    assert!(tracked.status.success(),"native software window adapter has not been implemented/committed");
}
