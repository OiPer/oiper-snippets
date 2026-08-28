import { readdir, readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { applySnippets } from './apply-snippets'
import { parseConfig } from './parse-config'

interface TestCase {
  id: string
  config: unknown
  input: string
  expected: { kind: 'output'; value: string } | { kind: 'error' }
}

const fixtureDirectory = fileURLToPath(
  new URL('../../../fixtures/cases/', import.meta.url)
)

async function readFixtureNames() {
  return (await readdir(fixtureDirectory))
    .filter((name) => name.endsWith('.json'))
    .sort()
}

describe('applySnippets', () => {
  it('applies every output fixture', async () => {
    const fixtureNames = await readFixtureNames()
    let caseCount = 0

    for (const fixtureName of fixtureNames) {
      const fixture = JSON.parse(
        await readFile(`${fixtureDirectory}/${fixtureName}`, 'utf8')
      ) as TestCase[]

      for (const testCase of fixture) {
        if (testCase.expected.kind === 'error') {
          continue
        }

        caseCount++

        expect(
          applySnippets(testCase.input, parseConfig(testCase.config)),
          testCase.id
        ).toBe(testCase.expected.value)
      }
    }

    expect(caseCount).toBeGreaterThan(0)
  })
})
