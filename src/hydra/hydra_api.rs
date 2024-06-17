use reqwest::{
    blocking::{get, Response},
    Result,
};

use super::hydra_builder::HydraInstance;

pub fn get_projects(hydra_instance: HydraInstance) -> Result<Response> {
    get(hydra_instance.hydra_url)
}

pub fn get_jobsets(hydra_instance: HydraInstance) -> Result<Response> {
    get(format!(
        "{}/api/jobsets?project={}",
        hydra_instance.hydra_url, hydra_instance.project
    ))
}

pub fn get_project_by_name(hydra_instance: HydraInstance) -> Result<Response> {
    get(format!(
        "{}/projects/{}",
        hydra_instance.hydra_url, hydra_instance.project
    ))
}

pub fn get_jobset(hydra_instance: HydraInstance) -> Result<Response> {
    get(format!(
        "{}/jobset/{}/{}",
        hydra_instance.hydra_url,
        hydra_instance.project,
        hydra_instance.jobset.unwrap()
    ))
}

pub fn get_jobset_evals(hydra_instance: HydraInstance) -> Result<Response> {
    get(format!(
        "{}/jobset/{}/{}/evals",
        hydra_instance.hydra_url,
        hydra_instance.project,
        hydra_instance.jobset.unwrap()
    ))
}

pub fn get_jobset_evals_paginated(hydra_instance: HydraInstance, page: u64) -> Result<Response> {
    get(format!(
        "{}/jobset/{}/{}/evals?page={}",
        hydra_instance.hydra_url,
        hydra_instance.project,
        hydra_instance.jobset.unwrap(),
        page
    ))
}

pub fn get_build(hydra_instance: HydraInstance) -> Result<Response> {
    get(format!(
        "{}/build/{}",
        hydra_instance.hydra_url,
        hydra_instance.build.unwrap()
    ))
}
