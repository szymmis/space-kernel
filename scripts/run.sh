#!/usr/bin/env sh

# Boot the disk image using QEMU emulator
# mount the image as a floppy disk
qemu-system-i386 -fda target/disk.img
