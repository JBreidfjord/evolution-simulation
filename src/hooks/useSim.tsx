import { useContext } from 'react';

import { SimContext, SimContextProps } from '../context/SimContext';

export const useSim = (): SimContextProps => {
  const context = useContext(SimContext);

  if (!context) {
    throw new Error('useSim must be used inside a SimProvider');
  }

  return context;
};
