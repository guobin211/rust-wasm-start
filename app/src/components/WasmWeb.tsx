import React from 'react';
import { callAlert } from '../core/@wasm-web';

interface WasmWebProps {
  className?: string;
}

const WasmWeb: React.FC<WasmWebProps> = () => {
  function handleClick() {
    callAlert('Hello WasmWeb');
  }

  return (
    <div className="section">
      <h2>WasmWeb</h2>
      <div>
        <button type={'button'} onClick={handleClick}>call_alert</button>
      </div>
    </div>
  );
};
export {
  WasmWebProps,
};
export default WasmWeb;
