use juniper::{graphql_object, FieldResult};

use crate::db::Database;
use crate::types::{Category, ConvertableVec};
use accountant_core::category;

pub struct CategoryQuery;

#[graphql_object(context = Database)]
impl CategoryQuery {
    async fn list(context: &Database) -> FieldResult<Vec<Category>> {
        let conn = context.get_connection();
        let categories = category::CategoryQuery::list_roots(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Category> = categories.convert();
        Ok(result)
    }
}
