use super::mutation::*;
use async_graphql::*;

#[derive(Default, MergedObject)]
pub struct RootMutation(TopicPublishMutation);
