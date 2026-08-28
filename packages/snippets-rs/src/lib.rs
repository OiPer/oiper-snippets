mod apply_snippets;
mod parse_config;

pub use apply_snippets::apply_snippets;
pub use parse_config::{Config, ConfigError, ConfigErrorKind, parse_config};
