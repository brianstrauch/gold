#![allow(non_snake_case)]

use serde::Deserialize;
use serde_yaml::Value;

use super::golangci::GolangciConfiguration;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub G0000: Option<Value>,
    pub G0001: Option<Vec<String>>,
    pub SA1000: Option<Value>,
}

impl Configuration {
    pub fn default() -> Self {
        Self {
            G0000: Some(Value::Null),
            G0001: Some(vec![String::from("standard"), String::from("default")]),
            SA1000: Some(Value::Null),
        }
    }

    pub fn from(golangci_configuration: GolangciConfiguration) -> Self {
        let mut configuration = Self::default();

        configuration.G0001 = Some(golangci_configuration.linters_settings.gci.sections);

        configuration
    }
}
