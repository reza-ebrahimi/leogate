#include "callback_queue.h"
#include <ros/ros.h>
#include <ros/callback_queue.h>
#include <string>

callback_queue *callback_queue_create(bool enabled) {
  return reinterpret_cast<callback_queue *>(new ros::CallbackQueue(enabled));
}

void callback_queue_destroy(callback_queue *cq) {
  delete reinterpret_cast<ros::CallbackQueue *>(cq);
}

CallOneResult callback_queue_callOne(callback_queue *cq) {
  auto cor = reinterpret_cast<ros::CallbackQueue *>(cq)->callOne();
  return static_cast<CallOneResult>(cor);
}

void callback_queue_callAvailable(callback_queue *cq) {
  reinterpret_cast<ros::CallbackQueue *>(cq)->callAvailable();
}

bool callback_queue_empty(callback_queue *cq) {
  return reinterpret_cast<ros::CallbackQueue *>(cq)->empty();
}

bool callback_queue_isEmpty(callback_queue *cq) {
  return reinterpret_cast<ros::CallbackQueue *>(cq)->isEmpty();
}

void callback_queue_clear(callback_queue *cq) {
  reinterpret_cast<ros::CallbackQueue *>(cq)->clear();
}

void callback_queue_enable(callback_queue *cq) {
  reinterpret_cast<ros::CallbackQueue *>(cq)->enable();
}

void callback_queue_disable(callback_queue *cq) {
  reinterpret_cast<ros::CallbackQueue *>(cq)->disable();
}

bool callback_queue_isEnabled(callback_queue *cq) {
  return reinterpret_cast<ros::CallbackQueue *>(cq)->isEnabled();
}