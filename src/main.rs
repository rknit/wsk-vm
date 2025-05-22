use std::{env, fs, process::exit};

use wsk_vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.reset();

    let path = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("usage: wsk-vm <path-to-binary>");
            exit(1);
        }
    };
    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("read from file error: {:?}", e);
            exit(2);
        }
    };

    if let Err(e) = vm.load_program_from_bytes(&bytes) {
        eprintln!("load prog error: {:?}", e);
        exit(3);
    }

    if let Err(e) = vm.run() {
        eprintln!("vm error: {:?}", e);
        exit(4);
    }
}
