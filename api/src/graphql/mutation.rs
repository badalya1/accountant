use crate::db::Database;

use juniper::graphql_object;

use super::account::AccountMutation;
use super::currency::CurrencyMutation;

pub struct Mutation;

#[graphql_object(context = Database)]
impl Mutation {
    fn accounts(&self) -> AccountMutation {
        AccountMutation
    }

    fn currencies(&self) -> CurrencyMutation {
        CurrencyMutation
    }
}
