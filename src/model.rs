// [GPLv3 License] Copyright (C) 2025  Michel Novus
use crate::{
    api::Translation,
    settings::{InputKind, Settings},
};

/// Mensages de comunicación que genera la lógica de negocio hacia la vista.
pub enum Message {
    Translation(Translation),
    InputKind(InputKind),
    Settings(Settings),
    Error {
        header: String,
        source: anyhow::Error,
    },
    Quit,
}
