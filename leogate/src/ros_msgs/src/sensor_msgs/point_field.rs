use async_graphql::*;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct PointField {
    name: String,
    offset: u32,
    datatype: u8,
    count: u32,
}
