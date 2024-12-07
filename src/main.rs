// OIT: Omnipresent Instant Translator.
// Copyright (C) 2025  Michel Novus
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::{path::PathBuf, sync::mpsc, thread};

use anyhow::{anyhow, Result};
use directories::BaseDirs;
use oit::{
    api::{translator::GoogleTranslate, Lang, Translator},
    input,
    model::Message,
    settings::{resolve_settings, Settings},
    ui::{UserInterface, UI},
};

fn main() -> Result<()> {
    let settings = try_load_settings().unwrap_or_else(|err| {
        eprintln!("{:#?}", err);
        Settings::default()
    });
    let (tx, rx) = mpsc::channel::<Message>();
    let (m_settings, m_tx) = (settings.clone(), tx.clone());
    let model_handler = thread::spawn(move || {
        match input::collect(&m_settings.order()) {
            Ok(inp) => {
                match GoogleTranslate.translate(
                    inp.text(),
                    Lang {
                        from: *m_settings.source(),
                        to: *m_settings.target(),
                    },
                    Some(5),
                ) {
                    Ok(tr) => {
                        let _ = m_tx.send(Message::InputKind(*inp.kind()));
                        let _ = m_tx.send(Message::Translation(tr));
                    }
                    Err(err) => {
                        let _ = m_tx.send(Message::Error {
                            header: err.to_string(),
                            source: err,
                        });
                    }
                }
            }
            Err(err) => {
                let _ = m_tx.send(Message::Error {
                    header: String::from("buffer error"),
                    source: err,
                });
            }
        }
        #[cfg(not(feature = "egui"))]
        let _ = m_tx.send(Message::Quit);
    });
    let view_result = UserInterface::new(settings, rx).mainloop();
    let _ = model_handler.join();
    view_result
}

fn try_load_settings() -> Result<Settings> {
    let user_settings_file: Option<PathBuf> = match BaseDirs::new() {
        Some(dir) => {
            let mut config_dir: PathBuf = dir.config_local_dir().into();
            config_dir.push("iot/settings.toml");
            Some(config_dir)
        }
        None => {
            return Err(anyhow!(
                "Warning: No se pudo resolver el directorio de configuración de usuario"
            ));
        }
    };
    Ok(resolve_settings(
        if user_settings_file.is_some()
            && user_settings_file
                .as_ref()
                .unwrap()
                .try_exists()
                .unwrap_or_else(|_| false)
        {
            user_settings_file.as_ref()
        } else {
            None
        },
        "iot",
    )
    .unwrap_or_default())
}
