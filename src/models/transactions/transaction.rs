use juniper::{graphql_object, GraphQLEnum, GraphQLObject, ID};

use crate::models::account::{Account, AccountType};
use crate::models::money::Money;

#[derive(GraphQLEnum)]
pub enum TransactionType {
    Expense,
    Income,
}

#[derive(GraphQLObject)]
pub struct TransactionCategory {
    pub name: String,
}

pub struct Transaction {
    pub id: ID,
    pub target_id: String,
    pub king: TransactionType,
    pub amount: Money,
    pub date: i32,
    pub category: TransactionCategory,
    pub notes: String,
}

#[derive(GraphQLObject)]
pub struct Transfer {
    pub id: ID,
    pub source: Account,
    pub target: Account,
    pub amount: Money,
    pub date: i32,
    pub category: TransactionCategory,
    pub notes: String,
}

#[graphql_object]
impl Transaction {
    fn target(&self) -> Account {
        Account {
            id: ID::new("1"),
            kind: AccountType::Credit,
            name: "Master Card".to_owned(),
            currency_id: "1".to_owned(),
        }
    }
}
