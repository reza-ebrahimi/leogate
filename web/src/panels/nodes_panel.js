import React from "react";
import styled from "styled-components";
import { gql, useQuery } from "@apollo/client";
import Network from "../network_interface";

const QUERY = gql`
  query {
    masterInfo {
      nodes
    }
  }
`;

const NodesPanelStyled = styled.div`
  display: grid;
  grid-template-rows: auto;
  grid-row-gap: 10px;
`;

const NodesPanel = (props) => {
  const { loading, error, data } = useQuery(QUERY, {
    client: Network.default_client.handle,
  });

  if (error) {
    return `Query Error! ${error.message}`;
  }

  if (loading) {
    return null;
  }

  const nodes = data.masterInfo.nodes;

  return (
    <NodesPanelStyled>
      {nodes.map((node, idx) => {
        return <div key={idx}>{node}</div>;
      })}
    </NodesPanelStyled>
  );
};

export default NodesPanel;
