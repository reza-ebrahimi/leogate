use async_graphql::*;
use futures::prelude::*;

use ros_sys::RosTime;

use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(250);

#[derive(Clone, Default)]
pub struct TimeSubscription;

#[Subscription]
impl TimeSubscription {
  async fn time_echo(&self, _ctx: &Context<'_>) -> impl Stream<Item = Option<RosTime>> {
    async_std::stream::interval(INTERVAL).map(|_| Some(RosTime::new().into()))
  }
}
