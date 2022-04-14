use crate::data_types::event_protocol_messages::
	application_constructs::application_contracts::{
		utility_types::{
			ParticipantUsers, CompensationJson, UnixTimestamp,
			CoordinateDMSFormat
		}
};

use serde::{Deserialize, Serialize};

////
//// EXCHANGE APPLICATION CONTRACT
////
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExchangeContract {
	pub offer: String,
	pub participants: ParticipantUsers,
	pub compensation: CompensationJson,
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}