use crate::models::money::Currency;
pub enum AccountType {
    Vault,
    Credit,
    Loan,
    Promise,
}

pub struct Account {
    pub kind: AccountType,
    pub name: String,
    pub currency: Currency,
}
