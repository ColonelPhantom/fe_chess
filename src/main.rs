#[macro_use]
mod board;
mod eval;
mod movegen;
mod search;
mod ui;

fn main() {

    let arg = std::env::args().nth(1);
    if arg.is_some() {
        match arg.unwrap().trim() {
            "uci" => return ui::uci::main(),
            "interactive" => return ui::interactive::main(),
            "perft" => ui::perft::main(),
            "test" => ui::test::main(),
            "quit" => return,
            _ => panic!("Invalid mode"),
        }
    } else {
        ui::interactive::main();
    }
}
