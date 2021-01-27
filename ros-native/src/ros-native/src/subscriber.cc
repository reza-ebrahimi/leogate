#include "subscriber.h"
#include <ros/ros.h>

subscriber *subscriber_create() {
  return reinterpret_cast<subscriber *>(new ros::Subscriber());
}

void subscriber_destroy(subscriber *sub) {
  delete reinterpret_cast<ros::Subscriber *>(sub);
}

void subscriber_shutdown(subscriber *sub) {
  reinterpret_cast<ros::Subscriber *>(sub)->shutdown();
}

const char *subscriber_getTopic(subscriber *sub) {
  return reinterpret_cast<ros::Subscriber *>(sub)->getTopic().c_str();
}

uint32_t subscriber_getNumPublishers(subscriber *sub) {
  return reinterpret_cast<ros::Subscriber *>(sub)->getNumPublishers();
}