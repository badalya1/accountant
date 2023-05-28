use accountant_core::sea_orm::prelude::ChronoDateTimeUtc;
use juniper::{graphql_object, FieldResult};

use crate::context::Context;
use crate::types::{ConvertableVec, TimespanInput, Transaction};
use accountant_core::transaction;
use chrono;

pub struct TransactionQuery;

#[derive(Debug, Clone)]
pub struct TransactionTimespan {
    from: ChronoDateTimeUtc,
    to: ChronoDateTimeUtc,
}

#[graphql_object(context = Context)]
impl TransactionQuery {
    async fn list(context: &Context, timespan: TimespanInput) -> FieldResult<Vec<Transaction>> {
        let conn = context.get_connection();
        let from = chrono::NaiveDateTime::from_timestamp_opt(timespan.from as i64, 0)
            .expect("Invalid from date");
        let to = chrono::NaiveDateTime::from_timestamp_opt(timespan.to as i64, 0)
            .expect("Invalid to date");

        let timespan = TransactionTimespan {
            from: chrono::TimeZone::from_utc_datetime(&chrono::Utc, &from),
            to: chrono::TimeZone::from_utc_datetime(&chrono::Utc, &to),
        };

        println!("Timespan: {:?}", timespan);

        let transactions =
            transaction::TransactionQuery::get_all_transactions(conn, timespan.from, timespan.to)
                .await
                .map_err(|e| e.to_string())
                .unwrap();
        let result: Vec<Transaction> = transactions.convert();
        Ok(result)
    }
}
