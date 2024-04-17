use std::fs;

use crate::config::{Config, Create, TuringAction};
use crate::create::{create_machine, create_tape};
use crate::machine::{Machine, Tape};
use serde_json::from_str;

pub fn run(config: Config) {
    match config.command {
        TuringAction::Run { machine_path, tape_path } => {
            let mut machine = from_str::<Machine>(&fs::read_to_string(machine_path).unwrap()).unwrap();
            let tape        = from_str::<Tape   >(&fs::read_to_string(tape_path)   .unwrap()).unwrap();

            machine.run(tape);

        },
        TuringAction::Create(Create::Both) => {
            create_machine();
            create_tape();
        },
        TuringAction::Create(Create::Machine) => {
            create_machine();
        },
        TuringAction::Create(Create::Tape) => {
            create_tape();
        }
    }
}
