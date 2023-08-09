OUTPUT_ARCH(riscv)
SEARCH_DIR(.)

MEMORY
{
  INST_ROM (rx) : ORIGIN = 0x0001000, LENGTH = 0x8000
}

SECTIONS
{
  .text :
   {
     . = ALIGN(4);
     *(.text)
     *(.text.*)
     _etext  =  .;
     . = ALIGN(4);
  }  > INST_ROM
}

