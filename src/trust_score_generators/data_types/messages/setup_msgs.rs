use crate::trust_score_generators::data_types::messages::transaction_msgs::{Contract};

pub struct SetupMessage {
    pub contract: Contract,
	pub max_witnesses: u32,
	pub payment_to_node: f32,
	pub max_payment_per_witness: f32,
}