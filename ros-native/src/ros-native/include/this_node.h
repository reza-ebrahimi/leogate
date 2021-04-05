#ifndef ROS_ONE_THIS_NODE_H_
#define ROS_ONE_THIS_NODE_H_

/*
 * PATH: /opt/ros/melodic/include/ros/this_node.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include "common.h"

/**
 * \brief Returns the name of the current node.
 */
ROSCPP_DECL const char *this_node_getName();

/**
 * \brief Returns the namespace of the current node.
 */
ROSCPP_DECL const char *this_node_getNamespace();


/** @brief Get the list of topics advertised by this node
 *
 * @param[out] topics The advertised topics
 */
ROSCPP_DECL const char *this_node_getAdvertisedTopics();


/** @brief Get the list of topics subscribed to by this node
 *
 * @param[out] The subscribed topics
 */
ROSCPP_DECL const char *this_node_getSubscribedTopics();

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_THIS_NODE_H_
