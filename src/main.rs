mod turing;

// use std::{thread, time};
use crossterm_cursor;
use turing::TuringMachine;
use std::env;
use std::process;

fn main() {
    let cursor = crossterm_cursor::cursor();
    cursor.show().expect("error");

    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error : no file provided");
        println!("usage: turing [filename]");
        process::exit(1);
    }

    let mut tm = TuringMachine::parse(args.get(1).expect("no file provided"));
    let mut tapes = args[2..].to_vec();
    tm.reset(&mut tapes);

    // tm.step();

    // let duration = time::Duration::new(1, 0);
    // cursor.save_position().expect("cant save");
    //
    // cursor.hide().expect("cant hide");
    //
    // print!("0000000");
    //
    //
    // cursor.reset_position().expect("cant reset");
    // thread::sleep(duration);
    // print!("1000000");
    //
    // cursor.reset_position().expect("cant reset");
    // thread::sleep(duration);
    // print!("0100000");
    //
    // cursor.reset_position().expect("cant reset");
    // thread::sleep(duration);
    // print!("0010000");
    //
    // cursor.reset_position().expect("cant reset");
    // thread::sleep(duration);
    // print!("0001000");

}
