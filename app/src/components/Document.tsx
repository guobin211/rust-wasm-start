import React, { useEffect } from 'react';
import App from './App';

export interface DocumentProps {
  title?: string;
  keywords?: string;
  description?: string;
}

const Document: React.FC<DocumentProps> = (props) => {
  const { title } = props;
  useEffect(() => {
    if (title && document.title !== title) {
      document.title = title;
    }
  }, [title]);
  return (
    <App />
  );
};
export default Document;
