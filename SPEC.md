# OiPer Snippets Specification

## Overview

This is a general-purpose library for applying configured snippets to an input
string. It is independent of any application, input source, or output
destination.

This specification covers both libraries:

- TypeScript: `@oiper/snippets`
- Rust: `oiper-snippets`

Given the same valid configuration and input, both libraries must return the
same output.

## Configuration

A configuration is an ordered array of snippets:

```json
[
  {
    "when": [
      { "value": "brb" },
      { "regex": "\\bbr+b\\b", "flags": "i" }
    ],
    "body": "be right back"
  }
]
```

Each snippet contains:

- `when`: A non-empty ordered array of matchers.
- `body`: The non-empty string inserted when any matcher succeeds.

A matcher has exactly one of these forms:

```text
{ value: string }
{ regex: string, flags?: string }
```

`value` is a case-insensitive literal. `regex` is an ECMAScript regular
expression. All matchers in one snippet use the same body.

Configuration order and `when` order are significant.

## API

Configuration is parsed once and then reused when applying snippets.

TypeScript:

```ts
const config = parseConfig(rawConfig)
const output = applySnippets(input, config)
```

Rust:

```rust
let config = parse_config(raw_config)?;
let output = apply_snippets(input, &config);
```

Parsing validates and prepares the configuration, including compiling regexes.
It returns an opaque `Config` that cannot be modified.

TypeScript throws a configuration error when parsing fails. Rust returns an
error result. `applySnippets` and `apply_snippets` accept only a parsed config
and return the output string directly.

## Validation

`parseConfig` and `parse_config` enforce these rules:

- The configuration is an array. An empty array is valid.
- Every snippet has a non-empty `when` array and a non-empty string `body`.
- Every matcher contains either a string `value` or a string `regex`, not both.
- Literal values are trimmed. A value empty after trimming is invalid.
- Literal values must be unique across the entire configuration after trimming
  and case-insensitive comparison.
- Regex sources are not trimmed and must be non-empty and valid.
- Regex flags may contain only `i`, `m`, `s`, and `u`, without duplicates.
- Regex matchers with the same source and normalized flags must be unique across
  the entire configuration.

Bodies are not trimmed. A whitespace-only body is valid.

Validation does not try to detect equivalent regexes, overlapping matchers, or
overlap between a literal and a regex. Matcher order resolves those cases.

## Applying Snippets

Snippet application is deterministic, left-to-right, and single-pass.

At each position in the input:

1. Test snippets in configuration order.
2. Test each snippet's matchers in `when` order.
3. Use the first matcher that succeeds at the current position.
4. Append its snippet body and advance past the matched input.
5. If nothing matches, append the next input character unchanged.

An earlier matcher always wins. The library does not prefer the longest match.
For example, if `a` appears before `ab`, input `ab` matches `a` first.

Literal matchers:

- Match case-insensitively using ECMAScript Unicode semantics.
- Match their complete value at the current position.
- Treat regex metacharacters as normal characters.
- Do not add word boundaries and can match inside words.

Regex matchers:

- Must begin at the current position; they do not search ahead.
- Use the complete input as context for anchors and assertions.
- Are case-sensitive unless the `i` flag is present.
- Ignore zero-length results and continue testing other matchers.

The body is inserted verbatim. Regex captures such as `$1` are not expanded.
Inserted bodies are not scanned again, so snippets do not recurse or chain.

> Implementation note: Keep the cursor on the original input. After a match,
> advance it by the matched input length, append the body directly to the
> output, and never run matchers against that body.

Input and output are not trimmed or Unicode-normalized.

## Regex Standard

Regex syntax and behavior follow ECMAScript 2018 regular expressions in Unicode
mode:

<https://262.ecma-international.org/9.0/#sec-regexp-regular-expression-objects>

Unicode mode is always enabled, whether or not `u` is provided. The supported
user flags are:

- `i`: Case-insensitive matching.
- `m`: Multiline anchors.
- `s`: Dot matches line terminators.
- `u`: Unicode mode; accepted but redundant.

The flags `d`, `g`, `v`, and `y` are not supported. The library controls global
scanning and cursor positioning.

The TypeScript library uses Node.js's native `RegExp`. The Rust library uses a
native Rust ECMAScript-compatible engine, initially `regress`:

<https://docs.rs/regress/latest/regress/>

The Rust library does not embed JavaScript, and the TypeScript library does not
use the Rust implementation through WebAssembly.

## Cross-Language Parity

Both libraries use the same language-neutral JSON fixtures. The fixtures cover
configuration validation, ordering, literals, regexes, flags, single-pass
behavior, whitespace, and Unicode.

Every fixture must produce the same validation result and exact output in both
libraries. Regex engine or Unicode-data upgrades must pass the shared fixtures
before release.

## Non-Goals

The libraries do not provide configuration storage, JSON/JSONC file parsing,
CRUD, UI behavior, recursive expansion, capture interpolation, templates, or
automatic word boundaries.
