use std::{
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
    let mut paths = fs::read_dir(fixture_root().join("config"))
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
fn rejects_every_invalid_configuration_fixture() {
    let mut case_count = 0;

    for fixture_path in fixture_paths() {
        let fixture = read_json(&fixture_path);

        for (description, config) in fixture.as_object().expect("fixture must be an object") {
            case_count += 1;

            assert!(
                parse_config(config).is_err(),
                "{description}: expected a configuration error, got a valid config"
            );
        }
    }

    assert!(case_count > 0, "no fixture cases found");
}
