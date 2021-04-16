use async_graphql::*;
use futures::lock::Mutex;
use std::sync::Arc;

use ros2_sys::{Node, Ros2Manager, TopicInfo};

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct MasterInfo {
  host: Option<String>,
  port: Option<u32>,
  uri: Option<String>,
  aliveness: Option<bool>,
  topics: Vec<TopicInfo>,
  nodes: Vec<String>,
}

#[derive(Clone, Default)]
pub struct Ros2MasterQuery;

#[Object]
impl Ros2MasterQuery {
  async fn master_info(&self, ctx: &Context<'_>) -> Option<MasterInfo> {
    let mut guard = ctx.data_unchecked::<Arc<Mutex<Ros2Manager>>>().lock().await;

    Some(
      MasterInfo {
        host: None,
        port: None,
        uri: None,
        aliveness: None,
        topics: guard.node().get_topic_names_and_types(),
        nodes: guard.node().get_node_names(),
      }
      .into(),
    )
  }
}
