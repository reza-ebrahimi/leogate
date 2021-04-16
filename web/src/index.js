import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";
import { ApolloProvider } from "@apollo/client";

import App from "./App";
import Network from "./network_interface";
import NetworkInitializer from "./network_initializer";

import reportWebVitals from "./reportWebVitals";

Network.create_client("sys", 8000);

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <ApolloProvider client={Network.client("sys").handle}>
        <NetworkInitializer>
          <App />
        </NetworkInitializer>
      </ApolloProvider>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById("root")
);

reportWebVitals();
