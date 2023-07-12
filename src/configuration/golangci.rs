#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GolangciConfiguration {
    pub run: Run,
    #[serde(rename(deserialize = "linters-settings"))]
    pub linters_settings: LintersSettings,
}

#[derive(Debug, Deserialize)]
pub struct Run {
    #[serde(rename(deserialize = "skip-dirs"))]
    pub skip_dirs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LintersSettings {
    pub gci: Gci,
}

#[derive(Debug, Deserialize)]
pub struct Gci {
    pub sections: Vec<String>,
}
