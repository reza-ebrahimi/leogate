#ifndef ROS_NATIVE_INCLUDE_CALLBACK_H_
#define ROS_NATIVE_INCLUDE_CALLBACK_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*callback)(const void *phantom_data, const char *);

#ifdef __cplusplus
}
#endif

#endif //ROS_NATIVE_INCLUDE_CALLBACK_H_
