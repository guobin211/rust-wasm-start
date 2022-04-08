import { call_alert } from 'rust-wasm-web/pkg';

export function callAlert(message: string) {
  return call_alert(message);
}
