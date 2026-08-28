import type { AnySchema } from 'ajv'
import Ajv2020 from 'ajv/dist/2020.js'
import { readdir, readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { parseConfig, SnippetConfigError } from './parse-config'

interface TestCase {
  id: string
  config: unknown
  expected: { kind: 'output'; value: string } | { kind: 'error' }
}

const fixtureDirectory = fileURLToPath(
  new URL('../../../fixtures/cases/', import.meta.url)
)
const schemaPath = fileURLToPath(
  new URL('../../../fixtures/schema.json', import.meta.url)
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
    .filter((name) => name.endsWith('.json'))
    .sort()
}

describe('parseConfig', () => {
  it('validates the fixture schema and unique case IDs', async () => {
    const schema = await readJson(schemaPath)

    if (!isSchema(schema)) {
      throw new TypeError('fixture schema must be a JSON object or boolean')
    }

    const validate = new Ajv2020({ allErrors: true }).compile(schema)
    const fixtureNames = await readFixtureNames()
    const caseIds = new Set<string>()

    expect(fixtureNames.length).toBeGreaterThan(0)

    for (const fixtureName of fixtureNames) {
      const fixture = await readJson(`${fixtureDirectory}/${fixtureName}`)
      const valid = validate(fixture)

      expect(
        valid,
        `${fixtureName}: ${JSON.stringify(validate.errors, null, 2)}`
      ).toBe(true)

      if (!valid || !Array.isArray(fixture)) {
        continue
      }

      for (const testCase of fixture) {
        if (
          typeof testCase !== 'object' ||
          testCase === null ||
          !('id' in testCase) ||
          typeof testCase.id !== 'string'
        ) {
          continue
        }

        expect(
          caseIds.has(testCase.id),
          `duplicate case ID: ${testCase.id}`
        ).toBe(false)
        caseIds.add(testCase.id)
      }
    }
  })

  it('parses every fixture configuration', async () => {
    const fixtureNames = await readFixtureNames()
    let caseCount = 0

    for (const fixtureName of fixtureNames) {
      const fixture = (await readJson(
        `${fixtureDirectory}/${fixtureName}`
      )) as TestCase[]

      for (const testCase of fixture) {
        caseCount++

        if (testCase.expected.kind === 'error') {
          expect(() => parseConfig(testCase.config), testCase.id).toThrow(
            SnippetConfigError
          )
        } else {
          expect(() => parseConfig(testCase.config), testCase.id).not.toThrow()
        }
      }
    }

    expect(caseCount).toBeGreaterThan(0)
  })
})
