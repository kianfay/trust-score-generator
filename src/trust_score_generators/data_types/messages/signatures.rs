use crate::trust_score_generators::data_types::messages::transaction_msgs::{Contract, WitnessClients, ArrayOfWnSignitures};
use serde::{Deserialize, Serialize};

// the signature is of the upper fields
// timeout included to give participants freedom over how long to be exposed

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactingPreSig {
    pub contract: Contract,
    pub signer_channel_pubkey: String,
    pub witnesses: WitnessClients,
    pub wit_node_sigs: ArrayOfWnSignituresBytes,
    pub org_cert: OrgCert,
    pub timeout: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactingSig {
    pub contract: Contract,
    pub signer_channel_pubkey: String,
    pub witnesses: WitnessClients,
    pub wit_node_sigs: ArrayOfWnSignituresBytes,
    pub org_cert: OrgCert,
    pub timeout: u32,
    pub signer_did_pubkey: String,
    pub signature: Vec<u8>,
}

// This is the certificate granted by an organization's, which includes the organization's public 
// key, the duration(sec) of the certificate and the sig of all this information. This would be 
// derived from a certificate on the participants DID.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrgCertPreSig {
    pub client_pubkey: String,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrgCert {
    pub client_pubkey: String,
    pub duration: u32,
    pub org_pubkey: String,
    pub signature: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignituresBytes(pub Vec<Vec<u8>>);


pub fn extract_sig_from_wn_sig_struct(sig_struct: WitnessSig) -> Vec<u8> {
    match sig_struct {
        WitnessSig {
            contract: _,
            signer_channel_pubkey: _,
            org_cert: _,
            timeout: _,
            signer_did_pubkey: _,
            signature,
        } => return signature
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

// to be deleted
pub fn extract_org_from_wn_sig_struct(sig_struct: WitnessSig) -> OrgCert {
    return sig_struct.org_cert;
}

pub trait Sig {
    fn get_did_pubkey(&self) -> String;
    fn get_channel_pubkey(&self) -> String;
}

impl Sig for WitnessSig {
    fn get_did_pubkey(&self) -> String {
        return self.signer_did_pubkey.clone();
    }
    fn get_channel_pubkey(&self) -> String {
        return self.signer_channel_pubkey.clone();
    }
}

impl Sig for TransactingSig {
    fn get_did_pubkey(&self) -> String {
        return self.signer_did_pubkey.clone();
    }
    fn get_channel_pubkey(&self) -> String {
        return self.signer_channel_pubkey.clone();
    }
}