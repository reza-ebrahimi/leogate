import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";
import { ApolloProvider } from "@apollo/client";

import App from "./App";
import network from "./network_interface";

import reportWebVitals from "./reportWebVitals";

network.sys_client = network.client(8000, 8000);
network.ros_client = network.client(8001, 8001);

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <ApolloProvider client={network.sys_client}>
        <App />
      </ApolloProvider>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById("root")
);

reportWebVitals();
