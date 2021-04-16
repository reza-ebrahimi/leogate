#ifndef ROS2_CALLBACK_H
#define ROS2_CALLBACK_H

#include <stddef.h>

typedef struct _binary_payload {
  const unsigned char *payload;
  size_t size;
} binary_payload;

typedef struct _json_payload {
  const char *payload;
  size_t size;
} json_payload;

typedef void (*callback)(
  const char *topic,
  const json_payload *payload,
  const binary_payload *bin_payload);

#endif //ROS2_CALLBACK_H
