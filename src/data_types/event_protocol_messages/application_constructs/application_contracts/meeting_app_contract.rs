use crate::data_types::event_protocol_messages::
	application_constructs::application_contracts::{
		utility_types::{UnixTimestamp, CoordinateDMSFormat}
};

use serde::{Deserialize, Serialize};

////
//// MEETING APPLICATION CONTRACT
////
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeetingContract {
	pub purpose: String,
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}

