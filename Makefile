TARGET = release
BINARY = $(shell basename $(CURDIR))
INSTALL_DIR = $(HOME)/.local/bin

.PHONY: all build install clean

all: build

build:
	cargo build --$(TARGET)

install: build
	install -Dm755 target/$(TARGET)/$(BINARY) $(INSTALL_DIR)/$(BINARY)

clean:
	cargo clean
