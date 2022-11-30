use crate::models::account::Account;
use crate::models::money::Money;
use chrono::DateTime;
use chrono::Local;
pub enum TransactionType {
    Expense,
    Income,
}

pub struct TransactionCategory {
    pub name: String,
}

pub struct Transaction {
    pub target: Account,
    pub king: TransactionType,
    pub amount: Money,
    pub date: DateTime<Local>,
    pub category: TransactionCategory,
    pub notes: String,
}

pub struct Transfer {
    pub source: Account,
    pub target: Account,
    pub amount: Money,
    pub date: DateTime<Local>,
    pub category: TransactionCategory,
    pub notes: String,
}
