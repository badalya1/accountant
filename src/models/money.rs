use std::fmt;

use juniper::{GraphQLObject, ID};

#[derive(GraphQLObject, Debug, PartialEq, Clone)]
pub struct Currency {
    pub id: ID,
    pub name: String,
    pub code: String,
    pub symbol: Option<String>,
    pub numeric_code: i32,
    pub digits: i32,
}

#[derive(GraphQLObject, Debug, Clone)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

#[derive(GraphQLObject, Debug)]
pub struct ExchangeRate {
    pub id: ID,
    pub from: Currency,
    pub to: Currency,
    pub rate: f64,
}

impl fmt::Display for Money {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "{number:.prec$}{currency}",
            number = self.amount,
            prec = self.currency.digits as usize,
            currency = match &self.currency.symbol {
                None => format!(" {}", self.currency.code),
                Some(x) => x.to_string(),
            }
        )
    }
}
