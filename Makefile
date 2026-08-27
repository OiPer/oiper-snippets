.PHONY: test ts\:lint ts\:typecheck ts\:test ts\:build ts\:dev rs\:test

test: ts\:test rs\:test

ts\:lint:
	pnpm --filter @oiper/snippets lint

ts\:typecheck:
	pnpm --filter @oiper/snippets typecheck

ts\:test:
	pnpm --filter @oiper/snippets test

ts\:build:
	pnpm --filter @oiper/snippets build

ts\:dev:
	pnpm --filter @oiper/snippets dev

rs\:test:
	cargo test --manifest-path packages/snippets-rs/Cargo.toml
