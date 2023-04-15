use entity::transaction;
use juniper::{graphql_interface, graphql_object, ID};

#[graphql_interface]
trait Transaction {
    fn id(&self) -> ID;
}

#[derive(Debug, Clone)]
pub struct Income {
    model: transaction::Model,
}

impl From<transaction::Model> for Income {
    fn from(value: transaction::Model) -> Self {
        Income { model: value }
    }
}

#[graphql_object(impl = TransactionValue)]
impl Transaction for Income {
    fn id(&self) -> ID {
        ID::from(self.model.id.to_string())
    }
}
