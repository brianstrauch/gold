#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GolangciConfiguration {
    pub run: Option<Run>,
    #[serde(rename(deserialize = "linters-settings"))]
    pub linters_settings: Option<LintersSettings>,
}

#[derive(Debug, Deserialize)]
pub struct Run {
    #[serde(rename(deserialize = "skip-dirs"))]
    pub skip_dirs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct LintersSettings {
    pub gci: Option<Gci>,
}

#[derive(Debug, Deserialize)]
pub struct Gci {
    pub sections: Option<Vec<String>>,
}
