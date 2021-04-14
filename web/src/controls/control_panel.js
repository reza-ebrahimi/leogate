import React from "react";
import { Switch, Route } from "react-router-dom";

import styled from "styled-components";

import Panel from "./panel";

import SystemStatusPanel from "../panels/system_status_panel";
import MasterStatusPanel from "../panels/master_status_panel";
import NodesPanel from "../panels/nodes_panel";
import TopicsPanel from "../panels/topics_panel";

const ControlPanelContainer = styled.div`
  padding: 0 10px;
`;

const ControlPanel = () => {
  return (
    <ControlPanelContainer>
      <Switch>
        <Route path="/panels/system_status">
          <Panel title="System Status" component={<SystemStatusPanel />} />
        </Route>
        <Route path="/panels/master_status">
          <Panel title="Master Status" component={<MasterStatusPanel />} />
        </Route>
        <Route path="/panels/nodes">
          <Panel title="Nodes" component={<NodesPanel />} />
        </Route>
        <Route path="/panels/topics">
          <Panel title="Topics" component={<TopicsPanel />} />
        </Route>
      </Switch>
    </ControlPanelContainer>
  );
};

export default ControlPanel;
