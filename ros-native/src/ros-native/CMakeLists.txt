cmake_minimum_required(VERSION 3.0.2)
project(ros-native)

find_package(nlohmann_json REQUIRED CONFIG PATHS /usr/lib/cmake/)
find_package(catkin REQUIRED COMPONENTS
  roscpp
  rospy
  roslib
  std_msgs
)

catkin_package(
  INCLUDE_DIRS include
  CATKIN_DEPENDS roscpp rospy std_msgs roslib
)

include_directories(
  ${catkin_INCLUDE_DIRS}
  ${PROJECT_SOURCE_DIR}/include
)

add_library(${PROJECT_NAME} SHARED
  src/callback_queue.cc
  src/common.cc
  src/init.cc
  src/master.cc
  src/node_handle.cc
  src/publisher.cc
  src/spinner.cc
  src/subscriber.cc
  src/subscriber_handler.cc
  )

target_link_libraries(${PROJECT_NAME}
  ${catkin_LIBRARIES}
  nlohmann_json
  )

add_dependencies(${PROJECT_NAME}
  ${catkin_EXPORTED_TARGETS}
  )

install(TARGETS ${PROJECT_NAME} DESTINATION .)