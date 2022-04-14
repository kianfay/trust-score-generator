use crate::{
    data_types::tsg_data_types::{
        message, verdict::Verdict
    },
};

pub trait TsgFramework {
    fn tsg_algorithm(&self, msgs: Vec<message::MessageAndPubkey>) -> (Verdict, Verdict);
}