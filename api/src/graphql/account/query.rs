use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::{Account, ConvertableVec};
use accountant_core::account;
pub struct AccountQuery;

#[graphql_object(context = Context)]
impl AccountQuery {
    async fn list(context: &Context) -> FieldResult<Vec<Account>> {
        let conn = context.get_connection();
        let accounts = account::AccountQuery::get_all_accounts(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Account> = accounts.convert();
        Ok(result)
    }
}
