# Kernel

This is a simple x86\_64 kernel written in rust. It's here to allow me to get
used to writing low level code like this, without stdlib.

## Requirements

- qemu-system-x86\_64
- Rust nightly

Besides a working computer, that's it really. You can run this on apple silicon
macs, however you will have to use qemu in emulation mode as this will only
compile x86\_64 code for now. The bootloader only handles Intel code.

## Building and Running

This is quite simple, as cross compilation is built right into Rust. The
project is configured to target the LLVM target specified in the
`x86\_64-kernel.json` toolchain file.

### To build:

```shell
cargo build
```

### And to run:

```shell
cargo run
```
