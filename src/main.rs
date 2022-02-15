mod trust_score_generators;

fn main() {
    let mut arr1: Vec<u8> = Vec::from([12,14,12,13,12,11,11,15]);
    arr1.sort();
    println!("{:?}", arr1);
}
