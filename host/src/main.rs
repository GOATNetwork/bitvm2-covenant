use std::env;
use std::fs::File;
use std::io::Read;
use zkm2_sdk::{include_elf, utils, ProverClient, ZKMProofWithPublicValues, ZKMStdin};

const ELF: &[u8] = include_elf!("bitvm2-covenant");

fn prove_revm() {
    let mut stdin = ZKMStdin::new();
    let goat_withdraw_txid: Vec<u8> =
        hex::decode(std::env::var("GOAT_WITHDRAW_TXID").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    //assert!(goat_withdraw_txid.len() == 32);
    stdin.write(&goat_withdraw_txid);
    // size: 20bytes
    let withdraw_contract_address: Vec<u8> =
        hex::decode(std::env::var("WITHDRAW_CONTRACT_ADDRESS").unwrap_or("86a77bdfcaff7435e1f1df06a95304d35b112ba8".to_string()))
            .unwrap();
    stdin.write(&withdraw_contract_address);
    //assert!(withdraw_contract_address.len() == 20);

    let withdraw_map_base_key = 
        hex::decode(std::env::var("WITHDRAW_MAP_BASE_KEY").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    stdin.write(&withdraw_map_base_key);
    let withdraw_map_index = 
        hex::decode(std::env::var("WITHDRAW_MAP_INDEX").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    stdin.write(&withdraw_map_index);
    let peg_in_txid: Vec<u8> =
        hex::decode(std::env::var("PEG_IN_TXID").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    stdin.write(&peg_in_txid);

    // 1. split ELF into segs
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let json_path =
        env::var("JSON_PATH").unwrap_or(format!("{}/../test-vectors/3168249.json", manifest_path));
    let mut f = File::open(json_path).unwrap();
    let mut data = vec![];
    f.read_to_end(&mut data).unwrap();
    stdin.write(&data);

    // Create a `ProverClient` method.
    let client = ProverClient::new();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(ELF, stdin.clone()).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let proof = client.prove(&pk, stdin).run().unwrap();

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Test a round trip of proof serialization and deserialization.
    proof.save("proof-with-pis.bin").expect("saving proof failed");
    let deserialized_proof =
        ZKMProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client.verify(&deserialized_proof, &vk).expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}

fn main() {
    utils::setup_logger();
    prove_revm();
}
