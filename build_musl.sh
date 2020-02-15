#!/bin/bash
set -ex
LLVM_ROOT=/usr/local/opt/llvm
MUSL_DIR=$PWD/../musl
#export PATH=$LLVM_ROOT/bin:$PATH

pushd $MUSL_DIR
TARGET=mips-unknown-linux-musl
export CC=clang CFLAGS="-fPIC --target=mips-linux-musl -march=mips32r2 -mtune=24kc -msoft-float" LDFLAGS="-fuse-ld=lld" AR=llvm-ar RANLIB=llvm-ranlib

# TARGET=mips64-unknown-linux-muslabi64
# export CC=clang CFLAGS="-fPIC -mabicalls --target=mips64-unknown-linux-muslabi64 -mdouble-float" LDFLAGS="-fuse-ld=lld" AR=llvm-ar RANLIB=llvm-ranlib

#TARGET=armv7-unknown-linux-muslabi64
#export CC=clang CFLAGS="-fPIC --target=armv7-unknown-linux-musleabihf" LDFLAGS="-fuse-ld=lld" AR=llvm-ar RANLIB=llvm-ranlib

# hack -lgcc -lgcc_eh no-undefined hash-style=both

./configure --host=$TARGET
#make clean
make
sudo make install DESTDIR=/opt/cross/$TARGET
# hack libunwind.a which is not used by this project
sudo ln -sf libc.a /opt/cross/$TARGET/usr/local/musl/lib/libunwind.a
