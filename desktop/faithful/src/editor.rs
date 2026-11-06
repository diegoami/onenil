//! [confirmed] T46 steps2–8/T59 steps150–153, corrected by research T64.
//! [designed] Eighteen captured example identities and one experiment reserve slot.
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

pub const NAMES: [&str; 18] = [
    "T Flowers",
    "I Pearce",
    "C Hendry",
    "G Le Saux",
    "H Berg",
    "M Atkins",
    "T Sherwood",
    "J Wilcox",
    "S Ripley",
    "C Sutton",
    "A Shearer",
    "T Gale",
    "P Warhurst",
    "R Slater",
    "N Marker",
    "M Newell",
    "A Wright",
    "B Mimms",
];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    First(usize),
    Reserve,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Skills,
    Details,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Editor {
    pub first: [Option<u8>; 18],
    pub reserve: Option<u8>,
    pub selected: Location,
    pub panel: Panel,
    carried: Option<(Location, u8)>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Saved {
    pub schema: String,
    pub version: u32,
    pub first: [Option<u8>; 18],
    pub reserve: Option<u8>,
}
impl Default for Editor {
    fn default() -> Self {
        Self {
            first: std::array::from_fn(|i| Some(i as u8)),
            reserve: None,
            selected: Location::First(0),
            panel: Panel::Skills,
            carried: None,
        }
    }
}
impl Editor {
    fn at(&self, at: Location) -> Option<u8> {
        match at {
            Location::First(i) => self.first[i],
            Location::Reserve => self.reserve,
        }
    }
    fn put(&mut self, at: Location, value: Option<u8>) {
        match at {
            Location::First(i) => self.first[i] = value,
            Location::Reserve => self.reserve = value,
        }
    }
    pub fn select(&mut self, at: Location) -> Result<(), String> {
        if matches!(at, Location::First(i) if i >= 18) {
            return Err("outside captured experiment roster".into());
        }
        self.selected = at;
        Ok(())
    }
    pub fn card(&self) -> Option<u8> {
        self.at(self.selected)
    }
    pub fn carried_player(&self) -> Option<u8> {
        self.carried.map(|(_, p)| p)
    }
    pub fn pick(&mut self) -> Result<(), String> {
        if self.carried.is_some() {
            return Err("move already pending".into());
        }
        self.carried = Some((self.selected, self.card().ok_or("empty source")?));
        Ok(())
    }
    pub fn place(&mut self) -> Result<(), String> {
        let (source, player) = self.carried.ok_or("no pending move")?;
        let destination = self.card();
        self.put(source, destination);
        self.put(self.selected, Some(player));
        self.carried = None;
        Ok(())
    }
    pub fn cancel(&mut self) {
        self.carried = None;
    }
    pub fn save(&self, path: &Path) -> Result<(), String> {
        if self.carried.is_some() {
            return Err("finish or cancel the pending move before saving".into());
        }
        let saved = Saved {
            schema: "onenil-captured-editor".into(),
            version: 1,
            first: self.first,
            reserve: self.reserve,
        };
        let bytes = serde_json::to_vec(&saved).map_err(|e| e.to_string())?;
        Self::decode(&bytes)?;
        let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        let result = (|| {
            file.write_all(&bytes)?;
            file.sync_all()?;
            fs::rename(&temporary, path)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result.map_err(|e| e.to_string())
    }
    fn decode(bytes: &[u8]) -> Result<Saved, String> {
        let saved: Saved = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if saved.schema != "onenil-captured-editor" || saved.version != 1 {
            return Err("unsupported experiment save".into());
        }
        let mut ids: Vec<u8> = saved
            .first
            .iter()
            .copied()
            .chain([saved.reserve])
            .flatten()
            .collect();
        ids.sort();
        if ids != (0..18).collect::<Vec<_>>() {
            return Err("missing, duplicate or unknown player identity".into());
        }
        Ok(saved)
    }
    pub fn load_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        let saved = Self::decode(bytes)?;
        self.first = saved.first;
        self.reserve = saved.reserve;
        self.carried = None;
        Ok(())
    }
    pub fn load(&mut self, path: &Path) -> Result<(), String> {
        self.load_bytes(&fs::read(path).map_err(|e| e.to_string())?)
    }
}
