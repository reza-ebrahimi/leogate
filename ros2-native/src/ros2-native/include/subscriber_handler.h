#ifndef ROS2_SUBSCRIBER_HANDLER_H
#define ROS2_SUBSCRIBER_HANDLER_H

#include <map>
#include <string>
#include <memory>

#include "callback.h"

class point_cloud2_callback;

class subscriber_handler {
public:
  static subscriber_handler &instance();

  subscriber_handler(subscriber_handler const &) = delete;
  void operator=(subscriber_handler const &) = delete;

  void subscribe(
    void *n,
    const std::string &topic,
    const std::string &type,
    const callback &cb);

private:
  subscriber_handler();
  ~subscriber_handler();

  std::map<std::string, std::unique_ptr<point_cloud2_callback>> callback_map;
};

#endif //ROS2_SUBSCRIBER_HANDLER_H
