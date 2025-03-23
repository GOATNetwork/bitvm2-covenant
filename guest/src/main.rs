#![no_std]
#![no_main]

use revm::{
    db::CacheState,
    primitives::{calc_excess_blob_gas, Bytecode, Env, SpecId, TransactTo, keccak256},
    Evm,
    primitives::{b256, U256},
};

//use zkm2_zkvm::lib::hasher::Hasher;

extern crate libc;

//use models::*;
use covenant_lib::{
    recover_address,
    read_suite,
    check_withdraw,
    execute_test_suite,
};

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::boxed::Box;
use revm::primitives::Address;
zkm2_zkvm::entrypoint!(main);

const WITHDRAW_CONFIRMING: u8 = 1;
pub fn main() {
    ethereum_test();
}

fn ethereum_test() {
    // all private inputs
    // size: 32bytes
    let goat_withdraw_txid: Vec<u8> = zkm2_zkvm::io::read();
    assert!(goat_withdraw_txid.len() == 32);
    // size: 20bytes
    let withdraw_contract_address: Vec<u8> = zkm2_zkvm::io::read();
    assert!(withdraw_contract_address.len() == 20);
    // size: 20bytes
    let operator_address: Vec<u8> = zkm2_zkvm::io::read();
    assert!(operator_address.len() == 20);

    let withdraw_contract_map_index: u8 = 5;

    let tx_list: Vec<u8> = zkm2_zkvm::io::read();
    let suite = read_suite(&tx_list);

    assert!(check_withdraw(&withdraw_contract_address, &goat_withdraw_txid, withdraw_contract_map_index, &suite).is_ok());
    assert!(execute_test_suite(suite).is_ok());

    // public inputs
    zkm2_zkvm::io::commit(&goat_withdraw_txid);
    zkm2_zkvm::io::commit(&withdraw_contract_address);
}