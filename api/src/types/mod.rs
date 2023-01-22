mod account;
mod currency;

pub use account::*;
pub use currency::*;
use juniper::FieldResult;
use std::{convert::From, fmt::Display};

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
