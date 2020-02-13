#!/bin/bash
set -ex
LLVM_ROOT=/usr/local/opt/llvm
#TARGET=x86_64-unknown-linux-musl
TARGET=mips-unknown-linux-musl
#TARGET=armv7-unknown-linux-musleabihf
#TARGET=mips64-unknown-linux-muslabi64
SYSROOT=/opt/cross/$TARGET/usr/local/musl/
export PATH=$LLVM_ROOT/bin:$PATH
export CC=clang
#export CFLAGS="-fPIC -mabicalls --target=$TARGET -mdouble-float --sysroot=$SYSROOT"
export CFLAGS="-fPIC --target=$TARGET -march=mips32r2 -mtune=24kc -msoft-float --sysroot=$SYSROOT"
export LDFLAGS="-fuse-ld=lld"
#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
# rm -rf ~/.xargo
# cargo clean
export RUST_COMPILER_RT_ROOT=$PWD/../llvm-project/compiler-rt
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/build.rs
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/lib.rs

#RUSTFLAGS="-C target-feature=-crt-static -C linker=lld -C link-arg=--sysroot=$SYSROOT -C link-arg=-dynamic-linker=/lib/ld-musl-mips-sf.so.1 -L native=/opt/cross/mips-unknown-linux-musl/usr/local/musl/lib" xargo build --target=mips-unknown-linux-musl
export RUSTFLAGS="-C target-feature=-crt-static -C linker=lld -C link-arg=--sysroot=$SYSROOT -C link-arg=-dynamic-linker=/lib/ld-musl-mips-sf.so.1 -C link-arg=-znotext -L native=$SYSROOT/lib"
xargo build --target=$TARGET
#llvm-strip target/mips-unknown-linux-musl/release/test-rust
scp target/$TARGET/debug/test-rust root@172.21.0.1:
#scp target/$TARGET/debug/test-rust yux@172.23.1.2:
#scp target/$TARGET/debug/test-rust yux@172.22.0.1:
