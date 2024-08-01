use risc0_zkvm::guest::env;

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

fn main() {
    // TODO: Implement your guest code here

    // Start CPU cycle count
    let start = env::cycle_count();
    
    // read the input
    // let input: u32 = env::read();
    let signatures: Vec<(EncodedPoint, Vec<u8>, Signature)> = env::read();

    // TODO: do something with the input
    for(encoded_verifying_key, message, signature) in &signatures {
        let verifying_key = VerifyingKey::from_encoded_point(encoded_verifying_key).unwrap();
        verifying_key.verify(message, signature).expect("ECDSA verification failed.");
    }

    // write public output to the journal
    let output = true;
    env::commit(&output);
    // eprintln!("cycle count: {}", end - start);
    // eprintln!("cycle count: {:?}", signatures);
    let end = env::cycle_count();
    eprintln!("cycle count: {}", end - start);
}
