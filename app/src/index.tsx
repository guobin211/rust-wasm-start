import React from 'react';
import { createRoot, Root } from 'react-dom/client';
import Document, { DocumentProps } from './components/Document';
import './index.css';

const app = document.getElementById('app') as HTMLDivElement;
const appRoot: Root = createRoot(app);
const initProps: DocumentProps = {
  title: 'Webpack',
};
appRoot.render(<Document {...initProps} />);
prefetch();

async function prefetch() {
  setTimeout(() => {
    initProps.title = 'Webpack WebAssembly';
    initProps.description = 'Webpack WebAssembly';
    appRoot.render(<Document {...initProps} />);
  }, 500);
}
