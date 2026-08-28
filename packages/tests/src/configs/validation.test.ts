import { oiperRs, oiperTs, type JsonValue } from '@/lib'
import { describe, expect, it } from 'vitest'

interface Implementation {
  name: string
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

const implementations = [
  {
    name: 'TypeScript',
    validateConfig(config) {
      oiperTs.parseConfig(config)
    },
  },
  {
    name: 'Rust',
    validateConfig(config) {
      oiperRs.validateConfig(JSON.stringify(config))
    },
  },
] satisfies Implementation[]

describe.each(implementations)('$name configuration', (implementation) => {
  it.each(Object.entries(invalidConfigs))('%s', (_description, config) => {
    expect(() => implementation.validateConfig(config)).toThrow()
  })
})
