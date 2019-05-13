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

    shell.new_command_noargs("print", "Prints the current board", |_,s| { super::print_board(&s.board); Ok(())});
    shell.new_command_noargs("undo", "Undoes the most recent move", |_,s| { s.board.unmake(); Ok(())});

    shell.new_command("do", "Perform a move (format: O-O, b1c3, e7e8q", 1, |io,s,a| {
        // match board::Move::from_str(a, s.board) {
        //     Some(m) => s.board.make(m),
        //     Err(e) => Err(e),
        // }
        match board::Move::from_str(a[0], &s.board) {
            Err(e) => writeln!(io, "Error parsing move! {:?}", e)?,
            Ok(m) => { s.board.make(&m); },
        };
        return Ok(());
    });

    shell.new_command_noargs("think", "Let the engine think about a move", |io,s| {
        s.thought = search::search(&mut s.board, s.depth).pv.pop();
        let m = s.thought.clone().expect("No move found!");
        writeln!(io, "Move: {}", m)?;
        Ok(())
    });

    shell.new_command_noargs("ok", "Perform the suggested move", |io,s| {
        match &s.thought {
            None => writeln!(io, "No suggested move!")?,
            Some(m) => s.board.make(&m)
        };
        s.thought = None;
        Ok(())
    });

    shell.new_command_noargs("reset", "Put the board back in the starting position", |_,s| {
        s.board = board::Board::new();
        Ok(())
    });
    shell.run_loop(&mut shrust::ShellIO::default());
}