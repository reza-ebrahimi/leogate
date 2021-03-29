#ifndef ROS_NATIVE_INCLUDE_CALLBACK_H_
#define ROS_NATIVE_INCLUDE_CALLBACK_H_

#ifdef __cplusplus
extern "C" {
#endif

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

#ifdef __cplusplus
}
#endif

#endif //ROS_NATIVE_INCLUDE_CALLBACK_H_
