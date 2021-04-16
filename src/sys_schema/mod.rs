use async_graphql::*;

mod query;
use query::*;

#[derive(Default, MergedObject)]
pub struct RootQuery(ClientQuery, SystemStatusQuery);

pub type SysSchema =
  async_graphql::Schema<RootQuery, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;
