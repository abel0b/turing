mod display;
mod turing;

use display::TuringMachineDisplay;
use std::env;
use std::process;
use turing::TuringMachine;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error : no file provided");
        println!("usage: turing [filename]");
        process::exit(1);
    }

    let mut tm = TuringMachine::parse(args.get(1).expect("no file provided"));
    let mut tapes = args[2..].to_vec();

    tm.reset(&mut tapes);

    TuringMachineDisplay::run(&mut tm);
}
