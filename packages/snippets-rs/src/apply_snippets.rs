use crate::parse_config::Config;

fn find_match<'a>(input: &str, cursor: usize, config: &'a Config) -> Option<(&'a str, usize)> {
    for snippet in &config.snippets {
        for matcher in &snippet.matchers {
            let Some(found) = matcher.regex.find_from(input, cursor).next() else {
                continue;
            };
            let range = found.range();

            if range.start != cursor || range.end == range.start {
                continue;
            }

            return Some((snippet.body.as_str(), range.end - range.start));
        }
    }

    None
}

pub fn apply_snippets(input: &str, config: &Config) -> String {
    let mut output = String::new();
    let mut cursor = 0;

    while cursor < input.len() {
        if let Some((body, length)) = find_match(input, cursor, config) {
            output.push_str(body);
            cursor += length;

            continue;
        }

        let character = input[cursor..]
            .chars()
            .next()
            .expect("cursor must rest on a character boundary inside the input");

        output.push(character);
        cursor += character.len_utf8();
    }

    output
}
