import React from "react";
import { Switch, Route } from "react-router-dom";
import styled from "styled-components";

import StreamViewer from "../controls/stream_viewer";
import ControlPanel from "../controls/control_panel";

const ControlPanelContainer = styled.div`
  height: 450px;
  background: purple;
  overflow-y: auto;
`;

const MainContainer = styled.div`
  display: grid;
  grid-template-rows: 1fr auto;
`;

const MainLayout = (props) => (
  <MainContainer>
    <StreamViewer />
    <Switch>
      <Route path="/panels">
        <ControlPanelContainer>
          <ControlPanel />
        </ControlPanelContainer>
      </Route>
    </Switch>
  </MainContainer>
);

export default MainLayout;
