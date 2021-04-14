import React from "react";
import styled from "styled-components";

import NavigationBar from "../controls/navigation_bar";

const NavigationLayoutStyled = styled.div`
  background: #ededed;
`;

const NavigationLayout = (props) => (
  <NavigationLayoutStyled>
    <NavigationBar />
  </NavigationLayoutStyled>
);

export default NavigationLayout;
