import { oiperRs, oiperTs } from '@/lib'
import { expect, it } from 'vitest'

it('TypeScript rejects a configuration that is not an array', () => {
  expect(() => oiperTs.parseConfig({})).toThrow()
})

it('TypeScript rejects a snippet without a matcher', () => {
  expect(() =>
    oiperTs.parseConfig([{ when: [], body: 'replacement' }])
  ).toThrow()
})

it('TypeScript rejects a blank literal', () => {
  expect(() =>
    oiperTs.parseConfig([
      {
        when: [{ value: '   ' }],
        body: 'replacement',
      },
    ])
  ).toThrow()
})

it('TypeScript rejects duplicate literals ignoring case', () => {
  expect(() =>
    oiperTs.parseConfig([
      {
        when: [{ value: 'brb' }],
        body: 'first',
      },
      {
        when: [{ value: 'BRB' }],
        body: 'second',
      },
    ])
  ).toThrow()
})

it('TypeScript rejects an unsupported regex flag', () => {
  expect(() =>
    oiperTs.parseConfig([
      {
        when: [{ regex: 'brb', flags: 'g' }],
        body: 'replacement',
      },
    ])
  ).toThrow()
})

it('Rust rejects a configuration that is not an array', () => {
  expect(() => oiperRs.validateConfig('{}')).toThrow()
})

it('Rust rejects a snippet without a matcher', () => {
  expect(() =>
    oiperRs.validateConfig('[{"when":[],"body":"replacement"}]')
  ).toThrow()
})

it('Rust rejects a blank literal', () => {
  expect(() =>
    oiperRs.validateConfig('[{"when":[{"value":"   "}],"body":"replacement"}]')
  ).toThrow()
})

it('Rust rejects duplicate literals ignoring case', () => {
  expect(() =>
    oiperRs.validateConfig(
      '[{"when":[{"value":"brb"}],"body":"first"},{"when":[{"value":"BRB"}],"body":"second"}]'
    )
  ).toThrow()
})

it('Rust rejects an unsupported regex flag', () => {
  expect(() =>
    oiperRs.validateConfig(
      '[{"when":[{"regex":"brb","flags":"g"}],"body":"replacement"}]'
    )
  ).toThrow()
})
