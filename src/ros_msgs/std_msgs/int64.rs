use async_graphql::*;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct Int64 {
  data: i64,
}
