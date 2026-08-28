use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use oiper_snippets::{apply_snippets, parse_config};
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
fn shared_fixtures_conform_to_schema_and_have_unique_case_ids() {
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
fn runs_every_case_against_parse_config_and_apply_snippets() {
    let mut case_count = 0;

    for fixture_path in fixture_paths() {
        let fixture = read_json(&fixture_path);

        for test_case in fixture.as_array().expect("fixture must be an array") {
            let case_id = test_case["id"].as_str().expect("case ID must be a string");
            let input = test_case["input"].as_str().expect("input must be a string");
            let expected = &test_case["expected"];

            case_count += 1;

            let config = parse_config(&test_case["config"]);

            if expected["kind"] == "error" {
                assert!(
                    config.is_err(),
                    "{case_id}: expected a configuration error, got a valid config"
                );

                continue;
            }

            let config = match config {
                Ok(config) => config,
                Err(error) => panic!("{case_id}: expected a valid config, got: {error}"),
            };
            let expected_output = expected["value"]
                .as_str()
                .expect("expected value must be a string");

            assert_eq!(
                apply_snippets(input, &config),
                expected_output,
                "{case_id}: unexpected output"
            );
        }
    }

    assert!(case_count > 0, "no fixture cases found");
}
