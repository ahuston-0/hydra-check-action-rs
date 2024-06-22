use reqwest::StatusCode;

use super::hydra_api_schema::{
    HydraBuild, HydraEvalPaginated, HydraJobset, HydraJobsetOverview, HydraProject, Result,
};
use super::hydra_builder::HydraInstance;
use super::hydra_utils::get_wrapper;

pub fn get_projects(hydra_instance: &HydraInstance) -> Result<Vec<HydraProject>> {
    get_wrapper(hydra_instance.hydra_url.as_str())
        .unwrap()
        .json::<Vec<HydraProject>>()
}

pub fn get_jobsets(hydra_instance: &HydraInstance) -> Result<Vec<HydraJobsetOverview>> {
    let resp = get_wrapper(
        format!(
            "{}/api/jobsets?project={}",
            hydra_instance.hydra_url, hydra_instance.project
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<Vec<HydraJobsetOverview>>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

pub fn get_project_by_name(hydra_instance: &HydraInstance) -> Result<HydraProject> {
    let resp = get_wrapper(
        format!(
            "{}/project/{}",
            hydra_instance.hydra_url, hydra_instance.project
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<HydraProject>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

pub fn get_jobset(hydra_instance: &HydraInstance) -> Result<HydraJobset> {
    let resp = get_wrapper(
        format!(
            "{}/jobset/{}/{}",
            hydra_instance.hydra_url,
            hydra_instance.project,
            hydra_instance.jobset.as_ref().unwrap()
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

pub fn get_jobset_evals(hydra_instance: &HydraInstance, page:Option<u64>) -> Result<HydraEvalPaginated> {
    let requested_page = match page {
        None => 1,
        Some(value) => value
    };

    let resp = get_wrapper(
        format!(
            "{}/jobset/{}/{}/evals?page={}",
            hydra_instance.hydra_url,
            hydra_instance.project,
            hydra_instance.jobset.as_ref().unwrap(),
            requested_page
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<HydraEvalPaginated>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

pub fn get_build_by_eval(hydra_instance: &HydraInstance) -> Result<Vec<HydraBuild>> {
    let resp = get_wrapper(
        format!(
            "{}/eval/{}/builds",
            hydra_instance.hydra_url,
            hydra_instance.eval.unwrap()
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<Vec<HydraBuild>>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

pub fn get_build(hydra_instance: &HydraInstance) -> Result<HydraBuild> {
    let resp = get_wrapper(
        format!(
            "{}/build/{}",
            hydra_instance.hydra_url,
            hydra_instance.build.unwrap()
        )
        .as_str(),
    )
    .unwrap();

    match resp.status() {
        StatusCode::OK => Ok(resp.json::<HydraBuild>().unwrap()),
        StatusCode::NOT_FOUND => Err(resp.error_for_status().err().unwrap()),
        _ => panic!("Status code not expected"),
    }
}

// TODO: https://github.com/seanmonstar/reqwest/issues/154#issuecomment-1552850065
#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use isahc::{prelude::*, Request};
    use serde_json::{json, Value};

    #[test]
    fn test_get_projects() {
        // Arrange
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/users")
                .header("content-type", "application/json");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "Hans" }));
        });

        // Act: Send the request and deserialize the response to JSON
        let mut response = Request::post(&format!("http://{}/users", server.address()))
            .header("content-type", "application/json")
            .body(json!({ "name": "Fred" }).to_string())
            .unwrap()
            .send()
            .unwrap();

        let user: Value =
            serde_json::from_str(&response.text().unwrap()).expect("cannot deserialize JSON");

        // Assert
        m.assert();
        assert_eq!(response.status(), 201);
        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "Hans");
    }
}
