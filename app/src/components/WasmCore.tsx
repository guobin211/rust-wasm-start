import React, { useState } from 'react';
import { parseString, reverseMap, WasmCoreModule } from '../core/@wasm-core';
import { useModuleKeys } from '../hooks/use-module-keys';

export interface WasmCoreProps {
  className?: string;
}

declare const window: Window & {
  takes_immutable_closure: () => void;
};
const WasmCore: React.FC<WasmCoreProps> = () => {
  const [value, setValue] = useState<string>('0');
  const [encode, setEncode] = useState<string>('');
  const [methods] = useModuleKeys(WasmCoreModule);
  const source = new Map([['name', 'jack'], ['age', '18']]);

  function handleClick() {
    const safeValue = parseString(value);
    setEncode(safeValue);
  }

  function handleReverseMapClick() {
    const result = reverseMap(source);
    console.log('reverseMap source:', source);
    console.log('reverseMap result:', result);
  }

  function handleTakeClick() {
    if (!window.takes_immutable_closure) {
      window.takes_immutable_closure = () => {
        console.log('takes_immutable_closure');
      };
    }
  }

  return (
    <div className="section">
      <h2>WasmCore</h2>
      <div className="flex-row">
        <div className="flex-left">
          <ul>
            {methods.map((key) => (
              <li key={key}>
                <span>{key}</span>
              </li>
            ))}
          </ul>
        </div>
        <div className="flex-right">
          <div className="flex-item">
            <input type="text" value={value} onChange={e => setValue(e.target.value)} />
            <p>encode: {encode}</p>
          </div>
          <div className="flex-item">
            <button type={'button'} onClick={handleClick}>parseString</button>
          </div>
          <div className="flex-item">
            <button type={'button'} onClick={handleReverseMapClick}>reverse_map</button>
          </div>
          <div className="flex-item">
            <button type={'button'} onClick={handleTakeClick}>takes_immutable_closure</button>
          </div>
        </div>
      </div>
    </div>
  );
};
export default WasmCore;
