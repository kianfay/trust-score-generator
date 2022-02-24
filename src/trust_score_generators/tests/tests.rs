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
    let msg = String::from("{\"TransactionMsg\":{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3\",\"org_cert\":{\"client_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[17,20,98,109,244,104,6,196,80,158,6,241,3,116,18,97,44,19,93,0,69,1,94,69,148,89,164,19,11,220,50,217,131,215,243,88,11,66,103,81,90,137,214,184,144,59,223,176,84,66,78,92,241,189,84,229,139,139,237,195,77,181,63,6]},\"timeout\":120,\"signer_did_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"signature\":[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW\",\"org_cert\":{\"client_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[246,4,58,192,200,95,167,255,40,18,33,217,104,228,5,147,132,206,0,249,108,52,79,229,206,52,114,242,231,118,37,70,136,133,144,250,199,146,49,213,84,15,44,116,46,216,193,22,161,212,101,177,142,54,199,134,145,229,207,15,51,161,146,0]},\"timeout\":120,\"signer_did_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"signature\":[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]}],\"tx_client_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[22,31,193,148,14,208,180,103,108,100,224,161,185,241,182,117,224,113,233,226,101,117,152,244,197,89,126,146,196,73,2,195,134,73,199,124,195,146,164,60,131,31,192,75,9,24,139,4,212,13,38,113,73,54,159,206,47,57,170,32,138,209,134,4]},\"timeout\":120,\"signer_did_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"signature\":[152,79,163,65,57,165,142,1,234,105,37,128,138,14,41,106,17,147,245,14,191,102,9,23,182,149,41,134,62,200,176,68,145,92,230,128,138,228,29,82,33,61,18,155,147,67,188,65,249,25,76,59,129,201,88,234,34,179,224,147,28,130,247,7]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[172,37,72,229,140,176,147,51,220,153,84,41,254,91,183,133,199,42,111,72,76,191,197,26,163,238,216,17,136,131,212,13,169,195,160,171,97,5,254,248,110,215,112,184,49,1,27,198,243,97,91,201,127,204,156,246,115,113,193,241,192,157,118,3]},\"timeout\":120,\"signer_did_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"signature\":[219,244,111,204,67,156,96,128,44,66,151,184,187,170,1,90,149,241,164,34,109,10,84,166,192,51,227,186,121,112,0,72,186,151,17,193,56,131,173,115,163,42,77,253,170,101,39,145,52,177,68,216,218,70,161,17,6,142,208,240,229,74,86,8]}]}}");
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

