mod apply_snippets;
mod parse_config;

#[cfg(test)]
mod apply_snippets_test;
#[cfg(test)]
mod parse_config_test;

pub use apply_snippets::apply_snippets;
pub use parse_config::{Config, ConfigError, ConfigErrorKind, parse_config};
