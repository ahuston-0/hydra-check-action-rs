use std::collections::HashMap;

use serde::Deserialize;

pub type Result<T> = std::result::Result<T, reqwest::Error>;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct HydraJobsetOverview {
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

#[derive(Debug, Deserialize)]
pub struct HydraError {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct HydraJobsetInput {
    #[serde(alias = "type")]
    pub inputtype: String,
    pub name: String,
    pub emailresponsible: bool,
    pub jobsetinputalts: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HydraJobset {
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

#[derive(Debug, Deserialize)]
pub struct HydraEvalInput {
    #[serde(alias = "type")]
    pub inputtype: String,
    pub uri: String,
    pub revision: String,

    // cannot find any record of a type for these two fields... need to investigate
    pub dependency: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HydraEval {
    pub builds: Vec<u64>,
    pub jobsetevalinputs: HashMap<String, HydraEvalInput>,
    pub id: u64,
    pub hasnewbuilds: u8,
}

#[derive(Debug, Deserialize)]
pub struct HydraEvalPaginated {
    pub first: String,
    pub last: String,
    pub next: Option<String>,
    pub evals: Vec<HydraEval>,
}

#[derive(Debug, Deserialize)]
pub struct HydraBuildProduct {
    pub filesize: Option<u64>,
    pub defaultpath: String,
    pub name: String,
    #[serde(alias = "type")]
    pub buildtype: String,
    pub sha256hash: Option<String>,
    pub subtype: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct HydraBuildOutput {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct HydraBuildMetrics {
    pub name: String,
    pub value: String,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct HydraBuild {
    pub buildproducts: HashMap<String, HydraBuildProduct>,
    pub id: u64,
    pub buildstatus: u64,
    pub nixname: String,
    pub finished: u8,
    pub jobsetevals: Vec<u64>,
    pub stoptime: Option<u64>,
    pub system: String,
    pub drvpath: String,
    pub buildoutputs: HashMap<String, HydraBuildOutput>,
    pub job: String,
    pub jobset: String,
    pub buildmetrics: HashMap<String, HydraBuildMetrics>,
    pub priority: u64,
    pub project: String,
    pub starttime: u64,
    pub timestamp: u64,
}
