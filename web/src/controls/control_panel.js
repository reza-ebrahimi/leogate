import React from "react";
import styled from "styled-components";

import ControlPanelHeader from "./control_panel_header";
import ControlPanelBody from "./control_panel_body";
import ControlPanelFooter from "./control_panel_footer";

import MasterInfoPanel from "./master_info_panel";
import StatusPanel from "./status_panel";

const ControlPanelContainer = styled.div`
  grid-area: panel;
  display: grid;
  grid-template-rows: 30px auto 30px;
  grid-template-areas:
    "header"
    "panel"
    "footer";
`;

const ControlPanel = ({ type }) => {
  if (type === "ros1") {
    return (
      <ControlPanelContainer>
        <ControlPanelHeader>ROS Control Panel</ControlPanelHeader>
        <ControlPanelBody>
          <MasterInfoPanel />
        </ControlPanelBody>
        <ControlPanelFooter>
          <StatusPanel />
        </ControlPanelFooter>
      </ControlPanelContainer>
    );
  }

  if (type === "ros2") {
    return (
      <ControlPanelContainer>
        <ControlPanelHeader>ROS2 Control Panel</ControlPanelHeader>
        <ControlPanelBody>Under Development</ControlPanelBody>
        <ControlPanelFooter>
          <StatusPanel />
        </ControlPanelFooter>
      </ControlPanelContainer>
    );
  }

  return <div>Type `{type}` is not defined</div>;
};

export default ControlPanel;
