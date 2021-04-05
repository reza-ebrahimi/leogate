import React from "react";
import styled from "styled-components";

const ControlPanelBodyStyled = styled.div`
  grid-area: panel;
  background: #1c1c1c;
`;

const ControlPanelBody = (props) => {
  return <ControlPanelBodyStyled>{props.children}</ControlPanelBodyStyled>;
};

export default ControlPanelBody;
