use entity::exchange_rate::Model as Rate;
use sea_orm::DbConn;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::exchange_rate::ExchangeRateQuery;

type CurrencyId = i32;
type RateId = i32;

#[derive(Debug, Clone, Copy)]
pub struct ConversionNode {
    pub currency_id: CurrencyId,
    pub rate_id: Option<RateId>,
    pub inverted: bool,
}

#[derive(Debug, Clone)]
pub struct CalculatedRate {
    pub to: CurrencyId,
    pub rate: f64,
    pub path: Vec<ConversionNode>,
}
#[derive(Clone)]
pub struct RateCalculator {
    db: DbConn,
    main_currency_id: CurrencyId,

    rates: HashMap<RateId, Rate>,
    nodes: HashMap<CurrencyId, ConversionNode>,
}

impl RateCalculator {
    pub async fn new(db: DbConn, main_currency_id: CurrencyId) -> Self {
        let mut new_calculator = Self {
            db,
            main_currency_id,
            rates: HashMap::new(),
            nodes: HashMap::new(),
        };

        new_calculator.calculate_rates().await;
        new_calculator
    }

    async fn calculate_rates(&mut self) {
        self.rates.clear();
        self.nodes.clear();

        let mut visited: HashSet<CurrencyId> = HashSet::new();

        let root_node = ConversionNode {
            currency_id: self.main_currency_id,
            rate_id: None,
            inverted: false,
        };
        self.nodes.insert(root_node.currency_id, root_node);

        let mut queue: VecDeque<ConversionNode> = VecDeque::new();
        visited.insert(root_node.currency_id);
        queue.push_back(root_node);

        while let Some(node) = queue.pop_front() {
            let rates = ExchangeRateQuery::get_from(&self.db, node.currency_id)
                .await
                .expect("Could not lookup rate");

            let inverted_rates = ExchangeRateQuery::get_to(&self.db, node.currency_id)
                .await
                .expect("Could not lookup rate");

            let mut new_nodes = Vec::<ConversionNode>::new();

            for rate in rates {
                let new_node = ConversionNode {
                    currency_id: rate.to_id,
                    rate_id: Some(rate.id),
                    inverted: false,
                };
                self.rates.insert(rate.id, rate);
                new_nodes.push(new_node);
            }
            for rate in inverted_rates {
                let new_node = ConversionNode {
                    currency_id: rate.from_id,
                    rate_id: Some(rate.id),
                    inverted: true,
                };
                self.rates.insert(rate.id, rate);
                new_nodes.push(new_node);
            }

            for new_node in new_nodes {
                if visited.contains(&new_node.currency_id) {
                    continue;
                }

                visited.insert(new_node.currency_id);
                self.nodes.insert(new_node.currency_id, new_node);
                queue.push_back(new_node);
            }
        }
    }

    pub fn get_rate(&self, currency_id: CurrencyId) -> Option<CalculatedRate> {
        let mut new_calculated_rate = CalculatedRate {
            path: Vec::new(),
            rate: 1f64,
            to: currency_id,
        };
        let mut conversion_node = match self.nodes.get(&currency_id) {
            Some(node) => node,
            None => {
                // Return None if the currency node is not found
                return None;
            }
        };
        while conversion_node.currency_id != self.main_currency_id {
            let rate = self.rates.get(&conversion_node.rate_id.unwrap()).unwrap();
            let next_rate: f64;
            let next_node_id: CurrencyId;

            if conversion_node.inverted {
                next_node_id = rate.to_id;
                next_rate = new_calculated_rate.rate / rate.rate
            } else {
                next_node_id = rate.from_id;
                next_rate = new_calculated_rate.rate * rate.rate
            };

            new_calculated_rate.rate = next_rate;
            new_calculated_rate.path.push(*conversion_node);
            conversion_node = self
                .nodes
                .get(&next_node_id)
                .expect("Cannot get currency from rate calculator");
        }
        Some(new_calculated_rate)
    }
}
