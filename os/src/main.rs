//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

fn sleep(time: u64) {
    extern "C" {
        fn wait_STIP();
    }
    sbi_rt::set_timer(time);
    unsafe {
        wait_STIP();
    }
}

// jump from function `thread_1_entry` in entry.asm
#[no_mangle]
fn thread_1_main(hartid: usize/*, opaque: usize*/) -> ! {
    info!("[kernel] Running from another thread, hartid = {}", hartid);
    loop {}
}

// holds the booting hart
static mut BOOT_HART: usize = 0;

fn start_one_hart() {
    extern "C" {
        fn thread_1_entry();
    }
    unsafe {
        for i in 0..10 { // find the first available hartid
            if i == BOOT_HART {
                continue;
            }
            let result: sbi_rt::SbiRet = sbi_rt::hart_start(i as usize, thread_1_entry as usize, 0);
            if result.is_ok() {
                info!("[kernel] Successfully started hart #{:x}H.", i);
                break;
            } else {
                error!("[kernel] Failed to started hart #{:x}H: err= {}", i, result.error as isize)
            }
        }
    }
}


/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    let mut boot_hart;
    unsafe {
        core::arch::asm!("mv {}, a0", out(reg) boot_hart);
    }
    clear_bss(); // this will clear BOOT_HART, so it must be set after this
    unsafe { BOOT_HART = boot_hart; }
    logging::init();
    start_one_hart();
    println!("Sleep test. Waiting for 5 seconds...");
    sleep(50_000_000);
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    // CI autotest success: sbi::shutdown(false)
    // CI autotest failed : sbi::shutdown(true)
    sbi::shutdown(false)
}
