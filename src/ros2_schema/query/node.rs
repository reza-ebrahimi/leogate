use async_graphql::*;
use futures::lock::Mutex;
use std::sync::Arc;

use ros2_sys::Ros2Manager;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct NodeInfo {
  name: String,
  namespace: String,
}

#[derive(Clone, Default)]
pub struct Ros2NodeQuery;

#[Object]
impl Ros2NodeQuery {
  async fn node_info(&self, ctx: &Context<'_>) -> Option<NodeInfo> {
    let mut guard = ctx.data_unchecked::<Arc<Mutex<Ros2Manager>>>().lock().await;
    Some(
      NodeInfo {
        name: guard.node().get_name(),
        namespace: guard.node().get_namespace(),
      }
      .into(),
    )
  }
}
