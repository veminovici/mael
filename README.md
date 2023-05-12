# MAEL-RS

## Kernel
The [kernel](./src/) library implements the functionality shared by all the challenges.

## Challenge #1: Echo
The implementation for the [Challenge #1: Echo](https://fly.io/dist-sys/1/) can be found in the [echo.rs](./examples/echo/main.rs) file.

```zsh
cargo build
cargo build --example echo
~/Atelier/maelstrom/maelstrom test -w echo --bin ./target/debug/examples/echo --node-count 1 --time-limit 10
```