use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::ConvertableResult;
use crate::types::{Account, NewAccountInput};
use accountant_core::account;

pub struct AccountMutation;

#[graphql_object(context = Context)]
impl AccountMutation {
    async fn create(&self, context: &Context, data: NewAccountInput) -> FieldResult<Account> {
        let conn = context.get_connection();
        let new_account = account::AccountMutation::create_account(conn, data.into()).await;
        new_account.convert()
    }
}
