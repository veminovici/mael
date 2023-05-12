# MAEL-RS

## Kernel
The [kernel](./src/) library implements the functionality shared by all the challenges.

```rust
use mael::kernel::{Kernel, StdKernel};

StdKernel::<YourNode>::spawn_and_run.unwrap();
```

## Challenge #1: Echo
The implementation for the [Challenge #1: Echo](https://fly.io/dist-sys/1/) can be found in the [echo](./examples/echo/main.rs) file.

```rust
pld!(
    enum EchoPayload {
        Echo { echo: String },
        EchoOk { echo: String },
    }
);

StdKernel::<EchoNode>::spawn_and_run().unwrap();
```

#### Build and Run
To build and run any of the examples, run in your command line the following commands:
```zsh
cargo build
cargo build --examples

~/Atelier/maelstrom/maelstrom test -w echo --bin ./target/debug/examples/echo --node-count 1 --time-limit 10 --log-net-send true
```

## Challenge #2: Unique Identifier
The implementation for the [Challenge #2: Unique ID Generatio](https://fly.io/dist-sys/2/) can be found in the [unique](./examples/unique/main.rs) file.

```rust
pld!(
    enum UniquePayload {
        Generate,
        GenerateOk { id: String },
    }
);

StdKernel::<UniqueNode>::spawn_and_run().unwrap();
```

#### Build and Run
To build and run any of the examples, run in your command line the following commands:
```zsh
cargo build
cargo build --examples

~/Atelier/maelstrom/maelstrom test -w unique-ids --bin ./target/debug/examples/unique --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --log-net-send true
```

## Challenge #3: Broadcast
The implementation for the [Challenge #3: Broadcast](https://fly.io/dist-sys/3a/) can be found in the [broadst](./examples/broadcast/main.rs) file.

```rust
pld!(
    enum BroadcastPayload {
        Broadcast {
            message: usize,
        },
        BroadcastOk,
        Read,
        ReadOk {
            messages: HashSet<usize>,
        },
        Topology {
            topology: HashMap<String, Vec<String>>,
        },
        TopologyOk,
    }
);

StdKernel::<BroadcastNode>::spawn_and_run().unwrap();
```

#### Build and Run
To build and run any of the examples, run in your command line the following commands:
```zsh
cargo build
cargo build --examples

~/Atelier/maelstrom/maelstrom test -w broadcast --bin ./target/debug/examples/broadcast --node-count 1 --time-limit 20 --rate 10
```
