import React from "react";

const RosTime = ({ time }) => {
  return (
    <span>
      <span>ROS Time </span>
      <span>{time}</span>
    </span>
  );
};

const WallTime = ({ time }) => {
  return (
    <span>
      <span>Wall Time </span>
      <span>{time}</span>
    </span>
  );
};

export { RosTime, WallTime };
