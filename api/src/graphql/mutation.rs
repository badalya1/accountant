use crate::context::Context;

use juniper::graphql_object;

use super::account::AccountMutation;
use super::category::CategoryMutation;
use super::currency::CurrencyMutation;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn accounts(&self) -> AccountMutation {
        AccountMutation
    }

    fn currencies(&self) -> CurrencyMutation {
        CurrencyMutation
    }
    fn categories(&self) -> CategoryMutation {
        CategoryMutation
    }
}
