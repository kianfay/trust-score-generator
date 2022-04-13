use crate::trust_score_generators::data_types::messages::{
    contract::{
        Contract, PublicKey
    },
    signatures::{
        interaction_sig::InteractionSig,
        witness_sig::WitnessSig
    }
};

use serde::{Deserialize, Serialize};

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

// Application messages. Designed such that all possible applications messages
// need to be included in the ApplicationMsg enum
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApplicationMsg {
    CompensationMsg {
        payments: Payments
    }
}

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WitnessClients       (pub Vec<PublicKey>);

// signitures are also simply arrays of bytes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfTxSignitures(pub Vec<InteractionSig>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignitures(pub Vec<WitnessSig>);

pub type Outcome = Vec<bool>;
pub type Payments = Vec<String>;

pub fn is_tx_msg(msg: &Message) -> bool {
    match msg {
        Message::InteractionMsg 
            {contract: _, witnesses: _, wit_node_sigs: _, tx_client_sigs: _}
                => return true,
        _       => return false
    };
}