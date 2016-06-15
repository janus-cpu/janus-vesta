# vesta

## About
The Vesta Virtual Machine (`vesta`) is a virtual CPU emulating the Janus (CISC) instruction set. 
Currently, it can execute programs assembled with the [Janus Assembler](https://github.com/janus-cpu/janus-jas); 
however, we are hoping to eventually write a more complex toolchain including a small C-like language and a linker.

## How do I test it?
If you want to check out `vesta` for yourself, first you will need a recent version of [Rust](https://www.rust-lang.org/) and its package builder `cargo`. 

Then you can clone the repo with:
```
git clone https://github.com/janus-cpu/janus-vesta
```
and compile the executable with `cargo build --release`. The executable will be in `target/release/vesta`. Have fun!

Check out `vesta --help` for more information on how to run things with the emulator. If you want to be able to run any meaningful test programs, make sure to grab [`jas`](https://github.com/janus-cpu/janus-jas) as well, unless you really like assembling files by hand.

## What's implemented right now?
Right now we have implemented:
* Decoding CPU instructions from memory
* (*almost*) All of the long (32-bit) instructions
* Very verbose debug output

And *hopefully* in the near future we will also have:
* Fault handing
* Interrupts
* CPU timer
* Memory paging scheme
* 2 CPU modes: Privileged/Kernel and Userland
* A real "terminal" with a VGA-like buffer.

### Special Thanks
Thanks to my friend Chris for writing `jas` and working continually on this project with me. Knowing that someone else is interested in this project is almost completely the reason why I haven't quit this yet.
