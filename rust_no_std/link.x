MEMORY
{
	FLASH : ORIGIN = 0x00000000, LENGTH = 256K
	RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* entry point is the reset handler */
ENTRY(Reset);

/* don't let linker discard reset vector */
EXTERN(RESET_VECTOR);

SECTIONS
{
	.vector_table ORIGIN(FLASH) :
	{
		/* first entry : initial stack poiter val .. remember stack grows down */
		LONG(ORIGIN(RAM) + LENGTH(RAM));

		/* second entry: reset vector */
		KEEP(*(.vector_table.reset_vector));
	} > FLASH

	.text :
	{
		/* include all input sections named like this */
		/* we don't use KEEP to let linker discard unused sections */
		*(.text .text.*);
	} > FLASH

	.rodata :
	{
		*(.rodata .rodata.*);
	} > FLASH

	.bss :
	{
		_sbss = .;
		*(.bss .bss.*);
		_ebss = .;
	} > RAM

	.data : AT(ADDR(.rodata) + SIZEOF(.rodata))
	{
		_sdata = .;
		*(.data .data.*);
		_edata = .;
	} > RAM

	/* we associate a symbol to the load addr of .data for later use */
	_sidata = LOADADDR(.data);

	/DISCARD/ :
	{
		/* these sections deal with exception handling and we explicitly discard them */
		*(.ARM.exidx .ARM.exidx.*);
	}
}

/* we send text (our code), rodata, vector table etc to FLASH because these fields won't be modified at runtime. */
/* on the contrary, data, bss can be modified, so they sit in RAM */
/* EXTERN + KEEP tells linker to keep extern symbols or sections that entry point doesn't call explicitly */

/* .data contains static vars with non-zero initial value */
/* .data section's virtual address is in RAM - its static vars are located there */
/* but its load address must be in non-volatile mem (in FLASH) */
/* cause the initial values of those static vars, however, must be allocated there */
/* the LMA is where in Flash those initial values are stored. */