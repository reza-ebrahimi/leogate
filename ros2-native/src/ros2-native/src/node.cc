#include "node.h"
#include "rclcpp/rclcpp.hpp"
#include "subscriber_handler.h"

#include <nlohmann/json.hpp>
using json = nlohmann::json;

node *node_create(const char *n) {
  return reinterpret_cast<node *>(new rclcpp::Node(n));
}

void node_destroy(node *n) {
  delete reinterpret_cast<rclcpp::Node *>(n);
}

const char *node_get_name(node *n) {
  return reinterpret_cast<rclcpp::Node *>(n)->get_name();
}

const char *node_get_namespace(node *n) {
  return reinterpret_cast<rclcpp::Node *>(n)->get_namespace();
}

const char *node_get_fully_qualified_name(node *n) {
  return reinterpret_cast<rclcpp::Node *>(n)->get_fully_qualified_name();
}

const char *node_get_node_names(node *n) {
  auto nodes = reinterpret_cast<rclcpp::Node *>(n)->get_node_names();
  auto json_str = json::array();

  for (size_t i = 0; i < nodes.size(); ++i) {
    json_str.push_back(nodes[i]);
  }

  std::string j = json_str.dump();
  char *buffer = new char[j.length()];
  std::memcpy(&buffer[0], j.c_str(), j.length());
  buffer[j.length()] = '\0';

  return &buffer[0];
}

const char *node_get_topic_names_and_types(node *n) {
  auto pair = reinterpret_cast<rclcpp::Node *>(n)->get_topic_names_and_types();
  auto json_str = json::array();

  for (auto p: pair) {
    auto j = json {
      {"name", p.first},
      {"datatypes", json::array()}
    };
    for (size_t i = 0; i < p.second.size(); ++i) {
      j["datatypes"].push_back(p.second[i]);
    }
    json_str.push_back(j);
  }

  std::string j = json_str.dump();
  char *buffer = new char[j.length()];
  std::memcpy(&buffer[0], j.c_str(), j.length());
  buffer[j.length()] = '\0';

  return &buffer[0];
}

const char *node_get_service_names_and_types(node *n) {
  auto pair = reinterpret_cast<rclcpp::Node *>(n)->get_service_names_and_types();
  auto json_str = json::array();

  for (auto p: pair) {
    auto j = json {
      {"name", p.first},
      {"types", json::array()}
    };
    for (size_t i = 0; i < p.second.size(); ++i) {
      j["types"].push_back(p.second[i]);
    }
    json_str.push_back(j);
  }

  std::string j = json_str.dump();
  char *buffer = new char[j.length()];
  std::memcpy(&buffer[0], j.c_str(), j.length());
  buffer[j.length()] = '\0';

  return &buffer[0];
}

const char *node_get_sub_namespace(node *n) {
  return reinterpret_cast<rclcpp::Node *>(n)->get_sub_namespace().c_str();
}

const char *node_get_effective_namespace(node *n) {
  return reinterpret_cast<rclcpp::Node *>(n)->get_effective_namespace().c_str();
}

double node_time_now_get_seconds(node *n) {
  return reinterpret_cast<rclcpp::Node *>(n)->now().seconds();
}

void node_subscribe(node *n, const char *topic, const char *type, callback cb) {
  subscriber_handler::instance().subscribe(n, std::string(topic), std::string(type), cb);
}