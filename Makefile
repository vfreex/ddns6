# NOTE for Cross compilation:
# 1. If compiling for musl fails with missing libunwind, consider to remove it
#   from ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libunwind/build.rs.
# 2. Define following environment variables for OpenSSL:
#   - OPENSSL_DIR=/path/to/openssl/dir
#   - OPENSSL_STATIC=yes if static linking is needed.
# 3. Add additional Lib search path to RUSTFLAGS:
#   EXTRA_RUSTFLAGS='-L native=/path/to/native/lib'


CARGO := cargo
XARGO := xargo
USE_XARGO := 0
DEBUG := 0
STATIC := 0

BUILD_TUPLE := $(shell rustup show | grep host | awk '{split($$0,a); print a[3];}')

ifeq ($(DEBUG), 0)
CARGO_BUILD_FLAGS := --release
BUILD_TYPE := release
else
BUILD_TYPE := debug
endif

export STRIP := strip

# NOTE: Consider to add `-C panic=abort` to RUSTFLAGS if the missing panic-unwind issue occurs with xargo.
ifeq ($(STATIC), 0)
export RUSTFLAGS=-C target-feature=-crt-static $(EXTRA_RUSTFLAGS)
else
export RUSTFLAGS=-C target-feature=+crt-static -C link-args=-static-libgcc $(EXTRA_RUSTFLAGS)
endif

all: build

build:
	@echo CC=\"$(CC)\" RUSTFLAGS=\"$(RUSTFLAGS)\"
ifeq ($(USE_XARGO), 0)
	$(CARGO) $(CARGOFLAGS) build $(CARGO_BUILD_FLAGS)
ifeq ($(DEBUG), 0)
	$(STRIP) target/release/ddns6
endif
	@echo Built target/$(BUILD_TYPE)/ddns6
else
	$(XARGO) +nightly $(CARGOFLAGS) build --target=$(BUILD_TUPLE) $(CARGO_BUILD_FLAGS)
ifeq ($(DEBUG), 0)
	$(STRIP) target/$(BUILD_TUPLE)/release/ddns6
endif
	@echo Built target/$(BUILD_TUPLE)/release/ddns6
endif

cross/%: TARGET=$*
cross/%: CHOST=$*
cross/%: CC=$(CHOST)-gcc
cross/%: CXX=$(CHOST)-g++
cross/%: STRIP=$(CHOST)-strip
cross/%:
	@echo TARGET=\"$(TARGET)\" CHOST=\"$(CHOST)\" CC=\"$(CC)\" RUSTFLAGS=\"$(RUSTFLAGS)\"
ifeq ($(USE_XARGO), 0)
	$(CARGO) $(CARGOFLAGS) build $(CARGO_BUILD_FLAGS) --target=$(TARGET)
else
	$(XARGO) +nightly $(CARGOFLAGS) build $(CARGO_BUILD_FLAGS) --target=$(TARGET)
endif
ifeq ($(DEBUG), 0)
	$(STRIP) target/$(TARGET)/release/ddns6
endif
	@echo Built target/$(TARGET)/$(BUILD_TYPE)/ddns6

clean:
	$(CARGO) clean


.PHONY: all build clean install cross cross/%
