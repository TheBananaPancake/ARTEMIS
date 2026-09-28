#!/bin/sh
set -e

cargo clean
cargo build

cp target/x86_64-unknown-none/debug/artemis iso_root/boot/artemis
cp bootloader/limine/limine-bios.sys iso_root/boot/limine-bios.sys
cp bootloader/limine/limine-bios-cd.bin iso_root/boot/limine-bios-cd.bin
cp bootloader/limine/limine-uefi-cd.bin iso_root/boot/limine-uefi-cd.bin

rm -f artemis-x86_64.iso
xorriso -as mkisofs \
        -R -r -J \
        -b boot/limine-bios-cd.bin \
        -no-emul-boot \
        -boot-load-size 4 \
        -boot-info-table \
        -hfsplus -apm-block-size 2048 \
        --efi-boot boot/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image \
        --protective-msdos-label \
        iso_root \
        -o artemis-x86_64.iso

./bootloader/limine/limine bios-install artemis-x86_64.iso

echo "Successfully built artemis-x86_64.iso!"
