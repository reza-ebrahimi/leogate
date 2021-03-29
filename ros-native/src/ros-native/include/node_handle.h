#ifndef ROS_ONE_NODE_HANDLE_H_
#define ROS_ONE_NODE_HANDLE_H_

/*
 * PATH: /opt/ros/melodic/include/ros/node_handle.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include "subscriber.h"
#include "publisher.h"
#include "callback_queue.h"
#include "callback.h"

typedef struct node_handle node_handle;

node_handle *node_handle_create(const char *nh);

void node_handle_destroy(node_handle *nh);

/**
 * \brief Set the default callback queue to be used by this NodeHandle.
 *
 * Setting this will cause any callbacks from
 * advertisements/subscriptions/services/etc. to happen through the
 * use of the specified queue.  NULL (the default) causes the global
 * queue (serviced by ros::spin() and ros::spinOnce()) to be used.
 */
void node_handle_setCallbackQueue(node_handle *nh, callback_queue *queue);

/**
 * \brief Returns the callback queue associated with this
 * NodeHandle.  If none has been explicitly set, returns the global
 * queue.
 */
callback_queue *node_handle_getCallbackQueue(node_handle *nh);

/**
 * \brief Returns the namespace associated with this NodeHandle
 */
const char *node_handle_getNamespace(node_handle *nh);

/**
 * \brief Returns the namespace associated with this NodeHandle as
 * it was passed in (before it was resolved)
 */
const char *node_handle_getUnresolvedNamespace(node_handle *nh);

/** \brief Resolves a name into a fully-qualified name
 *
 * Resolves a name into a fully qualified name, eg. "blah" =>
 * "/namespace/blah". By default also applies any matching
 * name-remapping rules (which were usually supplied on the command
 * line at startup) to the given name, returning the resulting
 * remapped name.
 *
 * \param name Name to remap
 *
 * \param remap Whether to apply name-remapping rules
 *
 * \return Resolved name.
 *
 * \throws InvalidNameException If the name begins with a tilde, or is an otherwise invalid graph resource name
 */
const char *node_handle_resolveName(node_handle *nh, const char *name, bool remap);

/**
 * \brief Subscribe to a topic, version for bare function
 *
 * This method connects to the master to register interest in a given
 * topic.  The node will automatically be connected with publishers on
 * this topic.  On each message receipt, fp is invoked and passed a shared pointer
 * to the message received.  This message should \b not be changed in place, as it
 * is shared with any other subscriptions to this topic.
 *
 * This version of subscribe is a convenience function for using bare functions, and can be used like so:
\verbatim
void callback(const std_msgs::Empty::ConstPtr& message)
{
}

ros::Subscriber sub = handle.subscribe("my_topic", 1, callback);
\endverbatim
 *
 * \param M [template] M here is the callback parameter type (e.g. const boost::shared_ptr<M const>& or const M&), \b not the message type, and should almost always be deduced
 * \param topic Topic to subscribe to
 * \param queue_size Number of incoming messages to queue up for
 * processing (messages in excess of this queue capacity will be
 * discarded).
 * \param fp Function pointer to call when a message has arrived
 * \param transport_hints a TransportHints structure which defines various transport-related options
 * \return On success, a Subscriber that, when all copies of it go out of scope, will unsubscribe from this topic.
 * On failure, an empty Subscriber which can be checked with:
\verbatim
void callback(const std_msgs::Empty::ConstPtr& message){...}
ros::NodeHandle nodeHandle;
ros::Subscriber sub = nodeHandle.subscribe("my_topic", 1, callback);
if (sub)  // Enter if subscriber is valid
{
...
}
\endverbatim
 *  \throws InvalidNameException If the topic name begins with a tilde, or is an otherwise invalid graph resource name
 *  \throws ConflictingSubscriptionException If this node is already subscribed to the same topic with a different datatype
 */
subscriber *node_handle_subscribe(node_handle *nh, const char *topic, const char *type, uint32_t queue_size, callback cb);

/**
 * \brief Advertise a topic, simple version
 *
 * This call connects to the master to publicize that the node will be
 * publishing messages on the given topic.  This method returns a Publisher that allows you to
 * publish a message on this topic.
 *
 * This version of advertise is a templated convenience function, and can be used like so
 *
 *   ros::Publisher pub = handle.advertise<std_msgs::Empty>("my_topic", 1);
 *
 * \param topic Topic to advertise on
 *
 * \param queue_size Maximum number of outgoing messages to be
 * queued for delivery to subscribers
 *
 * \param latch (optional) If true, the last message published on
 * this topic will be saved and sent to new subscribers when they
 * connect
 *
 * \return On success, a Publisher that, when it goes out of scope,
 * will automatically release a reference on this advertisement.  On
 * failure, an empty Publisher.
 *
 * \throws InvalidNameException If the topic name begins with a
 * tilde, or is an otherwise invalid graph resource name, or is an
 * otherwise invalid graph resource name
 */
publisher *node_handle_advertise(node_handle *nh, const char *topic, uint32_t queue_size, bool latch);

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_NODE_HANDLE_H_