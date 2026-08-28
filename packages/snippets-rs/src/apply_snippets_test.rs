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
fn applies_every_output_fixture() {
    let mut case_count = 0;

    for fixture_path in fixture_paths() {
        let fixture = read_json(&fixture_path);

        for test_case in fixture.as_array().expect("fixture must be an array") {
            let expected = &test_case["expected"];

            if expected["kind"] == "error" {
                continue;
            }

            case_count += 1;

            let case_id = test_case["id"].as_str().expect("case ID must be a string");
            let input = test_case["input"].as_str().expect("input must be a string");
            let config = parse_config(&test_case["config"])
                .unwrap_or_else(|error| panic!("{case_id}: expected a valid config, got: {error}"));
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

    assert!(case_count > 0, "no output fixture cases found");
}
