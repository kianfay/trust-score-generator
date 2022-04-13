use crate::trust_score_generators::data_types::event_protocol_messages::{
    event_protocol_messages::Contract,
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
pub struct WitnessPreSig {
    pub contract: Contract,
    pub signer_channel_pubkey: String,
    pub org_cert: OrgCert,
    pub timeout: u32,
}

// contains the data and a signature, as well the the key to verify with
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WitnessSig {
    pub contract: Contract,
    pub signer_channel_pubkey: String,
    pub org_cert: OrgCert,
    pub timeout: u32,
    pub signer_did_pubkey: String,
    pub signature: Vec<u8>,
}

impl Sig for WitnessSig {
    fn get_did_pubkey(&self) -> String {
        return self.signer_did_pubkey.clone();
    }
    fn get_channel_pubkey(&self) -> String {
        return self.signer_channel_pubkey.clone();
    }
}

pub fn find_associated_wnsig(sigs: Vec<WitnessSig>, did_pubkey: String) -> Option<WitnessSig> {
    for sig in sigs {
        if sig.signer_did_pubkey == did_pubkey {
            return Some(sig);
        }
    }

    return None;
}