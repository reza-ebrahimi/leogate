use async_graphql::*;

#[derive(Clone, Default)]
pub struct TopicPublishMutation;

#[Object]
impl TopicPublishMutation {
    async fn dummy(&self) -> Option<String> {
        None
    }
}
