use async_graphql::*;

mod inputs;
mod query;

pub use inputs::*;
use query::*;

#[derive(Default, MergedObject)]
pub struct RootQuery(RosMasterQuery, RosNodeQuery, TimeQuery);

pub type RosSchema =
  async_graphql::Schema<RootQuery, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;
