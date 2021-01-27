#ifndef ROS_ONE_PUBLISHER_H_
#define ROS_ONE_PUBLISHER_H_

/*
 * PATH: /opt/ros/melodic/include/ros/publisher.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>
#include <stdbool.h>

/**
 * \brief Manages an advertisement on a specific topic.
 *
 * A Publisher should always be created through a call to NodeHandle::advertise(), or copied from one
 * that was. Once all copies of a specific
 * Publisher go out of scope, any subscriber status callbacks associated with that handle will stop
 * being called.  Once all Publishers for a given topic go out of scope the topic will be unadvertised.
 */
typedef struct publisher publisher;

publisher *publisher_create();

void publisher_destroy(publisher *pub);

/**
 * \brief Shutdown the advertisement associated with this Publisher
 *
 * This method usually does not need to be explicitly called, as automatic shutdown happens when
 * all copies of this Publisher go out of scope
 *
 * This method overrides the automatic reference counted unadvertise, and does so immediately.
 * \note Note that if multiple advertisements were made through NodeHandle::advertise(), this will
 * only remove the one associated with this Publisher
 */
void publisher_shutdown(publisher *pub);

/**
 * \brief Returns the topic that this Publisher will publish on.
 */
const char *publisher_getTopic(publisher *pub);

/**
 * \brief Returns the number of subscribers that are currently connected to this Publisher
 */
uint32_t publisher_getNumSubscribers(publisher *pub);

/**
 * \brief Returns whether or not this topic is latched
 */
bool publisher_isLatched();

void publisher_publish(publisher *pub, const char* message);

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_PUBLISHER_H_