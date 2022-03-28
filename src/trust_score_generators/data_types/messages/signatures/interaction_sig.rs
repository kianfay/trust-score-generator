use crate::trust_score_generators::data_types::messages::{
    contract::{
        Contract
    },
    tx_messages::WitnessClients,
    signatures::{
        organization_cert::{
            OrgCert
        },
        sig::Sig
    }
};

use serde::{Deserialize, Serialize};

// contains the data being signed
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InteractionPreSig {
    pub contract: Vec<u8>,
    pub signer_channel_pubkey: String,
    pub wit_node_sigs: Vec<u8>,
    pub org_cert: OrgCert,
    pub timeout: u32,
}

// contains the data and a signature, as well the the key to verify with
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InteractionSig {
    /// Hash of the contract
    pub contract: Vec<u8>,
    /// Signer channel public key
    pub signer_channel_pubkey: String,
    /// Hash of the combined witness sigs
    pub wit_node_sigs: Vec<u8>,
    // Organization certificate
    pub org_cert: OrgCert,
    // The timeout for the interaction
    pub timeout: u32,
    // The signer's DID public key
    pub signer_did_pubkey: String,
    // Signature for this information
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignituresBytes(pub Vec<Vec<u8>>);


impl Sig for InteractionSig {
    fn get_did_pubkey(&self) -> String {
        return self.signer_did_pubkey.clone();
    }
    fn get_channel_pubkey(&self) -> String {
        return self.signer_channel_pubkey.clone();
    }
}