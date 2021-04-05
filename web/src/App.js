import React from "react";
import styled from "styled-components";

import { Switch, Route } from "react-router-dom";

import StreamViewer from "./controls/stream_viewer";
import NavigationBar from "./controls/navigation_bar";
import ControlPanel from "./controls/control_panel";

const GridContainer = styled.div`
  width: 100%;
  height: 100vh;
  display: grid;
  grid-template-rows: auto 400px;
  grid-template-columns: 70px auto;
  grid-template-areas:
    "nav main"
    "nav panel";
  color: white;
  font-family: Arial, Helvetica, sans-serif;
  overflow: hidden;

  -webkit-touch-callout: none; /* iOS Safari */
  -webkit-user-select: none; /* Safari */
  -khtml-user-select: none; /* Konqueror HTML */
  -moz-user-select: none; /* Old versions of Firefox */
  -ms-user-select: none; /* Internet Explorer/Edge */
  user-select: none; /* Non-prefixed version, currently supported by Chrome, Edge, Opera and Firefox */
`;

const NavContainer = styled.div`
  grid-area: nav;
  background: #ededed;
`;

const MainContainer = styled.div`
  grid-area: main;
`;

const App = () => {
  return (
    <GridContainer>
      <NavContainer>
        <NavigationBar />
      </NavContainer>
      <MainContainer>
        <StreamViewer />
      </MainContainer>
      <Switch>
        <Route path="/">
          <ControlPanel type="ros1" />
        </Route>
        <Route path="/ros2">
          <ControlPanel type="ros2" />
        </Route>
      </Switch>
    </GridContainer>
  );
};

export default App;
