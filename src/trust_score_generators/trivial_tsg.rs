use crate::trust_score_generators::{
    data_types::{
        message, verdict,
        messages::signatures::{
            WitnessSig, TransactingSig
        }
    },
};

pub enum HonestWho{
    Witnesses,
    Participants
}

/// The trivial tsg will either assume honestly of the particpants or
/// witnesses. So there are 4 possible outcomes, 1) assume participant
/// honest, so if witness outcome is all true, then witnesses are also
/// honest, otherwise they are dishonest, 2) assume witness honest,
/// so if witnesses outcomes are all true, participants are also honest,
/// otherwise they are dishonest.
/// 
/// Returns the verdicts for the transacting nodes, and then the witnesses
/// 
pub fn tsg(msgs: Vec<message::MessageAndPubkey>, assumeHonest: HonestWho) -> (verdict::TxVerdict, verdict::TxVerdict){

    // after the tx has been verified, these sigs can act as temporary identities of the participants
    let (tn_sigs, wn_sigs) = message::get_sigs_of_participants(&msgs[0]).unwrap();

    // determine if all outcomes are true
    let mut all_true = true;
    for msg in msgs.clone(){
        // only look at the witness statements
        if message::is_witness_statement_msg(&msg) {
            let witness_statement = message::get_witness_statement(&msg).unwrap();
            let to_check_with = vec![true; witness_statement.len()];

            // if the witness statement indicates a true output for all tns, we skip this
            if !(witness_statement == to_check_with){
                all_true = false;
            }
        }
    }

    // if all outcomes are true, then both are 100% honest
    if all_true {
        let verdicts_outcomes = vec![1.0; tn_sigs.len()];
        let tn_verdicts = verdict::generate_tx_verdict<WitnessSig>(&tn_sigs, verdicts_outcomes);

        let verdicts_outcomes = vec![1.0; wn_sigs.len()];
        let wn_verdicts = verdict::generate_tx_verdict<TransactingSig>(&wn_sigs, verdicts_outcomes);
    }

    // otherwise, the non-assumeHonest parties are 100% dishonest,
    // and the assumeParty is 100% dishonest

    return verdict::TxVerdict{
        verdicts: Vec::new()
    }
}