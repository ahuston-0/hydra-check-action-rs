// use hydra_check_action_rs::{init_logs, prelude::*};
use std::env;
use std::fs::write;
use std::process::exit;
use futures::executor::block_on;
use reqwest::{Error,Response};

async fn async_main(args: Vec<String>) -> String {                                               >{
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    return body;
}

fn main() {
    let github_output_path =
        env::var("GITHUB_OUTPUT").unwrap_or("/var/logs/hydra-check-action.log".to_string());

    let args: Vec<String> = env::args().collect();
    let error = &args[1];

    if !error.is_empty() {
        eprintln!("Error: {error}");
        write(github_output_path, format!("error={error}")).unwrap();
        exit(1);
    }

    write(github_output_path,format!(async_main(args).unwrap()));
}
