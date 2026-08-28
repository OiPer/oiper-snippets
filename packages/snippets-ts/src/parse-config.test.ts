import { readdir, readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { parseConfig, SnippetConfigError } from './parse-config'

const fixtureDirectory = fileURLToPath(
  new URL('../../../fixtures/config/', import.meta.url)
)

async function readFixtureNames() {
  return (await readdir(fixtureDirectory))
    .filter((name) => name.endsWith('.json'))
    .sort()
}

describe('parseConfig', () => {
  it('rejects every invalid configuration fixture', async () => {
    const fixtureNames = await readFixtureNames()
    let caseCount = 0

    for (const fixtureName of fixtureNames) {
      const fixture = JSON.parse(
        await readFile(`${fixtureDirectory}/${fixtureName}`, 'utf8')
      ) as Record<string, unknown>

      for (const [description, config] of Object.entries(fixture)) {
        caseCount++

        expect(() => parseConfig(config), description).toThrow(
          SnippetConfigError
        )
      }
    }

    expect(caseCount).toBeGreaterThan(0)
  })
})
