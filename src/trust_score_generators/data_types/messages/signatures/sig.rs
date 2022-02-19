pub trait Sig {
    fn get_did_pubkey(&self) -> String;
    fn get_channel_pubkey(&self) -> String;
}