use crate::trust_score_generators::data_types::messages::{
    message::Message,
    transaction_msgs::{
        TransactingClients, WitnessClients,
        ArrayOfTxSignitures, ArrayOfWnSignitures
    },
    signatures::{WitnessSig, TransactingSig}
};

#[derive(Clone, Debug)]
pub struct MessageAndPubkey {
    pub message: Message,
    pub sender_did: String
}

pub fn is_tx_msg(msg: &MessageAndPubkey) -> bool {
    match msg{
        MessageAndPubkey {
            message,
            sender_did: _
        } => {
            match message {
                Message::TransactionMsg {
                    contract: _, witnesses: _,
                    wit_node_sigs: _, tx_client_sigs: _,
                } => return true,
                _ => return false
            }
        }
    }
}

pub fn is_witness_statement_msg(msg: &MessageAndPubkey) -> bool {
    match msg{
        MessageAndPubkey {
            message,
            sender_did: _
        } => {
            match message {
                Message::WitnessStatement {
                    outcome: _
                } => return true,
                _ => return false
            }
        }
    }
}

pub fn get_witness_statement(msg: &MessageAndPubkey) -> Option<Vec<bool>> {
    if !is_witness_statement_msg(msg) {
        return None;
    }
    
    match msg{
        MessageAndPubkey {
            message,
            sender_did: _
        } => {
            match message {
                Message::WitnessStatement {
                    outcome
                } => return Some(outcome.clone()),
                _ => return None
            }
        }
    }
}

// Expects a MessageAndPubkey object with a TransactionMessage inside. Returns
// a tuple with the sigs of transacting nodes and then the of witnesses
pub fn get_sigs_of_participants(msg: &MessageAndPubkey) -> Option<(Vec<TransactingSig>, Vec<WitnessSig>)> {
    if !is_tx_msg(&msg) {
        return None;
    }
    
    match msg{
        MessageAndPubkey {
            message,
            sender_did: _
        } => {
            match message {
                Message::TransactionMsg {
                    contract: _, witnesses: _,
                    wit_node_sigs: ArrayOfWnSignitures(wit_node_sigs),
                    tx_client_sigs: ArrayOfTxSignitures(tx_client_sigs),
                } => Some((tx_client_sigs.clone(), wit_node_sigs.clone())),
                _ => return None
            }
        }
    }
}