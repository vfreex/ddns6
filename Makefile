CARGO := cargo
DEBUG := 0
STATIC := 0

ifeq ($(DEBUG), 0)
CARGO_FLAGS := --release
BUILD_TYPE := release
else
BUILD_TYPE := debug
endif

export STRIP := strip

ifeq ($(STATIC), 0)
export RUSTFLAGS=-C linker=$(CC) -C target-feature=-crt-static
else
export RUSTFLAGS=-C linker=$(CC) -C target-feature=+crt-static -C link-args=-lgcc
endif

all: build

build:
	@echo CC=$(CC) RUSTFLAGS=$(RUSTFLAGS)
	$(CARGO) rustc $(CARGO_FLAGS)
ifeq ($(DEBUG), 0)
	$(STRIP) target/release/ddns6
endif
	@echo Built target/$(BUILD_TYPE)/ddns6

cross/%: TARGET=$*
cross/%: CHOST=$*
cross/%: CC=$(CHOST)-gcc
cross/%: STRIP=$(CHOST)-strip
cross/%:
	@echo TARGET=$(TARGET) CHOST=$(CHOST) CC=$(CC) RUSTFLAGS=$(RUSTFLAGS)
	$(CARGO) rustc $(CARGO_FLAGS) --target=$(TARGET)
ifeq ($(DEBUG), 0)
	$(STRIP) target/$(TARGET)/release/ddns6
endif
	@echo Built target/$(TARGET)/$(BUILD_TYPE)/ddns6

clean:
	$(CARGO) clean


.PHONY: all build clean install cross cross/%
