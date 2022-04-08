import React from 'react';
import WasmCore from './WasmCore';
import WasmWeb from './WasmWeb';

export interface AppProps {
  className?: string;
}

const App: React.FC<AppProps> = (props) => {
  const { className } = props;
  return (
    <div className={'app' + className}>
      <WasmCore />
      <WasmWeb />
    </div>
  );
};
export default App;
