#include "master.h"
#include <ros/ros.h>
#include <string>

#include <json.hpp>
using json = nlohmann::json;

const char *ros_master_getHost() {
  return ros::master::getHost().c_str();
}

uint32_t ros_master_getPort() {
  return ros::master::getPort();
}

const char *ros_master_getURI() {
  return ros::master::getURI().c_str();
}

bool ros_master_check() {
  return ros::master::check();
}

const char *ros_master_getTopics() {
  ros::master::V_TopicInfo _topics;
  auto json_str = json::array();

  if (ros::master::getTopics(_topics)) {
    for (size_t i = 0; i < _topics.size(); ++i) {
      ros::master::TopicInfo ti = _topics[i];
      json_str.push_back(json{
          {"name", ti.name},
          {"datatype", ti.datatype}
      });
    }
  }

  std::string j = json_str.dump();
  char *buffer = new char[j.length()];
  std::memcpy(&buffer[0], j.c_str(), j.length());
  buffer[j.length()] = '\0';

  return &buffer[0];
}

const char *ros_master_getNodes() {
  ros::V_string _nodes;
  auto json_str = json::array();

  if (ros::master::getNodes(_nodes)) {
    for (size_t i = 0; i < _nodes.size(); ++i) {
      json_str.push_back(_nodes[i]);
    }
  }

  std::string j = json_str.dump();
  char *buffer = new char[j.length()];
  std::memcpy(&buffer[0], j.c_str(), j.length());
  buffer[j.length()] = '\0';

  return &buffer[0];
}