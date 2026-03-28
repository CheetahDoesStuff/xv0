ASM = nasm
ASMFLAGS = -f bin

SRC = $(wildcard *.asm)
BIN = $(SRC:.asm=.bin)

all: boot.bin

boot.bin: boot.asm
	$(ASM) $(ASMFLAGS) $< -o $@

run: boot.bin
	qemu-system-x86_64 -drive format=raw,file=boot.bin -display sdl

clean:
	rm -f *.bin