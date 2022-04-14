use crate::{
    data_types::event_protocol_messages::{
        event_protocol_messages::Outcome,
        application_constructs::application_outcomes::{
            exchange_app_outcome::ExchangeOutcome
        },
        signatures::{
            interaction_sig::InteractionSig,
            witness_sig::WitnessSig
        },
        event_protocol_messages::{
            ArrayOfTxSignitures, ArrayOfWnSignitures, Message
        },
    },
    utility::parse_messages::is_tx_msg as is_tx_msg_low_level
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
                is_tx_msg_low_level(message)
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

    pub fn get_witness_statement(&self) -> Option<Outcome> {
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
                    } => {
                        Some(outcome.clone())
                    },
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