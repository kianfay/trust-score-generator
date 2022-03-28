use trust_score_generators::{
    utility::parse_messages,
    trivial_tsg
};

use anyhow::Result;

mod trust_score_generators;

fn main() -> Result<()> {
    let msg = String::from("{\"InteractionMsg\":{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zA8xnrZEDyNXa1snoCqsp7ys6P1udw4EtS9DAeChYXbWb\",\"zG59H8ii1Dk6Vhf7H9RA95C4g1LGgpdE3uSgMqV68PJ4T\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"witnesses\":[\"zA78Vj539YdZnELKkVoBKLj88R8ZDfVHKr8PbHyQKAkXn\",\"z8YxvfusUYa2zTkcbHKsyes7EXFvZhRW5tY3rWMSMQ7Bs\"],\"wit_node_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zA8xnrZEDyNXa1snoCqsp7ys6P1udw4EtS9DAeChYXbWb\",\"zG59H8ii1Dk6Vhf7H9RA95C4g1LGgpdE3uSgMqV68PJ4T\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3\",\"timeout\":120,\"signer_did_pubkey\":\"zA78Vj539YdZnELKkVoBKLj88R8ZDfVHKr8PbHyQKAkXn\",\"signature\":[211,62,21,255,253,252,142,137,39,64,178,199,106,192,129,254,43,133,173,9,66,146,174,146,12,239,19,45,237,213,104,95,172,8,0,242,104,253,111,169,167,241,28,200,180,217,225,169,57,178,87,129,65,54,205,111,115,219,235,151,23,116,23,2]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zA8xnrZEDyNXa1snoCqsp7ys6P1udw4EtS9DAeChYXbWb\",\"zG59H8ii1Dk6Vhf7H9RA95C4g1LGgpdE3uSgMqV68PJ4T\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW\",\"timeout\":120,\"signer_did_pubkey\":\"z8YxvfusUYa2zTkcbHKsyes7EXFvZhRW5tY3rWMSMQ7Bs\",\"signature\":[216,31,22,208,194,177,106,113,172,83,15,171,223,59,131,90,55,26,197,112,14,193,137,17,141,18,72,56,219,79,15,129,13,204,165,178,159,35,245,180,162,93,36,89,28,20,11,66,74,30,92,219,127,182,4,216,91,134,207,151,46,234,68,5]}],\"tx_client_sigs\":[{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zA8xnrZEDyNXa1snoCqsp7ys6P1udw4EtS9DAeChYXbWb\",\"zG59H8ii1Dk6Vhf7H9RA95C4g1LGgpdE3uSgMqV68PJ4T\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i\",\"witnesses\":[\"zA78Vj539YdZnELKkVoBKLj88R8ZDfVHKr8PbHyQKAkXn\",\"z8YxvfusUYa2zTkcbHKsyes7EXFvZhRW5tY3rWMSMQ7Bs\"],\"wit_node_sigs\":[[211,62,21,255,253,252,142,137,39,64,178,199,106,192,129,254,43,133,173,9,66,146,174,146,12,239,19,45,237,213,104,95,172,8,0,242,104,253,111,169,167,241,28,200,180,217,225,169,57,178,87,129,65,54,205,111,115,219,235,151,23,116,23,2],[216,31,22,208,194,177,106,113,172,83,15,171,223,59,131,90,55,26,197,112,14,193,137,17,141,18,72,56,219,79,15,129,13,204,165,178,159,35,245,180,162,93,36,89,28,20,11,66,74,30,92,219,127,182,4,216,91,134,207,151,46,234,68,5]],\"timeout\":120,\"signer_did_pubkey\":\"zA8xnrZEDyNXa1snoCqsp7ys6P1udw4EtS9DAeChYXbWb\",\"signature\":[223,44,234,89,245,188,132,107,205,198,37,40,135,82,95,50,105,221,58,48,138,8,54,121,57,240,79,49,118,50,49,174,220,140,70,179,235,253,128,10,185,213,76,137,182,204,31,181,144,217,19,122,57,154,111,238,11,222,48,186,150,5,212,9]},{\"contract\":{\"contract_definition\":\"tn_b allows tn_a take their place in the queue\",\"participants\":[\"zA8xnrZEDyNXa1snoCqsp7ys6P1udw4EtS9DAeChYXbWb\",\"zG59H8ii1Dk6Vhf7H9RA95C4g1LGgpdE3uSgMqV68PJ4T\"],\"time\":1643572739,\"location\":[[53,20,27.036],[6,15,2.695]]},\"signer_channel_pubkey\":\"zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT\",\"witnesses\":[\"zA78Vj539YdZnELKkVoBKLj88R8ZDfVHKr8PbHyQKAkXn\",\"z8YxvfusUYa2zTkcbHKsyes7EXFvZhRW5tY3rWMSMQ7Bs\"],\"wit_node_sigs\":[[211,62,21,255,253,252,142,137,39,64,178,199,106,192,129,254,43,133,173,9,66,146,174,146,12,239,19,45,237,213,104,95,172,8,0,242,104,253,111,169,167,241,28,200,180,217,225,169,57,178,87,129,65,54,205,111,115,219,235,151,23,116,23,2],[216,31,22,208,194,177,106,113,172,83,15,171,223,59,131,90,55,26,197,112,14,193,137,17,141,18,72,56,219,79,15,129,13,204,165,178,159,35,245,180,162,93,36,89,28,20,11,66,74,30,92,219,127,182,4,216,91,134,207,151,46,234,68,5]],\"timeout\":120,\"signer_did_pubkey\":\"zG59H8ii1Dk6Vhf7H9RA95C4g1LGgpdE3uSgMqV68PJ4T\",\"signature\":[239,125,159,68,194,109,12,94,108,223,69,101,185,50,59,216,10,55,195,250,17,128,222,113,194,121,250,26,156,17,252,11,172,163,31,228,67,160,247,225,179,36,168,94,176,87,98,33,10,233,124,236,151,217,56,63,238,59,127,212,210,117,30,12]}]}}");
    let message_and_pubkey1 = (msg, String::from("z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i"));

    // wn 1 msg
    let msg = String::from("{\"WitnessStatement\":{\"outcome\":[true,true]}}");
    let message_and_pubkey2 = (msg, String::from("z8Q1L9E4nbBhDbduQmiABJxAgfywRsiSYkznhF71rgTy3"));

    // wn 2 msg
    let msg = String::from("{\"WitnessStatement\":{\"outcome\":[false,true]}}");
    let message_and_pubkey3 = (msg, String::from("z9qDrjFiAbG7Q953ocFD9QrWkUo5RCubhF93XRS9NkgjW"));

    // wn 1 msg
    let msg = String::from("{\"CompensationMsg\":{\"payments\":[\"wn_a: 0.01\",\"wn_b: 0.01\"]}}");
    let message_and_pubkey4 = (msg, String::from("z3aYfFFynHZ4sbhcQj7GGBTBBRvua9B5W3QXruesjrW8i"));

    // wn 2 msg
    let msg = String::from("{\"CompensationMsg\":{\"payments\":[\"wn_a: 0.01\",\"wn_b: 0.01\"]}}");
    let message_and_pubkey5 = (msg, String::from("zCYhZ2hM29Kg2WGwSuutryNPHATkZZ7RZjbeCvREvSbUT"));

    let to_pass = vec![message_and_pubkey1, message_and_pubkey2, message_and_pubkey3, message_and_pubkey4, message_and_pubkey5];
    let msgs_and_pks = parse_messages::parse_messages(&to_pass).unwrap();

    let (tn_ver, wn_ver) = trivial_tsg::tsg_assume_group(msgs_and_pks, trivial_tsg::HonestWho::TransactingNodes);

    println!("\n{:?}", tn_ver);
    println!("\n{:?}", wn_ver);

    return Ok(());
}
