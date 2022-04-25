use crate::{
    data_types::tsg_data_types::{
        message::MessageAndPubkey,
        verdict::Verdict
    },
    trust_score_generators::tsg_framework::TsgFramework
};

use std::collections::HashMap;

// The identity captures the channel client and the associated keypair,
// as well as the keypair associated to the participants DID. It also has their 
// 'reliability', [0,1], an unambiguous/simplistic measure of the honesty of their
// actions. A score of 1 means they are always honest, and 0 means always dishonest.
// A more dishonest participant will more likely to give a either not uphold their
// half of an agreement, or more likely to give a lazy witness statement (i.e. not
// depending on the actual event), or to possibly collude or act with malice to 
// either gain an advantage in monetary or trust score terms (or damage other 
// participant).
pub struct Identity<C, I> {
    /// The IOTA channels client
    pub channel_client: C,
    /// Information inherent to the participant outside of the simulation
    pub id_info: I,
    /// A Map from public keys to the perceived reliability of the associated participant
    pub reputation_map: ReputationMap,
    /// The minimum trust this participant needs to have to engage with another participant.
    /// For an organization, this is the minimum average reliability of the participants
    /// needed to allow the transaction.
    pub user_reliability_threshold: f32,
    /// The trust this participant has in a unknown participant
    pub user_default_reliability: f32
}

impl<C,I> Identity<C,I> {
    pub fn update_reliability(&mut self, new_estimates: Verdict){
        for estimate in new_estimates.verdicts {
            // inserts the default value if the entry does not yet exist
            self.add_if_not_already(&estimate.did_public_key);

            // remove the old mapping, and then update and re-insert
            let (score, estimate_count) = self.reputation_map.remove(&estimate.did_public_key).unwrap();
            self.reputation_map.insert(estimate.did_public_key, (score + estimate.estimated_reliablility, estimate_count + 1));
        }        
    }

    /// Average out the participants reliability scores and checks if this
    /// passes the reliability threshold
    pub fn check_avg_participants(&mut self, participants: &Vec<String>) -> bool{
        participants
            .iter()
            .for_each(|p| self.add_if_not_already(p)); 
        
        let reliability_sum: f32 = participants
            .iter()
            .map(|p| self.reputation_map.get(p).unwrap())
            .map(|r| self.calculate_score(r))
            .sum();
        return (reliability_sum / participants.len() as f32) >= self.user_reliability_threshold;
    }

    /// Checks if all participants pass the reliability threshold
    pub fn check_all_participants(&mut self, participants: &Vec<String>) -> bool{
        return participants.iter().all(|p| self.check_participant(p));
    }

    /// Checks if a participant passes the reliability threshold
    pub fn check_participant(&mut self, participant: &str) -> bool{
        self.add_if_not_already(participant); 

        let reliability = self.calculate_score(self.reputation_map.get(participant).unwrap());
        return reliability >= self.user_reliability_threshold;
    }

    /// Adds an entry into reputation_map if it does not yet exist
    pub fn add_if_not_already(&mut self, participant: &str) {
        self.reputation_map.entry(String::from(participant)).or_insert((0.0, 0));
    }

    // Because the default value in the reliability map is (0.0, 0), a div by zero
    // error must be avoided. Conveniently, (_, 0) means we should use the default value.
    pub fn calculate_score(&self, components: &ReliabilityComponents) -> f32 {
        if components.1 == 0{
            return self.user_default_reliability;
        }
        return components.0 / components.1 as f32;
    }

    pub fn get_reliability_scores_string(&self) -> String {
        let mut str = String::new();
        self.reputation_map
            .iter()
            .for_each(|(p,r)| {
                let score = self.calculate_score(r);
                let next = format!("{}: {}\n", p, score);
                str.push_str(&next)
            });
        return str;
    }

    /// Runs a TSG algorithm which implements TSGFramework, and automatically
    /// adds the verdicts into this user's Reputation Map.
    pub fn run_tsg_and_include_in_rm<T>(&mut self, messages: Vec<MessageAndPubkey>, tsg_algo: T) 
    where T : TsgFramework {
        let (tn_verdicts, wn_verdicts) = tsg_algo.tsg_algorithm(messages);
        self.update_reliability(tn_verdicts.clone());
        self.update_reliability(wn_verdicts.clone());
    }
}

// Reliability is calculated from reliability estimates. One reliability
// estimate of 0.5 causes the ReliabilityScore = (0.5,1), and the score
// is 0.5/1=0.5. After another estimate of 1.0, the ReliabilityScore = (1.5,2),
// and the score is 1.5/2=0.75
pub type ReliabilityComponents = (f32, usize);

/// Maps public keys to a reliability score
pub type ReputationMap = HashMap<String, ReliabilityComponents>;