use crate::trust_score_generators::{
    data_types::{
        messages::tx_messages::{Outcome}
    },
};

// A simple prediction algorithm which averages out the weighting of the
// outcomes to come to a single prediction. Assumes all outcomes are of the
// same length
pub fn predict_outcome(
    witness_statements: Vec<Outcome>,
    reliabilities: Vec<f32>
) -> Vec<bool> {
    let outcomes_per_stmt = witness_statements[0].len();
    let mut result = vec![0.0; outcomes_per_stmt];
    
    for i in 0..witness_statements.len() {
        for j in 0..outcomes_per_stmt {
            let cur_outcome = witness_statements[i][j];
            let to_add = match cur_outcome {
                true => 1.0 * reliabilities[i],
                false => -1.0 * reliabilities[i]
            };

            result[j] += to_add;
        }
    }

    // returns true if the outcome average is larger than 0, and
    // false if smaller than 0
    return result.iter().map(|x| x > &0.0).collect();
}

#[test]
fn test_predict_outcome() {
    let ws = vec![Vec::from([true, true]), Vec::from([false, true]),
                Vec::from([true, false]), Vec::from([false, false])];

    let reliabilities = vec![1.0, 1.0, 0.0, 1.0];
    assert_eq!(predict_outcome(ws, reliabilities), vec![false, true]);
}