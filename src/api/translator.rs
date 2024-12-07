// [GPLv3 License] Copyright (C) 2025  Michel Novus
use anyhow::{anyhow, Result};
use serde_json::Value as JsonEntity;

use super::{Dict, Lang, Translation, Translator};

/// API de traducción libre de Google.
pub struct GoogleTranslate;
impl GoogleTranslate {
    fn request(text: &str, from: &str, to: &str, timeout: Option<u64>) -> Result<JsonEntity> {
        let mut request = minreq::get(
            "https://translate.googleapis.com/translate_a/single?client=gtx&dt=t&dt=bd&dj=1",
        )
        .with_param("sl", from)
        .with_param("tl", to)
        .with_param("q", text);
        if let Some(t) = timeout {
            request = request.with_timeout(t);
        }
        let response = request.send()?;
        if response.status_code == 200 && &response.reason_phrase == "OK" {
            Ok(response.json()?)
        } else {
            Err(anyhow!("bad response"))
        }
    }
    fn parse(json: JsonEntity) -> Result<Translation> {
        let root = json.as_object().ok_or_else(|| anyhow!("api error"))?;
        let sentences = root
            .get("sentences")
            .ok_or_else(|| anyhow!("api error"))?
            .as_array()
            .ok_or_else(|| anyhow!("api error"))?;
        let dict = root.get("dict");
        if dict.is_none() {
            let mut source_pars: Vec<String> = vec![];
            let mut trans_pars: Vec<String> = vec![];
            sentences.iter().for_each(|entry| {
                source_pars.push(
                    entry
                        .get("orig")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .trim()
                        .to_owned(),
                );
                trans_pars.push(
                    entry
                        .get("trans")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .trim()
                        .to_owned(),
                );
            });
            Ok(Translation {
                src: source_pars,
                out: trans_pars,
                dict: None,
            })
        } else {
            let mut adjetive: Vec<String> = vec![];
            let mut adverb: Vec<String> = vec![];
            let mut noun: Vec<String> = vec![];
            let mut verb: Vec<String> = vec![];
            let mut conjuntion: Vec<String> = vec![];
            let mut pronoun: Vec<String> = vec![];
            let mut preposition: Vec<String> = vec![];
            dict.unwrap().as_array().unwrap().iter().for_each(|entry| {
                match entry
                    .as_object()
                    .unwrap()
                    .get("pos")
                    .unwrap()
                    .as_str()
                    .unwrap()
                {
                    "adjetive" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| adjetive.push(value.as_str().unwrap().to_owned())),

                    "adverb" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| adverb.push(value.as_str().unwrap().to_owned())),
                    "noun" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| noun.push(value.as_str().unwrap().to_owned())),
                    "verb" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| verb.push(value.as_str().unwrap().to_owned())),
                    "conjuntion" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| conjuntion.push(value.as_str().unwrap().to_owned())),
                    "pronoun" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| pronoun.push(value.as_str().unwrap().to_owned())),
                    "preposition" => entry
                        .as_object()
                        .unwrap()
                        .get("terms")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|value| preposition.push(value.as_str().unwrap().to_owned())),
                    _ => (),
                }
            });
            Ok(Translation {
                src: vec![sentences
                    .get(0)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("orig")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .trim()
                    .to_owned()],
                out: vec![sentences
                    .get(0)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("trans")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .trim()
                    .to_owned()],
                dict: Some(Dict {
                    adjetive: if adjetive.is_empty() {
                        None
                    } else {
                        Some(adjetive)
                    },
                    adverb: if adverb.is_empty() {
                        None
                    } else {
                        Some(adverb)
                    },
                    noun: if noun.is_empty() { None } else { Some(noun) },
                    verb: if verb.is_empty() { None } else { Some(verb) },
                    conjuntion: if conjuntion.is_empty() {
                        None
                    } else {
                        Some(conjuntion)
                    },
                    pronoun: if pronoun.is_empty() {
                        None
                    } else {
                        Some(pronoun)
                    },
                    preposition: if preposition.is_empty() {
                        None
                    } else {
                        Some(preposition)
                    },
                }),
            })
        }
    }
}

/// Separa el texto en párrafos válidos.
fn filter_newline(text: &str) -> String {
    text.replace('\n', " ").into()
}

impl Translator for GoogleTranslate {
    fn translate(&self, source: &str, language: Lang, timeout: Option<u64>) -> Result<Translation> {
        let api = GoogleTranslate::request(
            &filter_newline(source),
            language.from.to_639_1().unwrap(),
            language.to.to_639_1().unwrap(),
            timeout,
        )?;
        GoogleTranslate::parse(api)
    }
}

/// Consulta la base de datos local de traducciones previas.
pub struct Cache;

impl Translator for Cache {
    #[allow(unused)]
    fn translate(&self, source: &str, language: Lang, timeout: Option<u64>) -> Result<Translation> {
        unimplemented!()
    }
}
