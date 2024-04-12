//! SBI console driver, for text output

use crate::sbi::console_putchar;
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicU32, Ordering};

struct Stdout;

static STDOUT_LOCK: AtomicU32 = AtomicU32::new(0);

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    while let Err(_) = STDOUT_LOCK.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire) {}
    Stdout.write_fmt(args).unwrap();
    STDOUT_LOCK.store(0, Ordering::Release);
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
