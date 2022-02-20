use crate::trust_score_generators::{
    utility::get_did,
    data_types::{
        messages::{
            signatures::sig::Sig,
            tx_messages::{Message, ArrayOfTxSignitures, ArrayOfWnSignitures, is_tx_msg}
        },
        message
    }
};

use anyhow::Result;

pub fn parse_messages(message_and_pubkey: &Vec<(String, String)>) -> Result<Vec<message::MessageAndPubkey>> {
    let mut parsed_msgs: Vec<message::MessageAndPubkey> = Vec::new();
    
    // if the first message is not the transaction message, the function will panic in the call to parse_to_message
    let (msg, sigs) = parse_to_message(&message_and_pubkey[0], None)?;
    let unwrapped_sigs = sigs.unwrap();
    parsed_msgs.push(msg);

    for i in 1..message_and_pubkey.len() {
        let (msg, _) = parse_to_message(&message_and_pubkey[i], Some(&unwrapped_sigs))?;
        parsed_msgs.push(msg);
    }

    return Ok(parsed_msgs);
}

// Parses a message and pubkey (as strings) to a MessageAndPubkey object.
// Because when the first message is processes, the sigs won't yet be available,
// this function accepts sigs as an option. However, if sigs is None, than the
// message must be the tx_msg
pub fn parse_to_message(
    message_and_pubkey: &(String, String),
    sigs: Option<&Vec<Box<dyn Sig>>>
) -> Result<(message::MessageAndPubkey, Option<Vec<Box<dyn Sig>>>)> {
    let (msg, channel_pk) = message_and_pubkey;
    let deserialised_msg: Message = serde_json::from_str(msg.as_str())?;
    
    if let None = sigs {
        // extract the sigs if it is the tx_msg
        if is_tx_msg(&deserialised_msg) {
            let extracted_sigs = get_sigs(deserialised_msg.clone()).unwrap();

            let to_ret = message::MessageAndPubkey {
                message: deserialised_msg,
                sender_did: get_did::find_did_pk_from_channel_pk(&extracted_sigs, channel_pk).unwrap()
            };

            return Ok((to_ret, Some(extracted_sigs)));
        }
        // panic if it's not the tx_msg
        else {
            panic!("First message must be a TransactionMessage")
        }
    }

    let to_ret = message::MessageAndPubkey {
        message: deserialised_msg,
        sender_did: get_did::find_did_pk_from_channel_pk(sigs.unwrap(), channel_pk).unwrap()
    };

    return Ok((to_ret, None));

}

pub fn get_sigs(tx: Message) -> Option<Vec<Box<dyn Sig>>> {
    match tx {
        Message::TransactionMsg {
            contract: _,
            witnesses: _,
            wit_node_sigs,
            tx_client_sigs,
        } => {
            let ArrayOfWnSignitures(wit_sigs) = wit_node_sigs;
            let ArrayOfTxSignitures(tn_sigs) = tx_client_sigs;

            // combine these arrays of types WitnessSig and TransactingSig
            // respectively into an array of Sig
            let mut boxed_wn_sigs: Vec<Box<dyn Sig>> = Vec::new();
            for wn_sig in wit_sigs {
                let converted_sig: Box<dyn Sig> = Box::new(wn_sig);
                boxed_wn_sigs.push(converted_sig);
            }
            let mut boxed_tn_sigs: Vec<Box<dyn Sig>> = Vec::new();
            for tn_sig in tn_sigs {
                let converted_sig: Box<dyn Sig> = Box::new(tn_sig);
                boxed_wn_sigs.push(converted_sig);
            }

            let mut combined_vec: Vec<Box<dyn Sig>> = Vec::new();
            combined_vec.append(&mut boxed_wn_sigs);
            combined_vec.append(&mut boxed_tn_sigs);

            return Some(combined_vec);
        }
        _ => return None
    };
}