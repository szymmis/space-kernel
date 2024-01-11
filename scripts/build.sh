#!/usr/bin/env sh

SRC=src
OUT=target

# Create build output directory
mkdir -p $OUT

# Compile bootloader to 16bit binary format
nasm $SRC/bootloader/main.asm -I $SRC/bootloader -o $OUT/bootloader.bin

# Compile kernel entry assebly code to 32bit ELF format
nasm $SRC/kernel/main.asm -I $SRC/kernel -f elf -o $OUT/kernel_entry.o

# Compile Rust code using target defined by `x86_32-unknown-none.json` file
cargo build --release

# Link kernel entry asm code and kernel Rust code to 32bit binary format
# **-Ttext 0x8000**: Set the entry point of the kernel to 0x8000
# That's the address where the bootloader loads the kernel code using BIOS interrupt 0x02 (Read Sector)
# **no-PIE**: Disable position independend code generation
# https://en.wikipedia.org/wiki/Position-independent_code
# All addresses will be adjusted to explicit 0x8000 offset because that's where the code is loaded
ld -m elf_i386 -no-PIE -Ttext 0x8000 $OUT/kernel_entry.o $OUT/x86_32-unknown-none/release/libspace_kernel.a -o $OUT/kernel.bin --oformat binary

# Create  empty disk image and copy bootloader and kernel code to it
# 2880x512B = 1.44MB (floppy disk size)
# conv=notrunc: Do not truncate the output file
# seek=1: Skip the first 512 bytes of the output file
dd if=/dev/zero of=$OUT/disk.img bs=512 count=2880
dd if=$OUT/bootloader.bin of=$OUT/disk.img bs=512 count=1 conv=notrunc
dd if=$OUT/kernel.bin of=$OUT/disk.img bs=512 count=2879 seek=1 conv=notrunc
