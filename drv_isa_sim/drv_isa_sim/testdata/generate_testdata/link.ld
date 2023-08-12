OUTPUT_ARCH(riscv)
SEARCH_DIR(.)

MEMORY
{
  INST_ROM (rx) : ORIGIN = 0x1000000, LENGTH = 0x8000
  RAM      (rw) : ORIGIN = 0x2000000, LENGTH = 0x8000
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

