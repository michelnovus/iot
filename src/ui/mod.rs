// [GPLv3 License] Copyright (C) 2025  Michel Novus

mod gui;
mod tui;

use anyhow::Result;
use std::sync::mpsc;

use crate::{model::Message, settings::Settings};

#[cfg(feature = "egui")]
pub use gui::UserInterface;

#[cfg(not(feature = "egui"))]
pub use tui::UserInterface;

pub trait UI {
    fn new(settings: Settings, rx: mpsc::Receiver<Message>) -> Self;
    fn mainloop(self) -> Result<()>;
}
