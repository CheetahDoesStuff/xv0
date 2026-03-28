KERNEL_BIN = target/x86_64-cheetah-vx0/debug/bootimage-xv0.bin
QEMU = qemu-system-x86_64

all: build run
build:
	cargo bootimage

run:
	$(QEMU) -drive format=raw,file=$(KERNEL_BIN)