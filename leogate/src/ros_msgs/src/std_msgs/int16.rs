use async_graphql::*;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct Int16 {
    data: i16,
}
