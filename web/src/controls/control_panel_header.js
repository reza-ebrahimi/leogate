import React from "react";
import styled from "styled-components";

const ControlPanelHeaderStyle = styled.div`
  grid-area: header;
  display: flex;
  align-items: center;
  background: #363636;
`;

const ControlPanelHeader = (props) => {
  return <ControlPanelHeaderStyle>{props.children}</ControlPanelHeaderStyle>;
};

export default ControlPanelHeader;
