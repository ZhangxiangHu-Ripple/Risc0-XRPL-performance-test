//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey, signature::Verifier},
    EncodedPoint,
};

// use alloy_sol_types::SolType;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // let n = sp1_zkvm::io::read::<u32>();
    let signatures = sp1_zkvm::io::read::<Vec<(EncodedPoint, Vec<u8>, Signature)>>();

    // Compute the n'th fibonacci number using a function from the workspace lib crate.
    // let (a, b) = fibonacci(n);

    // Encode the public values of the program.
    // let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });
    // let output = 8u128;
    for(encoded_verifying_key, message, signature) in &signatures {
        let verifying_key = VerifyingKey::from_encoded_point(encoded_verifying_key).unwrap();
        verifying_key.verify(message, signature).expect("ECDSA verification failed.");
    }

    let output = true;

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&bytes);
    // sp1_zkvm::io::commit(&signatures);
    sp1_zkvm::io::commit(&output);
}
