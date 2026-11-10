//! [review #176 F2/F3/F4] The W030 screens show only authenticated values: the
//! advanced menu carries the following fixture consistently, the transfer news
//! footer advertises only supported keys, and the cash flow reconciles.
#![cfg(feature = "raster")]
use onenil_campaign::Campaign;
use onenil_desktop::{
    controller::{Controller, Screen},
    render::Renderer,
};
fn renderer() -> Renderer {
    let font = std::env::var_os("ONENIL_DESKTOP_FONT")
        .expect("set ONENIL_DESKTOP_FONT to independently verified replacement font");
    Renderer::new(std::path::Path::new(&font)).unwrap()
}
fn week_one() -> Controller {
    let mut c = Controller::default();
    c.editor.campaign = Campaign::initial().unwrap();
    let ticket = c.editor.campaign.match_ticket().unwrap();
    c.editor
        .campaign
        .record_match(&ticket, [1, 4], "0AB9:EBE2")
        .unwrap();
    c.editor
        .campaign
        .record_week_completion(38_761, 69_270, 1, 176, 0, "Sevilla")
        .unwrap();
    c
}
#[test]
fn advanced_menu_shows_the_following_fixture_consistently() {
    let mut c = week_one();
    c.screen = Screen::Main;
    let mut r = renderer();
    r.draw(&c);
    let t = r.screen_text();
    assert!(t.contains("Sevilla"), "{}", t);
    assert!(t.contains("Aragones"), "{}", t);
    assert!(t.contains("Friendly #2"), "{}", t);
    assert!(t.contains("away"), "{}", t);
    assert!(!t.contains("Fernandez"), "{}", t);
    assert!(!t.contains("Friendly #1"), "{}", t);
    assert!(t.contains("421,491"), "{}", t);
}
#[test]
fn cash_flow_shows_every_authenticated_category_and_reconciles() {
    let mut c = week_one();
    c.screen = Screen::CashFlow;
    let mut r = renderer();
    r.draw(&c);
    let t = r.screen_text();
    assert!(t.contains("Gate receipts") && t.contains("38,761"), "{}", t);
    assert!(
        t.contains("Player salaries") && t.contains("57,320"),
        "{}",
        t
    );
    assert!(t.contains("Win bonuses"), "{}", t);
    assert!(t.contains("Staff salaries") && t.contains("4,200"), "{}", t);
    assert!(
        t.contains("Ground maintenance") && t.contains("7,750"),
        "{}",
        t
    );
    assert!(t.contains("TOTAL OUT") && t.contains("69,270"), "{}", t);
    assert!(t.contains("Club funds") && t.contains("421,491"), "{}", t);
    assert!(
        t.contains("Overdraft facility") && t.contains("2,920,000"),
        "{}",
        t
    );
    let receipt = onenil_desktop::week::WeekReceipt::pinned();
    assert_eq!(
        receipt.cash.player_salaries
            + receipt.cash.win_bonuses
            + receipt.cash.staff_salaries
            + receipt.cash.ground_maintenance,
        receipt.cash.total_out
    );
}
#[test]
fn transfer_news_footer_advertises_only_supported_keys() {
    let mut c = week_one();
    c.screen = Screen::TransferNews;
    let mut r = renderer();
    r.draw(&c);
    let t = r.screen_text();
    assert!(!t.contains("F7"), "{}", t);
    assert!(t.contains("Space: management menu"), "{}", t);
    assert!(t.contains("M.Kree"), "{}", t);
}
