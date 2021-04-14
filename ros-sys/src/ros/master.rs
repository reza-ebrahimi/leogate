use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use serde_json::from_str;

use async_graphql::*;

use super::ffi;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct TopicInfo {
  name: String,
  datatype: String,
}

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct MasterInfo {
  host: String,
  port: u32,
  uri: String,
  aliveness: bool,
  topics: Vec<TopicInfo>,
  nodes: Vec<String>,
}

pub struct Master;

impl Master {
  pub fn get_info() -> MasterInfo {
    MasterInfo {
      host: Master::get_host(),
      port: Master::get_port(),
      uri: Master::get_uri(),
      aliveness: Master::check(),
      topics: Master::get_topics(),
      nodes: Master::get_nodes(),
    }
  }

  pub fn get_host() -> String {
    unsafe {
      CStr::from_ptr(ffi::ros_master_getHost())
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_port() -> u32 {
    unsafe { ffi::ros_master_getPort() }
  }

  pub fn get_uri() -> String {
    unsafe {
      CStr::from_ptr(ffi::ros_master_getURI())
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn check() -> bool {
    unsafe { ffi::ros_master_check() }
  }

  pub fn get_topics() -> Vec<TopicInfo> {
    let mut topics: Vec<TopicInfo> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::ros_master_getTopics();
      if ptr.is_null() {
        Err("Error in ros_master_getTopics")
      } else {
        Ok(CString::from_raw(ptr as *mut c_char))
      }
    };

    match buffer {
      Ok(buff) => match buff.to_str() {
        Ok(json_str) => {
          topics = from_str::<Vec<TopicInfo>>(json_str).ok().unwrap();
        }
        Err(err) => {
          println!("{}", err);
        }
      },
      Err(err) => {
        println!("{}", err);
      }
    }

    topics
  }

  pub fn get_nodes() -> Vec<String> {
    let mut nodes: Vec<String> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::ros_master_getNodes();
      if ptr.is_null() {
        Err("Error in ros_master_getNodes")
      } else {
        Ok(CString::from_raw(ptr as *mut c_char))
      }
    };

    match buffer {
      Ok(buff) => match buff.to_str() {
        Ok(json_str) => {
          nodes = from_str::<Vec<String>>(json_str).ok().unwrap();
        }
        Err(err) => {
          println!("{}", err);
        }
      },
      Err(err) => {
        println!("{}", err);
      }
    }

    nodes
  }
}
