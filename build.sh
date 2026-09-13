#!/bin/sh
set -e

cargo clean
cargo build

cp target/x86_64-unknown-none/debug/blaze iso_root/boot/blaze
cp bootloader/limine/limine-bios.sys iso_root/boot/limine-bios.sys
cp bootloader/limine/limine-bios-cd.bin iso_root/boot/limine-bios-cd.bin


rm -f blaze-x86_64.iso
xorriso -as mkisofs \
        -R -r -J \
        -b boot/limine-bios-cd.bin \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        --protective-msdos-label \
        iso_root \
        -o blaze-x86_64.iso

# Cleaning up temp files
rm -f iso_root/boot/blaze
rm -f iso_root/boot/limine-bios.sys
rm -f iso_root/boot/limine-bios-cd.bin

echo "Successfully built blaze-x86_64.iso!"
