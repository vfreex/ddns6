#!/bin/bash
set -ex
#LLVM_ROOT=/usr/local/opt/llvm
#TARGET=x86_64-unknown-linux-musl
TARGET=mips-unknown-linux-musl
#TARGET=armv7-unknown-linux-musleabihf
#TARGET=mips64-unknown-linux-muslabi64
SYSROOT=/opt/cross/$TARGET/usr/local/musl/
X_BIN_TOOLS=/Volumes/Builder/x-tools/mips-unknown-linux-musl/mips-unknown-linux-musl/bin
X_TOOLS=/Volumes/Builder/x-tools/mips-unknown-linux-musl/bin
#SYSROOT=/Volumes/Builder/x-tools/mips-unknown-linux-musl/mips-unknown-linux-musl/sysroot/usr
LIBGCC_S=/Volumes/Builder/x-tools/mips-unknown-linux-musl/mips-unknown-linux-musl/sysroot/lib
#SYSROOT=/usr/mips-linux-gnu
export PATH=$X_TOOLS:$LLVM_ROOT/bin:$PATH
export CC=clang
#export CFLAGS="-fPIC -mabicalls --target=$TARGET -mdouble-float --sysroot=$SYSROOT"
export CFLAGS="-fPIC --target=$TARGET -march=mips32r2 -mtune=24kc -msoft-float --sysroot=$SYSROOT"
export LDFLAGS="-fuse-ld=lld -nodefaultlibs"
#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
rm -rf ~/.xargo


# GCC_TOOLCHAIN=/Volumes/Builder/x-tools/mips-24kc-linux-musl
# export PATH=$GCC_TOOLCHAIN/bin:$PATH

export RUST_COMPILER_RT_ROOT=$PWD/../llvm-project/compiler-rt
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/build.rs
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/lib.rs

# link with LLD
RT=/Volumes/Builder/x-tools/mips-unknown-linux-musl/lib/gcc/mips-unknown-linux-musl/7.4.0
T=""
export RUSTFLAGS="-C target-feature=-crt-static -C linker-flavor=ld.lld -C linker=ld.lld $T -C link-arg=-dynamic-linker=/lib/ld-musl-mips-sf.so.1 -L native=$LIBGCC_S -L native=$SYSROOT/lib"

# link with GCC
#export RUSTFLAGS="-C target-feature=-crt-static -C linker=mips-24kc-linux-musl-ld -C link-arg=-nostdlib -C link-arg=-L$SYSROOT/lib -C link-arg=-L/Volumes/Builder/x-tools/mips-unknown-linux-musl/lib/gcc/mips-unknown-linux-musl/7.4.0 -C link-arg=-lgcc"
#cargo clean
cargo -v build --target=$TARGET
#llvm-strip target/mips-unknown-linux-musl/release/test-rust
scp target/$TARGET/debug/test-rust root@172.21.0.1:
#scp target/$TARGET/debug/test-rust yux@172.23.1.2:
scp target/$TARGET/debug/test-rust yux@172.22.0.1:
