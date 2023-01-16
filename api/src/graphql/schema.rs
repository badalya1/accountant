use juniper::{EmptyMutation, EmptySubscription};
use migration::{Migrator, MigratorTrait};

use crate::{db::Database, graphql::query::Query};

pub type Schema =
    juniper::RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn build_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}

// /// Builds the GraphQL Schema, attaching the Database to the context
// pub async fn build_schema() -> AppSchema {
//     let db = Database::new().await;

//     Migrator::up(db.get_connection(), None).await.unwrap();

//     Schema::build(Query::default(), EmptyMutation, EmptySubscription)
//         .data(db)
//         .finish()
// }
