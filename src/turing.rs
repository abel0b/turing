use std::collections::HashMap;
use std::fs;
use std::process;

type TransitionMap = HashMap<(String, Vec<String>), (String, Vec<String>, Vec<Move>)>;

#[derive(Debug)]
pub struct TuringMachine {
    initial_state: String,
    states:  HashMap<String,State>,
    transitions: TransitionMap,
    pub current_state: String,
    pub tapes: Vec<Tape>,
}

#[derive(Debug)]
pub struct Tape {
    pub name: String,
    pub content: Vec<String>,
    pub cursor: usize,
}

impl Tape {
    pub fn new(name : String) -> Tape {
        Tape {
            name,
            content: vec![String::from(">")],
            cursor: 0,
        }
    }

    pub fn reset(&mut self, content: &str) {
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
    pub fn new(name : &str) -> Move {
        match name {
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
    is_initial: bool,
}

impl State {
    pub fn new(name : &str, is_final : bool, is_initial : bool) -> State {
        State {
            name: name.to_string(),
            is_final,
            is_initial,
        }
    }
}

enum ParserState {
    Begin,
    Alphabet,
    Tapes,
    States,
    StateOptions,
    Comment,
    TransitionInputState,
    TransitionInputSymbol,
    Transition,
    TransitionOutputState,
    TransitionOutputSymbol,
    TransitionOutputMoves,
}

impl TuringMachine {
    pub fn new(initial_state: String, states: HashMap<String,State>, tapes: Vec<Tape>, transitions: TransitionMap) -> TuringMachine {
        TuringMachine {
            initial_state: initial_state.clone(),
            states,
            tapes,
            current_state: initial_state,
            transitions,
        }
    }

    pub fn parse(filepath : &str) -> TuringMachine {
        let program = fs::read_to_string(filepath).expect("could not read file");
        let mut parser_state = ParserState::Begin;
        let mut buffer = String::new();
        let mut flags_buffer = String::new();
        let mut initial_state : Option<String> = None;

        let mut alphabet : Vec<String> = Vec::new();
        let mut tapes : Vec<Tape> = Vec::new();
        let mut states : HashMap<String,State> = HashMap::new();
        let mut state_is_final = false;
        let mut state_is_initial = false;
        let mut transitions : TransitionMap = HashMap::new();

        let mut state_input = String::new();
        let mut symbols_input : Vec<String> = Vec::new();
        let mut state_output = String::new();
        let mut symbols_output : Vec<String> = Vec::new();
        let mut moves : Vec<Move> = Vec::new();
        let mut line = 1;

        for char in program.chars() {
            if char == '\n' {
                line += 1;
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
                            eprintln!("syntax error: unknown command '{}' at line {}", buffer, line);
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
                ParserState::Comment => if let '\n' = char {
                    parser_state = ParserState::Begin;
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
                    ' ' | '\n' => {
                        if state_is_initial {
                            match initial_state {
                                Some(_) => {
                                    eprintln!("cannot have multiple initial states");
                                    process::exit(1);
                                }
                                None => {
                                    initial_state = Some(buffer.clone());
                                }

                            }
                        }
                        let new_state = State::new(&buffer, state_is_final, state_is_initial);
                        states.insert(buffer, new_state);
                        buffer = String::new();
                        state_is_final = false;
                        state_is_initial = false;
                        if char == '\n' {
                            parser_state = ParserState::Begin;
                        }
                    },
                    '(' => {
                        parser_state = ParserState::StateOptions;
                    },
                    _ => {
                        buffer.push(char);
                    },
                },
                ParserState::StateOptions => match char {
                    ' ' | ')' => {
                        if flags_buffer == "initial" {
                            state_is_initial = true;
                        }
                        else if flags_buffer == "final" {
                            state_is_final = true;
                        }
                        else {
                            eprintln!("unknown state option {} at line {}", buffer, line);
                            process::exit(1);
                        }

                        flags_buffer.clear();

                        if char == ')' {
                            parser_state = ParserState::States;
                        }
                    },
                    '\n' => {
                        eprintln!("unexpected line break at line {}", line);
                        process::exit(1);
                    },
                    _ => {
                        flags_buffer.push(char);
                    },
                },
                ParserState::TransitionInputState => match char {
                    ' ' => {
                        match states.get(&buffer) {
                            Some(_) => {
                                state_input = buffer;
                            },
                            None => {
                                eprintln!("syntax error: undefined state '{}' at line {}", buffer, line);
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
                ParserState::Transition => if let '[' = char {
                    parser_state = ParserState::TransitionOutputState;
                },
                ParserState::TransitionOutputState => match char {
                    ' ' => {
                        match states.get(&buffer) {
                            Some(_) => {
                                state_output = buffer;
                            },
                            None => {
                                eprintln!("undefined state '{}' at line {}", buffer, line);
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
                        eprintln!("syntax error: unexpected line break at line {}", line-1);
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
            initial_state.unwrap(),
            states,
            tapes,
            transitions,
        )
    }

    pub fn reset(&mut self, tapes: &mut Vec<String>) {
        for (n, tape) in tapes.iter().enumerate() {
            (&mut self.tapes[n]).reset(tape)
        }
    }

    pub fn step(&mut self) {
        let mut values : Vec<String> = Vec::new();
        for tape in self.tapes.iter_mut() {
            values.push(tape.content[tape.cursor].clone());
        }
        let current = (self.current_state.clone(), values);

        match self.transitions.get(&current) {
            Some((next_state, next_values, next_moves)) => {
                self.current_state = next_state.clone();
                for i in 0..self.tapes.len() {
                    let tape = &mut self.tapes[i];
                    match &next_values[i][..] {
                        "@" => {

                        }
                        _ => {
                            tape.content[tape.cursor] = next_values[i].clone();
                        }
                    }
                    match next_moves[i] {
                        Move::Left => {
                            if tape.cursor == 0 {
                                eprintln!("cursor cant go left");
                                process::exit(1);
                            }
                            tape.cursor -= 1;
                        },
                        Move::Right => {
                            tape.cursor += 1;
                            if tape.cursor == tape.content.len() {
                                tape.content.push(String::from("_"));
                            }
                        },
                        Move::Stay => {

                        }
                    }
                }
            }
            None => {
                eprintln!("missing transition {:?}", current);
                process::exit(1);
            }
        }
    }

    pub fn done(&self) -> bool {
        self.states[&self.current_state].is_final
    }
}
