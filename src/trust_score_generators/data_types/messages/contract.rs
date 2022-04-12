use serde::{Deserialize, Serialize};

////
//// DATA FORMATS FOR THE INTERACTION MESSAGE
////

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContractWrapper {
    pub contract: Contract,               
}

pub struct ExchangeContract {
	pub application_name: String,
	pub offer: String,
	pub participants: TransactingClients,
	pub compensation: CompensationJson
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}

pub trait Contract {}
impl Contract for ExchangeContract {}

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactingClients   (pub Vec<PublicKey>);

pub type PublicKey = String;

// u64 used for timestamp as u32 runs out in 2038 (2147483647 as unixtime)
pub type UnixTimestamp = u64;

// CoordinateDMSFormat(North Ordinate, West Ordinate)
pub type CoordinateDMSFormat = (Ordinate,Ordinate);
pub type Ordinate = (u16,u16,f32);
