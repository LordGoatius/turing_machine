use std::{fs, io::{self, Write}, usize};

use serde_json::to_string;

use crate::machine::{Action, Machine, State, Tape};

pub fn create_machine() {
    let mut states = String::new();

    print!("How many states will this machine have?: ");

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut states).expect("Failed to read line");

    let states: usize = loop {
        if let Ok(state) = states.trim_end().parse::<usize>() {
            break state;
        } 
        print!("Try again: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut states).expect("Failed to read line");
    };

    let mut all_states: Vec<State> = vec![];

    for i in 0..states {
        println!("What should state {} do for 0?", i);
        let action_0 = make_action(states);

        println!("What should state {} do for 1?", i);
        let action_1 = make_action(states);

        all_states.push(State::new(action_0, action_1));
    }

    let mut start = String::new();

    print!("Which state should the machine start in?: ");
    let _  = io::stdout().flush();

    let start = loop {
        if let Ok(start) = start.trim_end().parse::<usize>() {
            if start < states {
                break start;
            }
        }
        print!("Try again (must be less than the number of states 0 indexed): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut start).expect("Failed to read line");
    };

    let mut machine = Machine::new(all_states);
    machine.set_state(start);

    println!("WARNING: THIS WILL OVERRITE ANY CONTENTS OF machine.json IN DIRECTORY THE PROGRAM IS RAN IN");

    fs::write("machine.json", to_string(&machine).expect("Failed stringify")).expect("Failed to write file");
}

fn make_action(states: usize) -> Action {
    let mut action = String::new();

    print!("Which state should this action go to?: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut action).expect("Failed to read line");

    let next: Option<usize> = loop {
        if let Ok(next) = action.trim_end().parse::<usize>() {
            if next < states {
                break Some(next);
            }
        } else if let "HLT\n" = &action[..] {
            break None;
        }
        print!("Try again (must be less than the number of states or HLT): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut action).expect("Failed to read line");
    };

    let mut write = String::new();

    print!("What should this action write?: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut write).expect("Failed to read line");

    let write: u8 = loop {
        if let Ok(to_write) = write.trim_end().parse::<u8>() {
            if to_write <= 1 {
                break to_write;
            }
        }
        print!("Try again (must be 1 or 0): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut write).expect("Failed to read line");
    };

    let mut inc_dec = String::new();

    print!("Where should this move the tape?\n1 = right, 0 = stay, -1 = left: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut inc_dec).expect("Failed to read line");

    let inc_dec: isize = loop {
        if let Ok(inc_dec) = inc_dec.trim_end().parse::<isize>() {
            if inc_dec >= -1 && inc_dec <= 1 {
                break inc_dec;
            }
        }
        print!("Try again (must be 1, 0, or -1): ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut inc_dec).expect("Failed to read line");
    };

    Action::new(inc_dec, write, next)
}

pub fn create_tape() {
    let mut tape = String::new();

    println!("Input tape as 1's and 0's (ex: '1000100110110110')");
    io::stdin().read_line(&mut tape).expect("Failed to read line");

    let tape = tape.chars()
        .filter_map(|char| char.to_digit(2))
        .map(|num| num as u8)
        .collect::<Vec<u8>>();

    let mut start_index = String::new();

    println!("Print this index of the tape to start at");
    io::stdin().read_line(&mut start_index).expect("Failed to read line");

    let start_index: usize = loop {
        if let Ok(val) = start_index.trim_end().parse::<usize>() {
            if val < tape.len() {
                break val;
            }
        }
        print!("Must be less than the length of the tape");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut start_index).expect("Failed to read line");
    };

    let tape = Tape::new(tape, start_index);
    println!("WARNING: THIS WILL OVERRITE ANY CONTENTS OF tape.json IN DIRECTORY THE PROGRAM IS RAN IN");

    fs::write("tape.json", to_string(&tape).expect("Failed stringify")).expect("Failed to write file");
}
