#!/bin/sh -e

TOP="$(dirname "$0")"

CC="riscv64-none-elf-gcc"
CFLAGS="-DUSE_MUL_MONT_384_ASM -DCKB_DECLARATION_ONLY -fno-builtin-printf -fPIC -O3 -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -g -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections -I ../ckb-miscellaneous-scripts/deps/ckb-c-stdlib-202106 -I ../ckb-miscellaneous-scripts/deps/ckb-c-stdlib-202106/libc"

(cd "${TOP}"; set -x; "${CC}" ${CFLAGS} -c -o build/ckb-vm/server.o src/server.c) &
(cd "${TOP}"; set -x; "${CC}" ${CFLAGS} -c -o build/ckb-vm/blst_mul_mont_384.o build/ckb-vm/blst_mul_mont_384.riscv.S) &
(cd "${TOP}"; set -x; "${CC}" ${CFLAGS} -c -o build/ckb-vm/blst_mul_mont_384x.o build/ckb-vm/blst_mul_mont_384x.riscv.S) &
wait
"${AR:-ar}" rcs build/ckb-vm/libblst.a build/ckb-vm/*.o
