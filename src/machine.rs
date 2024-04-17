use core::panic;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Action {
    tape_change: isize,
    write: u8,
    next: Option<usize> 
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct State(Action, Action);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    states: Vec<State>,
    state: usize,
    actions: usize
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tape {
    tape: Vec<u8>,
    curr: usize
}

impl Action {
    pub fn new(tape_change: isize, write: u8, next: Option<usize>) -> Action {
        Action { tape_change, write, next }
    }
}

impl State {
    pub fn new(action1: Action, action2: Action) -> State {
        State(action1, action2)
    }
}

impl Machine {
    pub fn new(states: Vec<State>) -> Machine {
        Machine { states, state: 0, actions: 0 }
    }

    pub fn set_state(&mut self, state: usize) {
        self.state = state;
    }

    pub fn run(&mut self, mut tape: Tape) {
        loop {
            self.actions += 1;
            let state = match tape.at() {
                0 => self.states[self.state].0,
                1 => self.states[self.state].1,
                _ => panic!("wat")
            };

            tape.set(state.write);

            if let Some(next) = state.next {
                self.state = next;
            } else {
                println!("{tape}, State: {}, Actions: {}", self.state, self.actions);
                return;
            }

            tape.inc_dec(state.tape_change);

            println!("{tape}, State: {}, Actions: {}", self.state, self.actions);
        }
    }
}


impl Tape {
    pub fn new(tape: Vec<u8>, curr: usize) -> Tape {
        Tape { tape, curr }
    }

    pub fn inc_dec(&mut self, val: isize) {
        match val {
            -1 => self.dec(),
            0 => (),
            1 => self.inc(),
            _ => panic!("invalid value"),
        }
    }

    pub fn inc(&mut self) {
        if self.curr == self.tape.len() - 1 {
            self.tape.extend(vec![0; self.tape.len()]);
        }
        self.curr += 1;
    }

    pub fn dec(&mut self) {
        if self.curr == 0 {
            let mut prepend = vec![0; self.tape.len()];
            prepend.extend(self.tape.clone());
            self.curr = self.tape.len() - 1;
            self.tape = prepend;
        } else {
            self.curr -= 1;
        }
    }

    pub fn at(&self) -> u8 {
        self.tape[self.curr]
    }

    pub fn set(&mut self, set: u8) {
        self.tape[self.curr] = set;
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, Tape Pos: {}", self.tape, self.curr)
    }
}
