import type { AnySchema } from 'ajv'
import Ajv2020 from 'ajv/dist/2020.js'
import { readdir, readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { applySnippets } from './apply-snippets'
import { parseConfig } from './parse-config'

interface TestCase {
  config: unknown
  input: string
  output: string
}

const fixtureDirectory = fileURLToPath(
  new URL('../../../fixtures/snippets/', import.meta.url)
)
const schemaPath = fileURLToPath(
  new URL('../../../fixtures/snippets/schema.json', import.meta.url)
)

async function readJson(path: string) {
  return JSON.parse(await readFile(path, 'utf8')) as unknown
}

function isSchema(value: unknown): value is AnySchema {
  return (
    typeof value === 'boolean' || (typeof value === 'object' && value !== null)
  )
}

async function readFixtureNames() {
  return (await readdir(fixtureDirectory))
    .filter((name) => name.endsWith('.json') && name !== 'schema.json')
    .sort()
}

describe('applySnippets', () => {
  it('validates the fixtures', async () => {
    const schema = await readJson(schemaPath)

    if (!isSchema(schema)) {
      throw new TypeError('fixture schema must be a JSON object or boolean')
    }

    const validate = new Ajv2020({ allErrors: true }).compile(schema)
    const fixtureNames = await readFixtureNames()

    expect(fixtureNames.length).toBeGreaterThan(0)

    for (const fixtureName of fixtureNames) {
      const fixture = await readJson(`${fixtureDirectory}/${fixtureName}`)

      expect(
        validate(fixture),
        `${fixtureName}: ${JSON.stringify(validate.errors, null, 2)}`
      ).toBe(true)
    }
  })

  it('applies every output fixture', async () => {
    const fixtureNames = await readFixtureNames()
    let caseCount = 0

    for (const fixtureName of fixtureNames) {
      const fixture = JSON.parse(
        await readFile(`${fixtureDirectory}/${fixtureName}`, 'utf8')
      ) as Record<string, TestCase>

      for (const [description, testCase] of Object.entries(fixture)) {
        caseCount++

        expect(
          applySnippets(testCase.input, parseConfig(testCase.config)),
          description
        ).toBe(testCase.output)
      }
    }

    expect(caseCount).toBeGreaterThan(0)
  })
})
