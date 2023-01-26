use juniper::{graphql_object, FieldResult};

use crate::db::Database;
use crate::types::ConvertableResult;
use crate::types::{Account, NewAccountInput};
use accountant_core;

pub struct CurrencyMutation;

#[graphql_object(context = Database)]
impl CurrencyMutation {
    async fn create(&self, context: &Database, data: NewAccountInput) -> FieldResult<Account> {
        let conn = context.get_connection();
        let new_account = accountant_core::Mutation::create_account(conn, data.into()).await;
        new_account.convert()
    }
}
