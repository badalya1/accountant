use std::collections::{HashMap, HashSet, VecDeque};

type CurrencyId = i32;
type RateId = i32;

struct Currency {
    id: CurrencyId,
    code: String,
}

struct Rate {
    id: RateId,
    from: CurrencyId,
    to: CurrencyId,
    rate: f64,
}

#[derive(Debug, Clone)]
struct ConversionEdge {
    rate_id: RateId,
    inverted: bool,
}

struct RateCalculator {
    currencies: HashMap<CurrencyId, Currency>,
    rates: HashMap<RateId, Rate>,
}

// fn test() {
// }
// type CurrencyId = i32;

impl RateCalculator {
    fn new() -> Self {
        let mut currencies = HashMap::new();
        let mut rates = HashMap::new();
        let test_currs = [
            Currency {
                id: 0,
                code: "USD".to_string(),
            },
            Currency {
                id: 1,
                code: "CAD".to_string(),
            },
            Currency {
                id: 2,
                code: "EUR".to_string(),
            },
            Currency {
                id: 3,
                code: "AMD".to_string(),
            },
        ];

        let test_rates = [
            Rate {
                id: 0,
                from: 0,
                to: 1,
                rate: 1.3721699,
            },
            Rate {
                id: 1,
                from: 1,
                to: 2,
                rate: 0.68632711,
            },
            Rate {
                id: 2,
                from: 3,
                to: 1,
                rate: 0.0035544465,
            },
        ];

        for curr in test_currs.into_iter() {
            currencies.insert(curr.id, curr);
        }

        for rate in test_rates.into_iter() {
            rates.insert(rate.id, rate);
        }

        Self { currencies, rates }
    }

    fn get_rates(&self, start: CurrencyId) -> Vec<CalculatedRate> {
        let mut current_path: Vec<ConversionEdge> = Vec::new();
        let mut result = Vec::new();
        let mut visited: HashSet<CurrencyId> = HashSet::new();
        let mut queue: VecDeque<(CurrencyId, Vec<ConversionEdge>)> = VecDeque::new();
        visited.insert(start);
        queue.push_back((start, vec![]));

        while let Some((curr_id, mut current_path)) = queue.pop_front() {
            //lookup
            let direct_rates = self.rates.values().filter(|rate| rate.from == curr_id);

            for rate in direct_rates {
                if visited.contains(&rate.to) {
                    continue;
                }

                let mut next_path = current_path.clone();
                next_path.push(ConversionEdge {
                    rate_id: rate.id,
                    inverted: false,
                });

                queue.push_back((rate.to, next_path.clone()));
                visited.insert(rate.to);

                let rate_number = next_path.iter().fold(1f64, |acc, next| {
                    let next_rate = self.rates.get(&next.rate_id).unwrap();
                    match next.inverted {
                        false => acc * next_rate.rate,
                        true => acc * 1f64 / next_rate.rate,
                    }
                });
                let calculated_rate = CalculatedRate {
                    rate: rate_number,
                    to: rate.to,
                    path: next_path.clone(),
                };
                result.push(calculated_rate);
            }

            //reverse lookup

            let reversed_rates = self.rates.values().filter(|rate| rate.to == curr_id);

            for rate in reversed_rates {
                if visited.contains(&rate.from) {
                    continue;
                }

                let mut next_path = current_path.clone();
                next_path.push(ConversionEdge {
                    rate_id: rate.id,
                    inverted: true,
                });

                queue.push_back((rate.from, next_path.clone()));
                visited.insert(rate.from);
                let rate_number = next_path.iter().fold(1f64, |acc, next| {
                    let next_rate = self.rates.get(&next.rate_id).unwrap();
                    match next.inverted {
                        false => acc * next_rate.rate,
                        true => acc * 1f64 / next_rate.rate,
                    }
                });
                let calculated_rate = CalculatedRate {
                    rate: rate_number,
                    to: rate.from,
                    path: next_path.clone(),
                };
                result.push(calculated_rate);
            }
        }
        result
    }
}

#[derive(Debug)]
struct CalculatedRate {
    to: CurrencyId,
    rate: f64,
    path: Vec<ConversionEdge>,
}
fn app_test() {
    const USD_ID: CurrencyId = 0;
    let calculator = RateCalculator::new();
    let usd_rates = calculator.get_rates(USD_ID);
    dbg!(usd_rates);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        app_test();
    }
}

/*

A -> B (R01 1.5) -> D (R02 2.0)
                 <- F (R03 3.0)
  -> C -> D
  -> F
  -> E



B: [[B, R01, false]]
C: nothing
D: [[A, 1.0], [B, 1.2,], [D, 2.0]]
E: nothing

Path: Vec<ConversionEdge>::new(); = []

[[R01, false]]

Start with A

B

 */
