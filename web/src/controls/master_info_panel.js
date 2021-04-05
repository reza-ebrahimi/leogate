import React from "react";
import styled from "styled-components";
import { gql, useQuery } from "@apollo/client";

import NodesInfo from "./nodes_table";
import TopicsInfo from "./topics_table";

const MASTER_INFO = gql`
  query {
    masterInfo {
      aliveness
      host
      port
      uri
      nodes
      topics {
        name
        datatype
      }
    }
  }
`;

const MasterInfoStyled = styled.div`
  display: grid;
  grid-template-columns: auto auto auto;
  grid-template-rows: minmax(1px, 330px);
  grid-column-gap: 5px;
  color: white;
`;

const MasterInfoHostStyled = styled.div`
  display: grid;
  grid-template-rows: repeat(auto-fill, 30px);
  grid-template-columns: 100px auto;
  padding: 20px;
  border: 3px solid gray;
`;

const MasterInfoPanel = () => {
  const { loading, error, data } = useQuery(MASTER_INFO);

  if (error) return `Error! ${error.message}`;

  if (loading) {
    return <MasterInfoStyled></MasterInfoStyled>;
  }

  return (
    <MasterInfoStyled>
      <MasterInfoHostStyled>
        <div>Aliveness</div>
        <div>{data.masterInfo.aliveness ? "Running" : "Stopped"}</div>
        <div>Host</div>
        <div>{data.masterInfo.host}</div>
        <div>Port</div>
        <div>{data.masterInfo.port}</div>
        <div>URI</div>
        <div>{data.masterInfo.uri}</div>
      </MasterInfoHostStyled>
      <NodesInfo nodes={data.masterInfo.nodes}></NodesInfo>
      <TopicsInfo topics={data.masterInfo.topics}></TopicsInfo>
    </MasterInfoStyled>
  );
};

export default MasterInfoPanel;
