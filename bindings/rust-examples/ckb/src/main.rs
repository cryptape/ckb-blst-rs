#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::default_alloc;

ckb_std::entry!(program_entry);
default_alloc!();

use blst::*;
use blst::min_pk::*;

/// program entry
fn program_entry() -> i8 {


    let ikm: [u8; 32] = [
        0x93, 0xad, 0x7e, 0x65, 0xde, 0xad, 0x05, 0x2a, 0x08, 0x3a, 0x91, 0x0c, 0x8b, 0x72, 0x85,
        0x91, 0x46, 0x4c, 0xca, 0x56, 0x60, 0x5b, 0xb0, 0x56, 0xed, 0xfe, 0x2b, 0x60, 0xa6, 0x3c,
        0x48, 0x99,
    ];

    let sk = SecretKey::key_gen(&ikm, &[]).unwrap();
    let pk = sk.sk_to_pk();

    let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
    let msg = b"hello foo";
    let sig = sk.sign(msg, dst, &[]);

    let err = sig.verify(true, msg, dst, &[], &pk, true);
    assert_eq!(err, BLST_ERROR::BLST_SUCCESS);
    return 0;
}
