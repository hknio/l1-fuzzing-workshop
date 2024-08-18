#![no_main]

use std::io::Write;

use std::sync::Mutex;
use libfuzzer_sys::fuzz_target;
use radix_engine_common::dec;
use scrypto::{address::Bech32Encoder, prelude::{manifest_decode, manifest_encode, FromPublicKey, NonFungibleGlobalId, RADIX_TOKEN}};
use scrypto_unit::TestRunner;
use transaction::builder::ManifestBuilder;
use utils::ContextualDisplay;
use once_cell::sync::Lazy;

static TEST_RUNNER: Lazy<Mutex<TestRunner>> = Lazy::new(|| {
    Mutex::new(TestRunner::builder().without_trace().build())
});

fuzz_target!(|data: &[u8]| {
    let manifest = manifest_decode(&data);
    if manifest.is_err() {
        return;
    }
    let manifest = manifest.unwrap();

    let mut test_runner = TEST_RUNNER.lock().unwrap();
    let (public_key, _, account) = test_runner.new_allocated_account();
    let receipt = test_runner.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    if receipt.is_commit_success() {
        panic!("Execution was ok: {:?}", receipt);
    }
});
