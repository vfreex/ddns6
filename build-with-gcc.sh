#!/bin/bash
set -ex
TARGET=mips-unknown-linux-musl
SYSROOT=/Volumes/Builder/x-tools/$TARGET/$TARGET/sysroot
# LIBGCC_STATIC=/Volumes/Builder/x-tools/$TARGET/lib/gcc/$TARGET/7.4.0/
# LIBGCC_SHARED=/Volumes/Builder/x-tools/$TARGET/$TARGET/sysroot/lib
GCC_BIN=/Volumes/Builder/x-tools/$TARGET/bin
export PATH=$GCC_BIN:$PATH

export CC=mips-unknown-linux-musl-gcc
export CFLAGS="-fPIC -march=mips32r2 -mtune=24kc -msoft-float" # -nostdinc --sysroot=$SYSROOT -I$SYSROOT/include -Os"
#export LDFLAGS="-L$SYSROOT/usr/lib" #"-fuse-ld=lld -nostdlib"

export RUST_COMPILER_RT_ROOT=$PWD/../llvm-project/compiler-rt
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/build.rs
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/lib.rs

# link with GCC
export RUSTFLAGS="-C target-feature=-crt-static -C linker=mips-unknown-linux-musl-gcc -L native=$SYSROOT/usr/lib"
#RUSTFLAGS="$RUSTFLAGS -C link-arg=-znotext"

# cleanup
#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
# rm -rf ~/.xargo
# cargo clean

# custom target
export RUST_TARGET_PATH=$PWD
TARGET=mips-openwrt-linux-musl

# build
RUSTC_ARGS=
xargo -v rustc --target=$TARGET -- $RUSTC_ARGS # -C link-arg=-test
#cargo -v rustc --target=$TARGET -- $RUSTC_ARGS

# strip and copy
llvm-strip target/$TARGET/*/test-rust
scp target/$TARGET/debug/test-rust root@172.21.0.1:
#scp target/$TARGET/debug/test-rust yux@172.23.1.2:
scp target/$TARGET/debug/test-rust yux@172.22.0.1:
