import { oiperRs, oiperTs } from '@/lib'
import { expect, it } from 'vitest'

it('TypeScript leaves input unchanged with an empty configuration', () => {
  expect(oiperTs.applySnippets('unchanged', oiperTs.parseConfig([]))).toBe(
    'unchanged'
  )
})

it('TypeScript matches literals case-insensitively', () => {
  expect(
    oiperTs.applySnippets(
      'BRB!',
      oiperTs.parseConfig([
        {
          when: [{ value: 'brb' }],
          body: 'be right back',
        },
      ])
    )
  ).toBe('be right back!')
})

it('TypeScript uses the first matching matcher', () => {
  expect(
    oiperTs.applySnippets(
      'ab',
      oiperTs.parseConfig([
        {
          when: [{ value: 'a' }, { value: 'ab' }],
          body: 'first',
        },
      ])
    )
  ).toBe('firstb')
})

it('TypeScript does not rescan inserted bodies', () => {
  expect(
    oiperTs.applySnippets(
      'a',
      oiperTs.parseConfig([
        {
          when: [{ value: 'a' }],
          body: 'b',
        },
        {
          when: [{ value: 'b' }],
          body: 'c',
        },
      ])
    )
  ).toBe('b')
})

it('TypeScript matches regular expressions case-insensitively', () => {
  expect(
    oiperTs.applySnippets(
      'BRRB',
      oiperTs.parseConfig([
        {
          when: [{ regex: '\\bbr+b\\b', flags: 'i' }],
          body: 'be right back',
        },
      ])
    )
  ).toBe('be right back')
})

it('Rust leaves input unchanged with an empty configuration', () => {
  expect(oiperRs.applySnippets('[]', 'unchanged')).toBe('unchanged')
})

it('Rust matches literals case-insensitively', () => {
  expect(
    oiperRs.applySnippets(
      '[{"when":[{"value":"brb"}],"body":"be right back"}]',
      'BRB!'
    )
  ).toBe('be right back!')
})

it('Rust uses the first matching matcher', () => {
  expect(
    oiperRs.applySnippets(
      '[{"when":[{"value":"a"},{"value":"ab"}],"body":"first"}]',
      'ab'
    )
  ).toBe('firstb')
})

it('Rust does not rescan inserted bodies', () => {
  expect(
    oiperRs.applySnippets(
      '[{"when":[{"value":"a"}],"body":"b"},{"when":[{"value":"b"}],"body":"c"}]',
      'a'
    )
  ).toBe('b')
})

it('Rust matches regular expressions case-insensitively', () => {
  expect(
    oiperRs.applySnippets(
      '[{"when":[{"regex":"\\\\bbr+b\\\\b","flags":"i"}],"body":"be right back"}]',
      'BRRB'
    )
  ).toBe('be right back')
})
