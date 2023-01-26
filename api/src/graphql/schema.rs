use juniper::EmptySubscription;

use crate::db::Database;

use super::mutation::Mutation;
use super::query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub fn build_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Database>::new())
}
