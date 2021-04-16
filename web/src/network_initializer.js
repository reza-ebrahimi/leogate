import React from "react";
import { gql, useQuery } from "@apollo/client";
import Network from "./network_interface";

const QUERY = gql`
  query {
    clients {
      name
      port
      default
    }
  }
`;

const NetworkInitializer = (props) => {
  const { loading, error, data } = useQuery(QUERY);

  if (error) {
    return `NetworkInitializer Query Error! ${error.message}`;
  }

  if (loading) {
    return null;
  }

  if (Network.clients.length === 1) {
    data.clients.map((client) => {
      Network.create_client(client.name, client.port);
      if (client.default) {
        Network.default_client = Network.client(client.name);
      }
      return client;
    });
  }

  return <>{props.children}</>;
};

export default NetworkInitializer;
