//! [confirmed] T65 saved club/real IDs; T64 destination-following card during PUT HERE.
use onenil_campaign::{storage::SaveOutcome, Campaign, Location as StoredLocation, Player};
use std::path::Path;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    First(usize),
    Reserve(usize),
}
impl From<Location> for StoredLocation {
    fn from(at: Location) -> Self {
        match at {
            Location::First(i) => Self::First(i),
            Location::Reserve(i) => Self::Reserve(i),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Skills,
    Details,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Editor {
    pub campaign: Campaign,
    pub selected: Location,
    pub panel: Panel,
    carried: Option<(Location, u16)>,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            campaign: Campaign::initial().expect("verified compiled original campaign seed"),
            selected: Location::First(0),
            panel: Panel::Skills,
            carried: None,
        }
    }
}
impl Editor {
    pub fn select(&mut self, at: Location) -> Result<(), String> {
        self.campaign.at(at.into())?;
        self.selected = at;
        Ok(())
    }
    pub fn card(&self) -> Option<u16> {
        self.campaign.at(self.selected.into()).ok().flatten()
    }
    pub fn player(&self, id: u16) -> Option<&Player> {
        self.campaign.players.iter().find(|p| p.id == id)
    }
    pub fn carried_player(&self) -> Option<u16> {
        self.carried.map(|(_, id)| id)
    }
    pub fn pick(&mut self) -> Result<(), String> {
        if self.carried.is_some() {
            return Err("move already pending".into());
        }
        self.carried = Some((self.selected, self.card().ok_or("empty source")?));
        Ok(())
    }
    pub fn place(&mut self) -> Result<(), String> {
        let (source, id) = self.carried.ok_or("no pending move")?;
        if self.campaign.at(source.into())? != Some(id) {
            return Err("carried source identity changed".into());
        }
        self.campaign
            .exchange(source.into(), self.selected.into())?;
        self.carried = None;
        Ok(())
    }
    pub fn cancel(&mut self) {
        self.carried = None;
    }
    pub fn save(&self, path: &Path) -> Result<SaveOutcome, String> {
        if self.carried.is_some() {
            return Err("finish or cancel pending move before saving".into());
        }
        self.campaign.save(path)
    }
    pub fn load_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        let candidate = Campaign::decode(bytes)?;
        self.campaign = candidate;
        self.carried = None;
        Ok(())
    }
    pub fn load(&mut self, path: &Path) -> Result<(), String> {
        let candidate = Campaign::load(path)?;
        self.campaign = candidate;
        self.carried = None;
        Ok(())
    }
}
