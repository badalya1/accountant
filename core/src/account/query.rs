use ::entity::{account, account::Entity as Account};
use ::entity::{transaction, transaction::Entity as Transaction};
use sea_orm::prelude::ChronoDateTimeWithTimeZone;

use sea_orm::*;

use crate::settings::SettingsQuery;
pub struct AccountQuery;

pub struct AccountBalances {
    pub opening: f64,
    pub closing: f64,
}

impl AccountQuery {
    pub async fn get_all_accounts(db: &DbConn) -> Result<Vec<account::Model>, DbErr> {
        Account::find().all(db).await
    }
    pub async fn find_account_by_id(db: &DbConn, id: String) -> Result<account::Model, DbErr> {
        let account = Account::find_by_id(id)
            .one(db)
            .await
            .expect("Could not execute query on account");

        account.ok_or(DbErr::RecordNotFound("Account not found".to_owned()))
    }
    pub async fn get_account_balance(db: &DbConn, id: String) -> Result<AccountBalances, DbErr> {
        let mut opening_balance = 0.0;
        let mut closing_balance = 0.0;

        let from_string = SettingsQuery::get(db, "FromDate").await?.value;
        let to_string = SettingsQuery::get(db, "ToDate").await?.value;

        //
        let datetime_fmt = "%s";

        let from = ChronoDateTimeWithTimeZone::parse_from_rfc3339(&from_string)
            .expect("Could not parse from date");
        let to = ChronoDateTimeWithTimeZone::parse_from_rfc3339(&to_string)
            .expect("Could not parse to date");

        println!("From: {}, To: {}", from, to);

        let transactions = Transaction::find()
            .filter(transaction::Column::AccountId.eq(id))
            .filter(transaction::Column::Date.between(from, to))
            .all(db)
            .await?;

        println!("Found {} transactions", transactions.len());

        for t in transactions.iter() {
            // add to opening balance if from is provided and if the transaction date is less than from
            let date_string = &t.date;
            let transaction_date = ChronoDateTimeWithTimeZone::parse_from_rfc3339(date_string)
                .expect("Could not parse transaction date");
            if transaction_date < from {
                opening_balance += t.amount;
            }

            closing_balance += t.amount;
        }

        Ok(AccountBalances {
            opening: opening_balance,
            closing: closing_balance,
        })
    }
}
