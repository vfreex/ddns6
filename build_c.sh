#!/bin/bash
set -ex

TARGET=mips-unknown-linux-musl
SYSROOT=/opt/cross/$TARGET/usr/local/musl
LIBGCC_STATIC=/Volumes/Builder/x-tools/mips-unknown-linux-musl/lib/gcc/mips-unknown-linux-musl/7.4.0/
LIBGCC_SHARED=/Volumes/Builder/x-tools/mips-unknown-linux-musl/mips-unknown-linux-musl/sysroot/lib

export CC=clang
export CFLAGS="--target=$TARGET $CFLAGS -fPIC -march=mips32r2 -msoft-float --sysroot=$SYSROOT -I$SYSROOT/include"
export LDFLAGS="-fuse-ld=lld -nostdlib -L$LIBGCC_SHARED -L$LIBGCC_STATIC -Wl,--rpath=\$ORIGIN -Wl,--dynamic-linker=/lib/ld-musl-mips-sf.so.1"

$CC $CFLAGS -c -o foo.o foo.c
$CC $CFLAGS $LDFLAGS -shared -o libfoo.so foo.o
$CC $CFLAGS -c -o test-c-app.o main.c
$CC $CFLAGS $LDFLAGS -Wl,--no-pie -o test-c-app test-c-app.o -lfoo -L.  $SYSROOT/lib/crt1.o $SYSROOT/lib/crti.o $SYSROOT/lib/crtn.o -lc -lgcc_s # or -lgcc
scp ./test-c-app ./libfoo.so root@172.21.0.1:
