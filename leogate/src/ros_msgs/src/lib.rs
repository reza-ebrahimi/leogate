pub mod sensor_msgs;
pub mod std_msgs;

#[derive(Clone, Debug, async_graphql::Union)]
pub enum RosMessage {
  Int8(std_msgs::Int8),
  Int16(std_msgs::Int16),
  Int32(std_msgs::Int32),
  Int64(std_msgs::Int64),
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct RosMessageType {
  pub msg_type: String,
}
