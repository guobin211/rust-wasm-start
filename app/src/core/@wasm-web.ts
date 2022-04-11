import * as WasmWebModule from 'rust-wasm-web/pkg';

const { call_alert } = WasmWebModule;

function callAlert(message: string) {
  return call_alert(message);
}

export {
  WasmWebModule,
  callAlert,
};
