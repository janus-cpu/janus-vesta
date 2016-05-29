use std::process::exit;
use std::io::Write;

pub static mut debug: bool = false;

macro_rules! println_stderr (
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

#[macro_export]
macro_rules! debug (
    ($($arg:tt)*) => { {
        unsafe {
            use die;
            if die::debug {
                use std::io::Write;
                let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
                r.expect("failed printing to stderr");
            }
        }
    } }
);

pub trait Die<T> {
    fn die(self, err_message: &'static str) -> T;
}

impl <S, T> Die<S> for Result<S, T> {
    fn die(self, err_message: &'static str) -> S {
        match self {
            Ok(s) => s,
            Err(_) => {
                println_stderr!("Fatal error: {}", err_message);
                exit(1);
            }
        }
    }
}

pub fn die(err_message: &'static str) -> ! {
    println_stderr!("Fatal error: {}", err_message);
    exit(1);
}

// ---------------- ERROR MESSAGES ---------------- //

pub const CANNOT_PARSE_MEMSIZE: &'static str =
"Cannot recognize the integer given. Please provide a positive number for the memory \
size.";

pub const INVALID_FILE: &'static str =
"Cannot open the given file. Please check that it exists and that \
you have the right permissions to read it.";

pub const CANNOT_READ_FILE: &'static str =
"Cannot read the given file. Make sure that it is not corrupted and that \
you have the right permissions to read it.";

pub const INSUFFICIENT_MEMORY: &'static str =
"Insufficient memory to store the kernel binary in the virtual RAM. Please use the \
-M flag to provide more memory to the CPU.";
