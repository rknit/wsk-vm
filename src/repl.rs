use std::{
    collections::HashSet,
    fs,
    io::{Write, stdin, stdout},
    path::Path,
    process::exit,
    str::SplitWhitespace,
};

use log::log_enabled;

use crate::{VM, VMRunError};

pub fn run_repl(path: &Path) -> Result<u8, VMRunError> {
    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("read from file error: {e}");
            exit(2);
        }
    };

    let mut vm = VM::new();
    vm.reset();

    if let Err(e) = vm.load_executable_bytes(&bytes) {
        eprintln!("load prog error: {e:?}");
        exit(3);
    }

    let mut brk: HashSet<usize> = HashSet::new();
    let mut cmd = String::new();

    let step = |vm: &mut VM| -> Result<(), VMRunError> {
        vm.step()?;
        if !log_enabled!(log::Level::Trace) {
            println!("{}", vm.report());
        }
        Ok(())
    };

    'q: loop {
        print!(">> ");
        stdout().flush().unwrap();

        cmd.clear();
        stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim();
        let mut toks = cmd.split_whitespace();

        let Some(cmd) = toks.next() else {
            continue;
        };

        match cmd {
            "h" => {
                println!("q           : quit REPL");
                println!("s           : display VM status");
                println!("n           : advance one instruction");
                println!("c           : advance until breakpoint or halt");
                println!("b <address> : toggle breakpoint at address");
                println!("j <address> : force jump to address");
            }
            "q" => break 'q,
            "s" => vm.display(&mut stdout()).unwrap(),
            "n" => step(&mut vm)?,
            "c" => {
                while !vm.halted() {
                    step(&mut vm)?;

                    let addr = vm.pc;
                    if brk.contains(&addr) {
                        println!("breakpoint hit: addr = {addr:x}");
                        break;
                    }
                }
            }
            "b" => {
                let Some(addr) = parse_address(&mut toks) else {
                    continue;
                };
                if brk.contains(&addr) {
                    println!("unset breakpoint at {addr:x}");
                    brk.remove(&addr);
                } else {
                    println!("set breakpoint at {addr:x}");
                    brk.insert(addr);
                }
            }
            "j" => {
                let Some(addr) = parse_address(&mut toks) else {
                    continue;
                };
                vm.pc = addr;
                println!("set program counter to {addr:x}");
            }
            _ => {
                eprintln!("unknown command '{cmd}'");
                continue;
            }
        }

        if vm.halted() {
            println!("VM exited with code {}", vm.exit_code());
        }
    }
    Ok(vm.exit_code())
}

fn parse_address(toks: &mut SplitWhitespace) -> Option<usize> {
    let Some(addr) = toks.next() else {
        eprintln!("b: expect address");
        return None;
    };
    let radix = if addr.starts_with("0x") { 16 } else { 10 };
    let addr = if addr.starts_with("0x") {
        addr.trim_start_matches("0x")
    } else {
        addr
    };
    let addr: usize = match usize::from_str_radix(addr, radix) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("b: {e}");
            return None;
        }
    };
    Some(addr)
}
