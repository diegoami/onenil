//! [designed] Desktop feasibility controller; original evidence in docs/desktop-feasibility.md.
pub mod application;
pub mod campaign_match;
pub mod controller;
pub mod editor;
pub mod history;
#[cfg(feature = "raster")]
pub mod render;
#[cfg(feature = "software")]
pub mod software;
pub mod timeline;
pub mod week;
pub mod worker;

#[cfg(feature = "egui")]
pub mod egui_window;

pub mod live_worker;
