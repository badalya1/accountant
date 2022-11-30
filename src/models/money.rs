#[derive(Debug)]
pub struct Currency {
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub numeric_code: i32,
    pub digits: i32,
}

#[derive(Debug)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

#[derive(Debug)]
pub struct ExchangeRate {
    pub from: Currency,
    pub to: Currency,
    pub rate: f64,
}
