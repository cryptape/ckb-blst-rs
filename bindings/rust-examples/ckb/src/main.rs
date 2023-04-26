#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{debug, default_alloc};

ckb_std::entry!(main);
default_alloc!();

use alloc::{vec, vec::Vec};

use ckb_blst::min_pk::*;

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

struct BenchData {
    sk: SecretKey,
    pk: PublicKey,
    msg: Vec<u8>,
    dst: Vec<u8>,
    sig: Signature,
}

fn gen_bench_data(rng: &mut rand_chacha::ChaCha20Rng) -> BenchData {
    let mut ikm = [0u8; 32];
    rng.fill_bytes(&mut ikm);

    let sk = SecretKey::key_gen(&ikm, &[]).unwrap();
    let pk = sk.sk_to_pk();
    let dst = "BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_"
        .as_bytes()
        .to_vec();
    let msg_len = (rng.next_u64() & 0x3F) + 1;
    let mut msg = vec![0u8; msg_len as usize];
    rng.fill_bytes(&mut msg);

    let sig = sk.sign(&msg, &dst, &[]);

    let bd = BenchData {
        sk,
        pk,
        dst,
        msg,
        sig,
    };
    bd
}

fn main() -> i8 {
    let seed = [42u8; 32];
    let mut rng = ChaCha20Rng::from_seed(seed);

    let sizes = vec![8, 16, 32, 64, 128];

    let bds: Vec<_> = (0..*sizes.iter().max().unwrap())
        .map(|_| gen_bench_data(&mut rng))
        .collect();

    let msg = &bds[0].msg;
    let dst = &bds[0].dst;

    for size in sizes.iter() {
        let pks_refs = bds
            .iter()
            .take(*size)
            .map(|s| &s.pk)
            .collect::<Vec<&PublicKey>>();
        let sigs_refs = bds
            .iter()
            .take(*size)
            .map(|s| &s.sig)
            .collect::<Vec<&Signature>>();

        fast_aggregate_verify(&pks_refs, &sigs_refs, msg, dst);

        let agg_pk = match AggregatePublicKey::aggregate(&pks_refs, false) {
            Ok(agg_pks) => agg_pks.to_public_key(),
            Err(err) => panic!("aggregate failure: {:?}", err),
        };
        fast_aggregate_verify_pre_aggregated(&agg_pk, &sigs_refs, msg, dst);
    }
    return 0;
}

fn fast_aggregate_verify(
    pks_refs: &[&PublicKey],
    sigs_refs: &[&Signature],
    msg: &[u8],
    dst: &[u8],
) {
    let num_sigs = sigs_refs.len();

    let agg_start = ckb_std::syscalls::current_cycles();

    let agg_sig = match AggregateSignature::aggregate(&sigs_refs, false) {
        Ok(agg) => agg.to_signature(),
        Err(err) => panic!("aggregate failure: {:?}", err),
    };

    agg_sig.fast_aggregate_verify(true, msg, dst, pks_refs);

    let agg_end = ckb_std::syscalls::current_cycles();
    debug!(
        "Cycles consumed to fast_aggregate_verify {} aggregated signatures: {}",
        num_sigs,
        agg_end - agg_start
    );
}

fn fast_aggregate_verify_pre_aggregated(
    pk: &PublicKey,
    sigs_refs: &[&Signature],
    msg: &[u8],
    dst: &[u8],
) {
    let num_sigs = sigs_refs.len();

    let agg_start = ckb_std::syscalls::current_cycles();

    let agg_sig = match AggregateSignature::aggregate(&sigs_refs, false) {
        Ok(agg) => agg.to_signature(),
        Err(err) => panic!("aggregate failure: {:?}", err),
    };

    agg_sig.fast_aggregate_verify_pre_aggregated(true, msg, dst, pk);

    let agg_end = ckb_std::syscalls::current_cycles();
    debug!(
        "Cycles consumed to fast_aggregate_verify_pre_aggregated {} aggregated signatures: {}",
        num_sigs,
        agg_end - agg_start
    );
}
