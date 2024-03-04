#[cfg(test)]
mod zk_fibo_verify_test {

    use sp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};

    /// The ELF we want to execute inside the zkVM.
    const ELF: &[u8] = include_bytes!("../elf/riscv32im-succinct-zkvm-elf");

    #[test]
    fn it_verifies_fibo() {
        // Setup a tracer for logging.
        utils::setup_tracer();

        // Create an input stream.
        let stdin = SP1Stdin::new();

        // Generate the proof for the given program.
        let proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

        // Verify proof.
        SP1Verifier::verify(ELF, &proof).expect("verification failed");

        // Save the proof.
        proof
            .save("proof-with-pis.json")
            .expect("saving proof failed");

        println!("succesfully generated and verified proof for the program!")
    }
}
