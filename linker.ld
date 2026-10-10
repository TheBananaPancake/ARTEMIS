OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

PHDRS
{
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0xffffffff80000000;
    .text :
    {
        *(.text .text.*)
    } :text

    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata .rodata.*)
    } :rodata

    .requests :
    {
        KEEP(*(.requests-start))
        KEEP(*(.requests))
        KEEP(*(.requests-end))
    } :rodata

    . = ALIGN(4096);
    .data :
    {
        *(.data .data.*)
    } :data

    .bss :
    {
        *(.bss .bss.*)
    } :data
}
