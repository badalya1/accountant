use super::{node::Node, CurrencyId};

pub struct Edge {
    to: CurrencyId,
    rate: f64,

    inverted: bool,
}
