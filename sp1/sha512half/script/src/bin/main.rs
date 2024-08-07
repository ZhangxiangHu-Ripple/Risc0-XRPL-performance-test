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

use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::time::{Instant};
use sha2::{Digest, Sha512};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA512HALF_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

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

    // Program here
    let base_message = "test data";
    let num_hash = 1u32;

    let digest = Sha512::digest(&base_message.as_bytes());
    let truncated_bytes = &digest[..32];
    // let mut first_256_bits = [0u8; 32];
    println!("{:?}", hex::encode(&truncated_bytes));

    println!("Digest: {:?}", digest);
    println!("Truncated bytes: {:?}", truncated_bytes);

    let mut input_str: Vec<String> = Vec::new();

    for i in 0..num_hash {
        let test_message = format!("{}{}", base_message, i);

        input_str.push(test_message);
    }
    println!("Input value: {:?}", input_str);

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&input_str);

    // println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(SHA512HALF_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        // println!("n: {}", n);
        // println!("a: {}", a);
        // println!("b: {}", b);

        // let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        // assert_eq!(a, expected_a);
        // assert_eq!(b, expected_b);
        // println!("Values are correct!");

        // Record the number of cycles executed.
        // println!("Execute output is: {:?}", output);
        println!("The execution output is: {:?}\n", output);
        println!("The execution report is: {:?}\n", report);
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        println!("**********************************");
        println!("Generating proof receipt now");
        let start_proof = Instant::now();

        // Setup the program for proving.
        let (pk, vk) = client.setup(SHA512HALF_ELF);

        // Generate the default proof
        let mut proof = client.prove(&pk, stdin).run().expect("failed to generate proof");

        // Generate the compressed proof
        // let mut proof = client.prove(&pk, stdin).compressed().run().expect("proving failed");

        let end_proof = Instant::now();
        let elapsed_proof = end_proof - start_proof;
        println!("Generating proof done");
        println!("**********************************");
        // println!("Successfully generated proof!");

        let a = proof.public_values.read::<Vec<[u8; 32]>>();
        // let (output, report) = client.execute(SHA512HALF_ELF, stdin).run().unwrap();

        // println!("The proof output is: {:?}", output);
        println!("The proof output is: {:?}", a);

        // Verify the proof.
        println!("**********************************");
        println!("Verifying proof receipt now");
        let start_verify = Instant::now();

        client.verify(&proof, &vk).expect("failed to verify proof");
        // println!("Successfully verified proof!");

        let end_verify = Instant::now();
        let elapsed_verify = end_verify - start_verify;
        println!("Verifying proof done");
        println!("**********************************");

        proof
            .save("proof-with-io.json")
            .expect("saving proof failed");

        println!("successfully generated and verified proof for the program!");
        println!("Elapsed time for proof generation: {:?}", elapsed_proof);
        println!("Elapsed time for proof verification: {:?}", elapsed_verify);
    }
}
