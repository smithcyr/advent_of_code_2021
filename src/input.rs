use std::fs;
use std::path;

pub fn load_resource(name: &str) -> String {
    fs::read_to_string(path::PathBuf::from(
        vec![env!("CARGO_MANIFEST_DIR"), "resources", name].join("/"),
    ))
    .unwrap()
}

pub fn load_lines(name: &str) -> Vec<String> {
    parse_lines(load_resource(name))
}

pub fn parse_lines(input: String) -> Vec<String> {
    input
        .split('\n')
        .map(|s| String::from(s.to_string().trim()))
        .collect()
}
