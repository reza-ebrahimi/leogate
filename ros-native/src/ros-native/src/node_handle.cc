#include "node_handle.h"
#include "subscriber_handler.h"

#include <ros/ros.h>
#include <ros/callback_queue.h>
#include <string>

node_handle *node_handle_create(const char *nh) {
  return reinterpret_cast<node_handle *>(new ros::NodeHandle(std::string(nh)));
}

void node_handle_destroy(node_handle *nh) {
  delete reinterpret_cast<ros::NodeHandle *>(nh);
}

void node_handle_setCallbackQueue(node_handle *nh, callback_queue *queue) {
  reinterpret_cast<ros::NodeHandle *>(nh)->setCallbackQueue(reinterpret_cast<ros::CallbackQueue *>(nh));
}

callback_queue *node_handle_getCallbackQueue(node_handle *nh) {
  ros::CallbackQueueInterface *cq = reinterpret_cast<ros::NodeHandle *>(nh)->getCallbackQueue();
  return reinterpret_cast<callback_queue *>(cq);
}

const char *node_handle_getNamespace(node_handle *nh) {
  return reinterpret_cast<ros::NodeHandle *>(nh)->getNamespace().c_str();
}

const char *node_handle_getUnresolvedNamespace(node_handle *nh) {
  return reinterpret_cast<ros::NodeHandle *>(nh)->getUnresolvedNamespace().c_str();
}

const char *node_handle_resolveName(node_handle *nh, const char *name, bool remap) {
  return reinterpret_cast<ros::NodeHandle *>(nh)->resolveName(std::string(name), remap).c_str();
}

subscriber *node_handle_subscribe(node_handle *nh, const char *topic, const char *type, uint32_t queue_size, const void *phantom_data, callback cb) {
  ros::Subscriber sub;

  static subscriber_handler handler;
  handler.subscribe(nh, &sub, std::string(topic), std::string(type), queue_size, phantom_data, cb);

  return reinterpret_cast<subscriber *>(new ros::Subscriber(std::move(sub)));
}

publisher *node_handle_advertise(node_handle *nh, const char *topic, uint32_t queue_size, bool latch) {
  //ros::Publisher pub = reinterpret_cast<ros::NodeHandle *>(nh)->advertise<std_msgs::Int8>(std::string(topic), queue_size, latch);
  //return reinterpret_cast<publisher *>(new ros::Publisher(std::move(pub)));
  return 0;
}
