use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{apply_snippets, parse_config};
use serde_json::Value;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn read_json(path: &Path) -> Value {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    serde_json::from_str(&contents)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn fixture_paths() -> Vec<PathBuf> {
    let mut paths = fs::read_dir(fixture_root().join("snippets"))
        .expect("failed to read fixture directory")
        .map(|entry| entry.expect("failed to read fixture entry").path())
        .filter(|path| {
            path.file_name().is_some_and(|name| name != "schema.json")
                && path
                    .extension()
                    .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();

    paths.sort();
    paths
}

#[test]
fn validates_the_fixtures() {
    let schema = read_json(&fixture_root().join("snippets/schema.json"));
    let validator = jsonschema::validator_for(&schema)
        .unwrap_or_else(|error| panic!("failed to compile fixture schema: {error}"));
    let fixture_paths = fixture_paths();

    assert!(!fixture_paths.is_empty(), "no fixture files found");

    for fixture_path in fixture_paths {
        let fixture = read_json(&fixture_path);

        if let Err(error) = validator.validate(&fixture) {
            panic!(
                "{} does not match the schema: {error}",
                fixture_path.display()
            );
        }
    }
}

#[test]
fn applies_every_output_fixture() {
    let mut case_count = 0;

    for fixture_path in fixture_paths() {
        let fixture = read_json(&fixture_path);

        for (description, test_case) in fixture.as_object().expect("fixture must be an object") {
            case_count += 1;

            let input = test_case["input"].as_str().expect("input must be a string");
            let config = parse_config(&test_case["config"]).unwrap_or_else(|error| {
                panic!("{description}: expected a valid config, got: {error}")
            });
            let output = test_case["output"]
                .as_str()
                .expect("output must be a string");

            assert_eq!(
                apply_snippets(input, &config),
                output,
                "{description}: unexpected output"
            );
        }
    }

    assert!(case_count > 0, "no output fixture cases found");
}
