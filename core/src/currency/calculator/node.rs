use super::{edge::Edge, CurrencyId};

pub struct Node<'a> {
    pub parentEdge: Option<&'a Edge>,
    pub currencyId: CurrencyId,
    pub children: Vec<Edge>,
}
