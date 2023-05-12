# MAEL-RS

## Kernel
The [kernel](./src/) library implements the functionality shared by all the challenges.

```rust
use mael::kernel::{Kernel, StdKernel};

StdKernel::<YourNode>::spawn_and_run.unwrap();
```

## Challenge #1: Echo
The implementation for the [Challenge #1: Echo](https://fly.io/dist-sys/1/) can be found in the [echo.rs](./examples/echo/main.rs) file.

```rust
pld!(
    enum MyEcho {
        Echo { echo: String },
        EchoOk { echo: String },
    }
);
```

## Build and Run
To build and run any of the examples, run in your command line the following commands (the commands are for **echo** example):
```zsh
cargo build
cargo build --examples

~/Atelier/maelstrom/maelstrom test -w echo --bin ./target/debug/examples/echo --node-count 1 --time-limit 10 --log-net-send true
```