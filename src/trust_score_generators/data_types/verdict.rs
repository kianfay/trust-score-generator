use crate::trust_score_generators::{
    data_types::{
        message, verdict,
        messages::signatures::{
            WitnessSig, TransactingSig, PullDID
        }
    },
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
    msgs: &Vec<S>,
    verdicts: Vec<f32>
) -> TxVerdict
    where S: PullDID
{
    let verdicts: Vec<verdict::ParticipantVerdict> = msgs
            .iter()
            .map(|sig| {
                let did_pubkey = sig.get_did_pubkey();
                return verdict::ParticipantVerdict{
                    did_public_key: did_pubkey,
                    estimated_reliablility: 1.0
                }
            })
            .collect();
        
        return verdict::TxVerdict {
            verdicts: verdicts
        }
}