use reqwest::{
    blocking::{Client,  Response},
    header::ACCEPT,
    Result,
};

use super::hydra_builder::HydraInstance;


fn get_wrapper(url: &str) -> Result<Response> {
    Client::new().get(url).header(ACCEPT,"application/json").send()
}

pub fn get_projects(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(hydra_instance.hydra_url.as_str())
}

pub fn get_jobsets(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(format!(
        "{}/api/jobsets?project={}",
        hydra_instance.hydra_url, hydra_instance.project
    ).as_str())
}

pub fn get_project_by_name(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(format!(
        "{}/projects/{}",
        hydra_instance.hydra_url, hydra_instance.project
    ).as_str())
}

pub fn get_jobset(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(format!(
        "{}/jobset/{}/{}",
        hydra_instance.hydra_url,
        hydra_instance.project,
        hydra_instance.jobset.unwrap()
    ).as_str())
}

pub fn get_jobset_evals(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(format!(
        "{}/jobset/{}/{}/evals",
        hydra_instance.hydra_url,
        hydra_instance.project,
        hydra_instance.jobset.unwrap()
    ).as_str())
}

pub fn get_jobset_evals_paginated(hydra_instance: HydraInstance, page: u64) -> Result<Response> {
    get_wrapper(format!(
        "{}/jobset/{}/{}/evals?page={}",
        hydra_instance.hydra_url,
        hydra_instance.project,
        hydra_instance.jobset.unwrap(),
        page
    ).as_str())
}

pub fn get_build(hydra_instance: HydraInstance) -> Result<Response> {
    get_wrapper(format!(
        "{}/build/{}",
        hydra_instance.hydra_url,
        hydra_instance.build.unwrap()
    ).as_str())
}
