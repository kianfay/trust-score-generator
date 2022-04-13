use crate::trust_score_generators::data_types::event_protocol_messages::{
    application_messages::exchange_app_messages::{ApplicationMsg},
    contracts::{
        utility_types::{PublicKey, UserOrWitnesses},
        exchange_app_contract::ExchangeContract,
        meeting_app_contract::MeetingContract
    },
    signatures::{
        interaction_sig::InteractionSig,
        witness_sig::WitnessSig
    }
};

use serde::{Deserialize, Serialize};

////
//// MESSAGES
////

// The top level types, such as InteractionMsg and WitnessStatement, are common
// for all applications of the event protocol. The ApplicationMsg allows for
// the inclusion of application specific messages.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message{
    WitnessStatement {
        outcome: Outcome
    },
    InteractionMsg {
        contract: Contract,
        witnesses: WitnessClients,
        wit_node_sigs: ArrayOfWnSignitures,
        tx_client_sigs: ArrayOfTxSignitures,
    },
    ApplicationMsg(ApplicationMsg)
}

////
//// MESSAGES UTLITY TYPES
////

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WitnessClients       (pub Vec<PublicKey>);
// signitures are also simply arrays of bytes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfTxSignitures(pub Vec<InteractionSig>);
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignitures(pub Vec<WitnessSig>);
pub type Outcome = Vec<bool>;

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
//// GENERIC CONTRACT UTILITY TYPES
////

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactingClients   (pub Vec<PublicKey>);
pub type CompensationJson = Vec<(UserOrWitnesses, f32)>;
