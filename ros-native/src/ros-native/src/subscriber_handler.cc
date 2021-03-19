#include "subscriber_handler.h"

#include <std_msgs/Int8.h>
#include <std_msgs/Int16.h>
#include <std_msgs/Int32.h>
#include <std_msgs/Int64.h>
#include <sensor_msgs/PointField.h>
#include <sensor_msgs/PointCloud2.h>

#include <chrono>

#include <json.hpp>
using json = nlohmann::json;

namespace sensor_msgs {
void to_json(json &j, const sensor_msgs::PointField &p) {
  j = json{{"name", p.name}, {"offset", p.offset}, {"datatype", p.datatype}, {"count", p.count}};
}

void from_json(const json &j, sensor_msgs::PointField &p) {
  p = j.get<sensor_msgs::PointField>();
}
}

static const std::string base64_chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    "abcdefghijklmnopqrstuvwxyz"
    "0123456789+/";
typedef unsigned char BYTE;

std::string base64_encode(BYTE const *buf, unsigned int bufLen) {
  std::string ret;
  int i = 0;
  int j = 0;
  BYTE char_array_3[3];
  BYTE char_array_4[4];

  while (bufLen--) {
    char_array_3[i++] = *(buf++);
    if (i == 3) {
      char_array_4[0] = (char_array_3[0] & 0xfc) >> 2;
      char_array_4[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xf0) >> 4);
      char_array_4[2] = ((char_array_3[1] & 0x0f) << 2) + ((char_array_3[2] & 0xc0) >> 6);
      char_array_4[3] = char_array_3[2] & 0x3f;

      for (i = 0; (i < 4); i++)
        ret += base64_chars[char_array_4[i]];
      i = 0;
    }
  }

  if (i) {
    for (j = i; j < 3; j++)
      char_array_3[j] = '\0';

    char_array_4[0] = (char_array_3[0] & 0xfc) >> 2;
    char_array_4[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xf0) >> 4);
    char_array_4[2] = ((char_array_3[1] & 0x0f) << 2) + ((char_array_3[2] & 0xc0) >> 6);
    char_array_4[3] = char_array_3[2] & 0x3f;

    for (j = 0; (j < i + 1); j++)
      ret += base64_chars[char_array_4[j]];

    while ((i++ < 3))
      ret += '=';
  }

  return ret;
}

class callback_impl {
  callback p_cb;
  const void *phantom_data;

 public:
  callback_impl(const void *data, const callback &cb) : p_cb(cb), phantom_data(data) {}

  __always_inline void call_it(json_payload *payload, binary_payload *bin_payload = nullptr) const {
    p_cb(phantom_data, payload, bin_payload);
  }

  void callback_handler(const std_msgs::Int8::ConstPtr &msg) {
    ROS_INFO("[std_msgs/Int8] Message Received");

    std::string json_str = json{
        {"msg_type", "std_msgs/Int8"},
        {"data", static_cast<int8_t>(msg->data)}
    }.dump(2);

    json_payload payload = json_payload{
        .payload =  json_str.c_str(),
        .size =  json_str.length()
    };

    call_it(&payload);
  }

  void callback_handler(const std_msgs::Int16::ConstPtr &msg) {
    ROS_INFO("[std_msgs/Int16] Message Received");

    std::string json_str = json{
        {"msg_type", "std_msgs/Int16"},
        {"data", static_cast<int16_t>(msg->data)}
    }.dump(2);

    json_payload payload = json_payload{
        .payload =  json_str.c_str(),
        .size =  json_str.length()
    };

    call_it(&payload);
  }

  void callback_handler(const std_msgs::Int32::ConstPtr &msg) {
    ROS_INFO("[std_msgs/Int32] Message Received");

    std::string json_str = json{
        {"msg_type", "std_msgs/Int32"},
        {"data", static_cast<int32_t>(msg->data)}
    }.dump(2);

    json_payload payload = json_payload{
        .payload =  json_str.c_str(),
        .size =  json_str.length()
    };

    call_it(&payload);
  }

