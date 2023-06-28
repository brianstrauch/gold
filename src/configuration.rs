#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Configuration {
    pub G0000: Option<Empty>,
    pub G0001: Option<Vec<String>>,
    pub SA1000: Option<Empty>,
}

#[derive(Deserialize)]
pub struct Empty {}

impl Configuration {
    pub fn new() -> Self {
        Self {
            G0000: Some(Empty {}),
            G0001: Some(vec![String::from("standard"), String::from("default")]),
            SA1000: Some(Empty {}),
        }
    }
}
