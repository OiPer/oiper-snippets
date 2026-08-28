import { describe, expect, it } from 'vitest'
import { oiperRs, oiperTs } from './lib'

type JsonValue =
  boolean | number | string | null | JsonValue[] | { [key: string]: JsonValue }

interface ApplicationCase {
  config: JsonValue
  input: string
  output: string
}

interface Implementation {
  name: string
  applySnippets(config: JsonValue, input: string): string
  validateConfig(config: JsonValue): void
}

const invalidConfigs = {
  'requires configuration to be an array': {},
  'requires at least one matcher': [{ when: [], body: 'replacement' }],
  'rejects blank literals': [
    {
      when: [{ value: '   ' }],
      body: 'replacement',
    },
  ],
  'rejects duplicate literals ignoring case': [
    {
      when: [{ value: 'brb' }],
      body: 'first',
    },
    {
      when: [{ value: 'BRB' }],
      body: 'second',
    },
  ],
  'rejects unsupported regex flags': [
    {
      when: [{ regex: 'brb', flags: 'g' }],
      body: 'replacement',
    },
  ],
} satisfies Record<string, JsonValue>

const applicationCases = {
  'leaves input unchanged with an empty configuration': {
    config: [],
    input: 'unchanged',
    output: 'unchanged',
  },
  'matches literals case-insensitively': {
    config: [
      {
        when: [{ value: 'brb' }],
        body: 'be right back',
      },
    ],
    input: 'BRB!',
    output: 'be right back!',
  },
  'uses the first matching matcher': {
    config: [
      {
        when: [{ value: 'a' }, { value: 'ab' }],
        body: 'first',
      },
    ],
    input: 'ab',
    output: 'firstb',
  },
  'does not rescan inserted bodies': {
    config: [
      {
        when: [{ value: 'a' }],
        body: 'b',
      },
      {
        when: [{ value: 'b' }],
        body: 'c',
      },
    ],
    input: 'a',
    output: 'b',
  },
  'matches regular expressions case-insensitively': {
    config: [
      {
        when: [{ regex: '\\bbr+b\\b', flags: 'i' }],
        body: 'be right back',
      },
    ],
    input: 'BRRB',
    output: 'be right back',
  },
} satisfies Record<string, ApplicationCase>

const implementations = [
  {
    name: 'TypeScript',
    applySnippets(config, input) {
      return oiperTs.applySnippets(input, oiperTs.parseConfig(config))
    },
    validateConfig(config) {
      oiperTs.parseConfig(config)
    },
  },
  {
    name: 'Rust',
    applySnippets(config, input) {
      return oiperRs.applySnippets(JSON.stringify(config), input)
    },
    validateConfig(config) {
      oiperRs.validateConfig(JSON.stringify(config))
    },
  },
] satisfies Implementation[]

describe.each(implementations)('$name implementation', (implementation) => {
  it.each(Object.entries(invalidConfigs))('%s', (_description, config) => {
    expect(() => implementation.validateConfig(config)).toThrow()
  })

  it.each(Object.entries(applicationCases))('%s', (_description, testCase) => {
    expect(implementation.applySnippets(testCase.config, testCase.input)).toBe(
      testCase.output
    )
  })
})
