mod account;
mod category;
mod currency;
mod exchange_rate;
mod json;
mod settings;
mod transaction;

pub use account::*;
pub use category::*;
pub use currency::*;
pub use exchange_rate::*;
pub use json::*;
pub use settings::*;
pub use transaction::*;

use juniper::{FieldResult, ID};
use std::fmt::Display;

pub trait ConvertableVec<V, U> {
    fn convert(self) -> Vec<U>
    where
        U: From<V>;
}

impl<V, U> ConvertableVec<V, U> for Vec<V> {
    fn convert(self) -> Vec<U>
    where
        U: From<V>,
    {
        self.into_iter().map(U::from).collect()
    }
}

pub trait ConvertableResult<M, E, A>
where
    A: From<M>,
{
    fn convert(self) -> FieldResult<A>;
}

impl<M, E: Display, A> ConvertableResult<M, E, A> for Result<M, E>
where
    A: From<M>,
{
    fn convert(self) -> FieldResult<A> {
        match self {
            Ok(value) => Ok(value.into()),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct IDi32(pub i32);

impl From<ID> for IDi32 {
    fn from(value: ID) -> Self {
        IDi32(
            value
                .parse::<i32>()
                .expect("Could not convert from ID to i32"),
        )
    }
}
