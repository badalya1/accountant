mod account;
mod currency;

pub use account::*;
pub use currency::*;

use std::convert::From;
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
