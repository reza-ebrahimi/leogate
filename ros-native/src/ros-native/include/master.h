#ifndef ROS_ONE_MASTER_H_
#define ROS_ONE_MASTER_H_

/*
 * PATH: /opt/ros/melodic/include/ros/master.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>
#include <stdbool.h>

#include "common.h"

/**
 * \brief Contains functions which allow you to query information about the master
 */

/** @brief Get the hostname where the master runs.
 *
 * @return The master's hostname, as a string
 */
ROSCPP_DECL const unsigned char* ros_master_getHost(int32_t *len);

/** @brief Get the port where the master runs.
 *
 * @return The master's port.
 */
ROSCPP_DECL uint32_t ros_master_getPort();

/**
 * \brief Get the full URI to the master (eg. http://host:port/)
 */
ROSCPP_DECL const unsigned char* ros_master_getURI(int32_t *len);

/** @brief Check whether the master is up
 *
 * This method tries to contact the master.  You can call it any time
 * after ros::init has been called.  The intended usage is to check
 * whether the master is up before trying to make other requests
 * (subscriptions, advertisements, etc.).
 *
 * @returns true if the master is available, false otherwise.
 */
ROSCPP_DECL bool ros_master_check();

/** @brief Get the list of topics that are being published by all nodes.
 *
 * This method communicates with the master to retrieve the list of all
 * currently advertised topics.
 *
 * @param topics A place to store the resulting list.  Each item in the
 * list is a pair <string topic, string type>.  The type is represented
 * in the format "package_name/MessageName", and is also retrievable
 * through message.__getDataType() or MessageName::__s_getDataType().
 *
 * @return true on success, false otherwise (topics not filled in)
 */
ROSCPP_DECL bool ros_master_getTopics(unsigned char** topics, int32_t *len);

/**
 * \brief Retreives the currently-known list of nodes from the master
 */
ROSCPP_DECL bool ros_master_getNodes(unsigned char** nodes, int32_t *len);

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_MASTER_H_