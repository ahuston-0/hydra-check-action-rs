use reqwest::{
    blocking::{Client, Response},
    header::ACCEPT,
    StatusCode,
};

use super::hydra_api_schema::{HydraJobset, HydraProject, Result};
use super::hydra_builder::HydraInstance;

fn get_wrapper(url: &str) -> Result<Response> {
    Client::new()
        .get(url)
        .header(ACCEPT, "application/json")
        .send()
}

pub fn get_projects(hydra_instance: &HydraInstance) -> Result<Vec<HydraProject>> {
    get_wrapper(hydra_instance.hydra_url.as_str())
        .unwrap()
        .json::<Vec<HydraProject>>()
}

pub fn get_jobsets(hydra_instance: &HydraInstance) -> Result<Vec<HydraJobset>> {
    let resp = get_wrapper(
        format!(
            "{}/api/jobsets?project={}",
            hydra_instance.hydra_url, hydra_instance.project
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<Vec<HydraJobset>>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

pub fn get_project_by_name(hydra_instance: HydraInstance) -> Result<HydraJobset> {
    let resp = get_wrapper(
        format!(
            "{}/projects/{}",
            hydra_instance.hydra_url, hydra_instance.project
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<HydraJobset>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

pub fn get_jobset(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(
        format!(
            "{}/jobset/{}/{}",
            hydra_instance.hydra_url,
            hydra_instance.project,
            hydra_instance.jobset.unwrap()
        )
        .as_str(),
    )
}

pub fn get_jobset_evals(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(
        format!(
            "{}/jobset/{}/{}/evals",
            hydra_instance.hydra_url,
            hydra_instance.project,
            hydra_instance.jobset.unwrap()
        )
        .as_str(),
    )
}

pub fn get_jobset_evals_paginated(hydra_instance: HydraInstance, page: u64) -> Result<Response> {
    get_wrapper(
        format!(
            "{}/jobset/{}/{}/evals?page={}",
            hydra_instance.hydra_url,
            hydra_instance.project,
            hydra_instance.jobset.unwrap(),
            page
        )
        .as_str(),
    )
}

pub fn get_build(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(
        format!(
            "{}/build/{}",
            hydra_instance.hydra_url,
            hydra_instance.build.unwrap()
        )
        .as_str(),
    )
}

// TODO: https://github.com/seanmonstar/reqwest/issues/154#issuecomment-1552850065
