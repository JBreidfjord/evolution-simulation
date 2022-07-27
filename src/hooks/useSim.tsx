import { useContext } from 'react';

import { SimContext } from '../context/SimContext';

export const useSim = () => {
  const context = useContext(SimContext);

  if (!context) {
    throw new Error('useSim must be used inside a SimProvider');
  }

  return context;
};
