use crate::trust_score_generators::data_types::event_protocol_messages::contracts::{
	utility_types::{
		TransactingClients, CompensationJson, UnixTimestamp,
		CoordinateDMSFormat
	}
};

use serde::{Deserialize, Serialize};

////
//// EXCHANGE APPLICATION CONTRACT
////
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExchangeContract {
	pub offer: String,
	pub participants: TransactingClients,
	pub compensation: CompensationJson,
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}