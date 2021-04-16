#include "init.h"
#include "rclcpp/rclcpp.hpp"

void ros2_init() {
  int argc = 0;
  char **argv = NULL;
  rclcpp::init(argc, argv);
}

void ros2_shutdown() {
  rclcpp::shutdown();
}

bool ros2_sok() {
  return rclcpp::ok();
}