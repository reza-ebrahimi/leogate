#include "subscriber_handler.h"

#include "rclcpp/rclcpp.hpp"

#include <sensor_msgs/msg/point_field.hpp>
#include <sensor_msgs/msg/point_cloud2.hpp>
#include <chrono>

#include <nlohmann/json.hpp>
using json = nlohmann::json;

namespace sensor_msgs {
namespace msg {
void to_json(json &j, const sensor_msgs::msg::PointField &p) {
  j = json{{"name", p.name}, {"offset", p.offset}, {"datatype", p.datatype}, {"count", p.count}};
}

void from_json(const json &j, sensor_msgs::msg::PointField &p) {
  j.at("name").get_to(p.name);
  j.at("offset").get_to(p.offset);
  j.at("datatype").get_to(p.datatype);
  j.at("count").get_to(p.count);
}
}
}

class point_cloud2_callback {
  callback p_cb;
  std::string topic;

  __always_inline void call_it(json_payload *payload, binary_payload *bin_payload = nullptr) const {
    p_cb(topic.c_str(), payload, bin_payload);
  }
public:
  point_cloud2_callback(const std::string &topic, const callback &cb) : p_cb(cb), topic(topic) {}
  rclcpp::Subscription<sensor_msgs::msg::PointCloud2>::SharedPtr subscription;

  void callback_handler(sensor_msgs::msg::PointCloud2::SharedPtr msg) {
    std::cout << "[sensor_msgs/msgs/PointCloud2] Message Received" << std::endl;

    auto json_str = json{
      {"msg_type", "sensor_msgs/msg/PointCloud2"},
      {"header", {
        {"seq", msg->header.stamp.sec},
        {"frame_id", msg->header.frame_id}
      }},
      {"height", msg->height},
      {"width", msg->width},
      {"fields", msg->fields},
      {"is_bigendian", static_cast<bool>(msg->is_bigendian)},
      {"point_step", msg->point_step},
      {"row_step", msg->row_step},
      {"is_dense", static_cast<bool>(msg->is_dense)}
    }.dump();

    json_payload payload = json_payload{
      .payload =  json_str.c_str(),
      .size =  json_str.length()
    };

    unsigned char buffer[sizeof(size_t) + json_str.length() + msg->data.size()];
    std::memcpy(&buffer[0], &payload.size, sizeof(size_t));
    std::memcpy(&buffer[0] + sizeof(size_t), json_str.c_str(),
                json_str.length() * sizeof(uint8_t));
    std::memcpy(&buffer[0] + sizeof(size_t) + (json_str.length() * sizeof(uint8_t)),
                msg->data.data(), msg->data.size() * sizeof(uint8_t));

    binary_payload bin_payload = binary_payload{
      .payload =  &buffer[0],
      .size =  sizeof(json_str.length()) + json_str.length() + msg->data.size()
    };

    call_it(&payload, &bin_payload);
  }
};

subscriber_handler::subscriber_handler() {
}

subscriber_handler::~subscriber_handler() {
  callback_map.clear();
}

subscriber_handler &subscriber_handler::instance() {
  static subscriber_handler _instance;
  return _instance;
}

void subscriber_handler::subscribe(void *n,
                                   const std::string &topic,
                                   const std::string &type,
                                   const callback &cb) {
  rclcpp::Node *node = reinterpret_cast<rclcpp::Node *>(n);

  // sensor_msgs
  if (type == "sensor_msgs/msg/PointCloud2") {
    callback_map[topic] = std::make_unique<point_cloud2_callback>(topic, cb);
    point_cloud2_callback *cb_obj = callback_map[topic].get();

    auto bind = std::bind(&point_cloud2_callback::callback_handler,
                          cb_obj,
                          std::placeholders::_1);
    cb_obj->subscription = node->create_subscription<sensor_msgs::msg::PointCloud2>(topic, 32, bind);
  }
}
