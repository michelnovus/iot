// [GPLv3 License] Copyright (C) 2025  Michel Novus
use std::io::{self, prelude::*, IsTerminal};

use anyhow::{anyhow, Result};
use arboard::{Clipboard, Get, GetExtLinux, LinuxClipboardKind};
use clap::Parser;
use getset::Getters;

use crate::settings::InputKind;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args;

#[derive(Getters)]
pub struct Input {
    #[getset(get = "pub")]
    kind: InputKind,
    #[getset(get = "pub")]
    text: String,
}

pub fn collect(query_order: &[InputKind]) -> Result<Input> {
    for input_kind in query_order {
        match input_kind {
            &InputKind::Stdin => {
                let mut buf = String::new();
                let mut stdin = io::stdin();
                if !stdin.is_terminal() {
                    stdin.read_to_string(&mut buf)?;
                    return Ok(Input {
                        kind: InputKind::Stdin,
                        text: buf,
                    });
                } else {
                    continue;
                }
            }
            &InputKind::SelectionPrimary => {
                let mut clip = match Clipboard::new() {
                    Ok(clip) => clip,
                    Err(err) => return Err(anyhow!(err)),
                };
                let text = match Get::clipboard(clip.get(), LinuxClipboardKind::Primary).text() {
                    Ok(text) => text,
                    Err(err) => return Err(anyhow!(err)),
                };
                if text.is_empty() {
                    continue;
                } else {
                    return Ok(Input {
                        kind: InputKind::SelectionPrimary,
                        text,
                    });
                }
            }
            &InputKind::SelectionClipboard => {
                let mut clip = match Clipboard::new() {
                    Ok(clip) => clip,
                    Err(err) => return Err(anyhow!(err)),
                };
                let text = match Get::clipboard(clip.get(), LinuxClipboardKind::Clipboard).text() {
                    Ok(text) => text,
                    Err(err) => return Err(anyhow!(err)),
                };
                if text.is_empty() {
                    continue;
                } else {
                    return Ok(Input {
                        kind: InputKind::SelectionClipboard,
                        text,
                    });
                }
            }
        }
    }
    Err(anyhow!("empty buffers"))
}
