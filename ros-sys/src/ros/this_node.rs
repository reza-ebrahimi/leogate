use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use serde_json::from_str;

use async_graphql::SimpleObject;

use super::ffi;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct ThisNode {
  name: String,
  namespace: String,
  advertised_topics: Vec<String>,
  subscribed_topics: Vec<String>,
}

impl ThisNode {
  pub fn get_info() -> Self {
    Self {
      name: ThisNode::get_name(),
      namespace: ThisNode::get_namespace(),
      advertised_topics: ThisNode::get_advertised_topics(),
      subscribed_topics: ThisNode::get_subscribed_topics(),
    }
  }

  pub fn get_name() -> String {
    unsafe {
      CStr::from_ptr(ffi::this_node_getName())
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_namespace() -> String {
    unsafe {
      CStr::from_ptr(ffi::this_node_getNamespace())
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn get_advertised_topics() -> Vec<String> {
    let mut topics: Vec<String> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::this_node_getAdvertisedTopics();
      if ptr.is_null() {
        Err("No Advertised Topics on this node")
      } else {
        Ok(CString::from_raw(ptr as *mut c_char))
      }
    };

    match buffer {
      Ok(buff) => match buff.to_str() {
        Ok(json_str) => {
          topics = from_str::<Vec<String>>(json_str).ok().unwrap();
        }
        Err(err) => {
          println!("{}", err);
        }
      },
      Err(_) => {}
    }

    topics
  }

  pub fn get_subscribed_topics() -> Vec<String> {
    let mut topics: Vec<String> = vec![];

    let buffer = unsafe {
      let mut ptr: *const c_char = ffi::this_node_getSubscribedTopics();
      if ptr.is_null() {
        Err("No Subscribed Topics on this node")
      } else {
        Ok(CString::from_raw(ptr as *mut c_char))
      }
    };

    match buffer {
      Ok(buff) => match buff.to_str() {
        Ok(json_str) => {
          topics = from_str::<Vec<String>>(json_str).ok().unwrap();
        }
        Err(err) => {
          println!("{}", err);
        }
      },
      Err(_) => {}
    }

    topics
  }
}
