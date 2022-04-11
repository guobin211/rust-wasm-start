import * as WasmCoreModule from 'rust-wasm-core/pkg';

const { parse_string, reverse_map, call_js_method, call_take_method } = WasmCoreModule;

function parseString(n: string) {
  if (typeof n === 'string') {
    return parse_string(n);
  }
  throw new Error(`Expected string, got ${typeof n}`);
}

function reverseMap(p: Map<string, string>) {
  return reverse_map(p);
}

function callJsMethod(): number {
  console.log('callJsMethod start');
  const result = call_js_method();
  console.log('callJsMethod end');
  return result;
}

function callTakeMethod(p: string) {
  return call_take_method(p);
}

export {
  WasmCoreModule,
  parseString,
  reverseMap,
  callJsMethod,
  callTakeMethod,
};
