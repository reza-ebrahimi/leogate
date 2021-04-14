import React from "react";
import styled, { createGlobalStyle, css } from "styled-components";

import NavigationLayout from "./layout/navigation";
import MainLayout from "./layout/main";

const ResetCss = css`
  ${import("./reset.css")}
`;

const GlobalStyle = createGlobalStyle`
  ${ResetCss}
`;

const GridContainer = styled.div`
  min-width: 100%;
  min-height: 100vh;

  display: grid;
  grid-template-columns: 70px 1fr;
`;

const App = () => (
  <>
    <GlobalStyle />
    <GridContainer>
      <NavigationLayout />
      <MainLayout />
    </GridContainer>
  </>
);

export default App;
