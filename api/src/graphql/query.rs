use super::{
    account::AccountQuery, category::CategoryQuery, currency::CurrencyQuery,
    settings::SettingsQuery,
};
use crate::context::Context;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }
    fn accounts(&self) -> AccountQuery {
        AccountQuery
    }
    fn currencies(&self) -> CurrencyQuery {
        CurrencyQuery
    }
    fn categories(&self) -> CategoryQuery {
        CategoryQuery
    }
    fn settings(&self) -> SettingsQuery {
        SettingsQuery
    }
}
