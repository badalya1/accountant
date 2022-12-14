use crate::models::{money::Money, transaction::Transaction};

pub struct Year {
    pub month_number: u32,
    pub month_blocks: Block<Month>,
}
pub struct Month {
    pub month_number: u32,
    pub day_blocks: Block<Day>,
}

pub struct Day {
    pub day_number: u32,
    pub transaction_blocks: Block<Transactions>,
}

pub struct Transactions {
    pub id: String,
    pub transactions: Vec<Transaction>,
}

pub struct Block<T> {
    pub id: String,
    pub block_identifier: T,
    pub initial_balance: Money,
    pub current_balance: Money,
}
