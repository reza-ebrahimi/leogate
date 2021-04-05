import React from "react";
import styled from "styled-components";
import { gql, useSubscription } from "@apollo/client";

import { RosTime, WallTime } from "./ros_time";

const TIME_ECHO = gql`
  subscription {
    timeEcho {
      timeNowSec
      wallNowSec
    }
  }
`;

const StyledStatus = styled.div`
  grid-area: status;
  display: grid;
  grid-template-columns: 300px 300px;
  color: white;
  margin-left: 10px;
`;

const StatusPanel = (time) => {
  const { data, loading } = useSubscription(TIME_ECHO);

  if (loading) {
    return <StyledStatus></StyledStatus>;
  }

  return (
    <StyledStatus>
      <RosTime time={data.timeEcho.timeNowSec} />
      <WallTime time={data.timeEcho.wallNowSec} />
    </StyledStatus>
  );
};

export default StatusPanel;
