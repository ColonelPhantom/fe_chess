#[macro_export]
macro_rules! measure_time {
    ($op:expr, $message:expr) => {
        let start = std::time::Instant::now();
        $op;
        println!($message, start.elapsed().as_millis());
    };
}

#[macro_use]
mod board;
mod eval;
mod movegen;
#[macro_use]
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
