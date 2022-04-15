use serde::{Deserialize, Serialize};

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ParticipantUsers   (pub ParticipantMap);
pub type ParticipantMap = Vec<(PublicKey, String)>;

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WitnessUsers       (pub Vec<PublicKey>);

// compensation json
pub type CompensationJson = Vec<(UserOrWitnesses, f32)>;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserOrWitnesses {
	User(String),
	Witnesses
}

// type alias for a public key
pub type PublicKey = String;

// u64 used for timestamp as u32 runs out in 2038 (2147483647 as unixtime)
pub type UnixTimestamp = u64;

// CoordinateDMSFormat(North Ordinate, West Ordinate)
pub type CoordinateDMSFormat = (Ordinate,Ordinate);
pub type Ordinate = (u16,u16,f32);
