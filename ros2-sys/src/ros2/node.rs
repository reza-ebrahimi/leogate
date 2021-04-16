use std::collections::HashMap;
use std::ffi::*;
use std::os::raw::*;
use std::sync::Arc;

use async_graphql::SimpleObject;
use futures::lock::Mutex;
use serde_json::from_str;

use super::ffi;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct TopicInfo {
  name: String,
  datatypes: Vec<String>,
}

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct ServiceInfo {
  name: String,
  datatypes: Vec<String>,
}

pub struct Node {
  __handle: *mut ffi::node,
}

impl Node {
  pub fn new(name: &str) -> Self {
    let ns = CString::new(name).expect("CString::new failed");
    let nh = unsafe { ffi::node_create(ns.as_ptr()) };

    Self { __handle: nh }
  }

  pub fn subscribe(&mut self, topic: &str, msg_type: &str) {

  }

  pub fn get_name(&self) -> String {
    unsafe {
      CStr::from_ptr(ffi::node_get_name(self.__handle))
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_namespace(&self) -> String {
    unsafe {
      CStr::from_ptr(ffi::node_get_namespace(self.__handle))
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_fully_qualified_name(&self) -> String {
    unsafe {
      CStr::from_ptr(ffi::node_get_fully_qualified_name(self.__handle))
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_node_names(&self) -> Vec<String> {
    let mut nodes: Vec<String> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::node_get_node_names(self.__handle);
      if ptr.is_null() {
        Err("Error in node_get_node_names")
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

  pub fn get_topic_names_and_types(&self) -> Vec<TopicInfo> {
    let mut topics: Vec<TopicInfo> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::node_get_topic_names_and_types(self.__handle);
      if ptr.is_null() {
        Err("Error in node_get_topic_names_and_types")
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

  pub fn get_service_names_and_types(&self) -> Vec<ServiceInfo> {
    let mut services: Vec<ServiceInfo> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::node_get_service_names_and_types(self.__handle);
      if ptr.is_null() {
        Err("Error in node_get_service_names_and_types")
      } else {
        Ok(CString::from_raw(ptr as *mut c_char))
      }
    };

    match buffer {
      Ok(buff) => match buff.to_str() {
        Ok(json_str) => {
          services = from_str::<Vec<ServiceInfo>>(json_str).ok().unwrap();
        }
        Err(err) => {
          println!("{}", err);
        }
      },
      Err(err) => {
        println!("{}", err);
      }
    }

    services
  }

  pub fn get_sub_namespace(&self) -> String {
    unsafe {
      CStr::from_ptr(ffi::node_get_sub_namespace(self.__handle))
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_effective_namespace(&self) -> String {
    unsafe {
      CStr::from_ptr(ffi::node_get_effective_namespace(self.__handle))
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn time_now_seconds(&self) -> f64 {
    unsafe {
      ffi::node_time_now_get_seconds(self.__handle)
    }
  }
}

impl Drop for Node {
  fn drop(&mut self) {
    unsafe {
      ffi::node_destroy(self.__handle);
    }

    println!("[Drop] Node");
  }
}

unsafe impl Send for Node {}
