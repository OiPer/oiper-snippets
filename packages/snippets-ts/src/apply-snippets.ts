import type { Config } from './config'

interface Match {
  readonly body: string
  readonly length: number
}

function findMatch(input: string, cursor: number, config: Config): Match | null {
  for (const snippet of config.snippets) {
    for (const matcher of snippet.matchers) {
      matcher.regex.lastIndex = cursor

      const result = matcher.regex.exec(input)

      if (result === null || result[0].length === 0) {
        continue
      }

      return { body: snippet.body, length: result[0].length }
    }
  }

  return null
}

export function applySnippets(input: string, config: Config): string {
  let output = ''
  let cursor = 0

  while (cursor < input.length) {
    const match = findMatch(input, cursor, config)

    if (match !== null) {
      output += match.body
      cursor += match.length
      continue
    }

    const codePoint = input.codePointAt(cursor)

    if (codePoint === undefined) {
      throw new TypeError('cursor moved outside the input')
    }

    const character = String.fromCodePoint(codePoint)

    output += character
    cursor += character.length
  }

  return output
}
