pub static mut DEBUGGING: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if unsafe { DEBUGGING } {
                write!(&mut ::std::io::stderr(), "[vesta] {}\n", format_args!($($arg)*)).unwrap();
            }
        }
    )
}

macro_rules! fatal {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            use std::process::exit;
            write!(&mut ::std::io::stderr(), "[FATAL] {}\n", format_args!($($arg)*)).unwrap();
            exit(1);
        }
    )
}

pub trait UnwrapOrDie<T> {
    fn unwrap_or_die(self, err_msg: &str) -> T;
}

impl<T, E> UnwrapOrDie<T> for Result<T, E> {
    fn unwrap_or_die(self, err_msg: &str) -> T {
        self.unwrap_or_else(|_| {
            fatal!("{}", err_msg);
        })
    }
}

pub const ERR_PARSE_MEM_SIZE: &str =
"Cannot parse memory size argument. Check formatting!";

pub const INVALID_FILE: &'static str =
"Cannot open the given file. Please check that it exists and that \
you have the right permissions to read it.";

pub const CANNOT_READ_FILE: &'static str =
"Cannot read the given file. Make sure that it is not corrupted and that \
you have the right permissions to read it.";
