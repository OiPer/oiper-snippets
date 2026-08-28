use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use crate::parse_config;
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
    let mut paths = fs::read_dir(fixture_root().join("cases"))
        .expect("failed to read fixture directory")
        .map(|entry| entry.expect("failed to read fixture entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();

    paths.sort();
    paths
}

#[test]
fn validates_the_fixture_schema_and_unique_case_ids() {
    let schema = read_json(&fixture_root().join("schema.json"));
    let validator = jsonschema::validator_for(&schema)
        .unwrap_or_else(|error| panic!("failed to compile fixture schema: {error}"));
    let fixture_paths = fixture_paths();
    let mut case_ids = HashSet::new();

    assert!(!fixture_paths.is_empty(), "no fixture files found");

    for fixture_path in fixture_paths {
        let fixture = read_json(&fixture_path);

        if let Err(error) = validator.validate(&fixture) {
            panic!(
                "{} does not match the schema: {error}",
                fixture_path.display()
            );
        }

        for test_case in fixture
            .as_array()
            .expect("validated fixture must be an array")
        {
            let case_id = test_case["id"]
                .as_str()
                .expect("validated case ID must be a string");

            assert!(
                case_ids.insert(case_id.to_owned()),
                "duplicate case ID: {case_id}"
            );
        }
    }
}

#[test]
fn parses_every_fixture_configuration() {
    let mut case_count = 0;

    for fixture_path in fixture_paths() {
        let fixture = read_json(&fixture_path);

        for test_case in fixture.as_array().expect("fixture must be an array") {
            case_count += 1;

            let case_id = test_case["id"].as_str().expect("case ID must be a string");
            let config = parse_config(&test_case["config"]);

            if test_case["expected"]["kind"] == "error" {
                assert!(
                    config.is_err(),
                    "{case_id}: expected a configuration error, got a valid config"
                );
            } else if let Err(error) = config {
                panic!("{case_id}: expected a valid config, got: {error}");
            }
        }
    }

    assert!(case_count > 0, "no fixture cases found");
}
