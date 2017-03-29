extern crate getopts;
use getopts::Options;

#[macro_use]
mod debug;
use debug::*;

mod default;
use default::*;

mod cpu;
use cpu::Cpu;

mod wrapping_util;
mod operation;
mod interrupt;
mod flag;
mod mem;
mod execute;

fn print_usage(opts: Options) -> ! {
    let brief = &"Usage: vesta KERNEL [options]";
    fatal!("{}", opts.usage(brief));
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("M", "memsize", "Memory size (in bytes) for the CPU to use as RAM", "SIZE");
    opts.optflag("D", "debug", "Print extremely verbose debug output");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
    }

    if matches.opt_present("D") {
        unsafe { DEBUGGING = true; }
        debug!("Debugging is enabled!");
    }

    //TODO: put this default somewhere nice.
    let memory_size: u32 = matches.opt_str("M")
                                  .map(|s| s.parse().unwrap_or_die(ERR_PARSE_MEM_SIZE))
                                  .unwrap_or(DEFAULT_MEM_SIZE);

    let kernel_file = if matches.free.len() == 1 {
        &matches.free[0]
    } else {
        print_usage(opts);
    };

    let cpu = Cpu::new(kernel_file, memory_size);
    cpu.boot();
}
