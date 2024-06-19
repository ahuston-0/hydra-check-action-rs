use hydra_check_action_rs::hydra::{hydra_api, hydra_builder::HydraInstanceBuilder, hydra_simple};
use std::env;
use std::fs::write;
use std::process::exit;

fn main() {
    let github_output_path =
        env::var("GITHUB_OUTPUT").unwrap_or("/var/log/hydra-check-action.log".to_string());

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let error = &args[1];

        if !error.is_empty() {
            eprintln!("Error: {error}");
            write(github_output_path, format!("error={error}")).unwrap();
            exit(1);
        }
    }

    let hydra_instance_builder = HydraInstanceBuilder::new()
        .hydra_url(String::from("https://hydra.alicehuston.xyz"))
        .project(String::from("nix-dotfiles-build"))
        .jobset(String::from("branch-main"));

    let hydra_instance = hydra_instance_builder.clone().build();

    let body = hydra_api::get_projects(&hydra_instance).unwrap();

    println!(
        "jobsets for project {}:  {:?}",
        body[0].displayname, body[0].jobsets
    );

    let body = hydra_api::get_jobsets(&hydra_instance).unwrap();

    println!("jobset {} for project {}", body[0].name, body[0].project);

    let evals = hydra_simple::get_jobset_evals_paginated(&hydra_instance, 0, None);

    println!(
        "{} evals for project {} jobset {}",
        evals.len(),
        hydra_instance.project,
        hydra_instance.jobset.as_ref().unwrap()
    );

    let max_eval = evals.into_iter().max_by_key(|x| x.id).unwrap();

    let builds =
        hydra_api::get_build_by_eval(&(hydra_instance_builder.clone().eval(max_eval.id).build()))
            .unwrap()
            .into_iter()
            .map(|build| build.id)
            .collect();

    let builds = hydra_simple::poll_builds(&hydra_instance_builder.eval(max_eval.id), builds, 600);

    print!("{:#?}", builds);
    // let _ = write(github_output_path, format!("hi"));
}
