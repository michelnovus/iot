// [GPLv3 License] Copyright (C) 2025  Michel Novus
#![allow(unused)]
use std::sync::mpsc;

use anyhow::Result;

use super::UI;
use crate::{model::Message, settings::Settings};

pub struct UserInterface;

impl UI for UserInterface {
    fn new(settings: Settings, rx: mpsc::Receiver<Message>) -> Self {
        UserInterface
    }
    fn mainloop(self) -> Result<()> {
        Ok(())
    }
}
