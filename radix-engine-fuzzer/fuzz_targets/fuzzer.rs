#![no_main]

use std::sync::Mutex;
use libfuzzer_sys::fuzz_target;
use scrypto::{address::Bech32Encoder, crypto::EcdsaSecp256k1PublicKey, prelude::{manifest_decode, manifest_encode, FromPublicKey, NonFungibleGlobalId, RADIX_TOKEN}};
use scrypto_unit::TestRunner;
use transaction::{builder::ManifestBuilder, model::TransactionManifest};
use once_cell::sync::Lazy;

extern "C" {
    fn __llvm_profile_reset_counters();
}

struct Fuzzer {
    test_runner: TestRunner,
    public_key: EcdsaSecp256k1PublicKey
}

impl Fuzzer {
    pub fn new() -> Self {
        let mut test_runner = TestRunner::builder().without_trace().build();
        let (public_key, _, account) = test_runner.new_allocated_account();
        #[cfg(feature = "coverage")]
        unsafe {
            // reset coverage data from TestRunner::builder and new_allocated_account
            __llvm_profile_reset_counters();
        }
        Self { test_runner, public_key }
    }

    pub fn fuzz(&mut self, manifest: TransactionManifest) {
        let receipt = self.test_runner.execute_manifest(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&self.public_key)],
        );
    
        if receipt.is_commit_success() {
            println!("Execution was ok");
        }
    }
}

static FUZZER: Lazy<Mutex<Fuzzer>> = Lazy::new(|| {
    Mutex::new(Fuzzer::new())
});

fuzz_target!(|data: &[u8]| {
    let manifest = manifest_decode(&data);
    if manifest.is_err() {
        return;
    }
    let manifest : TransactionManifest = manifest.unwrap();
    let mut fuzzer = FUZZER.lock().unwrap();
    fuzzer.fuzz(manifest);
});
