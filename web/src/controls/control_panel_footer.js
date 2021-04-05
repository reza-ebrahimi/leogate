import React from "react";
import styled from "styled-components";

const ControlPanelFooterStyled = styled.div`
  grid-area: footer;
  display: flex;
  align-items: center;
  background: #363636;
`;

const ControlPanelFooter = (props) => {
  return <ControlPanelFooterStyled>{props.children}</ControlPanelFooterStyled>;
};

export default ControlPanelFooter;
