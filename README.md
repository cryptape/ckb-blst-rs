# ckb-blst-rs
[ckb-blst-rs](./bindings/rust) is a fork of [blst](https://github.com/supranational/blst) with performance optimizations for CKB.
The original documentation is available at [README-original.md](./README-original.md).

## Building

You will need a riscv64 gcc to compile this crate. Set the environment variable `CC_riscv64imac_unknown_none_elf` to your riscv64 gcc compiler,
e.g. running `export CC_riscv64imac_unknown_none_elf=riscv64-unknown-elf-gcc`.
One way to install the required toolchain is to use [nix](https://nixos.org/), a universal package manager, on normal Linux distros, WSL, and macOS.
Follow the [official nix installation instructions](https://nixos.org/download.html) to install nix.
You may also follow the platform-dependent instructions below to install a riscv64 gcc and then build this crate with

```
git submodule update --init
cd bindings/rust
rustup target add riscv64imac-unknown-none-elf
cargo build --target=riscv64imac-unknown-none-elf
```


### Nix
```
export CC_riscv64imac_unknown_none_elf="$(nix --extra-experimental-features nix-command build --print-out-paths --no-link "nixpkgs#pkgsCross.riscv64-embedded.stdenv.cc.out" 2>/dev/null)/bin/riscv64-none-elf-gcc"
```


### Arch Linux
```
pacman -S riscv64-elf-gcc
export CC_riscv64imac_unknown_none_elf=riscv64-elf-gcc
```

### Ubuntu
```
apt install gcc-riscv64-unknown-elf
export CC_riscv64imac_unknown_none_elf=riscv64-unknown-elf-gcc
```

## Usage
TODO: add a link to the docs.rs when it is alive

An example of integrating blst to you ckb contracts can be found at [here](./bindings/rust-examples/ckb).
Note that in order to verify the multisig of multiple parties to the same message effectively,
we need to aggregate `AggregateSignature::aggregate()` beforehand. See [the example](./bindings/rust-examples/ckb) for details.
