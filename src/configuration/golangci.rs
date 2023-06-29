#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GolangciConfiguration {
    #[serde(rename(deserialize = "linters-settings"))]
    pub linters_settings: LintersSettings,
}

#[derive(Debug, Deserialize)]
pub struct LintersSettings {
    pub gci: Gci,
}

#[derive(Debug, Deserialize)]
pub struct Gci {
    pub sections: Vec<String>,
}
