import React from "react";
import styled from "styled-components";

const PanelStyled = styled.div`
  padding: 10px;
`;

const PanelHeader = styled.div`
  padding: 5px 0;
  margin-bottom: 10px;
  border-bottom: 1px solid silver;
`;

const Panel = (props) => {
  return (
    <PanelStyled>
      <PanelHeader>{props.title}</PanelHeader>
      <div>{props.component}</div>
    </PanelStyled>
  );
};

export default Panel;
