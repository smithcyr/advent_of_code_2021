use std::fs;
use std::path;

pub fn load_resource(name: &str) -> String {
    fs::read_to_string(path::PathBuf::from(
        vec![env!("CARGO_MANIFEST_DIR"), "resources", name].join("/"),
    ))
    .unwrap()
}

pub fn load_lines(name: &str) -> Vec<String> {
    load_resource(name)
        .split('\n')
        .map(|s| String::from(s.to_string().trim()))
        .collect()
}
