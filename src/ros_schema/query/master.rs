use async_graphql::*;

use ros_sys::{Master, MasterInfo};

#[derive(Clone, Default)]
pub struct RosMasterQuery;

#[Object]
impl RosMasterQuery {
  async fn master_info(&self) -> Option<MasterInfo> {
    Some(Master::get_info().into())
  }
}
