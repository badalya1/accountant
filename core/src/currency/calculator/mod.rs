mod edge;
mod node;

use std::collections::{HashMap, HashSet};

use self::node::Node;

pub type CurrencyId = i32;
pub type ExchangeRate = f64;

struct RateCalculator<'a> {
    main_currency_id: CurrencyId,
    currencies: HashMap<CurrencyId, Node<'a>>,
    root: Node<'a>,
}

impl RateCalculator<'_> {
    fn new(currency_id: CurrencyId) -> Self {
        let calculator = Self {
            main_currency_id: currency_id,
            root: Node {
                currencyId: currency_id,
                children: Vec::new(),
                parentEdge: None,
            },
            currencies: HashMap::new(),
        };

        calculator.calculate();
        calculator
    }

    fn calculate(&self) {}
}
