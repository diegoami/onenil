//! [designed] Desktop feasibility controller; original evidence in docs/desktop-feasibility.md.
pub mod application;
pub mod controller;
pub mod editor;
#[cfg(feature = "raster")]
pub mod render;
#[cfg(feature = "software")]
pub mod software;
pub mod timeline;
pub mod worker;

#[cfg(feature = "egui")]
pub mod egui_window;
