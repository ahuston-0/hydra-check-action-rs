use std::collections::HashMap;

use serde::Deserialize;

pub type Result<T> = std::result::Result<T, reqwest::Error>;

#[derive(Deserialize)]
pub struct HydraProject {
    pub displayname: String,
    pub description: String,
    pub enabled: bool,
    pub owner: String,
    pub jobsets: Vec<String>,
    pub hidden: bool,
    pub homepage: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct HydraJobset {
    pub name: String,
    pub project: String,
    pub nrtotal: u64,
    pub checkinterval: u64,
    pub haserrormsg: bool,
    pub nrscheduled: u64,
    pub nrfailed: u64,
    pub errortime: u64,
    pub fetcherrormsg: Option<String>,
    pub starttime: Option<u64>,
    pub lastcheckedtime: u64,
    pub triggertime: Option<u64>,
}

#[derive(Deserialize)]
pub struct HydraError {
    pub error: String,
}

#[derive(Deserialize)]
pub struct HydraJobsetInput {
    #[serde(alias = "type")]
    pub inputtype: String,
    pub name: String,
    pub emailresponsible: bool,
    pub jobsetinputalts: Vec<String>,
}

#[derive(Deserialize)]
pub struct HydraJobsetDetails {
    pub triggertime: Option<u64>,
    pub enableemail: bool,
    pub jobsetinputs: HashMap<String, HydraJobsetInput>,
    pub fetcherrormsg: String,
    pub hidden: bool,
    pub schedulingshares: u64,
    pub emailoverride: String,
    pub starttime: Option<u64>,
    pub description: String,
    pub errormsg: String,
    pub lastcheckedtime: Option<u64>,
    pub nixexprinput: String,
    pub checkinterval: u64,
    pub project: String,
    pub flake: String,
    #[serde(alias = "type")]
    pub jobsettype: u64,
    pub enabled: u64,
    pub name: String,
    pub keepnr: u64,
    pub nixexprpath: String,
    pub errortime: Option<u64>,
}
