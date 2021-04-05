#include "init.h"

#include <ros/ros.h>
#include <string>

void ros_init(const unsigned char *name, uint32_t options) {
  int argc = 0;
  char **argv = NULL;

  const std::string &n = std::string((const char *) name);
  ros::init(argc, argv, n, ros::init_options::NoSigintHandler);
}

bool ros_isInitialized() {
  return ros::isInitialized();
}

bool ros_isShuttingDown() {
  return ros::isShuttingDown();
}

void ros_spin() {
  ros::spin();
}

void ros_spinOnce() {
  ros::spinOnce();
}

void ros_waitForShutdown() {
  ros::waitForShutdown();
}

bool ros_ok() {
  return ros::ok();
}

void ros_shutdown() {
  ros::shutdown();
}

void ros_requestShutdown() {
  ros::requestShutdown();
}

void ros_start() {
  ros::start();
}

bool ros_isStarted() {
  return ros::isStarted();
}

callback_queue *ros_getGlobalCallbackQueue() {
  return reinterpret_cast<callback_queue *>(ros::getGlobalCallbackQueue());
}

const char *ros_getROSArg(int argc, const char *const *argv, const char *arg) {
  return ros::getROSArg(argc, argv, arg).c_str();
}

const char *ros_getDefaultMasterURI() {
  return ros::getDefaultMasterURI().c_str();
}