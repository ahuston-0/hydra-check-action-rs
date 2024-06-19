use super::hydra_api::get_jobset_evals;
use super::hydra_api_schema::{HydraEval, HydraEvalPaginated};
use super::hydra_builder::HydraInstance;
use super::hydra_utils::get_wrapper;

pub fn get_jobset_evals_paginated(
    hydra_instance: &HydraInstance,
    first: u64,
    last: Option<u64>,
) -> Vec<HydraEval> {
    let initial = get_jobset_evals(&hydra_instance.clone()).unwrap();
    let initial_last = initial.last.split("=").last().unwrap();

    let last_page = last
        .or_else(|| Some(initial_last.parse::<u64>().unwrap()))
        .unwrap();

    let mut evals = Vec::<HydraEval>::new();
    for page in first..last_page {
        evals.append(
            &mut get_wrapper(
                format!(
                    "{}/jobset/{}/{}/evals?page={}",
                    hydra_instance.hydra_url,
                    hydra_instance.project,
                    hydra_instance.jobset.as_ref().unwrap(),
                    page
                )
                .as_str(),
            )
            .unwrap()
            .json::<HydraEvalPaginated>()
            .unwrap()
            .evals,
        );
    }
    evals
}
