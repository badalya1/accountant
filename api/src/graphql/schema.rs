use juniper::EmptySubscription;

use crate::db::Database;

use super::mutation::Mutation;
use super::query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub fn build_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Database>::new())
}

// /// Builds the GraphQL Schema, attaching the Database to the context
// pub async fn build_schema() -> AppSchema {
//     let db = Database::new().await;

//     Migrator::up(db.get_connection(), None).await.unwrap();

//     Schema::build(Query::default(), EmptyMutation, EmptySubscription)
//         .data(db)
//         .finish()
// }
