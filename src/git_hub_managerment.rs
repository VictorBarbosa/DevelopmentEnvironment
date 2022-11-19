use std::{
    process::{self, Command},
    str::from_utf8,
};

pub struct GitHubManagement {
    pub current_branch: String,
}

impl GitHubManagement {
    pub fn new() -> GitHubManagement {
        let current = Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output();
        let mut current_branch: String = "".to_string();
        match current {
            Ok(output) => {
                if output.stderr.len() > 0 {
                    let error_msg = from_utf8(&output.stderr).unwrap();
                    println!("Error,{:#?}", error_msg);
                    process::exit(1);
                } else {
                    current_branch = format!("{}", from_utf8(&output.stdout).unwrap());
                }
            }
            erro => {
                panic!("Error,{:#?}", erro);
            }
        }

        Self {
            current_branch: current_branch,
        }
    }
}
