#![allow(unused_imports)]

extern crate cc;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(all(target_env = "msvc", target_arch = "x86_64"))]
fn assembly(file_vec: &mut Vec<PathBuf>, base_dir: &Path) {
    let files =
        glob::glob(&format!("{}/win64/*-x86_64.asm", base_dir.display()))
            .expect("unable to collect assembly files");
    for file in files {
        file_vec.push(file.unwrap());
    }
}

#[cfg(all(target_env = "msvc", target_arch = "aarch64"))]
fn assembly(file_vec: &mut Vec<PathBuf>, base_dir: &Path) {
    let files =
        glob::glob(&format!("{}/win64/*-armv8.asm", base_dir.display()))
            .expect("unable to collect assembly files");
    for file in files {
        file_vec.push(file.unwrap());
    }
}

#[cfg(all(target_pointer_width = "64", not(target_env = "msvc")))]
fn assembly(file_vec: &mut Vec<PathBuf>, base_dir: &Path) {
    file_vec.push(base_dir.join("assembly.S"))
}

fn main() {
    /*
     * Use pre-built libblst.a if there is one. This is primarily
     * for trouble-shooting purposes. Idea is that libblst.a can be
     * compiled with flags independent from cargo defaults, e.g.
     * '../../build.sh -O1 ...'.
     */
    if Path::new("libblst.a").exists() {
        println!("cargo:rustc-link-search=.");
        println!("cargo:rustc-link-lib=blst");
        return;
    }

    let mut file_vec = Vec::new();

    let blst_base_dir = match env::var("BLST_SRC_DIR") {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            let local_blst = PathBuf::from("blst");
            if local_blst.exists() {
                local_blst
            } else {
                // Reach out to ../.., which is the root of the blst repo.
                // Use an absolute path to avoid issues with relative paths
                // being treated as strings by `cc` and getting concatenated
                // in ways that reach out of the OUT_DIR.
                env::current_dir()
                    .expect("can't access current directory")
                    .parent()
                    .and_then(|dir| dir.parent())
                    .expect(
                        "can't access parent of parent of current directory",
                    )
                    .into()
            }
        }
    };
    println!("Using blst source directory {}", blst_base_dir.display());

    let c_src_dir = blst_base_dir.join("src");

    file_vec.push(c_src_dir.join("server.c"));
    if cfg!(feature = "ckb-vm") {
        let target = std::env::var("TARGET").unwrap();
        assert!(
            target.starts_with("riscv64"),
            "Feature ckb-vm is enabled, but {} is not a riscv64 target. Try to build this crate with `cargo build --target=riscv64imac-unknown-none-elf`",
            target
        );

        let asm_src_dir = c_src_dir.join("asm");
        file_vec.push(asm_src_dir.join("blst_mul_mont_384.riscv.S"));
        file_vec.push(asm_src_dir.join("blst_mul_mont_384x.riscv.S"));

        let c_deps_dir = blst_base_dir.join("deps");

        let mut cc = cc::Build::new();

        // Since ckb-vm feature is enabled, we must use a riscv gcc to compile
        // the c files. We try to be a smart guy here, i.e. if environment variables
        // like CC_riscv64imac_unknown_none_elf (the environment variable used by the crate cc
        // to determine which c compiler to use for that target) is not set,
        // we try to use compilers like riscv64-unknown-elf-gcc (if it is found).
        let target_cc_env = format!("CC_{}", target.replace("-", "_"));
        if std::env::var(&target_cc_env).is_err() {
            for command in [
                "riscv64-unknown-elf-gcc",
                "riscv64-elf-gcc",
                "riscv64-none-elf-gcc",
            ] {
                match std::process::Command::new(command).arg("-v").spawn() {
                    Ok(_) => {
                        cc.compiler(command);
                    }
                    _ => {}
                }
            }
        }
        // CFLAGS="-DBUILD_FOR_CKB_VM -DUSE_MUL_MONT_384_ASM -DCKB_DECLARATION_ONLY -fno-builtin-printf -fPIC -fvisibility=hidden -fdata-sections -ffunction-sections -O3 -nostdinc -nostdlib -nostartfiles -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -g -Wl,-static -Wl,--gc-sections -I deps/ckb-c-stdlib -I deps/ckb-c-stdlib/libc"
        cc.define("BUILD_FOR_CKB_VM", None)
            .define("USE_MUL_MONT_384_ASM", None)
            .define("CKB_DECLARATION_ONLY ", None)
            .include(c_deps_dir.join("ckb-c-stdlib"))
            .include(c_deps_dir.join("ckb-c-stdlib").join("libc"))
            .pic(true)
            .opt_level(3)
            .debug(true)
            .static_flag(true)
            .flag_if_supported("-fno-builtin-printf")
            .flag_if_supported("-fvisibility=hidden")
            .flag_if_supported("-ffunction-sections")
            .flag_if_supported("-fdata-sections")
            .flag_if_supported("-nostdinc")
            .flag_if_supported("-nostdlib")
            .flag_if_supported("-nostartfiles")
            .flag_if_supported("-Wl,-static")
            .flag_if_supported("-Wl,--gc-sections");

        cc.files(&file_vec).compile("libblst.a");
        return;
    }

    #[cfg(all(target_pointer_width = "64"))]
    assembly(&mut file_vec, &blst_base_dir.join("build"));

    // Set CC environment variable to choose alternative C compiler.
    // Optimization level depends on whether or not --release is passed
    // or implied.
    let mut cc = cc::Build::new();

    // account for cross-compilation
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (cfg!(feature = "portable"), cfg!(feature = "force-adx")) {
        (true, false) => {
            println!("Compiling in portable mode without ISA extensions");
            cc.define("__BLST_PORTABLE__", None);
        }
        (false, true) => {
            if target_arch.eq("x86_64") {
                println!("Enabling ADX support via `force-adx` feature");
                cc.define("__ADX__", None);
            } else {
                println!("`force-adx` is ignored for non-x86_64 targets");
            }
        }
        (false, false) => {
            #[cfg(target_arch = "x86_64")]
            if target_arch.eq("x86_64") && std::is_x86_feature_detected!("adx")
            {
                println!("Enabling ADX because it was detected on the host");
                cc.define("__ADX__", None);
            }
        }
        (true, true) => panic!(
            "Cannot compile with both `portable` and `force-adx` features"
        ),
    }
    cc.flag_if_supported("-mno-avx") // avoid costly transitions
        .flag_if_supported("-fno-builtin-memcpy")
        .flag_if_supported("-Wno-unused-command-line-argument");
    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }
    cc.files(&file_vec).compile("libblst.a");
}


