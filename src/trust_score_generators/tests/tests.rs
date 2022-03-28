use crate::trust_score_generators::{
    utility::parse_messages,
    data_types::{
        verdict,
        message
    },
    trivial_tsg
};

////
//// TESTS
////

#[test]
fn test_pubkey_conversion() {
    let msg = String::from("{\"InteractionMsg\":{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3\",\"org_cert\":{\"client_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[17,20,98,109,244,104,6,196,80,158,6,241,3,116,18,97,44,19,93,0,69,1,94,69,148,89,164,19,11,220,50,217,131,215,243,88,11,66,103,81,90,137,214,184,144,59,223,176,84,66,78,92,241,189,84,229,139,139,237,195,77,181,63,6]},\"timeout\":120,\"signer_did_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"signature\":[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW\",\"org_cert\":{\"client_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[246,4,58,192,200,95,167,255,40,18,33,217,104,228,5,147,132,206,0,249,108,52,79,229,206,52,114,242,231,118,37,70,136,133,144,250,199,146,49,213,84,15,44,116,46,216,193,22,161,212,101,177,142,54,199,134,145,229,207,15,51,161,146,0]},\"timeout\":120,\"signer_did_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"signature\":[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]}],\"tx_client_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[22,31,193,148,14,208,180,103,108,100,224,161,185,241,182,117,224,113,233,226,101,117,152,244,197,89,126,146,196,73,2,195,134,73,199,124,195,146,164,60,131,31,192,75,9,24,139,4,212,13,38,113,73,54,159,206,47,57,170,32,138,209,134,4]},\"timeout\":120,\"signer_did_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"signature\":[152,79,163,65,57,165,142,1,234,105,37,128,138,14,41,106,17,147,245,14,191,102,9,23,182,149,41,134,62,200,176,68,145,92,230,128,138,228,29,82,33,61,18,155,147,67,188,65,249,25,76,59,129,201,88,234,34,179,224,147,28,130,247,7]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[172,37,72,229,140,176,147,51,220,153,84,41,254,91,183,133,199,42,111,72,76,191,197,26,163,238,216,17,136,131,212,13,169,195,160,171,97,5,254,248,110,215,112,184,49,1,27,198,243,97,91,201,127,204,156,246,115,113,193,241,192,157,118,3]},\"timeout\":120,\"signer_did_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"signature\":[219,244,111,204,67,156,96,128,44,66,151,184,187,170,1,90,149,241,164,34,109,10,84,166,192,51,227,186,121,112,0,72,186,151,17,193,56,131,173,115,163,42,77,253,170,101,39,145,52,177,68,216,218,70,161,17,6,142,208,240,229,74,86,8]}]}}");
    let message_and_pubkey = (msg, String::from("z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i"));
    
    let (parsed_msg, _) = parse_messages::parse_to_message(&message_and_pubkey, None).unwrap();
    match parsed_msg {
        message::MessageAndPubkey {
            message: _,
            sender_did,
        } => {
            println!("{}", sender_did);
            assert_eq!(sender_did, String::from("zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp"))
        }
    }
}

