#include <cstdio>
#include <iostream>
#include "init.h"
#include "node.h"
#include <nlohmann/json.hpp>
#include <rclcpp/rclcpp.hpp>
#include <rclcpp/utilities.hpp>

#include <sensor_msgs/msg/point_cloud2.hpp>

#include "subscriber_handler.h"

using json = nlohmann::json;

//void mycallback(
//  const char *topic,
//  const json_payload *payload,
//  const binary_payload *bin_payload) {
//  std::cout << "Message Received in Callback" << std::endl;
//};

int main(int argc, char **argv) {
  (void) argc;
  (void) argv;

  ros2_init();

  auto n = node_create("mynde");

  std::cout << "node_get_name => " << node_get_name(n) << std::endl;
  std::cout << "node_get_namespace => " << node_get_namespace(n) << std::endl;
  std::cout << "node_get_fully_qualified_name => " << node_get_fully_qualified_name(n) << std::endl;
  std::cout << "node_get_node_names => " << node_get_node_names(n) << std::endl;
  std::cout << "node_get_topic_names_and_types => " << node_get_topic_names_and_types(n) << std::endl;
  std::cout << "node_get_service_names_and_types => " << node_get_service_names_and_types(n) << std::endl;
  std::cout << "node_get_sub_namespace => " << node_get_sub_namespace(n) << std::endl;
  std::cout << "node_get_effective_namespace => " << node_get_effective_namespace(n) << std::endl;

  auto minimal_subscriber = std::make_shared<rclcpp::Node>("minimal_subscriber");
//  auto callback = [&](sensor_msgs::msg::PointCloud2::SharedPtr msg) {};
//  auto sub = minimal_subscriber->create_subscription<sensor_msgs::msg::PointCloud2>(
//    "udp_read",
//    10,
//    callback);

//  subscriber_handler::instance().subscribe(minimal_subscriber.get(),
//                                                       std::string("udp_read"),
//                                                       std::string("sensor_msgs/msg/PointCloud2"),
//                                                       mycallback);
  if (rclcpp::ok()) {
    std::cout << "hello world ros2-native packageee " << "True" << std::endl;
  } else {
    std::cout << "hello world ros2-native packageee " << "False" << std::endl;
  }
  rclcpp::WallRate loop_rate(1);
  while (rclcpp::ok()) {
    rclcpp::spin_some(minimal_subscriber);
    loop_rate.sleep();
  }
  node_destroy(n);
  ros2_shutdown();
  return 0;
}
