use async_graphql::*;

use ros_sys::RosTime;

#[derive(Clone, Default)]
pub struct TimeQuery;

#[Object]
impl TimeQuery {
  async fn time_echo(&self, _ctx: &Context<'_>) -> Option<RosTime> {
    Some(RosTime::new().into())
  }
}
