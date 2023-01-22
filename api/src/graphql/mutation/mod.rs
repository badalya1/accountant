use crate::{
    db::Database,
    types::{Account, ConvertableResult, NewAccountInput},
};
use accountant_core;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = Database)]
impl Mutation {
    async fn create_account(
        &self,
        context: &Database,
        data: NewAccountInput,
    ) -> FieldResult<Account> {
        let conn = context.get_connection();
        let new_account = accountant_core::Mutation::create_account(conn, data.into()).await;
        new_account.convert()
    }
}
