use crate::trust_score_generators::data_types::messages::{
    signatures::{
        interaction_sig::InteractionSig,
        witness_sig::WitnessSig
    },
    event_protocol_messages::{
        ArrayOfTxSignitures, ArrayOfWnSignitures, Message
    },
};

#[derive(Clone, Debug)]
pub struct MessageAndPubkey {
    pub message: Message,
    pub sender_did: String
}

impl MessageAndPubkey{
    pub fn is_tx_msg(&self) -> bool {
        match self{
            MessageAndPubkey {
                message,
                sender_did: _
            } => {
                match message {
                    Message::InteractionMsg {
                        contract: _, witnesses: _,
                        wit_node_sigs: _, tx_client_sigs: _,
                    } => return true,
                    _ => return false
                }
            }
        }
    }

    pub fn is_witness_statement_msg(&self) -> bool {
        match self{
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

    pub fn get_witness_statement(&self) -> Option<Vec<bool>> {
        if !self.is_witness_statement_msg() {
            return None;
        }
        
        match self{
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

    // Expects a MessageAndPubkey object with a InteractionMsg inside. Returns
    // a tuple with the sigs of transacting nodes and then the of witnesses
    pub fn get_sigs_of_participants(&self) -> Option<(Vec<InteractionSig>, Vec<WitnessSig>)> {
        if !self.is_tx_msg() {
            return None;
        }
        
        match self{
            MessageAndPubkey {
                message,
                sender_did: _
            } => {
                match message {
                    Message::InteractionMsg {
                        contract: _, witnesses: _,
                        wit_node_sigs: ArrayOfWnSignitures(wit_node_sigs),
                        tx_client_sigs: ArrayOfTxSignitures(tx_client_sigs),
                    } => Some((tx_client_sigs.clone(), wit_node_sigs.clone())),
                    _ => return None
                }
            }
        }
    }
}