#ifndef ROS_ONE_SUBSCRIBER_H_
#define ROS_ONE_SUBSCRIBER_H_

/*
 * PATH: /opt/ros/melodic/include/ros/subscriber.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>

/**
 * \brief Manages an subscription callback on a specific topic.
 *
 * A Subscriber should always be created through a call to NodeHandle::subscribe(), or copied from one
 * that was. Once all copies of a specific
 * Subscriber go out of scope, the subscription callback associated with that handle will stop
 * being called.  Once all Subscriber for a given topic go out of scope the topic will be unsubscribed.
 */
typedef struct subscriber subscriber;

subscriber *subscriber_create();

void subscriber_destroy(subscriber *sub);

/**
 * \brief Unsubscribe the callback associated with this Subscriber
 *
 * This method usually does not need to be explicitly called, as automatic shutdown happens when
 * all copies of this Subscriber go out of scope
 *
 * This method overrides the automatic reference counted unsubscribe, and immediately
 * unsubscribes the callback associated with this Subscriber
 */
void subscriber_shutdown(subscriber *sub);

const char *subscriber_getTopic(subscriber *sub);

/**
 * \brief Returns the number of publishers this subscriber is connected to
 */
uint32_t subscriber_getNumPublishers(subscriber *sub);

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_SUBSCRIBER_H_