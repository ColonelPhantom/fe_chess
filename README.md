[![Build Status](https://travis-ci.org/ColonelPhantom/fe_chess.svg?branch=master)](https://travis-ci.org/ColonelPhantom/fe_chess)

# fe_chess
A currently 0x88 mailbox-based chess engine in Rust

## Installation
fe_chess installs like a normal cargo crate: clone the repo and `cargo build --release`.
The executable will be in `target/release`.

It can also be ran with `cargo run --release` or the binaries on the Releases page.

## Usage
fe_chess currently only has a simple built-in shell that is not compatible with uci. Type `help` to get a list of command. (UCI not functional).

# Changelog
## [v0.2.2](https://github.com/ColonelPhantom/fe_chess/compare/v0.2.1...v0.2.2)
* Implement static exchange evaluation
    * Quiscence search does not consider a move with see < 0
    * Quiescence search does not consider moves where standing pat + SEE + a margin is below alpha
## [v0.2.1](https://github.com/ColonelPhantom/fe_chess/compare/v0.2.0...v0.2.1)
* Improve eval() significantly by evaluating mobility
## [v0.2.0](https://github.com/ColonelPhantom/fe_chess/compare/v0.1.2...v0.2.0)
* Implement a transposition table to speed up search significantly
    * Implement zobrist hashing
* Implement delta pruning
## [v0.1.2](https://github.com/ColonelPhantom/fe_chess/compare/v0.1.1...v0.1.2)
* Remove mobility from evaluation
* Implement piece-square tables adapted from [chessprogramming.org](https://www.chessprogramming.org/Simplified_Evaluation_Function)
## [v0.1.1](https://github.com/ColonelPhantom/fe_chess/compare/v0.1.0...v0.1.1)
* Store scores in a specialized struct
* Make search handle ending the game (it now can mate!)
* Let `think` output score and pv
## [v0.1.0](https://github.com/ColonelPhantom/fe_chess/commits/v0.1.0)
* Initial release