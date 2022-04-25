use crate::{
    data_types::{
        tsg_data_types::{
            message, verdict::{
                generate_tx_verdict, Verdict
            }
        },
        event_protocol_messages::{
            signatures::witness_sig,
            event_protocol_messages::{Outcome},
            application_constructs::application_outcomes::exchange_app_outcome::{
                ExchangeOutcome
            }
        }
    },
    trust_score_generators::{
        tsg_framework::TsgFramework,
        exchange_application_tsg::predict_outcome
    }
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
/* pub fn tsg_assume_group(
    msgs: Vec<message::MessageAndPubkey>,
    assume_honest: HonestWho
) -> Option<(verdict::Verdict, verdict::Verdict)>{

    // after the tx has been verified, these sigs can act as temporary identities of the participants
    let (tn_sigs, wn_sigs) = msgs[0].get_sigs_of_participants().unwrap();

    // a test to see the sigs stored together
    //let sigs: Vec<Box<dyn Sig>> = vec![Box::new(tn_sigs[0]), Box::new(wn_sigs[0])];

    // determine if all outcomes are true
    let mut all_true = true;
    for msg in msgs.clone(){
        // only look at the witness statements
        if msg.is_witness_statement_msg() {
            let witness_statement: Outcome = msg.get_witness_statement().unwrap();
            match witness_statement {
                Outcome::ExchangeApplication(outcome) => {
                    let to_check_with = vec![true; outcome.len()];

                    // if the witness statement indicates a true output for all tns, we skip this
                    if !(outcome == to_check_with){
                        all_true = false;
                    }
                }
                _ => return None
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

        return Some((tn_verdicts, wn_verdicts));
    } 
    else {
        let (tn_verdicts_outcomes, wn_verdicts_outcomes) = 
            match assume_honest {
                HonestWho::TransactingNodes => (vec![1.0; tn_sigs.len()], vec![0.0; tn_sigs.len()]),
                HonestWho::Witnesses => (vec![0.0; tn_sigs.len()], vec![1.0; tn_sigs.len()]),
            };
        let tn_verdicts = verdict::generate_tx_verdict(&tn_sigs, tn_verdicts_outcomes);
        let wn_verdicts = verdict::generate_tx_verdict(&wn_sigs, wn_verdicts_outcomes);

        return Some((tn_verdicts, wn_verdicts));
    }
} */

/// The trivial tsg will work off the knowledge of a participating
/// or watching user. The options are 1) we know the outcome was true
/// so anybody who disagrees is dishonest, or 2) we know the outcome 
/// was false, so the blamed tn and whichever witnesses disagree are
/// dishonest.
/// 
/// Returns the verdicts for the transacting nodes, and then the witnesses

pub struct TsgKnowOutcome {
    known_outcome: ExchangeOutcome
}

impl TsgFramework for TsgKnowOutcome {
    fn tsg_algorithm(&self, msgs: Vec<message::MessageAndPubkey>) -> (Verdict, Verdict) {
        return tsg_know_outcome(msgs, self.known_outcome.clone()).unwrap();
    }
}

pub fn tsg_know_outcome(
    msgs: Vec<message::MessageAndPubkey>,
    known_outcome: ExchangeOutcome
) -> Option<(Verdict, Verdict)> {
    let (tn_sigs, wn_sigs) = msgs[0].get_sigs_of_participants().unwrap();

    // determine verdict for witnesses
    let mut wn_verdicts_outcomes: Vec<f32> = Vec::new();
    for msg in msgs.clone(){
        // only look at the witness statements
        if msg.is_witness_statement_msg() {
            let witness_statement = msg.get_witness_statement().unwrap();

            match witness_statement {
                Outcome::ExchangeApplication(outcome) => {  
                    let to_check_with = known_outcome.clone();

                    // if the witness statement agrees with the known outcome, they are honest
                    if outcome == to_check_with{
                        wn_verdicts_outcomes.push(1.0);
                    } else {
                        wn_verdicts_outcomes.push(0.0);
                    }
                }
                _ => return None
            }
        }
    }

    // determine verdict for transacting nodes
    let mut tn_verdicts_outcomes: Vec<f32> = Vec::new();
    for participant_outcome in known_outcome {
        if participant_outcome {
            tn_verdicts_outcomes.push(1.0);
        } else {
            tn_verdicts_outcomes.push(0.0);
        }
    }

    let tn_verdicts = generate_tx_verdict(&tn_sigs, tn_verdicts_outcomes);
    let wn_verdicts = generate_tx_verdict(&wn_sigs, wn_verdicts_outcomes);

    return Some((tn_verdicts, wn_verdicts))
}

/// The trivial tsg will work off the assumption that participants
/// from a certain group are more reliable. Does so by assigning 
/// differentreliabilities to participants in the organization, vs 
/// those outside.
/// 
/// Returns the verdicts for the transacting nodes, and then the witnesses

pub struct TsgOrganization {
    pub org_pubkey: String,
    pub default_reputation: f32
}

impl TsgFramework for TsgOrganization {
    fn tsg_algorithm(&self, msgs: Vec<message::MessageAndPubkey>) -> (Verdict, Verdict) {
        return tsg_organization(msgs, self.org_pubkey.clone(), self.default_reputation).unwrap();
    }
}

pub fn tsg_organization(
    msgs: Vec<message::MessageAndPubkey>,
    org_pubkey: String,
    default_reputation: f32
) -> Option<(Verdict, Verdict)> {
    // after the tx has been verified, these sigs can act as temporary identities of the participants
    let (tn_sigs, wn_sigs) = msgs[0].get_sigs_of_participants().unwrap();

    // predicts the outcome by asssigning reputation scores to witnesses and averaging the outcomes
    let mut witness_statements: Vec<Outcome> = Vec::new();
    let mut witness_reliabilities: Vec<f32> = Vec::new();
    for msg in msgs.clone() {
        if msg.is_witness_statement_msg() {
            let witness_statement = msg.get_witness_statement().unwrap();
            witness_statements.push(witness_statement);

            // find the signature associated to the message sender, and extract the org_cert's pubkey
            let cur_did = witness_sig::find_associated_wnsig(wn_sigs.clone(), msg.sender_did).unwrap();
            if org_pubkey == cur_did.org_cert.org_pubkey {
                witness_reliabilities.push(1.0);
            } else {
                witness_reliabilities.push(default_reputation);
            }
        }
    }

    let mut ret_none = false;
    let outcomes: Vec<Vec<bool>> = witness_statements.into_iter().map(|ws| {
        match ws{
            Outcome::ExchangeApplication(outcome) => {
                return outcome;
            },
            _ => {
                ret_none = true;
                return vec![false];
            }
        }
    }).collect();

    if ret_none {
        return None;
    }

    let predicted_outcome = predict_outcome::predict_outcome(outcomes, witness_reliabilities);

    return tsg_know_outcome(msgs, predicted_outcome);
}