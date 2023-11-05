use std::env::args;

enum Action {
    ADD(Vec<String>),
    RM(Vec<String>),
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
        Action::UNKNOWN => {}
    }
}