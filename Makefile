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

#NOTE: Consider to add `-C panic=abort` to RUSTFLAGS if the missing panic-unwind issue occurs with xargo.
ifeq ($(STATIC), 0)
export RUSTFLAGS=-C linker=$(CC) -C target-feature=-crt-static
else
export RUSTFLAGS=-C linker=$(CC) -C target-feature=+crt-static -C link-args=-lgcc
endif

all: build

build:
	@echo CC=\"$(CC)\" RUSTFLAGS=\"$(RUSTFLAGS)\"
ifeq ($(USE_XARGO), 0)
	$(CARGO) $(CARGOFLAGS) build $(CARGO_BUILD_FLAGS)
else
	$(XARGO) $(CARGOFLAGS) build --target=$(BUILD_TUPLE) $(CARGO_BUILD_FLAGS)
endif
ifeq ($(DEBUG), 0)
	$(STRIP) target/release/ddns6
endif
	@echo Built target/$(BUILD_TYPE)/ddns6

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
	$(XARGO) $(CARGOFLAGS) build $(CARGO_BUILD_FLAGS) --target=$(TARGET)
endif
ifeq ($(DEBUG), 0)
	$(STRIP) target/$(TARGET)/release/ddns6
endif
	@echo Built target/$(TARGET)/$(BUILD_TYPE)/ddns6

clean:
	$(CARGO) clean


.PHONY: all build clean install cross cross/%
