#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{debug, default_alloc};

ckb_std::entry!(main);
default_alloc!();

use alloc::{vec, vec::Vec};

use blst::min_pk::*;
use blst::*;

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn gen_random_key(rng: &mut rand_chacha::ChaCha20Rng) -> SecretKey {
    let mut ikm = [0u8; 32];
    rng.fill_bytes(&mut ikm);
    SecretKey::key_gen(&ikm, &[]).unwrap()
}

fn main() -> i8 {
    multsig_test(1);
    multsig_test(10);
    multsig_test(100);
    return 0;
}

fn multsig_test(num_msgs: usize) {
    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

    let seed = [0u8; 32];
    let mut rng = ChaCha20Rng::from_seed(seed);

    let sks: Vec<_> = (0..num_msgs).map(|_| gen_random_key(&mut rng)).collect();
    let pks = sks.iter().map(|sk| sk.sk_to_pk()).collect::<Vec<_>>();
    let pks_refs: Vec<&PublicKey> = pks.iter().map(|pk| pk).collect();

    let mut msgs: Vec<Vec<u8>> = vec![vec![]; num_msgs];
    for i in 0..num_msgs {
        let msg_len = (rng.next_u64() & 0x3F) + 1;
        msgs[i] = vec![0u8; msg_len as usize];
        rng.fill_bytes(&mut msgs[i]);
    }

    let msgs_refs: Vec<&[u8]> = msgs.iter().map(|m| m.as_slice()).collect();

    let sign_start = ckb_std::syscalls::current_cycles();
    let sigs = sks
        .iter()
        .zip(msgs.iter())
        .map(|(sk, m)| (sk.sign(m, dst, &[])))
        .collect::<Vec<Signature>>();
    let sign_end = ckb_std::syscalls::current_cycles();
    debug!(
        "Cycles consumed to sign {} messages: {}",
        num_msgs,
        sign_end - sign_start
    );

    let verify_start = ckb_std::syscalls::current_cycles();
    let errs = sigs
        .iter()
        .zip(msgs.iter())
        .zip(pks.iter())
        .map(|((s, m), pk)| (s.verify(true, m, dst, &[], pk, true)))
        .collect::<Vec<BLST_ERROR>>();
    assert_eq!(errs, vec![BLST_ERROR::BLST_SUCCESS; num_msgs]);
    let verify_end = ckb_std::syscalls::current_cycles();
    debug!(
        "Cycles consumed to verify {} signatures: {}",
        num_msgs,
        verify_end - verify_start
    );

    let sig_refs = sigs.iter().map(|s| s).collect::<Vec<&Signature>>();

    let agg_start = ckb_std::syscalls::current_cycles();
    let agg = match AggregateSignature::aggregate(&sig_refs, true) {
        Ok(agg) => agg,
        Err(err) => panic!("aggregate failure: {:?}", err),
    };

    let agg_sig = agg.to_signature();
    let agg_end = ckb_std::syscalls::current_cycles();
    debug!(
        "Cycles consumed to aggregate {} signatures: {}",
        num_msgs,
        agg_end - agg_start
    );

    let result = agg_sig.aggregate_verify(false, &msgs_refs, dst, &pks_refs, false);
    let agg_verify_end = ckb_std::syscalls::current_cycles();
    debug!(
        "Cycles consumed to verify signature aggregated from {} signatures: {}",
        num_msgs,
        agg_verify_end - agg_end
    );
    assert_eq!(result, BLST_ERROR::BLST_SUCCESS);
}
