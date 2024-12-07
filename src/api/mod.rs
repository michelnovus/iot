// [GPLv3 License] Copyright (C) 2025  Michel Novus
pub mod translator;

use anyhow::Result;
use getset::Getters;
use isolang::Language;

#[derive(Debug, Getters)]
pub struct Translation {
    #[getset(get = "pub")]
    src: Vec<String>,
    #[getset(get = "pub")]
    out: Vec<String>,
    #[getset(get = "pub")]
    dict: Option<Dict>,
}

#[derive(Debug, Getters)]
pub struct Dict {
    #[getset(get = "pub")]
    adjetive: Option<Vec<String>>,
    #[getset(get = "pub")]
    adverb: Option<Vec<String>>,
    #[getset(get = "pub")]
    noun: Option<Vec<String>>,
    #[getset(get = "pub")]
    verb: Option<Vec<String>>,
    #[getset(get = "pub")]
    conjuntion: Option<Vec<String>>,
    #[getset(get = "pub")]
    pronoun: Option<Vec<String>>,
    #[getset(get = "pub")]
    preposition: Option<Vec<String>>,
}

/// Define el lenguaje de origen y de destino para la api de traducción.
#[derive(Debug, Clone, Copy)]
pub struct Lang {
    pub from: Language,
    pub to: Language,
}

pub trait Translator {
    fn translate(&self, source: &str, language: Lang, timeout: Option<u64>) -> Result<Translation>;
}
