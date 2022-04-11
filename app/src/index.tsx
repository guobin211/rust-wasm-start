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

/**
 * @param url {string} 文件地址
 * @description 原生加载wasm文件
 * @doc https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly
 */
async function loadWasm<T extends Record<string, any>>(url: string): Promise<T> {
  let instance = { exports: {} };
  const response = await fetch(url);
  if (response && WebAssembly) {
    const buffer = await response.arrayBuffer();
    const module = await WebAssembly.compile(buffer);
    instance = await WebAssembly.instantiate(module);
  }
  return instance.exports as any;
}
