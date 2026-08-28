mod apply;
mod config;

pub use apply::apply_snippets;
pub use config::{Config, ConfigError, ConfigErrorKind, parse_config};
