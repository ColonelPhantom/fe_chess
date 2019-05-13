#[macro_use]
mod board;
mod eval;
mod movegen;
mod search;
mod ui;

fn main() {

    let mut input = String::new();

    let arg = std::env::args().nth(1);
    if arg.is_none() {
        print!("Enter a mode: ");
        use std::io::prelude::*;
        std::io::stdout().flush().expect("Couldn't flush stdout?!?");
        std::io::stdin().read_line(&mut input).expect("io error");
    } else {
        input = arg.unwrap();
    }
    match input.trim() {
        "uci" => return ui::uci::main(),
        "interactive" => return ui::interactive::main(),
        "perft" => ui::perft::main(),
        "test" => ui::test::main(),
        "quit" => return,
        _ => panic!("Invalid mode"),
    }
}
