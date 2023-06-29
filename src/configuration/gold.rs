#![allow(non_snake_case)]

use serde::Deserialize;

use super::golangci::GolangciConfiguration;

#[derive(Debug, Default, Deserialize)]
pub struct Configuration {
    pub G0000: Option<Empty>,
    pub G0001: Option<Vec<String>>,
    pub SA1000: Option<Empty>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            G0000: Some(Empty {}),
            G0001: Some(vec![String::from("standard"), String::from("default")]),
            SA1000: Some(Empty {}),
        }
    }

    pub fn from(golangci_configuration: GolangciConfiguration) -> Self {
        let mut configuration = Self::new();

        configuration.G0001 = Some(golangci_configuration.linters_settings.gci.sections);

        configuration
    }
}

#[derive(Debug, Deserialize)]
pub struct Empty {}
