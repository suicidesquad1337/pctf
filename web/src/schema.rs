use async_graphql::{EmptySubscription, MergedObject};

/// A type alias that uses our merged query/mutation objects
pub type Schema = async_graphql::Schema<Queries, Mutations, EmptySubscription>;

#[derive(MergedObject, Default)]
/// This struct merges all queries
pub struct Queries;

#[derive(MergedObject, Default)]
/// This struct mrges all mutations
pub struct Mutations;
