#ifndef ROS_ONE_INIT_H_
#define ROS_ONE_INIT_H_

/*
 * PATH: /opt/ros/melodic/include/ros/init.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>
#include <stdbool.h>

#include "common.h"
#include "callback_queue.h"

enum InitOption {
  /**
   * Don't install a SIGINT handler.  You should install your own SIGINT handler in this
   * case, to ensure that the node gets shutdown correctly when it exits.
   */
  NoSigintHandler = 1 << 0,
  /** \brief Anonymize the node name.  Adds a random number to the end of your node's name, to make it unique.
   */
  AnonymousName = 1 << 1,
  /**
   * \brief Don't broadcast rosconsole output to the /rosout topic
   */
  NoRosout = 1 << 2,
};
typedef enum InitOption InitOption;

/** @brief ROS initialization function.
 *
 * This function will parse any ROS arguments (e.g., topic name
 * remappings), and will consume them (i.e., argc and argv may be modified
 * as a result of this call).
 *
 * Use this version if you are using the NodeHandle API
 *
 * \param argc
 * \param argv
 * \param name Name of this node.  The name must be a base name, ie. it cannot contain namespaces.
 * \param options [optional] Options to start the node with (a set of bit flags from \ref ros::init_options)
 * \throws InvalidNodeNameException If the name passed in is not a valid "base" name
 *
 */
ROSCPP_DECL void ros_init(const unsigned char *name, uint32_t options);
//ROSCPP_DECL void ros_init(int *argc, char **argv, const char *name, uint32_t options);

/**
 * \brief Returns whether or not ros::init() has been called
 */
ROSCPP_DECL bool ros_isInitialized();
/**
 * \brief Returns whether or not ros::shutdown() has been (or is being) called
 */
ROSCPP_DECL bool ros_isShuttingDown();

/** \brief Enter simple event loop
 *
 * This method enters a loop, processing callbacks.  This method should only be used
 * if the NodeHandle API is being used.
 *
 * This method is mostly useful when your node does all of its work in
 * subscription callbacks.  It will not process any callbacks that have been assigned to
 * custom queues.
 *
 */
ROSCPP_DECL void ros_spin();

/**
 * \brief Process a single round of callbacks.
 *
 * This method is useful if you have your own loop running and would like to process
 * any callbacks that are available.  This is equivalent to calling callAvailable() on the
 * global CallbackQueue.  It will not process any callbacks that have been assigned to
 * custom queues.
 */
ROSCPP_DECL void ros_spinOnce();

/**
 * \brief Wait for this node to be shutdown, whether through Ctrl-C, ros::shutdown(), or similar.
 */
ROSCPP_DECL void ros_waitForShutdown();

/** \brief Check whether it's time to exit.
 *
 * ok() becomes false once ros::shutdown() has been called and is finished
 *
 * \return true if we're still OK, false if it's time to exit
 */
ROSCPP_DECL bool ros_ok();

/**
 * \brief Disconnects everything and unregisters from the master.  It is generally not
 * necessary to call this function, as the node will automatically shutdown when all
 * NodeHandles destruct.  However, if you want to break out of a spin() loop explicitly,
 * this function allows that.
 */
ROSCPP_DECL void ros_shutdown();

/**
 * \brief Request that the node shut itself down from within a ROS thread
 *
 * This method signals a ROS thread to call shutdown().
 */
ROSCPP_DECL void ros_requestShutdown();

/**
 * \brief Actually starts the internals of the node (spins up threads, starts the network polling and xmlrpc loops,
 * connects to internal subscriptions like /clock, starts internal service servers, etc.).
 *
 * Usually unnecessary to call manually, as it is automatically called by the creation of the first NodeHandle if
 * the node has not already been started.  If you would like to prevent the automatic shutdown caused by the last
 * NodeHandle going out of scope, call this before any NodeHandle has been created (e.g. immediately after init())
 */
ROSCPP_DECL void ros_start();

/**
 * \brief Returns whether or not the node has been started through ros::start()
 */
ROSCPP_DECL bool ros_isStarted();

/**
 * \brief Returns a pointer to the global default callback queue.
 *
 * This is the queue that all callbacks get added to unless a different one is specified, either in the NodeHandle
 * or in the individual NodeHandle::subscribe()/NodeHandle::advertise()/etc. functions.
 */
ROSCPP_DECL callback_queue *ros_getGlobalCallbackQueue();

/**
 * \brief searches the command line arguments for the given arg parameter. In case this argument is not found
 * an empty string is returned.
 *
 * \param argc the number of command-line arguments
 * \param argv the command-line arguments
 * \param arg argument to search for
 */
ROSCPP_DECL const char *ros_getROSArg(int argc, const char *const *argv, const char *arg);

/**
 * \brief returns the default master uri that is used if no other is specified, e.g. by defining ROS_MASTER_URI.
 */
ROSCPP_DECL const char *ros_getDefaultMasterURI();

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_INIT_H_