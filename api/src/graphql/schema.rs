use juniper::EmptySubscription;

use crate::context::Context;

use super::mutation::Mutation;
use super::query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn build_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
