#! /usr/bin/env nix
#! nix shell nixpkgs#bash nixpkgs#jq nixpkgs#curl --command bash

cd "$(dirname "$0")" || exit
script_path=$(pwd)

function dump_to_json {
    curl -X 'GET' "https://hydra.alicehuston.xyz/$1" -H 'accept: application/json' | jq -S > "$script_path/$2.json"
}

dump_to_json "" "get_projects"
dump_to_json "/api/jobsets?project=nix-dotfiles-build" "get_jobsets_success"
dump_to_json "/api/jobsets?project=nix-dotfiles-buzzzzzild" "get_jobsets_not_found"
dump_to_json "/project/nix-dotfiles-build" "get_project_by_name_success"
dump_to_json "/project/nix-dotzzzfiles-build" "get_project_by_name_not_found"
dump_to_json "/jobset/nix-dotfiles-build/branch-main" "get_jobset_success"
dump_to_json "/jobset/nix-dotfiles-build/branch-lo-mein-zzzzz" "get_jobset_not_found"
dump_to_json "/jobset/nix-dotfiles-build/branch-main/evals" "get_jobset_evals_success"
dump_to_json "/jobset/nix-dotfiles-build/branch-lo-mein-zzzzz/evals" "get_jobset_evals_not_found"
dump_to_json "/eval/261599/builds" "get_build_by_eval_success"
dump_to_json "/eval/-1/builds" "get_build_by_eval_not_found"
dump_to_json "/build/6000" "get_build_success"
dump_to_json "/build/-1" "get_build_not_found"
