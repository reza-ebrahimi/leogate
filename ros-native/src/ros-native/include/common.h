#ifndef ROS_ONE_COMMON_H_
#define ROS_ONE_COMMON_H_

/*
 * PATH: /opt/ros/melodic/include/ros/common.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include "macros.h"

#define ROS_VERSION_MAJOR 1
#define ROS_VERSION_MINOR 14
#define ROS_VERSION_PATCH 6
#define ROS_VERSION_COMBINED(major, minor, patch) (((major) << 20) | ((minor) << 10) | (patch))
#define ROS_VERSION ROS_VERSION_COMBINED(ROS_VERSION_MAJOR, ROS_VERSION_MINOR, ROS_VERSION_PATCH)

#define ROS_VERSION_GE(major1, minor1, patch1, major2, minor2, patch2) (ROS_VERSION_COMBINED(major1, minor1, patch1) >= ROS_VERSION_COMBINED(major2, minor2, patch2))
#define ROS_VERSION_MINIMUM(major, minor, patch) ROS_VERSION_GE(ROS_VERSION_MAJOR, ROS_VERSION_MINOR, ROS_VERSION_PATCH, major, minor, patch)

// Import/export for windows dll's and visibility for gcc shared libraries.

#ifdef ROS_BUILD_SHARED_LIBS // ros is being built around shared libraries
#ifdef roscpp_EXPORTS // we are building a shared lib/dll
#define ROSCPP_DECL ROS_HELPER_EXPORT
#else // we are using shared lib/dll
#define ROSCPP_DECL ROS_HELPER_IMPORT
#endif
#else // ros is being built around static libraries
#define ROSCPP_DECL
#endif

void ros_disableAllSignalsInThisThread();

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_COMMON_H_