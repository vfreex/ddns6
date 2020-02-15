#!/bin/bash
set -ex
#LLVM_ROOT=/usr/local/opt/llvm
#TARGET=x86_64-unknown-linux-musl
TARGET=mips-unknown-linux-musl
#TARGET=armv7-unknown-linux-musleabihf
#TARGET=mips64-unknown-linux-muslabi64
SYSROOT=/opt/cross/$TARGET/usr/local/musl
LIBGCC_STATIC=/Volumes/Builder/x-tools/$TARGET/lib/gcc/$TARGET/7.4.0/
LIBGCC_SHARED=/Volumes/Builder/x-tools/$TARGET/$TARGET/sysroot/lib
GCC_BIN=/Volumes/Builder/x-tools/mips-unknown-linux-musl/bin
export PATH=$GCC_BIN:$PATH

export CC=clang
export CFLAGS="--target=$TARGET -fPIC -march=mips32r2 -mtune=24kc -msoft-float -nostdinc --sysroot=$SYSROOT -I$SYSROOT/include"
export LDFLAGS="-fuse-ld=lld -nostdlib"

#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
#rm -rf ~/.xargo

export RUST_COMPILER_RT_ROOT=$PWD/../llvm-project/compiler-rt
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/build.rs
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/lib.rs

# link with LLD
# don't set OBJS on static linking
#OBJS="-C link-arg=$SYSROOT/lib/crt1.o -C link-arg=$SYSROOT/lib/crti.o -C link-arg=$SYSROOT/lib/crtn.o"
export RUSTFLAGS="-C target-feature=-crt-static -C linker-flavor=ld.lld -C linker=ld.lld -C link-arg=-dynamic-linker=/lib/ld-musl-mips-sf.so.1 -L native=$SYSROOT/lib -L native=$LIBGCC_STATIC -L native=$LIBGCC_SHARED"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-znotext"

# cleanup
#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
#rm -rf ~/.xargo
# cargo clean

# custom target
export RUST_TARGET_PATH=$PWD
TARGET=mips-openwrt-linux-musl

# build
xargo -v rustc --target=$TARGET --release -- $OBJS # -C link-arg=-test

# strip and copy
llvm-strip target/$TARGET/*/test-rust
scp target/$TARGET/release/test-rust root@172.21.0.1:
#scp target/$TARGET/debug/test-rust yux@172.23.1.2:
scp target/$TARGET/release/test-rust yux@172.22.0.1:
