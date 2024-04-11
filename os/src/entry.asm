    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:

    .text
    .global wait_STIP
wait_STIP:
    li t1, (1<<5)
	csrrs t0, sie, t1
.loop:
    wfi
    csrr t2, sip
    and t2, t1, t2
    beqz t2, .loop
    csrrw a0, sie, t0
    ret