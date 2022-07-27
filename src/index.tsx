import './index.css';

import React from 'react';
import ReactDOM from 'react-dom';

import App from './App';
import { SimProvider } from './context/SimContext';

ReactDOM.render(
	<React.StrictMode>
		<SimProvider>
			<App />
		</SimProvider>
	</React.StrictMode>,
	document.getElementById('root')
);
