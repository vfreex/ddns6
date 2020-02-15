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

export CC=clang
export CFLAGS="--target=$TARGET -fPIC -march=mips32r2 -mtune=24kc -msoft-float -nostdinc --sysroot=$SYSROOT -I$SYSROOT/include -Os"
export LDFLAGS="-fuse-ld=lld -nostdlib"

#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
#rm -rf ~/.xargo

export RUST_COMPILER_RT_ROOT=$PWD/../llvm-project/compiler-rt
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/build.rs
# needs to hack ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/libunwind/lib.rs

# link with LLD
OBJS="-C link-arg=$SYSROOT/lib/crt1.o -C link-arg=$SYSROOT/lib/crti.o -C link-arg=$SYSROOT/lib/crtn.o"
export RUSTFLAGS="-C target-feature=-crt-static -C linker-flavor=ld.lld -C linker=ld.lld $OBJS -C link-arg=--no-pie -C link-arg=-dynamic-linker=/lib/ld-musl-mips-sf.so.1 -L native=$SYSROOT/lib -L native=$LIBGCC_STATIC -L native=$LIBGCC_SHARED"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-znotext"
# link with GCC
#export RUSTFLAGS="-C target-feature=-crt-static -C linker=mips-24kc-linux-musl-ld -C link-arg=-nostdlib -C link-arg=-L$SYSROOT/lib -C link-arg=-L/Volumes/Builder/x-tools/mips-unknown-linux-musl/lib/gcc/mips-unknown-linux-musl/7.4.0 -C link-arg=-lgcc"
#rm -rf ~/.xargo/lib/rustlib/mips-unknown-linux-musl
#rm -rf ~/.xargo
#cargo clean
xargo -v build --target=$TARGET #--release
llvm-strip target/$TARGET/*/test-rust
scp target/$TARGET/debug/test-rust root@172.21.0.1:
#scp target/$TARGET/debug/test-rust yux@172.23.1.2:
scp target/$TARGET/debug/test-rust yux@172.22.0.1:
