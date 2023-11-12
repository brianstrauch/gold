#![allow(non_snake_case)]

pub mod golangci;

use serde::Deserialize;

use self::golangci::GolangciConfiguration;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub enable: Option<Vec<String>>,
    pub settings: Option<Settings>,
    pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub F002: Vec<String>,
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            enable: None,
            settings: Some(Settings {
                F002: vec![String::from("standard"), String::from("default")],
            }),
            ignore: None,
        }
    }

    pub fn from(golangci_configuration: GolangciConfiguration) -> Self {
        let mut configuration = Configuration::default();

        let mut settings = configuration.settings.unwrap();
        if let Some(linters_settings) = golangci_configuration.linters_settings {
            if let Some(gci) = linters_settings.gci {
                if let Some(sections) = gci.sections {
                    settings.F002 = sections;
                }
            }
        }
        configuration.settings = Some(settings);

        if let Some(run) = golangci_configuration.run {
            if let Some(skip_dirs) = run.skip_dirs {
                configuration.ignore = Some(skip_dirs);
            }
        }

        configuration
    }

    pub fn is_enabled(&self, rule: String) -> bool {
        match &self.enable {
            None => true,
            Some(enable) => enable.contains(&rule),
        }
    }
}
