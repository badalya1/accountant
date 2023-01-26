use crate::db::Database;

use juniper::graphql_object;

use super::account::AccountMutation;

pub struct Mutation;

#[graphql_object(context = Database)]
impl Mutation {
    fn accounts(&self) -> AccountMutation {
        AccountMutation
    }
}
