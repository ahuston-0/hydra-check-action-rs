use super::hydra_api::{get_build, get_build_by_eval, get_jobset_evals};
use super::hydra_api_schema::{HydraBuild, HydraEval, Result};
use super::hydra_builder::{HydraInstance, HydraInstanceBuilder};
use std::time::{Duration, Instant};

pub fn get_jobset_evals_paginated(
    hydra_instance: &HydraInstance,
    first: u64,
    last: Option<u64>,
) -> Vec<HydraEval> {
    let initial_request = get_jobset_evals(hydra_instance, None).unwrap();
    let initial_request_last = initial_request.last.split('=').last().unwrap().parse::<u64>().unwrap();

    let last_page = match last {
        None => initial_request_last,
        Some(val) => std::cmp::max(val, initial_request_last)
    };

    let mut evals = Vec::<HydraEval>::new();
    for page in first..last_page {
        evals.append(
            &mut get_jobset_evals(hydra_instance, Some(page))
            .unwrap()
            .evals,
        );
    }
    evals
}

pub fn poll_builds(
    hydra_instance_builder: &HydraInstanceBuilder,
    builds: Vec<u64>,
    timeout: u64,
) -> Result<Vec<HydraBuild>> {
    let mut rem_builds = builds.clone();
    let start_time = Instant::now();
    let timeout_duration = Duration::from_secs(timeout);

    while !rem_builds.is_empty() && (Instant::now() - start_time) < timeout_duration {
        for build in builds.clone().into_iter() {
            if rem_builds.contains(&build) {
                let res = get_build(&(hydra_instance_builder.clone().build_id(build).build()));

                if res.unwrap().finished == 1 {
                    rem_builds.remove(
                        rem_builds
                            .iter()
                            .position(|x| *x == build)
                            .expect("build not found in rem_builds"),
                    );
                }
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }

    get_build_by_eval(&(hydra_instance_builder.clone().build()))
}
