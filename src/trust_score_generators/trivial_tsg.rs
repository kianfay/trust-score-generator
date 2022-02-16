use crate::trust_score_generators::{
    data_types::{
        message, verdict,
        messages::signatures::{
            WitnessSig, TransactingSig, Sig
        }
    },
};

pub enum HonestWho{
    Witnesses,
    TransactingNodes
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
pub fn tsg(msgs: Vec<message::MessageAndPubkey>, assume_honest: HonestWho) -> (verdict::TxVerdict, verdict::TxVerdict){

    // after the tx has been verified, these sigs can act as temporary identities of the participants
    let (tn_sigs, wn_sigs) = message::get_sigs_of_participants(&msgs[0]).unwrap();

    // a test to see the sigs stored together
    //let sigs: Vec<Box<dyn Sig>> = vec![Box::new(tn_sigs[0]), Box::new(wn_sigs[0])];

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

    // if all outcomes are true, then both are 100% honest,
    // otherwise the non-assumeHonest parties are 100% dishonest,
    // and the assumeParty is 100% dishonest
    if all_true {
        let tn_verdicts_outcomes = vec![1.0; tn_sigs.len()];
        let tn_verdicts = verdict::generate_tx_verdict(&tn_sigs, tn_verdicts_outcomes);

        let wn_verdicts_outcomes = vec![1.0; wn_sigs.len()];
        let wn_verdicts = verdict::generate_tx_verdict(&wn_sigs, wn_verdicts_outcomes);

        return (tn_verdicts, wn_verdicts);
    } 
    else {
        let (tn_verdicts_outcomes, wn_verdicts_outcomes) = 
            match assume_honest {
                HonestWho::TransactingNodes => (vec![1.0; tn_sigs.len()], vec![0.0; tn_sigs.len()]),
                HonestWho::Witnesses => (vec![0.0; tn_sigs.len()], vec![1.0; tn_sigs.len()]),
            };
        let tn_verdicts = verdict::generate_tx_verdict(&tn_sigs, tn_verdicts_outcomes);
        let wn_verdicts = verdict::generate_tx_verdict(&wn_sigs, wn_verdicts_outcomes);

        return (tn_verdicts, wn_verdicts);
    }
}