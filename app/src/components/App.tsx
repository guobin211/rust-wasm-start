import React from 'react';

export interface AppProps {
  className?: string;
}

const App: React.FC<AppProps> = (props) => {
  const { className } = props;
  return (
    <div className={className}>
      App
    </div>
  );
};
export default App;
