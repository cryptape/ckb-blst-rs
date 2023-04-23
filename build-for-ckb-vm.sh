#!/bin/sh -e

TOP="$(dirname "$0")"

CC="${CC:-riscv64-unknown-linux-gnu-gcc}"
CFLAGS="-DBUILD_FOR_CKB_VM -DUSE_MUL_MONT_384_ASM -DCKB_DECLARATION_ONLY -fno-builtin-printf -fPIC -O3 -nostdinc -nostdlib -nostartfiles -fvisibility=hidden -Wall -Werror -Wno-nonnull -Wno-nonnull-compare -Wno-unused-function -g -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections -I deps/ckb-c-stdlib -I deps/ckb-c-stdlib/libc"

(cd "${TOP}"; set -x; "${CC}" ${CFLAGS} -c -o build/ckb-vm/server-asm.o src/server.c) &
(cd "${TOP}"; set -x; "${CC}" ${CFLAGS} -c -o build/ckb-vm/blst_mul_mont_384.o build/ckb-vm/blst_mul_mont_384.riscv.S) &
(cd "${TOP}"; set -x; "${CC}" ${CFLAGS} -c -o build/ckb-vm/blst_mul_mont_384x.o build/ckb-vm/blst_mul_mont_384x.riscv.S) &
wait
"${AR:-ar}" rcs "${TOP}/build/ckb-vm/libblst.a" "${TOP}/build/ckb-vm/"*.o
