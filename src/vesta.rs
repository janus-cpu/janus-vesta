extern crate getopts;
use getopts::Options;
use std::env;
use std::process::exit;

pub mod cpu;
pub mod die;
pub mod mem;
pub mod instruction;
pub mod execute;

use cpu::Cpu;
use die::*;

fn print_usage(opts: Options) -> ! {
    let brief = "Usage: vesta KERNEL [options]";
    print!("{}", opts.usage(&brief));
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("M", "memsize", "Memory size (in bytes) for the CPU to use as RAM", "SIZE");
    opts.optflag("v", "verbose", "Print verbose output");
    opts.optflag("", "debug", "Print extremely verbose debug output");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
    }

    if matches.opt_present("v") {
        //TODO: set output mode to verbose
    }

    if matches.opt_present("debug") {
        //TODO: set output mode to debug
    }

    //TODO: put this default somewhere nice.
    let memory_size_str = matches.opt_str("M").unwrap_or("2048".to_string());

    let memory_size = memory_size_str.parse::<u32>().die(CANNOT_PARSE_MEMSIZE);

    let kernel_file = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        print_usage(opts);
    };

    let mut cpu = Cpu::new(kernel_file, memory_size);
    cpu.execute();
}
