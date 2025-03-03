SHELL := /bin/sh

VM := qemu-system-aarch64

SERIAL_SOCKET :=/tmp/virt_console.socket
VM_FLAGS := -machine raspi4b -smp 4 -m 2G -display none -serial mon:stdio -serial unix:$(SERIAL_SOCKET),server=on
VM_EXTRA_FLAGS := ""

TARGET_DIR := "$(shell pwd)/bin"
TARGET := "$(TARGET_DIR)/$(shell cargo metadata --format-version=1 | jq -r '.packages[0].name')"

.PHONY: all build install qemu clean distclean
all: build

build:
	cargo build

install:
	cargo install --path . --root . --debug

qemu: install
	$(VM) $(VM_FLAGS) $(VM_EXTRA_FLAGS) -kernel $(TARGET)

clean: 
	cargo clean

distclean: clean
	$(RM) -r $(TARGET_DIR)
