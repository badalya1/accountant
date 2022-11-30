mod models;

use crate::models::money::{Currency, Money};

fn main() {
    let usd = Currency {
        name: "United States Dollar".to_string(),
        code: "USD".to_string(),
        symbol: "$".to_string(),
        digits: 2,
        numeric_code: 840,
    };

    let _ms = Money {
        amount: 45.45,
        currency: usd,
    };
    println!("We have {_ms:?}");
}
