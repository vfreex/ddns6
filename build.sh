#!/bin/bash
set -ex
LLVM_ROOT=/usr/local/opt/llvm
export PATH=$LLVM_ROOT/bin:$PATH
export CC=clang
export CFLAGS="-msoft-float --sysroot=/opt/cross/mips-unknown-linux-musl/usr/local/musl/"
export LDFLAGS="-fuse-ld=lld"
rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
export RUST_COMPILER_RT_ROOT=$PWD/../llvm-project/compiler-rt
RUSTFLAGS="-C target-feature=+crt-static -C linker=lld -L native=/opt/cross/mips-unknown-linux-musl/usr/local/musl/lib" xargo build --release --target=mips-unknown-linux-musl
llvm-strip target/mips-unknown-linux-musl/release/test-rust
scp target/mips-unknown-linux-musl/release/test-rust root@172.21.0.1:
