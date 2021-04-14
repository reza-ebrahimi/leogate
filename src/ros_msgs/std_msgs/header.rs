use async_graphql::*;

// TDOO: stamp: ros::Time
#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct Header {
  pub seq: u32,
  pub frame_id: String,
}
