SECTIONS 
{
	.log 0 (INFO) : {
		*(.log);
		/* *(.log.error);
		__log_warning_start__ = .;
		*(.log.warning); */
	}
}

/* (INFO) mark is a non-allocatable section */
/* such section's kept in the ELF binary as metadata but not loaded onto the target device */
/* .log 0 -> start address is zero */