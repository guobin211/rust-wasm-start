import { useEffect, useState } from 'react';

export function getPublicKey(obj: any = {}): string[] {
  return Object.keys(obj).filter(k => !k.startsWith('_'));
}

export function useModuleKeys(obj: any = {}) {
  const [keys, setKeys] = useState([]);
  useEffect(() => {
    if (obj && typeof obj === 'object') {
      setKeys(getPublicKey(obj));
    }
  }, [obj]);
  return [keys];
}
