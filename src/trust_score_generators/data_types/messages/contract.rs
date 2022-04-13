use serde::{Deserialize, Serialize};

////
//// GENERIC CONTRACT
////

// Each Contract kind is for a specific application. Storing the
// contracts as en emum allows for abstraction away from the 
// event protocol application. 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Contract {
	ExchangeContract(ExchangeContract),
	MeetingContract(MeetingContract)
}


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

////
//// MEETING APPLICATION CONTRACT
////
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeetingContract {
	pub purpose: String,
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}

////
//// UTILITY TYPES
////
// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactingClients   (pub Vec<PublicKey>);
pub type CompensationJson = Vec<(UserOrWitnesses, f32)>;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UserOrWitnesses {
	User(String),
	Witnesses
}
pub type PublicKey = String;
// u64 used for timestamp as u32 runs out in 2038 (2147483647 as unixtime)
pub type UnixTimestamp = u64;
// CoordinateDMSFormat(North Ordinate, West Ordinate)
pub type CoordinateDMSFormat = (Ordinate,Ordinate);
pub type Ordinate = (u16,u16,f32);

#[test]
fn test_polymorphic_contract() {
    
}