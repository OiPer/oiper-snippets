ts-typecheck:
	pnpm --filter @oiper/snippets typecheck

ts-lint:
	pnpm --filter @oiper/snippets lint

rs-lint:
	cargo clippy --manifest-path packages/snippets-rs/Cargo.toml --all-targets -- -D warnings
	cargo clippy --manifest-path packages/tests/Cargo.toml --all-targets -- -D warnings

test:
	pnpm --filter @oiper/tests test
