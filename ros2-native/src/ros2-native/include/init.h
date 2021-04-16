#ifndef ROS2_INIT_H_
#define ROS2_INIT_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
  
/**
 * Initializes the global context which is accessible via the function
 * rclcpp::contexts::get_global_default_context().
 * Also, installs the global signal handlers with the function
 * rclcpp::install_signal_handlers().
 *
 * \sa rclcpp::Context::init() for more details on arguments and possible exceptions
 */
void ros2_init();

/**
 * If nullptr is given for the context, then the global context is used, i.e.
 * the context initialized by rclcpp::init().
 *
 * If the global context is used, then the signal handlers are also uninstalled.
 *
 * This will also cause the "on_shutdown" callbacks to be called.
 *
 * \return true if shutdown was successful, false if context was already shutdown
 */
void ros2_shutdown();

/**
 * This may return false for a context which has been shutdown, or for a
 * context that was shutdown due to SIGINT being received by the rclcpp signal
 * handler.
 *
 * If nullptr is given for the context, then the global context is used, i.e.
 * the context initialized by rclcpp::init().
 *
 * \return true if shutdown has been called, false otherwise
 */
bool ros2_ok();

#ifdef __cplusplus
}
#endif

#endif //ROS2_INIT_H_
