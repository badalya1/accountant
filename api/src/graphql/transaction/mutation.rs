use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::NewTransactionInput;
use crate::types::{ConvertableResult, Transaction};
use accountant_core::transaction;

pub struct TransactionMutation;

#[graphql_object(context = Context)]
impl TransactionMutation {
    async fn create(
        &self,
        context: &Context,
        data: NewTransactionInput,
    ) -> FieldResult<Transaction> {
        let conn = context.get_connection();
        let new_transaction =
            transaction::TransactionMutation::create_transaction(conn, data.into()).await;
        new_transaction.convert()
    }
}
