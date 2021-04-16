import React from "react";
import styled from "styled-components";
import { gql, useQuery } from "@apollo/client";
import Network from "../network_interface";

const QUERY = gql`
  query {
    masterInfo {
      topics {
        name
        datatypes
      }
    }
  }
`;

const TopicsPanelStyled = styled.div`
  display: grid;
  grid-template-columns: 400px auto;
  grid-row-gap: 10px;
`;

const TopicsPanel = (props) => {
  const { loading, error, data } = useQuery(QUERY, {
    client: Network.default_client.handle,
  });

  if (error) {
    return `Query Error! ${error.message}`;
  }

  if (loading) {
    return null;
  }

  const topics = data.masterInfo.topics;

  return (
    <>
      <TopicsPanelStyled>
        <div>Name</div>
        <div>Data Type</div>
      </TopicsPanelStyled>
      <div
        style={{
          marginTop: "5px",
          marginBottom: "15px",
          borderBottom: "1px solid silver",
        }}
      />
      <TopicsPanelStyled>
        {topics.map((topic, idx) => {
          return (
            <React.Fragment key={idx}>
              <div>{topic.name}</div>
              {
                topic.datatypes.map((datatype, idx) => {
                  return <div key={idx}>{datatype}</div>;
                })
              }
            </React.Fragment>
          );
        })}
      </TopicsPanelStyled>
    </>
  );
};

export default TopicsPanel;
