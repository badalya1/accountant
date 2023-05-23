use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::{ConvertableVec, Income};
use accountant_core::transaction;
pub struct TransactionQuery;

#[graphql_object(context = Context)]
impl TransactionQuery {
    async fn list(context: &Context) -> FieldResult<Vec<Income>> {
        let conn = context.get_connection();
        let transactions = transaction::TransactionQuery::get_all_transactions(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Income> = transactions.convert();
        Ok(result)
    }
}
