//! [designed:T120] Print the played league table/results model for its fixture.
fn main() -> Result<(), String> {
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
    let played = onenil_desktop::played_league::played_league(&campaign)
        .ok_or("missing the played league model")?;
    println!(
        "{}",
        serde_json::to_string_pretty(&played).map_err(|error| error.to_string())?
    );
    Ok(())
}
