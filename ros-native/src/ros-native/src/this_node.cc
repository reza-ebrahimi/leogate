#include "this_node.h"

#include <ros/ros.h>

#include <json.hpp>
using json = nlohmann::json;

const char *this_node_getName() {
  return ros::this_node::getName().c_str();
}

const char *this_node_getNamespace() {
  return ros::this_node::getNamespace().c_str();
}

const char *this_node_getAdvertisedTopics() {
  ros::V_string topics;
  ros::this_node::getAdvertisedTopics(topics);

  auto json_str = json::array();
  for (size_t i = 0; i < topics.size(); ++i) {
    json_str.push_back(topics[i]);
  }

  if (topics.size() > 0) {
    std::string j = json_str.dump();
    char *buffer = new char[j.length()];
    std::memcpy(&buffer[0], j.c_str(), j.length());
    buffer[j.length()] = '\0';

    return &buffer[0];
  }

  return nullptr;
}

const char *this_node_getSubscribedTopics() {
  ros::V_string topics;
  ros::this_node::getSubscribedTopics(topics);

  auto json_str = json::array();
  for (size_t i = 0; i < topics.size(); ++i) {
    json_str.push_back(topics[i]);
  }

  if (topics.size() > 0) {
    std::string j = json_str.dump();
    char *buffer = new char[j.length()];
    std::memcpy(&buffer[0], j.c_str(), j.length());
    buffer[j.length()] = '\0';

    return &buffer[0];
  }

  return nullptr;
}