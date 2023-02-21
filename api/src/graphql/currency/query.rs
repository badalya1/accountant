use juniper::{graphql_object, FieldResult};

use crate::db::Database;
use crate::types::{ConvertableVec, Currency};

pub struct CurrencyQuery;

#[graphql_object(context = Database)]
impl CurrencyQuery {
    async fn list(context: &Database) -> FieldResult<Vec<Currency>> {
        let conn = context.get_connection();
        let currencies = accountant_core::Query::get_all_currencies(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Currency> = currencies.convert();
        Ok(result)
    }
}