use serde::Deserialize;

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
