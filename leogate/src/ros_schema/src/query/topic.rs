use async_graphql::*;

#[derive(Clone, Default)]
pub struct TopicQuery;

#[Object]
impl TopicQuery {
  async fn dummy(&self) -> Option<String> {
    None
  }
}
