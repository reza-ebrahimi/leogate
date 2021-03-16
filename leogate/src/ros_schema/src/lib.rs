mod inputs;
mod mutation;
mod query;
mod root_mutation;
mod root_query;
mod root_subscription;
mod subscription;

pub use inputs::*;
pub use root_mutation::*;
pub use root_query::*;
pub use root_subscription::*;
pub use subscription::*;

pub type RosSchema = async_graphql::Schema<RootQuery, RootMutation, RootSubscription>;
