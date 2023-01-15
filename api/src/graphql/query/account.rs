use accountant_core::Query;
use async_graphql::{Context, Object, Result};
use entity::{account, async_graphql};

use crate::db::Database;

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn get_accounts(&self, ctx: &Context<'_>) -> Result<Vec<account::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::get_all_accounts(conn)
            .await
            .map_err(|e| e.to_string())?)
    }
}
