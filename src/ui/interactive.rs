use std::io::prelude::*;
use crate::*;

pub fn main() {

    struct State {
        thought: Option<board::Move>,
        board: board::Board,
        depth: usize
    };

    let s = State {
        thought: None,
        board: board::Board::new(),
        depth: 5,
    };
    let mut shell = shrust::Shell::new( s );


    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command_noargs("uci", "Continue as a UCI engine", |_,_| { super::uci::main(); Ok(()) });
    shell.new_command_noargs("perft", "Perform a perft() test", |_,_| { super::perft::main(); Ok(())});
    shell.new_command_noargs("test", "Runs a predetermined test routine", |_,_| { super::test::main(); Ok(())});

    shell.new_command_noargs("think", "Let the engine think about a move", |io,s| {
        s.thought = search::search(&mut s.board, s.depth).pv.pop();
        let m = s.thought.clone().expect("No move found!");
        writeln!(io, "Move: {:?}", m)?;
        Ok(())
    });
    shell.run_loop(&mut shrust::ShellIO::default());
}