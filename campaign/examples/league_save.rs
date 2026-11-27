//! [designed:T119] Write the committed late-season league save the desktop loads.
fn main() -> Result<(), String> {
    let path = std::env::args().nth(1).ok_or("usage: league_save <path>")?;
    let campaign = onenil_campaign::Campaign::initial_league()?;
    std::fs::write(path, campaign.encode()?).map_err(|error| error.to_string())
}
