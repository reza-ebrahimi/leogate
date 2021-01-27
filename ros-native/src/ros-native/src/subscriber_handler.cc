#include "subscriber_handler.h"

#include <ros/ros.h>

#include <std_msgs/Int8.h>
#include <std_msgs/Int16.h>
#include <std_msgs/Int32.h>
#include <std_msgs/Int64.h>

#include <json.hpp>
using json = nlohmann::json;

class callback_impl {
  callback p_cb;
  const void *phantom_data;
  
 public:
  callback_impl(const void *data, const callback &cb) : p_cb(cb), phantom_data(data) {}

  __always_inline void call_it(const json &payload) const {
    p_cb(phantom_data, payload.dump(2).c_str());
  }

  void callback_handler(const std_msgs::Int8::ConstPtr &msg) {
    ROS_INFO("[std_msgs::Int8] Message Received");

    json payload = {
        { "header", {{ "dt", "std_msgs::Int8" }}},
        { "data", static_cast<int8_t>(msg->data) }
    };

    call_it(payload);
  }

  void callback_handler(const std_msgs::Int16::ConstPtr &msg) {
    ROS_INFO("[std_msgs::Int16] Message Received");

    json payload = {
        { "header", {{ "dt", "std_msgs::Int16" }}},
        { "data", static_cast<int16_t>(msg->data) }
    };

    call_it(payload);
  }

  void callback_handler(const std_msgs::Int32::ConstPtr &msg) {
    ROS_INFO("[std_msgs::Int32] Message Received");

    json payload = {
        { "header", {{ "dt", "std_msgs::Int32" }}},
        { "data", static_cast<int32_t>(msg->data) }
    };

    call_it(payload);
  }

  void callback_handler(const std_msgs::Int64::ConstPtr &msg) {
    ROS_INFO("[std_msgs::Int64] Message Received");

    json payload = {
        { "header", {{ "dt", "std_msgs::Int64" }}},
        { "data", static_cast<int64_t>(msg->data) }
    };

    call_it(payload);
  }
};

subscriber_handler::subscriber_handler() {
}

subscriber_handler::~subscriber_handler() {
  callback_map.clear();
}

void subscriber_handler::subscribe(void *nh, void *subscriber, const std::string &topic, 
                                  const std::string &type, uint32_t queue_size, const void *phantom_data, 
                                  const callback &cb) {
  callback_map[topic] = std::move(std::make_unique<callback_impl>(phantom_data, cb));
  callback_impl *p_cb = callback_map[topic].get();

  ros::NodeHandle *nHandle = reinterpret_cast<ros::NodeHandle *>(nh);
  ros::Subscriber *sub = reinterpret_cast<ros::Subscriber *>(subscriber);

  if (type == "std_msgs::Int8") {
    *sub = std::move(nHandle->subscribe<std_msgs::Int8>(std::string(topic), queue_size, &callback_impl::callback_handler, p_cb));
  }
  if (type == "std_msgs::Int16") {
    *sub = std::move(nHandle->subscribe<std_msgs::Int16>(std::string(topic), queue_size, &callback_impl::callback_handler, p_cb));
  }
  if (type == "std_msgs::Int32") {
    *sub = std::move(nHandle->subscribe<std_msgs::Int32>(std::string(topic), queue_size, &callback_impl::callback_handler, p_cb));
  }
  if (type == "std_msgs::Int64") {
    *sub = std::move(nHandle->subscribe<std_msgs::Int64>(std::string(topic), queue_size, &callback_impl::callback_handler, p_cb));
  }
}
