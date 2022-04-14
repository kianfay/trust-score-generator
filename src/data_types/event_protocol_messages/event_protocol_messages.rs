use crate::data_types::event_protocol_messages::{    
    application_constructs::application_messages::{
        exchange_app_messages::CompensationMsg
    },
    application_constructs::application_contracts::{
        utility_types::{WitnessUsers},
        exchange_app_contract::ExchangeContract,
        meeting_app_contract::MeetingContract
    },
    application_constructs::application_outcomes::exchange_app_outcome::{
        ExchangeOutcome
    },
    signatures::{
        interaction_sig::InteractionSig,
        witness_sig::WitnessSig
    }
};


use serde::{Deserialize, Serialize};

////
//// GENERIC MESSAGES
////

// The top level types, such as InteractionMsg and WitnessStatement, are common
// for all applications of the event protocol. The ApplicationMsg allows for
// the inclusion of application specific messages.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message{
    InteractionMsg {
        contract: Contract,
        witnesses: WitnessUsers,
        wit_node_sigs: ArrayOfWnSignitures,
        tx_client_sigs: ArrayOfTxSignitures,
    },
    WitnessStatement {
        outcome: Outcome
    },
    ApplicationMsg(ApplicationMsg)
}

////
//// GENERIC APPLICATION MESSAGES
////

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApplicationMsg {
    ExchangeApplication(CompensationMsg)
}

////
//// GENERIC CONTRACT
////

// Each Contract kind is for a specific application. Storing the
// contracts as en emum allows for abstraction away from the 
// event protocol application. 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Contract {
	ExchangeApplication(ExchangeContract),
	MeetingApplication(MeetingContract)
}

////
//// GENERIC OUTPUT
////

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Outcome {
    ExchangeApplication(ExchangeOutcome),
    MeetingApplication(MeetingContract)
}

////
//// MESSAGES UTLITY TYPES
////

// signitures are also simply arrays of bytes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfTxSignitures(pub Vec<InteractionSig>);
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignitures(pub Vec<WitnessSig>);
