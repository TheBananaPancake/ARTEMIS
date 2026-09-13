OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(main)

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
        *(.text.main)
    } :text

    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata)
        *(.rodata.*)
    } :rodata

    .limine-requests :
    {
        KEEP(*(.limine-requests-start))
        KEEP(*(.limine-requests-base))
        KEEP(*(.limine-requests-end))
    } :rodata

    . = ALIGN(4096);
    .data :
    {
        *(.data)
        *(.data.*)
    } :data

    .bss :
    {
        *(.bss)
        *(.bss.*)
    } :data
}
