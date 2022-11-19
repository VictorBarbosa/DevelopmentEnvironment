use std::{fmt::format, fs, process, str::from_utf8};

use config::Config;

#[path = "config.rs"]
mod config;
#[path = "git_hub_managerment.rs"]
mod git_hub_managerment;

fn main() {
    let config = check_github_management_path_is_defined();
    println!("{:#?}", config);
    let git = git_hub_managerment::GitHubManagement::new();

    
}

#[warn(unreachable_code)]
fn check_github_management_path_is_defined() -> Config {
    let mut ret_path: String = String::new();
    if let Some(path) = option_env!("GITHUB_MANAGEMENT_PATH") {
        ret_path = String::from(path);
    } else {
        println!("You need define GITHUB_MANAGEMENT_PATH on variable PATH\nNote: You need to define a path where we can find the json with configuration");
        process::exit(1);
    }

    let file = fs::read(format!("{}/config.json", ret_path));
    match file {
        Ok(f) => {
            let json = from_utf8(&f).unwrap();
            let ret: Config = serde_json::from_str(json).unwrap();
            return ret;
        }
        Err(e) => {
            println!("Error {}\n", e);
            println!("File  config.json not found on {}", ret_path);
            process::exit(1);
        }
    }
}