#[test]
fn test_parse_tx_msg() {
   
    let tx = template_tx(vec!["[true, true]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();

    assert_eq!(parsed_msgs_and_pks.len(), 5);
}

#[test]
fn test_trivial_tsg_assume_participants_all_true() {
   
    let tx = template_tx(vec!["[true, true]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let (tn_ver, wn_ver) = trivial_tsg::tsg_assume_group(parsed_msgs_and_pks, trivial_tsg::HonestWho::TransactingNodes);

    assert_eq!(get_reliabilities(tn_ver), vec![1.0,1.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![1.0,1.0]);
}

#[test]
fn test_trivial_tsg_assume_participants_transacters_honest() {
   
    let tx = template_tx(vec!["[true, false]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let (tn_ver, wn_ver) = trivial_tsg::tsg_assume_group(parsed_msgs_and_pks, trivial_tsg::HonestWho::TransactingNodes);

    assert_eq!(get_reliabilities(tn_ver), vec![1.0,1.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![0.0,0.0]);
}

#[test]
fn test_trivial_tsg_assume_participants_witnesses_honest() {
   
    let tx = template_tx(vec!["[true, false]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let (tn_ver, wn_ver) = trivial_tsg::tsg_assume_group(parsed_msgs_and_pks, trivial_tsg::HonestWho::Witnesses);

    assert_eq!(get_reliabilities(tn_ver), vec![0.0,0.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![1.0,1.0]);
}

#[test]
fn test_trivial_tsg_know_outcome_all_true() {
   
    let tx = template_tx(vec!["[true, true]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let known_outcome = vec![true, true];
    let (tn_ver, wn_ver) = trivial_tsg::tsg_know_outcome(parsed_msgs_and_pks, known_outcome);

    assert_eq!(get_reliabilities(tn_ver), vec![1.0,1.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![1.0,1.0]);
}

#[test]
fn test_trivial_tsg_know_outcome_all_false() {
   
    let tx = template_tx(vec!["[false, false]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let known_outcome = vec![false, false];
    let (tn_ver, wn_ver) = trivial_tsg::tsg_know_outcome(parsed_msgs_and_pks, known_outcome);

    assert_eq!(get_reliabilities(tn_ver), vec![0.0,0.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![1.0,1.0]);
}

#[test]
fn test_trivial_tsg_know_outcome_all_false_dis_wn() {
   
    let tx = template_tx(vec!["[false, true]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let known_outcome = vec![false, false];
    let (tn_ver, wn_ver) = trivial_tsg::tsg_know_outcome(parsed_msgs_and_pks, known_outcome);

    assert_eq!(get_reliabilities(tn_ver), vec![0.0,0.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![0.0,0.0]);
}

#[test]
fn test_trivial_tsg_know_outcome_one_false_dis_wn() {
   
    let tx = template_tx(vec!["[false, true]"; 2]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let known_outcome = vec![false, true];
    let (tn_ver, wn_ver) = trivial_tsg::tsg_know_outcome(parsed_msgs_and_pks, known_outcome);

    assert_eq!(get_reliabilities(tn_ver), vec![0.0,1.0]);
    assert_eq!(get_reliabilities(wn_ver), vec![1.0,1.0]);
}

#[test]
fn test_trivial_tsg_organization() {
   
    // as each witness uses a different organization, and the trusted one
    // is the first org, the result should be true,true
    let tx = template_tx(vec!["[true, true]", "[false, false]"]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let (tn_ver, wn_ver) = trivial_tsg::tsg_organization(
        parsed_msgs_and_pks,
        String::from("z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm"),
        0.5
    );

    // because the second witness disagrees with the predicted outcome,
    // they receive low reliability
    assert_eq!(get_reliabilities(wn_ver), vec![1.0,0.0]);
    // because the predicted outcome is true, true, the participants both
    // recieve high reliability
    assert_eq!(get_reliabilities(tn_ver), vec![1.0,1.0]);
}

////
//// HELPER FUNCTIONS
////

fn get_reliabilities(tx_verdict: verdict::TxVerdict) -> Vec<f32> {
    match tx_verdict {
        verdict::TxVerdict {
            verdicts
        } => {
            return verdicts.iter().map(|ver| ver.estimated_reliablility).collect();
        }
    }
}

fn template_tx(outcomes: Vec<&str>) -> Vec<(String, String)> {
    // tx_msg
    let msg = String::from("{\"InteractionMsg\":{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3\",\"org_cert\":{\"client_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[17,20,98,109,244,104,6,196,80,158,6,241,3,116,18,97,44,19,93,0,69,1,94,69,148,89,164,19,11,220,50,217,131,215,243,88,11,66,103,81,90,137,214,184,144,59,223,176,84,66,78,92,241,189,84,229,139,139,237,195,77,181,63,6]},\"timeout\":120,\"signer_did_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"signature\":[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW\",\"org_cert\":{\"client_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[246,4,58,192,200,95,167,255,40,18,33,217,104,228,5,147,132,206,0,249,108,52,79,229,206,52,114,242,231,118,37,70,136,133,144,250,199,146,49,213,84,15,44,116,46,216,193,22,161,212,101,177,142,54,199,134,145,229,207,15,51,161,146,0]},\"timeout\":120,\"signer_did_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"signature\":[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]}],\"tx_client_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[22,31,193,148,14,208,180,103,108,100,224,161,185,241,182,117,224,113,233,226,101,117,152,244,197,89,126,146,196,73,2,195,134,73,199,124,195,146,164,60,131,31,192,75,9,24,139,4,212,13,38,113,73,54,159,206,47,57,170,32,138,209,134,4]},\"timeout\":120,\"signer_did_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"signature\":[152,79,163,65,57,165,142,1,234,105,37,128,138,14,41,106,17,147,245,14,191,102,9,23,182,149,41,134,62,200,176,68,145,92,230,128,138,228,29,82,33,61,18,155,147,67,188,65,249,25,76,59,129,201,88,234,34,179,224,147,28,130,247,7]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[172,37,72,229,140,176,147,51,220,153,84,41,254,91,183,133,199,42,111,72,76,191,197,26,163,238,216,17,136,131,212,13,169,195,160,171,97,5,254,248,110,215,112,184,49,1,27,198,243,97,91,201,127,204,156,246,115,113,193,241,192,157,118,3]},\"timeout\":120,\"signer_did_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"signature\":[219,244,111,204,67,156,96,128,44,66,151,184,187,170,1,90,149,241,164,34,109,10,84,166,192,51,227,186,121,112,0,72,186,151,17,193,56,131,173,115,163,42,77,253,170,101,39,145,52,177,68,216,218,70,161,17,6,142,208,240,229,74,86,8]}]}}");
    let message_and_pubkey1 = (msg, String::from("z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i"));

    // wn 1 msg
    let msg = format!("{{\"WitnessStatement\":{{\"outcome\":{}}}}}", outcomes[0]);
    let message_and_pubkey2 = (msg, String::from("z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3"));

    // wn 2 msg
    let msg = format!("{{\"WitnessStatement\":{{\"outcome\":{}}}}}", outcomes[1]);
    let message_and_pubkey3 = (msg, String::from("z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW"));

    // wn 1 msg
    let msg = String::from("{\"CompensationMsg\":{\"payments\":[\"wn_a: 0.01\",\"wn_b: 0.01\"]}}");
    let message_and_pubkey4 = (msg, String::from("z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i"));

    // wn 2 msg
    let msg = String::from("{\"CompensationMsg\":{\"payments\":[\"wn_a: 0.01\",\"wn_b: 0.01\"]}}");
    let message_and_pubkey5 = (msg, String::from("zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT"));

    return vec![message_and_pubkey1, message_and_pubkey2, message_and_pubkey3, message_and_pubkey4, message_and_pubkey5];
    
}