use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    pub command: TuringAction
}

#[derive(Subcommand, Debug)]
pub enum TuringAction {
    #[command(subcommand)]
    Create(Create),

    Run {
        #[arg(short, long)]
        machine_path: Box<Path>,
        #[arg(short, long)]
        tape_path:    Box<Path>,
        #[arg(short, long, default_value_t = 1)]
        log:          usize,
        #[arg(short, long, default_value_t = false)]
        verbose:      bool
    }
}

#[derive(Subcommand, Debug)]
pub enum Create {
    Both,
    Machine,
    Tape
}
