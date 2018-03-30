OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)

MEMORY
{
  ROM	   : ORIGIN = 0x08000000, LENGTH = 32M
	IWRAM	 : ORIGIN = 0x03000000, LENGTH = 32K
	EWRAM	 : ORIGIN = 0x02000000, LENGTH = 256K
}

__text_start	=	ORIGIN(rom);

SECTIONS
{
  . = __text_start;

  /* static data stored in cart ROM */
  .header :
  {
    KEEP(*(.header.header));
    /* KEEP(*(.header.reset_vector)); */
  } > ROM

  .text : { 
    *(.text .text.*); 
    . = ALIGN(4); 
  } > ROM

  .rodata : { 
    *(.rodata .rodata.*); 
    . = ALIGN(4); 
  } > ROM

  /* dynamic data placed in IWRAM */
  .data : { 
    _sidata = LOADADDR(.data);
    . = ALIGN(4);
    _sdata = .;

    *(.data);

    . = ALIGN(4);
     _edata = .; 
  } > IWRAM AT > ROM

  .bss : {
    . = ALIGN(4);
    _sbss = .;

    *(.bss); 

    . = ALIGN(4); 
    _ebss = .;  
  } > IWRAM

  /* discard arm symbols */
  /DISCARD/ : { *(.ARM.exidx.*); }
}
