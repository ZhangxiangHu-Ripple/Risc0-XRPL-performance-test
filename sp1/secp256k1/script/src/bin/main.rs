//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

// use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};

use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey, signature::Verifier},
    EncodedPoint,
};
use rand_core::OsRng;
use std::time::{Instant};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SECP256K1_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "20")]
    n: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Program here.
    let mut csprng = OsRng;
    let _num_sig = 1u32;

    let message = b"This is a message that will be signed, and verified within the zkVM";
    let mut signatures: Vec<(EncodedPoint, Vec<u8>, Signature)> = Vec::new();

    for i in 0.._num_sig {
        let test_signing_key: SigningKey = SigningKey::random(&mut csprng);
        let mut test_message = message.to_vec();
        test_message.extend_from_slice(format!("-{}", i).as_bytes());
        let test_signature: Signature = test_signing_key.sign(&test_message);
        // let test_verifying_key: EncodedPoint = test_signing_key.verifying_key().to_encoded_point(true);

        signatures.push((test_signing_key.verifying_key().to_encoded_point(true), test_message, test_signature));
    }

    println!("**********************************");
    println!("Generated examples: {:?}", signatures);
    println!("**********************************");

    println!("**********************************");
    println!("Generating proof receipt now");
    let start_proof = Instant::now();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // stdin.write(&args.n);
    stdin.write(&signatures);

    // println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(SECP256K1_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(SECP256K1_ELF);

        // Generate default proof
        let mut proof = client.prove(&pk, stdin).run().expect("failed to generate proof");

        // Generate compressed proof

        // Generate Plonk proof

        let end_proof = Instant::now();
        let elapsed_proof = end_proof - start_proof;
        println!("Generating proof done");
        println!("**********************************");

        // let a = proof.public_values.read::<Vec<(EncodedPoint, Vec<u8>, Signature)>>();
        let a = proof.public_values.read::<bool>();
        println!("The output is: {:?}", a);

        // Verify the proof.
        println!("**********************************");
        println!("Verifying proof receipt now");
        let start_verify = Instant::now();

        client.verify(&proof, &vk).expect("failed to verify proof");

        let end_verify = Instant::now();
        let elapsed_verify = end_verify - start_verify;
        println!("Verifying proof done");
        println!("**********************************");

        println!("successfully generated and verified proof for the program!");
        println!("Elapsed time for proof generation: {:?}", elapsed_proof);
        println!("Elapsed time for proof verification: {:?}", elapsed_verify);
    }
}
