use crate::trust_score_generators::data_types::messages::signatures::sig;

// Searches through each participant to see if the channel pk matches that of the participant.
// Uses the signature as ID for the participants
pub fn find_did_pk_from_channel_pk(
    participants: &Vec<Box<dyn sig::Sig>>,
    channel_pk: &str
) -> Option<String>
{
    for part in participants {
        if channel_pk == part.get_channel_pubkey() {
            return Some(part.get_did_pubkey());
        }
    }

    return None;
}