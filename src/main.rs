use std::env::args;
use std::fs::{DirBuilder, File};
use std::env::var_os;

use auth::auth;

use crate::config::create_default_config;
mod auth;
mod config;

enum Action {
    ADD(Vec<String>),
    RM(Vec<String>),
    INIT(String),
    PUBLISH,
    AUTH(String),
    UNKNOWN
}

fn parse_action(action: &str, args: &Vec<String>) -> Action {
    match action {
        "add" => {
            let mut packages: Vec<String> = Vec::new();

            for (i, package) in args.iter().enumerate() {
                if i == 1 || i == 0 { continue };

                packages.push(package.to_owned());
            }

            Action::ADD(packages)
        },
        "rm" => {
            let mut packages: Vec<String> = Vec::new();

            for (i, package) in args.iter().enumerate() {
                if i == 1 || i == 0 { continue };

                packages.push(package.to_owned());
            }

            Action::RM(packages)
        },
        "init" => {
            let project_name = args.get(2).expect("Expected a project name.");

            Action::INIT(project_name.to_owned())
        },
        "publish" => {
            Action::PUBLISH
        },
        "auth" => {
            let auth_token = args.get(1).expect("Expected a token.");
            Action::AUTH(auth_token.to_owned())
        },
        _ => Action::UNKNOWN
    }
}

// I swear to god I'm gonna kill whoever created OsStrings.
fn from_os_var(var: &str, path: &str) -> String {
    let os_var = var_os(var).unwrap();
    let os_var_str = os_var.to_str().unwrap();
    let stringified_osvar = String::from(os_var_str);
    let mut final_str = String::from(stringified_osvar);

    final_str.push_str(path);
    final_str
}

fn main() {
    let args: Vec<String> = args().collect();
    let action: &String = args.get(1).expect("Expected an action.");

    match parse_action(action.as_str(), &args) {
        Action::ADD(packages) => {
            println!("Action: add");

            for package in packages {
                println!("Adding: {}", package);
            }
        },
        Action::RM(packages) => {
            println!("Action: remove");

            for package in packages {
                println!("Removing: {}", package);
            }
        },
        Action::INIT(project_name) => {
            println!("Initializing {}", project_name);
            let mut packages_path = project_name.clone();
            packages_path.push_str("\\lusr_packages");
            let mut config_path = project_name.clone();
            config_path.push_str("\\config.json");
            let prj_name = project_name.clone();

            DirBuilder::new().create(project_name).unwrap();
            DirBuilder::new().create(packages_path).unwrap();
            create_default_config(config_path.as_str(), prj_name.as_str());
        },
        Action::PUBLISH => {
            println!("Publishing package.")
        },
        Action::AUTH(auth_token) => {
            match DirBuilder::new().create(from_os_var("USERPROFILE", "\\.lpm")) {
                Ok(_) => {},
                Err(_) => {},
            }

            match File::create(from_os_var("USERPROFILE", "\\.lpm\\config.json")) {
                Ok(_) => {},
                Err(_) => {}
            };

            auth(from_os_var("USERPROFILE", "\\.lpm\\config.json").as_str(), auth_token);
        },
        Action::UNKNOWN => {}
    }
}