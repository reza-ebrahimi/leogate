use async_graphql::*;

use super::PointField;
use crate::std_msgs::Header;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct PointCloud2 {
  header: Header,
  height: u32,
  width: u32,
  fields: Vec<PointField>,
  is_bigendian: bool,
  point_step: u32,
  row_step: u32,
  data: Vec<u8>,
  is_dense: bool,
}
