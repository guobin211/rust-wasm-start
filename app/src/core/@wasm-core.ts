import { parse_number } from 'rust-wasm-core/pkg';

export function parseNumber(n: number) {
  if (typeof n === 'number') {
    return parse_number(n);
  }
  throw new Error(`Expected number, got ${typeof n}`);
}
