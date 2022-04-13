use crate::trust_score_generators::data_types::event_protocol_messages::{
    signatures::{
        sig::Sig
    }
};

#[derive(Clone, Debug)]
pub struct TxVerdict {
    pub verdicts: Vec<ParticipantVerdict>
}

#[derive(Clone, Debug)]
pub struct ParticipantVerdict {
    pub did_public_key: String,
    pub estimated_reliablility: f32
}

pub fn generate_tx_verdict<S>(
    sigs: &Vec<S>,
    verdicts: Vec<f32>
) -> TxVerdict
    where S: Sig
{
    let verdicts: Vec<ParticipantVerdict> = sigs
            .iter()
            .enumerate()
            .map(|(i, sig)| {
                let did_pubkey = sig.get_did_pubkey();
                return ParticipantVerdict{
                    did_public_key: did_pubkey,
                    estimated_reliablility: verdicts[i]
                }
            })
            .collect();
        
        return TxVerdict {
            verdicts: verdicts
        }
}