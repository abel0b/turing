use std::collections::HashMap;
use std::fs;
use std::process;

pub struct TuringMachine {
    tapes: Vec<Tape>,
    transitions: HashMap<(String, Vec<String>), (String, Vec<String>, Move)>
}

#[derive(Debug)]
pub struct Tape {
    name: String,
    content: Vec<String>,
    cursor: u32,
}

impl Tape {
    pub fn input() -> Tape {
        Self::new(String::from("input"))
    }

    pub fn new(name : String) -> Tape {
        Tape {
            name,
            content: Vec::new(),
            cursor: 0,
        }
    }
}

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
    Stay,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct State {
    name: String,
    is_final: bool,
}

impl State {
    pub fn new(name : &String) -> State {
        State {
            name: name.clone(),
            is_final: false,
        }
    }
}

enum ParserState {
    Begin,
    Alphabet,
    Tapes,
    States,
    TransitionInputState,
    TransitionInputSymbol,
    Transition,
    TransitionOutputState,
    TransitionOutputSymbol,
}

impl TuringMachine {
    pub fn new(tapes: Vec<Tape>, transitions: HashMap<(String, Vec<String>), (String, Vec<String>, Move)>) -> TuringMachine {
        TuringMachine {
            tapes,
            transitions,
        }
    }

    pub fn parse(filepath : &str) -> TuringMachine {
        let program = fs::read_to_string(filepath).expect("could not read file");
        let mut parser_state = ParserState::Begin;
        let mut buffer = String::new();

        let mut alphabet : Vec<String> = Vec::new();
        let mut tapes : Vec<Tape> = Vec::new();
        let mut states : HashMap<String,State> = HashMap::new();
        let mut transitions : HashMap<(String, Vec<String>), (String, Vec<String>, Move)> = HashMap::new();

        let mut state_input = String::new();
        let mut symbols_input : Vec<String> = Vec::new();
        let mut state_output = String::new();
        let mut symbols_output : Vec<String> = Vec::new();

        for char in program.chars() {
            match parser_state {
                ParserState::Begin => match char {
                    ' ' => {
                        if buffer == "alphabet" {
                            parser_state = ParserState::Alphabet;
                        }
                        else if buffer == "tapes" {
                            parser_state = ParserState::Tapes;
                        }
                        else if buffer == "states" {
                            parser_state = ParserState::States;
                        }
                        buffer = String::new();
                    },
                    '[' => {
                        parser_state = ParserState::TransitionInputState;
                        buffer = String::new();
                    },
                    _ => {
                        buffer.push(char);
                    }
                },
                ParserState::Alphabet => match char {
                    ' ' => {
                        alphabet.push(buffer);
                        buffer = String::new();
                    },
                    '\n' => {
                        alphabet.push(buffer);
                        buffer = String::new();
                        parser_state = ParserState::Begin;
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::Tapes => match char {
                    ' ' => {
                        tapes.push(Tape::new(buffer));
                        buffer = String::new();
                    },
                    '\n' => {
                        tapes.push(Tape::new(buffer));
                        buffer = String::new();
                        parser_state = ParserState::Begin;
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::States => match char {
                    ' ' => {
                        let new_state = State::new(&buffer);
                        states.insert(buffer, new_state);
                        buffer = String::new();
                    },
                    '\n' => {
                        let new_state = State::new(&buffer);
                        states.insert(buffer, new_state);
                        buffer = String::new();
                        parser_state = ParserState::Begin;
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::TransitionInputState => match char {
                    ' ' => {
                        match states.get(&buffer) {
                            Some(_) => {
                                state_input = buffer;
                            },
                            None => {
                                println!("error: undefined state {}", buffer);
                                process::exit(1);
                            }
                        }
                        buffer = String::new();
                        parser_state = ParserState::TransitionInputSymbol;
                    },
                    '[' => {

                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::TransitionInputSymbol => match char {
                    ' ' => {
                        symbols_input.push(buffer);
                        buffer = String::new();
                    },
                    ']' => {
                        symbols_input.push(buffer);
                        buffer = String::new();
                        parser_state = ParserState::Transition
                    },
                    '[' => {

                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::Transition => match char {
                    '[' => {
                        parser_state = ParserState::TransitionOutputState;
                    },
                    _ => {},
                },
                ParserState::TransitionOutputState => match char {
                    ' ' => {
                        match states.get(&buffer) {
                            Some(_) => {
                                state_output = buffer;
                            },
                            None => {
                                println!("error: undefined state {}", buffer);
                                process::exit(1);
                            }
                        }
                        buffer = String::new();
                        parser_state = ParserState::TransitionOutputSymbol;
                    },
                    '[' => {
                        buffer = String::new();
                    },
                    ']' => {
                        parser_state = ParserState::Begin;
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::TransitionOutputSymbol => match char {
                    ' ' => {
                        symbols_output.push(buffer);
                        buffer = String::new();
                    },
                    ']' => {
                        symbols_output.push(buffer);
                        buffer = String::new();

                        let input = (state_input, symbols_input);
                        let output = match symbols_output.pop() {
                            Some(move_string) => {
                                match &move_string[..] {
                                    "->" => {
                                        (state_output, symbols_output, Move::Right)
                                    },
                                    "<-" => {
                                        (state_output, symbols_output, Move::Left)
                                    },
                                    "-" => {
                                        (state_output, symbols_output, Move::Stay)
                                    },
                                    _ => {
                                        println!("unknown move {}", move_string);
                                        process::exit(1);
                                    },
                                }
                            },
                            None => {
                                println!("no output symbol given");
                                process::exit(1);
                            }
                        };

                        transitions.insert(input, output);
                        state_input = String::new();
                        state_output = String::new();
                        symbols_input = Vec::new();
                        symbols_output = Vec::new();
                    },
                    '\n' => {
                        parser_state = ParserState::Begin;
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
            }
        }

        println!("{:?}", alphabet);
        println!("{:?}", tapes);
        println!("{:?}", states);
        // println!("{:?}", state_input);
        // println!("{:?}", symbols_input);
        // println!("{:?}", state_output);
        // println!("{:?}", symbols_output);

        Self::new(
            vec![Tape::input()],
            HashMap::new(),
        )
    }
}
