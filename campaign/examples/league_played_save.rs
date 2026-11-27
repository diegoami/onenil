//! [designed:T120] Write the T119 post-week league save the played screens load.
fn main() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: league_played_save <path>")?;
    let mut campaign = onenil_campaign::Campaign::initial_league()?;
    let ticket = campaign.match_ticket()?;
    campaign.record_league_week(
        &ticket,
        [0, 2],
        "0AB9:EBE2",
        726_972,
        80_639,
        0,
        103,
        0,
        "Chelsea",
    )?;
    std::fs::write(path, campaign.encode()?).map_err(|error| error.to_string())
}
