use crate::trust_score_generators::data_types::messages::signatures::{WitnessSig, TransactingSig};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionMsg {
    pub contract: Contract,
	pub witnesses: WitnessClients,
    pub wit_node_sigs: ArrayOfWnSignitures,
	pub tx_client_sigs: ArrayOfTxSignitures,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub contract_definition: String,               
	pub participants: TransactingClients,          
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
}

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactingClients   (pub Vec<PublicKey>);
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WitnessClients       (pub Vec<PublicKey>);

pub type PublicKey = String;

// u64 used for timestamp as u32 runs out in 2038 (2147483647 as unixtime)
pub type UnixTimestamp = u64;

// CoordinateDMSFormat(North Ordinate, West Ordinate)
pub type CoordinateDMSFormat = (Ordinate,Ordinate);
pub type Ordinate = (u16,u16,f32);

// signitures are also simply arrays of bytes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfTxSignitures(pub Vec<TransactingSig>);
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignitures(pub Vec<WitnessSig>);
