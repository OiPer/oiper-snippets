.PHONY: test tests\:build tests\:typecheck ts\:lint ts\:typecheck ts\:build ts\:dev

test:
	pnpm --filter @oiper/tests test

tests\:build:
	pnpm --filter @oiper/tests build

tests\:typecheck: tests\:build
	pnpm --filter @oiper/tests typecheck

ts\:lint:
	pnpm --filter @oiper/snippets lint

ts\:typecheck:
	pnpm --filter @oiper/snippets typecheck

ts\:build:
	pnpm --filter @oiper/snippets build

ts\:dev:
	pnpm --filter @oiper/snippets dev
