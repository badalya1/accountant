use crate::models::money::Currency;
use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum AccountType {
    Vault,
    Credit,
    Loan,
    Promise,
}

#[derive(GraphQLObject)]
pub struct Account {
    pub kind: AccountType,
    pub name: String,
    pub currency: Currency,
}
