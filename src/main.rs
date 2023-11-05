use std::env::args;

enum Action {
    ADD(Vec<String>),
    RM(Vec<String>),
    INIT(String),
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
        _ => Action::UNKNOWN
    }
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
        }
        Action::UNKNOWN => {}
    }
}