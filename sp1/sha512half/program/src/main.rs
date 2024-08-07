// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use sha2::{Digest, Sha512};

// use alloy_sol_types::SolType;
// use fibonacci_lib::{fibonacci, PublicValuesStruct};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // let n = sp1_zkvm::io::read::<u32>();
    let input = sp1_zkvm::io::read::<Vec<String>>();

    let mut digests: Vec<[u8; 32]> = Vec::new();

    println!("The input: {:?}", input);

    for item in &input {
        let digest = Sha512::digest(&item.as_bytes());

        let mut first_256_bits = [0u8; 32];
        first_256_bits.copy_from_slice(&digest[..32]);

        // digests.push(hex::encode(&truncated_bytes));
        digests.push(first_256_bits);
    }

    // let output = 8u128;

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&bytes);
    // sp1_zkvm::io::commit(&output);
    sp1_zkvm::io::commit(&digests);
}
