use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Source {
    pub uri: String,
    pub checksum: String,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub homepage: Option<String>,
    pub source: Vec<Source>,
}

