use std::{ascii::AsciiExt, env, process::Command, str::from_utf8};

pub struct GitHubManagement<'a> {
    current_branch: &'a str,
}

impl<'a> GitHubManagement<'a> {
    pub fn new() -> GitHubManagement<'a> {
        let current = Command::new("sh").arg("git branch --show-current").output();

        match current {
            Ok(mut output) => {
                if (output.stderr.len() > 0) {
                    let error_msg = from_utf8(&output.stderr).unwrap();
                    println!("Error,{:#?}", error_msg);
                } else {
                    println!("{:#?}", output)
                }
            }
            erro => {
                panic!("Error,{:#?}", erro);
            }
        }

        Self { current_branch: "" }
    }
}
