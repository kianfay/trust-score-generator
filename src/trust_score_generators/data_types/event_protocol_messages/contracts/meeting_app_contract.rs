use crate::trust_score_generators::data_types::event_protocol_messages::contracts::{
	utility_types::{UnixTimestamp, CoordinateDMSFormat}
};

use serde::{Deserialize, Serialize};

////
//// MEETING APPLICATION CONTRACT
////
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeetingContract {
	pub purpose: String,
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}

