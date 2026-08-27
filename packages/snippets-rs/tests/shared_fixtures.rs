use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

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

#[test]
fn shared_fixtures_conform_to_schema_and_have_unique_case_ids() {
    let fixture_root = fixture_root();
    let schema = read_json(&fixture_root.join("schema.json"));
    let validator = jsonschema::validator_for(&schema)
        .unwrap_or_else(|error| panic!("failed to compile fixture schema: {error}"));
    let mut fixture_paths = fs::read_dir(fixture_root.join("cases"))
        .expect("failed to read fixture directory")
        .map(|entry| entry.expect("failed to read fixture entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();
    let mut case_ids = HashSet::new();

    fixture_paths.sort();
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
#[ignore = "requires parse_config and apply_snippets"]
fn runs_every_case_against_parse_config_and_apply_snippets() {}
