#!/bin/bash
set -ex
LLVM_ROOT=/usr/local/opt/llvm
MUSL_DIR=$PWD/../musl
export PATH=$LLVM_ROOT/bin:$PATH

pushd $MUSL_DIR
export CC=clang CFLAGS="--target=mips-linux-musl -msoft-float" LDFLAGS="-fuse-ld=lld" AR=llvm-ar RANLIB=llvm-ranlib
./configure --host=mips-linux-musl
make clean
make
make install DEST_DIR=/opt/cross/mips-unknown-linux-musl
