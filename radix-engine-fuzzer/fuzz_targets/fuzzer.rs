#![no_main]

use std::sync::Mutex;
use libfuzzer_sys::fuzz_target;
use sbor::{Decoder, Encoder};
use scrypto::{address::Bech32Encoder, crypto::EcdsaSecp256k1PublicKey, math::test, prelude::{manifest_decode, manifest_encode, FromPublicKey, ManifestDecoder, ManifestEncoder, NonFungibleGlobalId, MANIFEST_SBOR_V1_MAX_DEPTH, MANIFEST_SBOR_V1_PAYLOAD_PREFIX, RADIX_TOKEN}};
use scrypto_unit::TestRunner;
use transaction::{builder::ManifestBuilder, model::TransactionManifest};
use once_cell::sync::Lazy;

extern "C" {
    fn __llvm_profile_reset_counters();
}

struct Fuzzer {
    test_runner: TestRunner
}

impl Fuzzer {
    pub fn new() -> Self {
        let mut test_runner = TestRunner::builder().without_trace().build();
        let merged_seed_corpus = include_bytes!("merged_seed_corpus.bin"); // run ./generate_seed_corpus.sh to generate
        let mut decoder = ManifestDecoder::new(merged_seed_corpus, MANIFEST_SBOR_V1_MAX_DEPTH);
        let mut manifest_and_nonces = vec![];
        loop {
            decoder.read_and_check_payload_prefix(MANIFEST_SBOR_V1_PAYLOAD_PREFIX).unwrap_or_default();
            let manifest_and_nonce = decoder.decode();
            if manifest_and_nonce.is_err() {
                break;
            }
            let (manifest, nonce) : (TransactionManifest, u64) = manifest_and_nonce.unwrap();
            manifest_and_nonces.push((manifest, nonce));
        }
        // sort by nonce, from low to high
        let mut successes = 0;
        let total = manifest_and_nonces.len();
        manifest_and_nonces.sort_by(|a, b| a.1.cmp(&b.1));
        for (manifest, nonce) in manifest_and_nonces {
            test_runner.set_nonce(nonce);
            let receipt = test_runner.execute_manifest(manifest, vec![]);
            if receipt.is_commit_success() {
                successes += 1;
            } else {
                //println!("{nonce} {:?}", receipt);
            }
        }
        println!("Successfully executed {} out of {} transactions", successes, total);
        test_runner.disable_commit();
        #[cfg(feature = "coverage")]
        unsafe {
            // reset coverage data from TestRunner::builder and new_allocated_account
            __llvm_profile_reset_counters();
        }
        Self { test_runner }
    }

    pub fn fuzz(&mut self, manifest: TransactionManifest, nonce: u64) {
        self.test_runner.set_nonce(nonce);
        let receipt = self.test_runner.execute_manifest(
            manifest,
            vec![],
        );
    
        //if receipt.is_commit_success() {
        //    println!("Execution was ok");
        //}
    }
}

static FUZZER: Lazy<Mutex<Fuzzer>> = Lazy::new(|| {
    Mutex::new(Fuzzer::new())
});

fuzz_target!(|data: &[u8]| {
    let manifest_and_nonce = ManifestDecoder::new(data, MANIFEST_SBOR_V1_MAX_DEPTH - 4)
        .decode_payload(MANIFEST_SBOR_V1_PAYLOAD_PREFIX);
    if manifest_and_nonce.is_err() {
        return;
    }
    let (manifest, nonce) : (TransactionManifest, u64) = manifest_and_nonce.unwrap();
    let mut fuzzer = FUZZER.lock().unwrap();
    fuzzer.fuzz(manifest, nonce);
});
