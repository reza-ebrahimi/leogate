use async_graphql::*;
use futures::lock::Mutex;
use std::sync::Arc;

use ros2_sys::{Ros2Manager, Node};

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct Ros2Time {
  time_now_sec: f64,
  wall_now_sec: f64,
}

#[derive(Clone, Default)]
pub struct Ros2TimeQuery;

#[Object]
impl Ros2TimeQuery {
  async fn time_echo(&self, ctx: &Context<'_>) -> Option<Ros2Time> {
    let mut guard = ctx.data_unchecked::<Arc<Mutex<Ros2Manager>>>().lock().await;
    Some(Ros2Time{
      time_now_sec: guard.node().time_now_seconds(),
      wall_now_sec: 0.0
    }.into())
  }
}
