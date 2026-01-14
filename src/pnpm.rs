use serde::Deserialize;
use std::{
    collections::BTreeMap,
    error::Error,
    io::{self, Read},
};

#[derive(Deserialize)]
pub struct PnpmOutdatedPackage {
    pub current: String,
    pub latest: String,
}

pub type PnpmOutdatedOutput = BTreeMap<String, PnpmOutdatedPackage>;

pub fn parse_input() -> Result<PnpmOutdatedOutput, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(serde_json::from_str(&input)?)
}
