#!/usr/bin/env sh

SRC=src
OUT=dist

# Create build output directory
mkdir -p $OUT

# Compile bootloader to 16bit binary format
nasm $SRC/bootloader/main.asm -I $SRC/bootloader -o $OUT/bootloader.bin

# Compile kernel entry assebly code to 32bit ELF format
nasm $SRC/kernel/kernel_entry.asm -f elf -o $OUT/kernel_entry.o
# Compile kernel C code to 32bit ELF format
# -ffreestanding: Do not use the C standard libraries
# -m32: Generate 32-bit code
# -fno-PIC: Disable position independent code
gcc -ffreestanding -m32 -fno-PIC -c $SRC/kernel/main.c -o $OUT/kernel.o
# Link kernel entry and kernel C code to 32bit binary format
# -Ttext 0x1000: Set the entry point of the kernel to 0x1000
# That's the address where the bootloader loads the kernel code using BIOS interrupt 0x02 (Read Sector)
# -no-PIE: Disable position independent executables
ld -m elf_i386 -no-PIE -Ttext 0x1000 $OUT/kernel_entry.o $OUT/kernel.o -o $OUT/kernel.bin --oformat binary

# Create  empty disk image and copy bootloader and kernel code to it
# 2880x512B = 1.44MB (floppy disk size)
# conv=notrunc: Do not truncate the output file
# seek=1: Skip the first 512 bytes of the output file
dd if=/dev/zero of=$OUT/disk.img bs=512 count=2880
dd if=$OUT/bootloader.bin of=$OUT/disk.img bs=512 count=1 conv=notrunc
dd if=$OUT/kernel.bin of=$OUT/disk.img bs=512 seek=1 conv=notrunc
