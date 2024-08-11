use clap::{arg, Command};

fn main() {
    let matches = Command::new("ansible lint")
        .version("1.0")
        .about("Ansible Lint is a command-line tool for linting playbooks, roles and collections")
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}