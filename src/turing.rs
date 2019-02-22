use std::collections::HashMap;
use std::fs;
use std::process;

pub struct TuringMachine {
    tapes: Vec<Tape>,
    transitions: HashMap<(String, Vec<String>), (String, Vec<String>, Vec<Move>)>
}

#[derive(Debug)]
pub struct Tape {
    name: String,
    content: Vec<String>,
    cursor: u32,
}

impl Tape {
    pub fn new(name : String) -> Tape {
        Tape {
            name,
            content: vec![String::from(">")],
            cursor: 0,
        }
    }

    pub fn reset(&mut self, content: &String) {
        self.content.clear();
        self.content.push(String::from(">"));
        for s in content.chars() {
            self.content.push(s.to_string());
        }
    }
}

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
    Stay,
}

impl Move {
    pub fn new(name : &String) -> Move {
        match &name[..] {
            "->" => {
                Move::Right
            },
            "<-" => {
                Move::Left
            },
            "-" => {
                Move::Stay
            },
            _ => {
                process::exit(1);
            }
        }
    }
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
    Comment,
    TransitionInputState,
    TransitionInputSymbol,
    Transition,
    TransitionOutputState,
    TransitionOutputSymbol,
    TransitionOutputMoves,
}

impl TuringMachine {
    pub fn new(tapes: Vec<Tape>, transitions: HashMap<(String, Vec<String>), (String, Vec<String>, Vec<Move>)>) -> TuringMachine {
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
        let mut transitions : HashMap<(String, Vec<String>), (String, Vec<String>, Vec<Move>)> = HashMap::new();

        let mut state_input = String::new();
        let mut symbols_input : Vec<String> = Vec::new();
        let mut state_output = String::new();
        let mut symbols_output : Vec<String> = Vec::new();
        let mut moves : Vec<Move> = Vec::new();
        let mut line = 1;

        for char in program.chars() {
            if char == '\n' {
                line = line + 1;
            }
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
                        else if buffer == "#" {
                            parser_state = ParserState::Comment;
                        }
                        else {
                            println!("syntax error: unknown command '{}' at line {}", buffer, line);
                            process::exit(1);
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
                ParserState::Comment => match char {
                    '\n' => {
                        parser_state = ParserState::Begin;
                    },
                    _ => {

                    },
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
                                println!("syntax error: undefined state '{}' at line {}", buffer, line);
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
                                println!("syntax error: undefined state '{}' at line {}", buffer, line);
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
                        if symbols_output.len() == tapes.len() {
                            parser_state = ParserState::TransitionOutputMoves
                        }
                    },
                    '\n' => {
                        println!("syntax error: unexpected line break at line {}", line-1);
                        process::exit(1);
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::TransitionOutputMoves => match char {
                    ' ' => {
                        moves.push(Move::new(&buffer));
                        buffer.clear()
                    },
                    ']' => {
                        moves.push(Move::new(&buffer));
                        buffer.clear();

                        let input = (state_input, symbols_input);
                        let output = (state_output, symbols_output, moves);

                        transitions.insert(input, output);

                        moves = Vec::new();
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

        Self::new(
            tapes,
            transitions,
        )
    }

    pub fn reset(&mut self, tapes: &mut Vec<String>) {
        for n in 0..tapes.len() {
            let mut self_tapes : &mut Tape = self.tapes.get(n).unwrap();
            self_tapes.reset(tapes.get(n).unwrap())
        }
        println!("{:?}", tapes);
    }
}
