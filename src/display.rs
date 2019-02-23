use crate::turing::TuringMachine;

use std::{thread, time};
use crossterm_cursor;

pub struct TuringMachineDisplay;

impl TuringMachineDisplay {
    pub fn display(turing_machine : &mut TuringMachine) {
        println!("state : {}", turing_machine.current_state);
        for tape in turing_machine.tapes.iter() {
            print!("tape {} : ", tape.name);
            for i in 0..tape.content.len() {
                if i == tape.cursor {
                    print!("\x1b[31;1;4m");
                }
                print!("{}", tape.content.get(i).unwrap());
                if i == tape.cursor {
                    print!("\x1b[0m");
                }
            }
            println!();
        }
    }

    pub fn run(turing_machine : &mut TuringMachine) {
        let cursor = crossterm_cursor::cursor();
        cursor.save_position().expect("cant save");
        let duration = time::Duration::new(1, 0);

        Self::display(turing_machine);

        while !turing_machine.done() {
            turing_machine.step();
            thread::sleep(duration);
            cursor.reset_position().expect("cant reset");
            Self::display(turing_machine);
        }
    }
}
