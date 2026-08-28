use std::{collections::HashSet, error::Error, fmt};

use regress::Regex;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigErrorKind {
    NotAnArray,
    SnippetNotAnObject,
    BodyNotAString,
    BodyEmpty,
    WhenNotAnArray,
    WhenEmpty,
    MatcherNotAnObject,
    MatcherDefinesBothForms,
    MatcherDefinesNoForm,
    ValueNotAString,
    ValueEmpty,
    FlagsOnLiteralMatcher,
    DuplicateLiteral(String),
    RegexNotAString,
    RegexEmpty,
    FlagsNotAString,
    UnsupportedFlag(char),
    DuplicateFlag(char),
    DuplicateRegex(String),
    InvalidRegex(String),
}

impl fmt::Display for ConfigErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnArray => write!(formatter, "configuration must be an array"),
            Self::SnippetNotAnObject => write!(formatter, "snippet must be an object"),
            Self::BodyNotAString => write!(formatter, "'body' must be a string"),
            Self::BodyEmpty => write!(formatter, "'body' must not be empty"),
            Self::WhenNotAnArray => write!(formatter, "'when' must be an array"),
            Self::WhenEmpty => write!(formatter, "'when' must not be empty"),
            Self::MatcherNotAnObject => write!(formatter, "matcher must be an object"),
            Self::MatcherDefinesBothForms => {
                write!(formatter, "matcher must not define both 'value' and 'regex'")
            }
            Self::MatcherDefinesNoForm => {
                write!(formatter, "matcher must define either 'value' or 'regex'")
            }
            Self::ValueNotAString => write!(formatter, "'value' must be a string"),
            Self::ValueEmpty => write!(formatter, "'value' must not be empty after trimming"),
            Self::FlagsOnLiteralMatcher => {
                write!(formatter, "'flags' is not allowed on a literal matcher")
            }
            Self::DuplicateLiteral(value) => write!(formatter, "duplicate literal '{value}'"),
            Self::RegexNotAString => write!(formatter, "'regex' must be a string"),
            Self::RegexEmpty => write!(formatter, "'regex' must not be empty"),
            Self::FlagsNotAString => write!(formatter, "'flags' must be a string"),
            Self::UnsupportedFlag(flag) => write!(formatter, "unsupported regex flag '{flag}'"),
            Self::DuplicateFlag(flag) => write!(formatter, "duplicate regex flag '{flag}'"),
            Self::DuplicateRegex(source) => write!(formatter, "duplicate regex '{source}'"),
            Self::InvalidRegex(message) => write!(formatter, "invalid regex: {message}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    kind: ConfigErrorKind,
    snippet: Option<usize>,
    matcher: Option<usize>,
}

impl ConfigError {
    pub fn kind(&self) -> &ConfigErrorKind {
        &self.kind
    }

    pub fn snippet(&self) -> Option<usize> {
        self.snippet
    }

    pub fn matcher(&self) -> Option<usize> {
        self.matcher
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.snippet, self.matcher) {
            (Some(snippet), Some(matcher)) => {
                write!(formatter, "snippet {snippet}, matcher {matcher}: {}", self.kind)
            }
            (Some(snippet), None) => write!(formatter, "snippet {snippet}: {}", self.kind),
            _ => write!(formatter, "{}", self.kind),
        }
    }
}

impl Error for ConfigError {}

pub(crate) struct Matcher {
    pub(crate) regex: Regex,
}

pub(crate) struct Snippet {
    pub(crate) matchers: Vec<Matcher>,
    pub(crate) body: String,
}

pub struct Config {
    pub(crate) snippets: Vec<Snippet>,
}

