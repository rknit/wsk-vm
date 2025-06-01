use std::{fs, path::PathBuf, process::exit};

use clap::{Parser, Subcommand};
use wsk_vm::{VM, VMRunError, repl::run_repl};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Path to ELF executable file
    #[arg(short, long)]
    input: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Use REPL to control VM
    Repl,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let bytes = match fs::read(&cli.input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("read file error: {e}");
            exit(2);
        }
    };

    let r = match cli.command.as_ref() {
        Some(cmd) => match cmd {
            Commands::Repl => run_repl(&bytes),
        },
        None => run_prog(&bytes),
    };

    let exit_code = match r {
        Ok(v) => v,
        Err(e) => {
            eprintln!("vm error: {e}");
            exit(55);
        }
    };
    exit(exit_code as i32);
}

fn run_prog(prog: &[u8]) -> Result<u8, VMRunError> {
    let mut vm = VM::new();
    vm.reset();

    if let Err(e) = vm.load_executable_bytes(prog) {
        eprintln!("load program error: {e:?}");
        exit(3);
    }

    vm.run()
}
