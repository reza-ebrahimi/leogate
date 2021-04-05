import {
  ApolloClient,
  InMemoryCache,
  split,
  HttpLink,
  ApolloLink,
} from "@apollo/client";
import { getMainDefinition } from "@apollo/client/utilities";
import { WebSocketLink } from "@apollo/client/link/ws";

const httpLink = new HttpLink({
  uri: "http://localhost:8001",
});

const wsLink = new WebSocketLink({
  uri: `ws://localhost:8000`,
  options: {
    reconnect: true,
  },
});

const terminatingLink = split(
  ({ query }) => {
    const { kind, operation } = getMainDefinition(query);
    return (
      kind === "OperationDefinition" &&
      (operation === "subscription" || operation === "query")
    );
  },
  wsLink,
  httpLink
);

const client = new ApolloClient({
  link: ApolloLink.from([terminatingLink]),
  cache: new InMemoryCache(),
});

export default client;
