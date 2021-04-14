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

  client: function (httpPort, wsPort) {
    return new ApolloClient({
      link: ApolloLink.from([
        split(
          ({ query }) => {
            const { kind, operation } = getMainDefinition(query);
            return (
              kind === "OperationDefinition" &&
              (operation === "subscription" || operation === "query")
            );
          },
          this.wsLink(wsPort),
          this.httpLink(httpPort)
        ),
      ]),
      cache: new InMemoryCache(),
    });
  },

  sys_client: null,
  ros_client: null,
};

export default Network;
