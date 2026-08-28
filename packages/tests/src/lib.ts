export * as oiperTs from '../../snippets-ts/src/index'
export * as oiperRs from '../index.js'

export type JsonValue =
  boolean | number | string | null | JsonValue[] | { [key: string]: JsonValue }
