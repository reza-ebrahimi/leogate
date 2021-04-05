import React, { useState } from "react";
import { Switch, Route, Link, NavLink } from "react-router-dom";
import styled from "styled-components";
import { Button } from "@material-ui/core";
import InboxIcon from "@material-ui/icons/Inbox";

const navigationItems = [
  {
    id: 1,
    label: "ROS 1",
    url: "/",
  },
  {
    id: 2,
    label: "ROS 2",
    url: "/ros2",
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

  return (
    <StyledNavigationBar>
      <StyledNavigationLogo />
      {navigationItems.map((navigation) => {
        return (
          <React.Fragment key={navigation.id}>
            <NavigationItem
              id={navigation.id}
              label={navigation.label}
              to={navigation.url}
              resetSelection={resetSelection === navigation.id}
              resetHandler={resetHandler}
            />
          </React.Fragment>
        );
      })}
    </StyledNavigationBar>
  );
};

export default NavigationBar;
