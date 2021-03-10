#ifndef ROS_NATIVE_INCLUDE_SUBSCRIBER_HANDLER_H_
#define ROS_NATIVE_INCLUDE_SUBSCRIBER_HANDLER_H_

#include <map>
#include <string>
#include <memory>

#include "callback.h"

class callback_impl;

class subscriber_handler {
 public:
  subscriber_handler();
  ~subscriber_handler();

  void subscribe(
      void *nh,
      void *subscriber,
      const std::string &topic,
      const std::string &type,
      uint32_t queue_size,
      const void *phantom_data,
      const callback &cb);

  void create_subscriber(
      void *nh,
      void *subscriber,
      const std::string &topic,
      const std::string &type,
      uint32_t queue_size);

 private:
  std::map<std::string, std::unique_ptr<callback_impl>> callback_map;
};

#endif //ROS_NATIVE_INCLUDE_SUBSCRIBER_HANDLER_H_
