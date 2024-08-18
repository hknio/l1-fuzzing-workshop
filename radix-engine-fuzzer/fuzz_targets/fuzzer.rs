#![no_main]

use libfuzzer_sys::fuzz_target;
use radix_engine_common::dec;
use scrypto::{address::Bech32Encoder, prelude::{FromPublicKey, NonFungibleGlobalId, RADIX_TOKEN}};
use scrypto_unit::TestRunner;
use transaction::builder::ManifestBuilder;
use utils::ContextualDisplay;

fuzz_target!(|data: &[u8]| {

    let mut test_runner = TestRunner::builder().without_trace().build();
    let (public_key, _, account) = test_runner.new_allocated_account();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(account, dec!("10"))
        .create_proof_from_account_by_amount(account, RADIX_TOKEN, dec!("1"))
        .create_proof_from_account_by_amount(account, RADIX_TOKEN, dec!("1"))
        .create_proof_from_account_by_amount(account, RADIX_TOKEN, dec!("1"))
        .create_proof_from_account_by_amount(account, RADIX_TOKEN, dec!("1"))
        .build();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    //println!("{}", receipt.display(&Bech32Encoder::for_simulator()));

    // Assert
    receipt.expect_commit_success();

});
