use clap::Parser;
use config::Config;

pub mod machine;
pub mod config;
pub mod app;
pub mod create;

fn main() {
    let config = Config::parse();
    app::run(config);
}

#[cfg(test)]
pub mod tests {
    use serde_json::{from_str, to_string};
    use crate::machine::{Tape, Action, Machine, State};

    #[test]
    fn jsonify() {
        let action_0_0 = Action::new(1,  1, Some(1));
        let action_0_1 = Action::new(-1, 1, Some(2));
        let action_1_0 = Action::new(-1, 1, Some(0));
        let action_1_1 = Action::new(1,  1, Some(1));
        let action_2_0 = Action::new(-1, 1, Some(1));
        let action_2_1 = Action::new(1,  1, None   );

        let states = vec![State::new(action_0_0, action_0_1),
                          State::new(action_1_0, action_1_1),
                          State::new(action_2_0, action_2_1)];

        let machine = Machine::new(states);

        let tape = Tape::new(vec![0; 6], 3);

        let machine_json = from_str::<Machine>(&to_string(&machine).unwrap()).unwrap();
        let tape_json    = from_str::<Tape   >(&to_string(&tape)   .unwrap()).unwrap();

        assert_eq!(machine, machine_json);
        assert_eq!(tape, tape_json);
    }
}
