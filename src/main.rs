use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

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

    let r = match cli.command.as_ref() {
        Some(cmd) => match cmd {
            Commands::Repl => run_repl(&cli.input),
        },
        None => run_prog(&cli.input),
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

fn run_prog(path: &Path) -> Result<u8, VMRunError> {
    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("read file error: {e}");
            exit(2);
        }
    };

    let mut vm = VM::new();
    vm.reset();

    if let Err(e) = vm.load_executable_bytes(&bytes) {
        eprintln!("load program error: {e:?}");
        exit(3);
    }

    vm.run()
}
