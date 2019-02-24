use crate::turing::TuringMachine;

use std::{thread, time};
use crossterm_cursor;
use ansi_term::Colour::{Red, Green};
use ansi_term::Style;
use crossterm_terminal::{terminal,ClearType};

pub struct TuringMachineDisplay;

impl TuringMachineDisplay {
    pub fn display(turing_machine : &mut TuringMachine) {
        println!("{}: {}          ", Style::new().bold().paint("state"), turing_machine.current_state);
        let max_tape : usize = turing_machine.tapes.iter().max_by_key(|x| x.content.len()).unwrap().content.len();
        for tape in turing_machine.tapes.iter() {
            for i in 0..tape.content.len() {
                if i == tape.cursor {
                    print!("\x1b[31;1;1m");
                }
                print!("{}", tape.content.get(i).unwrap());
                if i == tape.cursor {
                    print!("\x1b[0m");
                }
            }
            for _ in 0..(max_tape - tape.content.len()) {
                print!(" ");
            }
            print!(" ({})", tape.name);
            println!();
            for n in 0..tape.content.len() {
                if n == tape.cursor {
                    print!("^");
                }
                else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn run(turing_machine : &mut TuringMachine) {
        let mut icount = 0;
        let cursor = crossterm_cursor::cursor();
        terminal().clear(ClearType::All).expect("cant clear");
        cursor.save_position().expect("cant save");
        let duration = time::Duration::new(1, 0);

        Self::display(turing_machine);

        while !turing_machine.done() {
            icount = icount + 1;
            turing_machine.step();
            thread::sleep(duration);
            cursor.reset_position().expect("cant reset");
            Self::display(turing_machine);
        }
        if (turing_machine.current_state == "no") || (turing_machine.current_state == "reject") {
            println!("program halted on state {}", Red.bold().paint(&turing_machine.current_state));
        }
        else {
            println!("program halted on state {}", Green.bold().paint(&turing_machine.current_state));
        }
    }
}