#[test]
fn test_trivial_tsg_organization_2() {
   
    // as each witness uses a different organization, and the trusted one
    // is the first org, the result should be true,true
    let tx = template_tx_2(vec!["[true, false]", "[true, true]"]);
    let parsed_msgs_and_pks = parse_messages::parse_messages(&tx).unwrap();
    let (tn_ver, wn_ver) = trivial_tsg::tsg_organization(
        parsed_msgs_and_pks,
        String::from("z5UFknc1fiHGq1YcrBPDrMcvSmqbqFFBFvoTdfwKyhFE2"),
        0.5
    );

    // because the second witness disagrees with the predicted outcome,
    // they receive low reliability
    assert_eq!(get_reliabilities(wn_ver), vec![1.1,0.0]);
    // because the predicted outcome is true, true, the participants both
    // recieve high reliability
    assert_eq!(get_reliabilities(tn_ver), vec![1.0,0.0]);
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
    let msg = String::from("{\"TransactionMsg\":{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3\",\"org_cert\":{\"client_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[17,20,98,109,244,104,6,196,80,158,6,241,3,116,18,97,44,19,93,0,69,1,94,69,148,89,164,19,11,220,50,217,131,215,243,88,11,66,103,81,90,137,214,184,144,59,223,176,84,66,78,92,241,189,84,229,139,139,237,195,77,181,63,6]},\"timeout\":120,\"signer_did_pubkey\":\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"signature\":[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW\",\"org_cert\":{\"client_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[246,4,58,192,200,95,167,255,40,18,33,217,104,228,5,147,132,206,0,249,108,52,79,229,206,52,114,242,231,118,37,70,136,133,144,250,199,146,49,213,84,15,44,116,46,216,193,22,161,212,101,177,142,54,199,134,145,229,207,15,51,161,146,0]},\"timeout\":120,\"signer_did_pubkey\":\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\",\"signature\":[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]}],\"tx_client_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"duration\":31536000,\"org_pubkey\":\"zHgQPBdcPABaJZuKB4y2R7Zwm2f2AUy7hFNmYiPgRk6q2\",\"signature\":[22,31,193,148,14,208,180,103,108,100,224,161,185,241,182,117,224,113,233,226,101,117,152,244,197,89,126,146,196,73,2,195,134,73,199,124,195,146,164,60,131,31,192,75,9,24,139,4,212,13,38,113,73,54,159,206,47,57,170,32,138,209,134,4]},\"timeout\":120,\"signer_did_pubkey\":\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"signature\":[152,79,163,65,57,165,142,1,234,105,37,128,138,14,41,106,17,147,245,14,191,102,9,23,182,149,41,134,62,200,176,68,145,92,230,128,138,228,29,82,33,61,18,155,147,67,188,65,249,25,76,59,129,201,88,234,34,179,224,147,28,130,247,7]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zP25M7SydfcU7JsaM4dfYrHzcSf3QpECormxRL6KAHMp\",\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT\",\"witnesses\":[\"z4SshCVR3SfWk1r5noYADgJUP7zS3Qzn1ZFTvDPBhrwRT\",\"zH1UiKoYW4QeBcdvqw5oQySmnXBY57TsuHjbEAEPwWrGF\"],\"wit_node_sigs\":[[157,249,32,16,76,132,253,165,101,61,40,111,208,178,162,6,203,108,181,236,19,134,49,117,14,250,222,247,118,59,76,13,33,241,95,198,249,148,229,254,103,47,114,96,145,175,205,72,130,94,126,71,117,87,136,214,64,105,142,193,92,191,201,9],[241,188,242,181,230,147,211,176,7,115,164,38,98,98,244,209,106,87,155,16,243,65,13,35,163,140,108,188,191,52,148,216,150,160,50,135,34,194,141,222,227,137,205,57,14,240,212,196,244,140,1,99,18,78,134,208,202,212,128,141,131,186,129,1]],\"org_cert\":{\"client_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"duration\":31536000,\"org_pubkey\":\"z8TsH2bGPCZbyfASdi4Kh3ZYTH4MNGjwFDexXs8LdqKFm\",\"signature\":[172,37,72,229,140,176,147,51,220,153,84,41,254,91,183,133,199,42,111,72,76,191,197,26,163,238,216,17,136,131,212,13,169,195,160,171,97,5,254,248,110,215,112,184,49,1,27,198,243,97,91,201,127,204,156,246,115,113,193,241,192,157,118,3]},\"timeout\":120,\"signer_did_pubkey\":\"zDgGaNLo7fHXgwWxyhQPjDsJwnKR4u6UVQEFo9pPkfXgk\",\"signature\":[219,244,111,204,67,156,96,128,44,66,151,184,187,170,1,90,149,241,164,34,109,10,84,166,192,51,227,186,121,112,0,72,186,151,17,193,56,131,173,115,163,42,77,253,170,101,39,145,52,177,68,216,218,70,161,17,6,142,208,240,229,74,86,8]}]}}");
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

fn template_tx_2(outcomes: Vec<&str>) -> Vec<(String, String)> {
    // tx_msg
    let msg = String::from("{\"TransactionMsg\":{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"witnesses\":[\"z6NoS8UJWNU4aVMsaRPth8niDPSjo2fqhzo5cDMV4KgyA\",\"z43xtYLnRQniCsZhUKJc5FVACyrzZa4C7KP6rReYcKERi\"],\"wit_node_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3\",\"org_cert\":{\"client_pubkey\":\"z6NoS8UJWNU4aVMsaRPth8niDPSjo2fqhzo5cDMV4KgyA\",\"duration\":31536000,\"org_pubkey\":\"z5UFknc1fiHGq1YcrBPDrMcvSmqbqFFBFvoTdfwKyhFE2\",\"signature\":[217,105,89,164,8,224,80,171,11,85,39,189,34,100,76,214,4,174,132,46,169,111,165,134,169,211,10,82,102,90,105,117,198,241,83,71,49,33,236,216,119,19,218,235,143,135,7,231,76,39,211,124,93,114,43,166,18,10,162,35,229,79,245,9]},\"timeout\":120,\"signer_did_pubkey\":\"z6NoS8UJWNU4aVMsaRPth8niDPSjo2fqhzo5cDMV4KgyA\",\"signature\":[195,82,143,56,84,16,195,12,53,166,206,88,48,117,87,169,35,160,127,233,180,78,86,108,51,34,249,255,10,120,216,143,34,74,170,255,139,31,45,4,14,105,30,241,25,175,27,104,218,38,147,39,57,22,185,128,10,192,131,98,228,56,247,4]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW\",\"org_cert\":{\"client_pubkey\":\"z43xtYLnRQniCsZhUKJc5FVACyrzZa4C7KP6rReYcKERi\",\"duration\":31536000,\"org_pubkey\":\"zAKYgaUFuqoryrPXB69qwgsnNzbguwoBgo76ZBphk1S56\",\"signature\":[48,3,173,100,86,163,82,169,170,89,242,50,251,128,57,79,190,127,171,234,140,115,3,112,124,183,91,124,249,109,219,8,247,221,211,202,162,192,100,162,158,239,42,174,88,74,108,174,91,95,217,111,234,39,86,243,72,201,127,7,203,239,45,10]},\"timeout\":120,\"signer_did_pubkey\":\"z43xtYLnRQniCsZhUKJc5FVACyrzZa4C7KP6rReYcKERi\",\"signature\":[35,171,129,213,131,163,220,68,174,3,175,74,198,116,122,59,26,54,240,110,79,97,39,42,3,61,5,165,51,1,54,201,240,214,204,245,209,48,136,195,61,107,44,77,186,103,219,58,190,137,143,116,247,217,175,160,145,53,148,240,210,29,155,13]}],\"tx_client_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i\",\"witnesses\":[\"z6NoS8UJWNU4aVMsaRPth8niDPSjo2fqhzo5cDMV4KgyA\",\"z43xtYLnRQniCsZhUKJc5FVACyrzZa4C7KP6rReYcKERi\"],\"wit_node_sigs\":[[195,82,143,56,84,16,195,12,53,166,206,88,48,117,87,169,35,160,127,233,180,78,86,108,51,34,249,255,10,120,216,143,34,74,170,255,139,31,45,4,14,105,30,241,25,175,27,104,218,38,147,39,57,22,185,128,10,192,131,98,228,56,247,4],[35,171,129,213,131,163,220,68,174,3,175,74,198,116,122,59,26,54,240,110,79,97,39,42,3,61,5,165,51,1,54,201,240,214,204,245,209,48,136,195,61,107,44,77,186,103,219,58,190,137,143,116,247,217,175,160,145,53,148,240,210,29,155,13]],\"org_cert\":{\"client_pubkey\":\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"duration\":31536000,\"org_pubkey\":\"zAKYgaUFuqoryrPXB69qwgsnNzbguwoBgo76ZBphk1S56\",\"signature\":[186,115,44,211,106,94,14,112,59,82,184,126,49,46,81,73,23,89,139,25,201,235,45,91,39,204,216,130,189,61,71,8,239,231,126,198,112,252,127,123,75,98,73,143,244,245,48,241,248,195,115,150,220,249,22,133,174,59,174,155,101,49,23,8]},\"timeout\":120,\"signer_did_pubkey\":\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"signature\":[101,48,92,42,163,88,34,80,123,46,31,255,76,111,160,142,148,21,231,62,178,162,236,192,226,233,71,238,28,3,247,11,146,6,26,98,111,103,89,76,102,79,146,1,181,233,10,216,86,202,247,242,126,132,253,73,40,106,175,41,205,253,55,6]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zG4ePu7PvcBB4XRSwfU2JanamUkvKEH7toyVzVQ7Cmes5\",\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT\",\"witnesses\":[\"z6NoS8UJWNU4aVMsaRPth8niDPSjo2fqhzo5cDMV4KgyA\",\"z43xtYLnRQniCsZhUKJc5FVACyrzZa4C7KP6rReYcKERi\"],\"wit_node_sigs\":[[195,82,143,56,84,16,195,12,53,166,206,88,48,117,87,169,35,160,127,233,180,78,86,108,51,34,249,255,10,120,216,143,34,74,170,255,139,31,45,4,14,105,30,241,25,175,27,104,218,38,147,39,57,22,185,128,10,192,131,98,228,56,247,4],[35,171,129,213,131,163,220,68,174,3,175,74,198,116,122,59,26,54,240,110,79,97,39,42,3,61,5,165,51,1,54,201,240,214,204,245,209,48,136,195,61,107,44,77,186,103,219,58,190,137,143,116,247,217,175,160,145,53,148,240,210,29,155,13]],\"org_cert\":{\"client_pubkey\":\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\",\"duration\":31536000,\"org_pubkey\":\"z5UFknc1fiHGq1YcrBPDrMcvSmqbqFFBFvoTdfwKyhFE2\",\"signature\":[102,54,255,255,121,29,200,149,189,57,145,0,114,95,116,197,41,56,222,203,240,124,148,134,218,110,167,152,22,216,137,89,92,32,33,150,45,71,3,193,236,217,237,229,209,80,67,3,96,127,9,195,26,229,29,31,223,239,46,93,119,146,142,7]},\"timeout\":120,\"signer_did_pubkey\":\"ziEtxw9TQAE18i7hScGJeBLeqQD68PAm2kmYxaeRfdnJ\",\"signature\":[133,22,203,201,216,62,189,149,76,172,99,24,142,51,150,221,165,223,122,136,192,185,6,119,240,151,146,176,94,124,109,204,206,64,27,212,141,235,130,243,217,101,81,230,85,191,154,193,107,227,130,210,217,160,85,98,227,229,168,241,75,146,195,13]}]}}");
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