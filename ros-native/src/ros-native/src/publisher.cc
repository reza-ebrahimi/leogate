#include "publisher.h"
#include <ros/ros.h>

publisher *publisher_create() {
  return reinterpret_cast<publisher *>(new ros::Publisher());
}

void publisher_destroy(publisher *pub) {
  delete reinterpret_cast<ros::Publisher *>(pub);
}

void publisher_shutdown(publisher *pub) {
  reinterpret_cast<ros::Publisher *>(pub)->shutdown();
}

const char *publisher_getTopic(publisher *pub) {
  return reinterpret_cast<ros::Publisher *>(pub)->getTopic().c_str();
}

uint32_t publisher_getNumSubscribers(publisher *pub) {
  return reinterpret_cast<ros::Publisher *>(pub)->getNumSubscribers();
}

bool publisher_isLatched(publisher *pub) {
  return reinterpret_cast<ros::Publisher *>(pub)->isLatched();
}

void publisher_publish(publisher *pub, const char* message) {
  ros::Publisher p = *reinterpret_cast<ros::Publisher *>(pub);
  // TODO for publish
}