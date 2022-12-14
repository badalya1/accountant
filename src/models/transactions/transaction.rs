use juniper::{GraphQLEnum, GraphQLObject};

use crate::models::account::Account;
use crate::models::money::Money;
use chrono::{DateTime, Utc};

#[derive(GraphQLEnum)]
pub enum TransactionType {
    Expense,
    Income,
}

#[derive(GraphQLObject)]
pub struct TransactionCategory {
    pub name: String,
}

#[derive(GraphQLObject)]
pub struct Transaction {
    pub target: Account,
    pub king: TransactionType,
    pub amount: Money,
    pub date: DateTime<Utc>,
    pub category: TransactionCategory,
    pub notes: String,
}

#[derive(GraphQLObject)]
pub struct Transfer {
    pub source: Account,
    pub target: Account,
    pub amount: Money,
    pub date: DateTime<Utc>,
    pub category: TransactionCategory,
    pub notes: String,
}
