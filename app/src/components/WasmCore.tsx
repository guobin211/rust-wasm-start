import React, { useState } from 'react';
import { parseNumber } from '../core/@wasm-core';

export interface WasmCoreProps {
  className?: string;
}

const WasmCore: React.FC<WasmCoreProps> = () => {
  const [value, setValue] = useState<string>('0');

  function handleClick() {
    const safeValue = Number(value);
    parseNumber(safeValue);
  }

  return (
    <div className="section">
      <h2>WasmCore</h2>
      <div>
        <input type="number" value={value} onChange={e => setValue(e.target.value)} />
        <button type={'button'} onClick={handleClick}>parseNumber</button>
      </div>
    </div>
  );
};
export default WasmCore;
