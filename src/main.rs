extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use yaml_rust::YamlLoader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing command");
    }
    let config_file = "/Users/aravindhs/Aravindh/projects/dev/dev/dev.yml";
    let mut file = File::open(config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let docs = YamlLoader::load_from_str(&contents).unwrap();

    let doc = &docs[0];
    let alias = &doc["dev"][args[1].as_str()];
    let command = &alias["command"].as_str().unwrap();
    let arguments: &yaml_rust::Yaml = &alias["args"].clone().to_owned();
    let mut cmd = Command::new(command);
    for argument in arguments.as_vec().unwrap() {
        cmd.arg(argument.as_str().unwrap());
    }
    let output = cmd.output().expect("error");
    println!("{:?}", String::from_utf8_lossy(&output.stdout));
}
