#[macro_use]
mod board;
mod eval;
mod movegen;
mod search;
mod ui;

fn main() {

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("io error");
    match input.trim() {
        "uci" => return ui::uci::main(),
        "interactive" => return ui::interactive::main(),
        "perft" => ui::perft::main(),
        "test" => ui::test::main(),
        "quit" => return,
        _ => panic!("Invalid mode"),
    }
}
