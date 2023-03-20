use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::{ConvertableVec, Currency};
use accountant_core::currency;

pub struct CurrencyQuery;

#[graphql_object(context = Context)]
impl CurrencyQuery {
    async fn list(context: &Context) -> FieldResult<Vec<Currency>> {
        let conn = context.get_connection();
        let currencies = currency::CurrencyQuery::get_all_currencies(conn)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        let result: Vec<Currency> = currencies.convert();
        Ok(result)
    }
}
