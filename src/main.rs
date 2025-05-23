use std::{env, fs, io::stdout, process::exit};

use wsk_vm::VM;

fn main() {
    env_logger::init();

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
            eprintln!("read from file error: {e:?}");
            exit(2);
        }
    };

    if let Err(e) = vm.load_executable_bytes(&bytes) {
        eprintln!("load prog error: {e:?}");
        exit(3);
    }

    if let Err(e) = vm.run() {
        eprintln!("vm error: {e:?}");
        exit(4);
    }

    vm.display(&mut stdout()).unwrap();
}
