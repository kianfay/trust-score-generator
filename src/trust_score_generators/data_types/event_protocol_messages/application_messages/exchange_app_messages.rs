use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompensationMsg {
    pub payments: Payments
}

pub type Payments = Vec<String>;