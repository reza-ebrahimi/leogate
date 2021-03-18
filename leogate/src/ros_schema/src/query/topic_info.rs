use async_graphql::*;

#[derive(Clone, Default)]
pub struct TopicInfoQuery;

#[Object]
impl TopicInfoQuery {
  async fn dummy(&self) -> Option<String> {
    None
  }
}
