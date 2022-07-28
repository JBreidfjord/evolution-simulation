import './index.css';

import React from 'react';
import ReactDOM from 'react-dom';

import App from './App';
import { SimContextProvider } from './context/SimContext';

ReactDOM.render(
  <React.StrictMode>
    <SimContextProvider>
      <App />
    </SimContextProvider>
  </React.StrictMode>,
  document.getElementById('root')
);
