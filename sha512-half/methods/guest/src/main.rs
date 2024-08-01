use risc0_zkvm::guest::env;
use sha2::{Digest, Sha512};

fn main() {
    // TODO: Implement your guest code here

    // Start CPU cycle count
    let start = env::cycle_count();
    
    // read the input
    let input: Vec<String> = env::read();
    let mut digests: Vec<[u8; 32]> = Vec::new();

    for item in &input {
        let digest = Sha512::digest(&item.as_bytes());
        // let truncated_bytes = &digest[..32];
    
        let mut first_256_bits = [0u8; 32];
        first_256_bits.copy_from_slice(&digest[..32]);
        // let new_digest = Digest::try_from(first_256_bits.as_slice()).unwrap();

        digests.push(first_256_bits);
    }

    // TODO: do something with the input
    // let output = 8u32;
    // write public output to the journal
    env::commit(&digests);
    let end = env::cycle_count();
    eprintln!("cycle count: {}", end - start);
}
