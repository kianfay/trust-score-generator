use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompensationMsg {
    payments: Payments
}

pub type Payments = Vec<String>;