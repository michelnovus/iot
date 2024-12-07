// [GPLv3 License] Copyright (C) 2025  Michel Novus
use std::{
    io::{self, Write},
    sync::mpsc,
    time::Duration,
};

use anyhow::{bail, Result};

use super::UI;
use crate::{model::Message, settings::Settings};

pub struct UserInterface {
    rx: mpsc::Receiver<Message>,
}

impl UI for UserInterface {
    fn new(settings: Settings, rx: mpsc::Receiver<Message>) -> Self {
        let _ = settings;
        UserInterface { rx }
    }
    fn mainloop(self) -> Result<()> {
        loop {
            if let Ok(message) = self.rx.recv_timeout(Duration::from_secs(20)) {
                match message {
                    Message::Translation(tr) => {
                        println!("==============================");
                        tr.out().iter().for_each(|p| println!("{p}"));
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.adjetive() {
                                print!("Adjetivo: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.adverb() {
                                print!("# Adverbio: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.noun() {
                                print!("# Sustantivo: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.verb() {
                                print!("# Verbo: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.conjuntion() {
                                print!("# Conjunción: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.pronoun() {
                                print!("# Pronombre: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                        if let Some(d) = tr.dict() {
                            if let Some(list) = d.preposition() {
                                print!("# Preposición: ");
                                let count = list.iter().count();
                                list.iter().enumerate().for_each(|(i, w)| {
                                    if count != i + 1 {
                                        print!("{w}, ")
                                    } else {
                                        print!("{w}.")
                                    }
                                });
                                println!();
                                io::stdout().flush()?;
                            }
                        }
                    }
                    Message::Quit => break,
                    Message::InputKind(ik) => println!("Tipo entrada: {:?}", ik),
                    Message::Error { header, source } => {
                        println!("{}", header);
                        println!("{:#?}", source);
                    }
                    _ => (),
                }
            } else {
                bail!("timeout error")
            }
        }
        Ok(())
    }
}
