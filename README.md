# 52-weeks-rust
Trying out Rust

## How to Install

* [Install](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Pro-tip, run `rustup update` to update your installation.
* Check version do this: `rustc --version`


## Episode 3



## Trying to learn Rust

Quick example:
``` rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

```
* [learn by example guide] (https://doc.rust-lang.org/stable/rust-by-example/)

