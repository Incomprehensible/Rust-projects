	.section .text.HFTrampoline
	.global HFTrampoline
	.thumb_func

HFTrampoline:
	mrs r0, MSP	// mrs -> loads value from special purpose register
			// into general purpose register
			// MSP - Main Stack Pointer;
			// it contains pointer to registers pushed
			// to the stack by exception
			// we invoke user-defined handler with extra data
	b HardFault

