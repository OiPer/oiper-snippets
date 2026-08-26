.PHONY: ts\:lint ts\:typecheck ts\:build ts\:dev

ts\:lint:
	pnpm --filter @oiper/snippets lint

ts\:typecheck:
	pnpm --filter @oiper/snippets typecheck

ts\:build:
	pnpm --filter @oiper/snippets build

ts\:dev:
	pnpm --filter @oiper/snippets dev
