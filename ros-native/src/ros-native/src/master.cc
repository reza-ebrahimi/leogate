#include "master.h"
#include <ros/ros.h>
#include <string>

const unsigned char* ros_master_getHost(int32_t *len) {
  const std::string &host = ros::master::getHost();
  *len = host.length();
  return (const unsigned char*)host.c_str();  
}

uint32_t ros_master_getPort() {
  return ros::master::getPort();
}

const unsigned char* ros_master_getURI(int32_t *len) {
  const std::string &uri = ros::master::getURI();
  *len = uri.length();
  return (const unsigned char*)uri.c_str();
}

bool ros_master_check() {
  return ros::master::check();
}

bool ros_master_getTopics(unsigned char** topics, int32_t *len) {
  ros::master::V_TopicInfo _topics;
  if (ros::master::getTopics(_topics)) {
    static std::string n;
    n.clear();

    for (size_t i = 0; i < _topics.size(); ++i) {   
      ros::master::TopicInfo ti = _topics[i];
      n += ti.name + '\n' + ti.datatype + ',';
    }
    
    *topics = (unsigned char*)n.c_str();
    *len = n.length();

    return true;
  }

  topics = NULL;
  return false;
}

bool ros_master_getNodes(unsigned char** nodes, int32_t *len) {
  ros::V_string _nodes;
  if (ros::master::getNodes(_nodes)) {
    static std::string n;
    n.clear();

    for (size_t i = 0; i < _nodes.size(); ++i) {      
       n += _nodes[i] + '\n';
    }
    
    *nodes = (unsigned char*)n.c_str();
    *len = n.length();
    
    return true;
  }

  return false;
}