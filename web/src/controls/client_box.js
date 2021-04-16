import React from "react";
import styled from "styled-components";
import Network from "../network_interface";

const ClientSelectorStyled = styled.div`
  width: 400px;
  margin-top: 10px;
  margin-left: 10px;
  background: transparent;
  border: 1px solid silver;
  padding: 5px;
`;

const ClientBox = (props) => {
  return (
    <ClientSelectorStyled>{Network.default_client.name}</ClientSelectorStyled>
  );
};

export default ClientBox;
