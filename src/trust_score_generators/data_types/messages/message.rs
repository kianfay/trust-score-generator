use crate::trust_score_generators::data_types::messages::transaction_msgs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message{
    WitnessStatement {
        outcome: Vec<bool>
    },
    TransactionMsg {
        contract: transaction_msgs::Contract,
        witnesses: transaction_msgs::WitnessClients,
        wit_node_sigs: transaction_msgs::ArrayOfWnSignitures,
        tx_client_sigs: transaction_msgs::ArrayOfTxSignitures,
    },
    CompensationMsg {
        payments: Vec<String>
    }
}

pub fn is_tx_msg(msg: &Message) -> bool {
    match msg {
        Message::TransactionMsg 
            {contract: _, witnesses: _, wit_node_sigs: _, tx_client_sigs: _}
                => return true,
        _       => return false
    };
}