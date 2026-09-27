#!/bin/sh
set -e

cargo clean
cargo build

cp target/x86_64-unknown-none/debug/artemis iso_root/boot/artemis
cp bootloader/limine/limine-bios.sys iso_root/boot/limine-bios.sys
cp bootloader/limine/limine-bios-cd.bin iso_root/boot/limine-bios-cd.bin


rm -f artemis-x86_64.iso
xorriso -as mkisofs \
        -R -r -J \
        -b boot/limine-bios-cd.bin \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        --protective-msdos-label \
        iso_root \
        -o artemis-x86_64.iso

# Cleaning up temp files
rm -f iso_root/boot/artemis
rm -f iso_root/boot/limine-bios.sys
rm -f iso_root/boot/limine-bios-cd.bin

echo "Successfully built artemis-x86_64.iso!"
