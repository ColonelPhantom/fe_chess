[![Build Status](https://travis-ci.org/ColonelPhantom/fe_chess.svg?branch=master)](https://travis-ci.org/ColonelPhantom/fe_chess)

# fe_chess
A currently 0x88 mailbox-based chess engine in Rust

## Installation
fe_chess installs like a normal cargo crate: clone the repo and `cargo build --release`.
The executable will be in `target/release`.

## Usage
fe_chess currently only performs a simple perft(). No interface like UCI is implemented.
Search and eval are completely absent.
