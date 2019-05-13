use std::io::prelude::*;

pub fn main() {
    let mut shell = shrust::Shell::new(());


    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command_noargs("uci", "Continue as a UCI engine", |_,_| { super::uci::main(); Ok(()) });
    shell.new_command_noargs("perft", "Perform a perft() test", |_,_| { super::perft::main(); Ok(())});
    shell.new_command_noargs("test", "Runs a predetermined test routine", |_,_| { super::test::main(); Ok(())});

    shell.new_command_noargs("think", "Let the engine think about a move", |_,_| {
        Ok(())
    });
    shell.run_loop(&mut shrust::ShellIO::default());
}