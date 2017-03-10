/// Module to handle all of the build process
use std::io::Read;
use std::fs::File;
use std::path::Path;

use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::yaml;

#[derive(Debug)]
enum BuildConfigError {
    UnexpectedFormat,
}

#[derive(Debug)]
struct BuildConfig {
    before_build: Option<Vec<String>>,
    build: Vec<String>,
    after_build: Option<Vec<String>>,
}

fn parse_build_file<P: AsRef<Path>>(file_name: P) -> Result<BuildConfig, BuildConfigError> {
    let mut content = String::new();
    // Will assume that the existence of the file is checked prior
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut content).unwrap();

    let yml = YamlLoader::load_from_str(&content).unwrap();

    // TODO Get some better error handling
    let yml = yml[0].as_hash().unwrap();
    let before_build = yaml_hash_parse(String::from("build_before"), yml).unwrap();
    let build = yaml_hash_parse(String::from("build"), yml)
        .expect("build script not correctly parsed")
        .expect("No build script provided");
    let after_build = yaml_hash_parse(String::from("build_after"), yml).unwrap();

    Ok(BuildConfig {
        before_build: before_build,
        build: build,
        after_build: after_build,
    })

}

fn yaml_hash_parse(key: String, yml: &yaml::Hash) -> Result<Option<Vec<String>>, BuildConfigError> {
    // TODO Get the unwrap out of here
    Ok(yml.get(&Yaml::String(key))
        .map(|x| yaml_array_to_string_vec(x.as_vec()).unwrap()))

}

fn yaml_array_to_string_vec(yml: Option<&Vec<Yaml>>) -> Result<Vec<String>, BuildConfigError> {
    if yml.is_none() {
        return Err(BuildConfigError::UnexpectedFormat);
    }
    let mut it = yml.unwrap().iter().map(|elem| elem.as_str());
    if it.all(|x| x.is_some()) {
        Ok(it.map(|x| String::from(x.unwrap())).collect())
    } else {
        Err(BuildConfigError::UnexpectedFormat)
    }
}

#[test]
fn test_parse_build_file() {
    parse_build_file("tests/resources/test_config.yml");
}