fn is_ecma_whitespace(character: char) -> bool {
    matches!(
        character,
        '\u{0009}'
            | '\u{000A}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{000D}'
            | '\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

fn escape_literal(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for character in value.chars() {
        if matches!(
            character,
            '^' | '$' | '\\' | '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|'
        ) {
            escaped.push('\\');
        }

        escaped.push(character);
    }

    escaped
}

fn normalize_flags(flags: &str) -> Result<String, ConfigErrorKind> {
    let mut seen = Vec::new();

    for flag in flags.chars() {
        if !matches!(flag, 'i' | 'm' | 's' | 'u') {
            return Err(ConfigErrorKind::UnsupportedFlag(flag));
        }

        if seen.contains(&flag) {
            return Err(ConfigErrorKind::DuplicateFlag(flag));
        }

        seen.push(flag);
    }

    seen.retain(|flag| *flag != 'u');
    seen.sort_unstable();

    Ok(seen.into_iter().collect())
}

struct Seen {
    literals: HashSet<String>,
    regexes: HashSet<String>,
}

fn parse_matcher(raw: &Value, seen: &mut Seen) -> Result<Matcher, ConfigErrorKind> {
    let Some(raw) = raw.as_object() else {
        return Err(ConfigErrorKind::MatcherNotAnObject);
    };

    let raw_value = raw.get("value");
    let raw_regex = raw.get("regex");

    if raw_value.is_some() && raw_regex.is_some() {
        return Err(ConfigErrorKind::MatcherDefinesBothForms);
    }

    if let Some(raw_value) = raw_value {
        let Some(value) = raw_value.as_str() else {
            return Err(ConfigErrorKind::ValueNotAString);
        };

        if raw.contains_key("flags") {
            return Err(ConfigErrorKind::FlagsOnLiteralMatcher);
        }

        let value = value.trim_matches(is_ecma_whitespace);

        if value.is_empty() {
            return Err(ConfigErrorKind::ValueEmpty);
        }

        if !seen.literals.insert(value.to_lowercase()) {
            return Err(ConfigErrorKind::DuplicateLiteral(value.to_owned()));
        }

        return match Regex::with_flags(&escape_literal(value), "iu") {
            Ok(regex) => Ok(Matcher { regex }),
            Err(error) => Err(ConfigErrorKind::InvalidRegex(error.to_string())),
        };
    }

    let Some(raw_regex) = raw_regex else {
        return Err(ConfigErrorKind::MatcherDefinesNoForm);
    };

    let Some(source) = raw_regex.as_str() else {
        return Err(ConfigErrorKind::RegexNotAString);
    };

    if source.is_empty() {
        return Err(ConfigErrorKind::RegexEmpty);
    }

    let flags = match raw.get("flags") {
        None => "",
        Some(raw_flags) => match raw_flags.as_str() {
            Some(flags) => flags,
            None => return Err(ConfigErrorKind::FlagsNotAString),
        },
    };

    let normalized_flags = normalize_flags(flags)?;

    if !seen.regexes.insert(format!("{normalized_flags} {source}")) {
        return Err(ConfigErrorKind::DuplicateRegex(source.to_owned()));
    }

    match Regex::with_flags(source, format!("{normalized_flags}u").as_str()) {
        Ok(regex) => Ok(Matcher { regex }),
        Err(error) => Err(ConfigErrorKind::InvalidRegex(error.to_string())),
    }
}

fn parse_snippet(raw: &Value, seen: &mut Seen) -> Result<Snippet, ConfigError> {
    let at_snippet = |kind: ConfigErrorKind| ConfigError {
        kind,
        snippet: None,
        matcher: None,
    };

    let Some(raw) = raw.as_object() else {
        return Err(at_snippet(ConfigErrorKind::SnippetNotAnObject));
    };

    let Some(raw_body) = raw.get("body") else {
        return Err(at_snippet(ConfigErrorKind::BodyNotAString));
    };

    let Some(body) = raw_body.as_str() else {
        return Err(at_snippet(ConfigErrorKind::BodyNotAString));
    };

    if body.is_empty() {
        return Err(at_snippet(ConfigErrorKind::BodyEmpty));
    }

    let Some(raw_when) = raw.get("when") else {
        return Err(at_snippet(ConfigErrorKind::WhenNotAnArray));
    };

    let Some(raw_when) = raw_when.as_array() else {
        return Err(at_snippet(ConfigErrorKind::WhenNotAnArray));
    };

    if raw_when.is_empty() {
        return Err(at_snippet(ConfigErrorKind::WhenEmpty));
    }

    let mut matchers = Vec::with_capacity(raw_when.len());

    for (matcher_index, raw_matcher) in raw_when.iter().enumerate() {
        match parse_matcher(raw_matcher, seen) {
            Ok(matcher) => matchers.push(matcher),
            Err(kind) => {
                return Err(ConfigError {
                    kind,
                    snippet: None,
                    matcher: Some(matcher_index),
                });
            }
        }
    }

    Ok(Snippet {
        matchers,
        body: body.to_owned(),
    })
}

pub fn parse_config(raw: &Value) -> Result<Config, ConfigError> {
    let Some(raw) = raw.as_array() else {
        return Err(ConfigError {
            kind: ConfigErrorKind::NotAnArray,
            snippet: None,
            matcher: None,
        });
    };

    let mut seen = Seen {
        literals: HashSet::new(),
        regexes: HashSet::new(),
    };
    let mut snippets = Vec::with_capacity(raw.len());

    for (snippet_index, raw_snippet) in raw.iter().enumerate() {
        match parse_snippet(raw_snippet, &mut seen) {
            Ok(snippet) => snippets.push(snippet),
            Err(mut error) => {
                error.snippet = Some(snippet_index);

                return Err(error);
            }
        }
    }

    Ok(Config { snippets })
}
