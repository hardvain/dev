extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing Mandatory file");
    }
    let config_file = &args[1];
    let mut file = File::open(config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let yaml = YamlLoader::load_from_str(&contents).unwrap();
    println!("{:?}", yaml);
}
