#[macro_use]
mod board;
mod eval;
mod movegen;
mod search;
mod ui;

fn main() {
    let mut b = board::Board::new();

    let mut searchinfo = search::search(&mut b, 5);
    while let Some(m) = searchinfo.pv.pop() {
        println!(
            "{:02x} {:02x} {:?} {:?} {:?}",
            m.from, m.to, m.promote_to, m.en_passant, m.castling
        );
    }
    println!("Score: {}", searchinfo.score);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("io error");
    match input.trim() {
        "uci" => return ui::uci::main(),
        "interactive" => return ui::interactive::main(),
        "perft" => ui::perft::main(),
        "quit" => return,
        _ => panic!("Invalid mode"),
    }
}
