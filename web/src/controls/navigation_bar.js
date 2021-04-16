import React, { useState } from "react";
import { NavLink } from "react-router-dom";
import styled from "styled-components";
import { Button } from "@material-ui/core";
import InboxIcon from "@material-ui/icons/Inbox";
import Network from "../network_interface";

const navigationItems = [
  {
    label: "Home",
    url: "/",
  },
  {
    label: "System Status",
    url: "/panels/system_status",
  },
  {
    label: "Master Status",
    url: "/panels/master_status",
  },
  {
    label: "Nodes",
    url: "/panels/nodes",
  },
  {
    label: "Topics",
    url: "/panels/topics",
  },
];

const StyledNavigationLogo = styled.div`
  height: 100px;
  background: #800080;
  margin: 0 9px;
`;

const StyledNavigationBar = styled.div`
  width: 100%;
  height: 100%;
`;

const StyledNavigationItem = styled.div`
  height: 50px;
  background: #ededed;

  .sidbar {
    width: 100%;
    height: 5px;
    background: red; //#00a8c9
  }

  .icon Button {
    width: 100%;
    height: calc(100% - 5px);
    text-align: center;
    vertical-align: bottom;
    line-height: 70px;
    border-radius: 0;
  }
`;

const NavigationItem = (props) => {
  return (
    <StyledNavigationItem>
      <div className="icon">
        <NavLink to={props.to}>
          <Button
            onClick={() => {
              props.resetHandler(props.id);
            }}
          >
            <InboxIcon fontSize="large" />
          </Button>
        </NavLink>
      </div>
      {props.resetSelection && <div className="sidbar" />}
    </StyledNavigationItem>
  );
};

const NavigationBar = () => {
  const [resetSelection, setResetSelection] = useState(false);
  const resetHandler = (id) => {
    setResetSelection(id);
  };

  if (Network.default_client === null || Network.default_client === undefined) {
    return (
      <StyledNavigationBar>
        <StyledNavigationLogo />
      </StyledNavigationBar>
    );
  }

  return (
    <StyledNavigationBar>
      <StyledNavigationLogo />
      {navigationItems.map((navigation, idx) => {
        return (
          <NavigationItem
            key={idx}
            id={idx}
            label={navigation.label}
            to={navigation.url}
            resetSelection={resetSelection === idx}
            resetHandler={resetHandler}
          />
        );
      })}
    </StyledNavigationBar>
  );
};

export default NavigationBar;
