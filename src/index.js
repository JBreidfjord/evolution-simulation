import "./index.css";

import App from "./App";
import React from "react";
import ReactDOM from "react-dom";
import { SimProvider } from "./context/SimContext";

ReactDOM.render(
  <React.StrictMode>
    <SimProvider>
      <App />
    </SimProvider>
  </React.StrictMode>,
  document.getElementById("root")
);
