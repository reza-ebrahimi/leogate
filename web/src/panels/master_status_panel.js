import React, { useEffect } from "react";
import styled from "styled-components";
import { gql, useQuery } from "@apollo/client";
import Network from "../network_interface";

const QUERY = gql`
  query {
    masterInfo {
      aliveness
      host
      port
      uri
    }
    timeEcho {
      timeNowSec
      wallNowSec
    }
  }
`;

const MasterStatusPanelStyled = styled.div`
  display: grid;
  grid-template-columns: 100px auto;
  grid-row-gap: 13px;
`;

const QUERY_POLLL_INTERVAL_MS = 500;

const RosTime = ({ time }) => {
  return (
    <>
      <div>ROS Time </div>
      <div>{time.timeNowSec.toFixed(2)}</div>
      <div>Wall Time </div>
      <div>{time.wallNowSec.toFixed(2)}</div>
    </>
  );
};

const RosMaster = ({ status }) => {
  const aliveness = (value) => {
    if (value === undefined || value === null) {
      return "N/A";
    }
    
    return value ? "Running" : "Stopped";
  }

  const check_and_return = (value) => {
    if (value === undefined || value === null) {
      return "N/A";
    }
    
    return value;
  }

  return (
    <>
      <div>Aliveness</div>
      <div>{aliveness(status.aliveness)}</div>
      <div>Host</div>
      <div>{check_and_return(status.host)}</div>
      <div>Port</div>
      <div>{check_and_return(status.port)}</div>
      <div>URI</div>
      <div>{check_and_return(status.uri)}</div>
    </>
  );
};

const MasterStatusPanel = () => {
  const { loading, error, data, startPolling, stopPolling } = useQuery(QUERY, {
    client: Network.default_client.handle,
  });

  useEffect(() => {
    startPolling(QUERY_POLLL_INTERVAL_MS);
    return () => {
      stopPolling();
    };
  }, [startPolling, stopPolling]);

  if (error) {
    return `Query Error! ${error.message}`;
  }

  if (loading) {
    return null;
  }

  return (
    <MasterStatusPanelStyled>
      <RosMaster status={data.masterInfo} />
      <RosTime time={data.timeEcho} />
    </MasterStatusPanelStyled>
  );
};

export default MasterStatusPanel;
