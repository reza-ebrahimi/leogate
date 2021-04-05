#ifndef ROS_ONE_TIME_H_
#define ROS_ONE_TIME_H_

/*
 * PATH: /opt/ros/melodic/include/ros/time.h
 */

#ifdef __cplusplus
extern "C" {
#endif

#include "common.h"

#include <inttypes.h>

/**
* \brief Returns the current wall clock time in seconds.
*/
double time_now_toSec();

/**
* \brief Returns the current wall clock time in nanoseconds.
*/
uint64_t time_now_toNSec();

void time_init();

void time_shutdown();

void time_useSystemTime();

bool time_isSimTime();

bool time_isSystemTime();

/**
 * \brief Returns whether or not the current time source is valid.  Simulation time is valid if it is non-zero.
 */
bool time_isValid();

/**
 * \brief Wait for time source to become valid
 */
void time_waitForValid();


/*
 * WallTime
 */

/**
* \brief Returns the current wall clock time in seconds.
*/
double walltime_now_toSec();

/**
* \brief Returns the current wall clock time in nanoseconds.
*/
uint64_t walltime_now_toNSec();

bool walltime_isSystemTime();

#ifdef __cplusplus
}
#endif

#endif //ROS_ONE_TIME_H_
