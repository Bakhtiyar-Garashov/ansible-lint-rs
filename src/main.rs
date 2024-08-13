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

    // parse yaml in json and build ast
    fn parse_yaml_file(file_path: &str) -> Result<serde_yaml::Value, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(file_path)?;
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&file_content)?;
        Ok(yaml_value)
    }

    fn build_ast(yaml_value: serde_yaml::Value) {
        println!("{:?}", yaml_value);
    }

    let yaml_file_path = "test.yaml";
    match parse_yaml_file(yaml_file_path) {
        Ok(yaml_value) => build_ast(yaml_value),
        Err(err) => eprintln!("Error: {}", err),
    }
}
