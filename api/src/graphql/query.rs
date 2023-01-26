use super::{account::AccountQuery, currency::CurrencyQuery};
use crate::db::Database;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Database)]
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
}
