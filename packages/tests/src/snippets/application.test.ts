import { oiperRs, oiperTs, type JsonValue } from '@/lib'
import { describe, expect, it } from 'vitest'

interface ApplicationCase {
  config: JsonValue
  input: string
  output: string
}

interface Implementation {
  name: string
  applySnippets(config: JsonValue, input: string): string
}

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
  },
  {
    name: 'Rust',
    applySnippets(config, input) {
      return oiperRs.applySnippets(JSON.stringify(config), input)
    },
  },
] satisfies Implementation[]

describe.each(implementations)('$name snippets', (implementation) => {
  it.each(Object.entries(applicationCases))('%s', (_description, testCase) => {
    expect(implementation.applySnippets(testCase.config, testCase.input)).toBe(
      testCase.output
    )
  })
})
