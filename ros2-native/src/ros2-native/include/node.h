#ifndef ROS2_NODE_H_
#define ROS2_NODE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "callback.h"

/// Node is the single point of entry for creating publishers and subscribers.
typedef struct node node;

/// Create a new node with the specified name.
/**
 * \param[in] node_name Name of the node.
 * \throws InvalidNamespaceError if the namespace is invalid
 */
node *node_create(const char *n);

/// Destroy a live node.
void node_destroy(node *n);

/// Get the name of the node.
/** \return The name of the node. */
const char *node_get_name(node *n);

/// Get the namespace of the node.
/**
 * This namespace is the "node's" namespace, and therefore is not affected
 * by any sub-namespace's that may affect entities created with this instance.
 * Use get_effective_namespace() to get the full namespace used by entities.
 *
 * \sa get_sub_namespace()
 * \sa get_effective_namespace()
 * \return The namespace of the node.
 */
const char *node_get_namespace(node *n);

/// Get the fully-qualified name of the node.
/**
 * The fully-qualified name includes the local namespace and name of the node.
 * \return fully-qualified name of the node.
 */
const char *node_get_fully_qualified_name(node *n);

/// Get the fully-qualified names of all available nodes.
/**
 * The fully-qualified name includes the local namespace and name of the node.
 * \return A vector of fully-qualified names of nodes.
 */
const char *node_get_node_names(node *n);

/// Return a map of existing topic names to list of topic types.
/**
 * \return a map of existing topic names to list of topic types.
 * \throws std::runtime_error anything that rcl_error can throw
 */
const char *node_get_topic_names_and_types(node *n);

/// Return a map of existing service names to list of service types.
/**
 * \return a map of existing service names to list of service types.
 * \throws std::runtime_error anything that rcl_error can throw
 */
const char *node_get_service_names_and_types(node *n);

/// Return the sub-namespace, if this is a sub-node, otherwise an empty string.
/**
 * The returned sub-namespace is either the accumulated sub-namespaces which
 * were given to one-to-many create_sub_node() calls, or an empty string if
 * this is an original node instance, i.e. not a sub-node.
 *
 * For example, consider:
 *
 * ```cpp
 * auto node = std::make_shared<rclcpp::Node>("my_node", "my_ns");
 * node->get_sub_namespace();  // -> ""
 * auto sub_node1 = node->create_sub_node("a");
 * sub_node1->get_sub_namespace();  // -> "a"
 * auto sub_node2 = sub_node1->create_sub_node("b");
 * sub_node2->get_sub_namespace();  // -> "a/b"
 * auto sub_node3 = node->create_sub_node("foo");
 * sub_node3->get_sub_namespace();  // -> "foo"
 * node->get_sub_namespace();  // -> ""
 * ```
 *
 * get_namespace() will return the original node namespace, and will not
 * include the sub-namespace if one exists.
 * To get that you need to call the get_effective_namespace() method.
 *
 * \sa get_namespace()
 * \sa get_effective_namespace()
 * \return the sub-namespace string, not including the node's original namespace
 */
const char *node_get_sub_namespace(node *n);

/// Return the effective namespace that is used when creating entities.
/**
 * The returned namespace is a concatenation of the node namespace and the
 * accumulated sub-namespaces, which is used as the namespace when creating
 * entities which have relative names.
 *
 * For example, consider:
 *
 * ```cpp
 * auto node = std::make_shared<rclcpp::Node>("my_node", "my_ns");
 * node->get_effective_namespace();  // -> "/my_ns"
 * auto sub_node1 = node->create_sub_node("a");
 * sub_node1->get_effective_namespace();  // -> "/my_ns/a"
 * auto sub_node2 = sub_node1->create_sub_node("b");
 * sub_node2->get_effective_namespace();  // -> "/my_ns/a/b"
 * auto sub_node3 = node->create_sub_node("foo");
 * sub_node3->get_effective_namespace();  // -> "/my_ns/foo"
 * node->get_effective_namespace();  // -> "/my_ns"
 * ```
 *
 * \sa get_namespace()
 * \sa get_sub_namespace()
 * \return the sub-namespace string, not including the node's original namespace
 */
const char *node_get_effective_namespace(node *n);

/// Get the seconds since epoch
/**
 * \warning Depending on sizeof(double) there could be significant precision loss.
 * When an exact time is required use nanoseconds() instead.
 *
 * \return the seconds since epoch as a floating point number.
 */
double node_time_now_get_seconds(node *n);

void node_subscribe(node *n, const char *topic, const char *type, callback cb);

#ifdef __cplusplus
}
#endif

#endif //ROS2_NODE_H_
