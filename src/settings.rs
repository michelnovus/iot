// [GPLv3 License] Copyright (C) 2025  Michel Novus
use std::path::PathBuf;

use anyhow::Result;
use config::Config;
use getset::Getters;
use isolang::Language;
use serde::Deserialize;

#[derive(Debug, Clone, Getters, Deserialize, PartialEq)]
pub struct Settings {
    // Preferencias de traducción.
    #[getset(get = "pub")]
    source: Language,
    #[getset(get = "pub")]
    target: Language,
    #[getset(get = "pub")]
    order: Vec<InputKind>,

    // Preferencias de la interfáz gráfica.
    #[getset(get = "pub")]
    width: u32,
    #[getset(get = "pub")]
    height: u32,
    #[getset(get = "pub")]
    centered: bool,
    #[getset(get = "pub")]
    theme: Theme,
    #[getset(get = "pub")]
    decorations: bool,
    #[getset(get = "pub")]
    textsize: u32,

    // Preferencias de la base de datos.
    #[getset(get = "pub")]
    database: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            source: Language::Eng,
            target: Language::Spa,
            order: [
                InputKind::Stdin,
                InputKind::SelectionPrimary,
                InputKind::SelectionClipboard,
            ]
            .into(),
            width: 720,
            height: 480,
            centered: true,
            theme: Theme::default(),
            decorations: true,
            textsize: 30,
            database: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum InputKind {
    #[serde(rename = "stdin")]
    Stdin,
    #[serde(rename = "primary")]
    SelectionPrimary,
    #[serde(rename = "clipboard")]
    SelectionClipboard,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub enum Theme {
    #[default]
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
}

/// Geneara el contenido del archivo de configuración derivado
/// de Settings::default() con comentarios descriptivos.
const DEFAULT_SETTINGS_TOML: &'static str = r#"### Instant Omnipresent Translator settings file.

### Preferencias de traducción.
## Idioma de origen, el que quieres traducir.
source = "en"

## Idioma destino, es tu idioma de preferencia o nativo.
target = "es"

## Especifica el orden y de dónde se obtiene el texto a traducir.
## Los valores son: stdin, primary, clip.
order = [
    "stdin",
    "primary",
    "clipboard",
]

### Preferencias de la Interfáz de Usuario.
## Define el ancho de la ventana en píxeles.
width = 720

## Define la altura de la ventana en píxeles.
height = 480

## Hace que la ventana aparezca centrada en la pantalla.
centered = true

## El tema de la interfáz, claro u oscuro (dark, light).
theme = "dark"

## Define si la ventana tiene bordes y título.
decorations = true

## El tamaño del texto.
textsize = 30

### Preferencias de la base de datos.
## La ruta absoluta a el archivo de la base de datos del programa.
## Comentar la variable hace que no se utilize una base de datos (no hay caché).
#database = ""
"#;

/// Carga la configuración de la aplicación en memoria desde las diferentes
/// fuentes disponibles; primero desde los datos por defecto, son sobrescritos por
/// el archivo de configuración si exisitese y finalmente por las variables de entorno.
pub fn resolve_settings(filepath: Option<&PathBuf>, env_prefix: &str) -> Result<Settings> {
    let mut builder = Config::builder();
    builder = builder.add_source(config::File::from_str(
        DEFAULT_SETTINGS_TOML,
        config::FileFormat::Toml,
    ));
    match filepath {
        Some(fp) => {
            builder =
                builder.add_source(config::File::with_name(fp.to_str().unwrap()).required(false));
        }
        None => (),
    };
    builder = builder.add_source(config::Environment::with_prefix(env_prefix));
    builder.build()?.try_deserialize().map_err(|err| err.into())
}
