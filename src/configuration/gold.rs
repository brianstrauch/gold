#![allow(non_snake_case)]

use serde::Deserialize;

use super::golangci::GolangciConfiguration;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub enable: Option<Vec<String>>,
    pub settings: Option<Settings>,
    pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub G0001: Vec<String>,
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            enable: None,
            settings: Some(Settings {
                G0001: vec![String::from("standard"), String::from("default")],
            }),
            ignore: None,
        }
    }

    pub fn from(golangci_configuration: GolangciConfiguration) -> Self {
        Configuration {
            enable: None,
            settings: Some(Settings {
                G0001: golangci_configuration.linters_settings.gci.sections,
            }),
            ignore: Some(golangci_configuration.run.skip_dirs),
        }
    }

    pub fn is_enabled(&self, rule: String) -> bool {
        match &self.enable {
            None => true,
            Some(enable) => enable.contains(&rule),
        }
    }
}
