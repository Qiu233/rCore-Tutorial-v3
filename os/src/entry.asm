    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack

    # thread 1 stack space
    .globl thread_1_stack_lower_bound
thread_1_stack_lower_bound:
    .space 4096 * 16
    .globl thread_1_stack_top
thread_1_stack_top:

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
    
    # thread 1 entry point
    .global thread_1_entry
thread_1_entry:
    la sp, thread_1_stack_top
    call thread_1_main