  void callback_handler(const std_msgs::Int64::ConstPtr &msg) {
    ROS_INFO("[std_msgs/Int64] Message Received");

    std::string json_str = json{
        {"msg_type", "std_msgs/Int64"},
        {"data", static_cast<int64_t>(msg->data)}
    }.dump(2);

    json_payload payload = json_payload{
        .payload =  json_str.c_str(),
        .size =  json_str.length()
    };

    call_it(&payload);
  }

  void callback_handler(const sensor_msgs::PointCloud2::ConstPtr &msg) {
    ROS_INFO("[sensor_msgs/PointCloud2] Message Received");

    std::string json_str = json{
        {"msg_type", "sensor_msgs/PointCloud2"},
        {"header", {
            {"seq", msg->header.seq},
            {"frame_id", msg->header.frame_id}
        }},
        {"height", msg->height},
        {"width", msg->width},
        {"fields", msg->fields},
        {"is_bigendian", static_cast<bool>(msg->is_bigendian)},
        {"point_step", msg->point_step},
        {"row_step", msg->row_step},
        //{"data", encodedData},
        {"is_dense", static_cast<bool>(msg->is_dense)}
    }.dump(2);

    //auto start = std::chrono::steady_clock::now();

    json_payload payload = json_payload{
        .payload =  json_str.c_str(),
        .size =  json_str.length()
    };

    unsigned char abc[sizeof(size_t) + json_str.length() + msg->data.size()];
    std::memcpy(&abc[0], &payload.size, sizeof(size_t));
    std::memcpy(&abc[0] + sizeof(size_t), json_str.c_str(),
                json_str.length() * sizeof(uint8_t));
    std::memcpy(&abc[0] + sizeof(size_t) + (json_str.length() * sizeof(uint8_t)),
                msg->data.data(), msg->data.size() * sizeof(uint8_t));

    binary_payload bin_payload = binary_payload{
        .payload =  &abc[0],
        .size =  sizeof(json_str.length()) + json_str.length() + msg->data.size()
    };

    call_it(&payload, &bin_payload);

    //auto end = std::chrono::steady_clock::now();
    //auto diff = end - start;
    //std::cout << std::chrono::duration<double, std::milli>(diff).count() << " ms" << std::endl;
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

ros::Subscriber subscriber_handler::subscribe(void *nh, const std::string &topic, const std::string &type,
                                              uint32_t queue_size, const void *phantom_data, const callback &cb) {
  callback_map[topic] = std::move(std::make_unique<callback_impl>(phantom_data, cb));
  callback_impl *cb_obj = callback_map[topic].get();

  ros::NodeHandle *nHandle = reinterpret_cast<ros::NodeHandle *>(nh);
  ros::Subscriber sub;

  // std_msgs
  if (type == "std_msgs/Int8") {
    sub = std::move(nHandle->subscribe<std_msgs::Int8>(std::string(topic), queue_size, &callback_impl::callback_handler, cb_obj));
  }
  if (type == "std_msgs/Int16") {
    sub = std::move(nHandle->subscribe<std_msgs::Int16>(std::string(topic), queue_size, &callback_impl::callback_handler, cb_obj));
  }
  if (type == "std_msgs/Int32") {
    sub = std::move(nHandle->subscribe<std_msgs::Int32>(std::string(topic), queue_size, &callback_impl::callback_handler, cb_obj));
  }
  if (type == "std_msgs/Int64") {
    sub = std::move(nHandle->subscribe<std_msgs::Int64>(std::string(topic), queue_size, &callback_impl::callback_handler, cb_obj));
  }

  // sensor_msgs
  if (type == "sensor_msgs/PointCloud2") {
    sub = std::move(nHandle->subscribe<sensor_msgs::PointCloud2>(std::string(topic), queue_size, &callback_impl::callback_handler, cb_obj));
  }

  return sub;
}
