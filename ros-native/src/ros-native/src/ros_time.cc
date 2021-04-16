#include "ros2_time.h"

#include <ros/ros.h>

double time_now_toSec() {
  return ros::Time::now().toSec();
}

uint64_t time_now_toNSec() {
  return ros::Time::now().toNSec();
}

void time_init() {
  ros::Time::init();
}

void time_shutdown() {
  ros::Time::shutdown();
}

void time_useSystemTime() {
  ros::Time::useSystemTime();
}

bool time_isSimTime() {
  return ros::Time::isSimTime();
}

bool time_isSystemTime() {
  return ros::Time::isSystemTime();
}

bool time_isValid() {
  return ros::Time::isValid();
}

void time_waitForValid() {
  ros::Time::waitForValid();
}

/*
 * WallTime
 */

double walltime_now_toSec() {
  return ros::WallTime::now().toSec();
}

uint64_t walltime_now_toNSec() {
  return ros::WallTime::now().toNSec();
}

bool walltime_isSystemTime() {
  return ros::WallTime::isSystemTime();
}