use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApplicationMsg {
    CompensationMsg {
        payments: Payments
    }
}

pub type Payments = Vec<String>;