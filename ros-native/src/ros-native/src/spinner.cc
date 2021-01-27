
#include "spinner.h"
#include <ros/ros.h>

single_threaded_spinner *single_threaded_spinner_create() {
  return reinterpret_cast<single_threaded_spinner *>(new ros::SingleThreadedSpinner());
}

void single_threaded_spinner_destroy(single_threaded_spinner *sp) {
  delete reinterpret_cast<ros::SingleThreadedSpinner *>(sp);
}

void single_threaded_spinner_spin(single_threaded_spinner *sp, callback_queue *cq) {
  reinterpret_cast<ros::SingleThreadedSpinner *>(sp)->spin(reinterpret_cast<ros::CallbackQueue *>(cq));
}

multi_threaded_spinner *multi_threaded_spinner_create(uint32_t thread_count) {
  return reinterpret_cast<multi_threaded_spinner *>(new ros::MultiThreadedSpinner(thread_count));
}

void multi_threaded_spinner_destroy(multi_threaded_spinner *sp) {
  delete reinterpret_cast<ros::MultiThreadedSpinner *>(sp);
}

void multi_threaded_spinner_spin(multi_threaded_spinner *sp, callback_queue *cq) {
  reinterpret_cast<ros::MultiThreadedSpinner *>(sp)->spin(reinterpret_cast<ros::CallbackQueue *>(cq));
}

async_spinner *async_spinner_create(uint32_t thread_count) {
  return reinterpret_cast<async_spinner *>(new ros::AsyncSpinner(thread_count));
}

async_spinner *async_spinner_create_with_queue(uint32_t thread_count, callback_queue *cq) {
  return reinterpret_cast<async_spinner *>(new ros::AsyncSpinner(thread_count, reinterpret_cast<ros::CallbackQueue *>(cq)));
}

void async_spinner_destroy(async_spinner *sp) {
  delete reinterpret_cast<ros::AsyncSpinner *>(sp);
}

void async_spinner_start(async_spinner *sp) {
  reinterpret_cast<ros::AsyncSpinner *>(sp)->start();
}

void async_spinner_stop(async_spinner *sp) {
  reinterpret_cast<ros::AsyncSpinner *>(sp)->stop();
}