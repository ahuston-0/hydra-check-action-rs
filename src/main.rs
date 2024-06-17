use hydra_check_action_rs::hydra::{hydra_builder::HydraInstanceBuilder,
                                   hydra_api};
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

    let hydra_instance = HydraInstanceBuilder::new()
        .hydra_url(String::from("https://hydra.alicehuston.xyz"))
        .project(String::from("nix-dotfiles-build"))
        .jobset(String::from("branch-main"))
        .build();

    let body = hydra_api::get_projects(hydra_instance).unwrap().text().unwrap();


    println!("{}",body);
    // error!("{:#?}",body);
    // let _ = write(github_output_path, format!("hi"));
}
