import * as WasmCoreModule from 'rust-wasm-core/pkg';

const { parse_string, reverse_map } = WasmCoreModule;

function parseString(n: string) {
  if (typeof n === 'string') {
    return parse_string(n);
  }
  throw new Error(`Expected string, got ${typeof n}`);
}

function reverseMap(p: Map<string, string>) {
  return reverse_map(p);
}

export {
  WasmCoreModule,
  parseString,
  reverseMap,
};
