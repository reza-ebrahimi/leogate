use async_graphql::*;

use ros_sys::ThisNode;

#[derive(Clone, Default)]
pub struct RosNodeQuery;

#[Object]
impl RosNodeQuery {
  async fn node_info(&self) -> Option<ThisNode> {
    Some(ThisNode::get_info().into())
  }
}
