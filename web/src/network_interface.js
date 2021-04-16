import {
  ApolloClient,
  InMemoryCache,
  split,
  HttpLink,
  ApolloLink,
} from "@apollo/client";
import { getMainDefinition } from "@apollo/client/utilities";
import { WebSocketLink } from "@apollo/client/link/ws";

var Network = {
  httpLink: function (port) {
    return new HttpLink({
      uri: `http://localhost:${port}`,
    });
  },

  wsLink: function (port) {
    return new WebSocketLink({
      uri: `ws://localhost:${port}`,
      options: {
        reconnect: true,
      },
    });
  },

  create_client: function (name, port) {
    this.clients.push({
      name,
      port,
      handle: new ApolloClient({
        link: ApolloLink.from([
          split(
            ({ query }) => {
              const { kind, operation } = getMainDefinition(query);
              return (
                kind === "OperationDefinition" &&
                (operation === "subscription" || operation === "query")
              );
            },
            this.wsLink(port),
            this.httpLink(port)
          ),
        ]),
        cache: new InMemoryCache(),
      })
    });    
  },

  client: function (name) {
    return this.clients.find((client) => {
      return client.name === name;
    })
  },
  
  clients: [],
  default_client: null
};

export default Network;
