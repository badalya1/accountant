use juniper::{graphql_object, FieldResult};

use crate::db::Database;
use crate::types::{Account, ConvertableVec};
use accountant_core::account;
pub struct AccountQuery;

#[graphql_object(context = Database)]
impl AccountQuery {
    async fn list(context: &Database) -> FieldResult<Vec<Account>> {
        let conn = context.get_connection();
        let accounts = account::AccountQuery::get_all_accounts(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Account> = accounts.convert();
        Ok(result)
    }
}
