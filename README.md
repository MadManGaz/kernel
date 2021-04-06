# Kernel

This is a simple x86\_64 kernel written in rust. It's here to allow me to get
used to writing low level code like this, without stdlib.

![example branch parameter](https://github.com/madmangaz/kernel/actions/workflows/rust.yml/badge.svg?branch=main)

I'm following along the excellent guide at [Writing an OS in Rust](https://os.phil-opp.com/)
by Phillip Oppermann.

## Requirements

- qemu-system-x86\_64
- Rust nightly
- `rust-src` rustup component
- `llvm-tools-preview` rustup component
- `bootimage` tool

Besides a working computer, that's it really. You can run this on apple silicon
macs, however you will have to use qemu in emulation mode as this will only
compile x86\_64 code for now. The bootloader only handles Intel code.

## Building and Running

This is made easy by nature of the Rust toolchain being an all-in-one cross
compiler, with a rust native linker. The kernel code is pure Rust when
possible. Assembly is required at points, however no assembler is required.
This is handled by the Rust toolchain.

### First Time Setup:

```shell
rustup default nightly
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
```

### To build:

```shell
cargo build
```

### And to run:

```shell
cargo run
```
