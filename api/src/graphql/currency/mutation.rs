use juniper::{graphql_object, FieldResult, ID};

use crate::context::Context;
use crate::types::Currency;
use crate::types::{ConvertableResult, IDi32};
use accountant_core::currency;

pub struct CurrencyMutation;

#[graphql_object(context = Context)]
impl CurrencyMutation {
    async fn setSelected(
        &self,
        context: &Context,
        currency_id: ID,
        value: bool,
    ) -> FieldResult<Currency> {
        let conn = context.get_connection();
        let id: IDi32 = currency_id.into();
        let new_account =
            currency::CurrencyMutation::set_currency_selected(conn, id.0.into(), value).await;
        new_account.convert()
    }
}
