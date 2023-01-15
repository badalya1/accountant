use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use entity::async_graphql;
use migration::{Migrator, MigratorTrait};

use crate::{db::Database, graphql::query::Query};

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Migrator::up(db.get_connection(), None).await.unwrap();

    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
