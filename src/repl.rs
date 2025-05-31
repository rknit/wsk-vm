use std::{
    collections::HashSet,
    io::{BufRead, BufReader, BufWriter, Read, Write, stdin, stdout},
    process::exit,
};

use crate::{InstReport, VM, VMRunError, format::RawFormat};

#[derive(Clone, Copy)]
enum RunUntil {
    Next(usize),
    Halt,
}

#[derive(Clone, Copy)]
enum RunWith {
    Report,
}

#[derive(PartialEq, Eq, Hash)]
enum Breakpoint {
    Addr(usize),
    Inst(String),
}
impl From<usize> for Breakpoint {
    fn from(value: usize) -> Self {
        Breakpoint::Addr(value)
    }
}
impl From<String> for Breakpoint {
    fn from(value: String) -> Self {
        Breakpoint::Inst(value)
    }
}

struct Repl<W: Write, R: BufRead> {
    cin: R,
    cout: W,
    exit: bool,
    vm: VM,
    start_addr: usize,
    brk: HashSet<Breakpoint>,
    cmd: String,
    args: Vec<String>,
}
impl<W: Write, R: BufRead> Repl<W, R> {
    fn hit_breakpoint(&self, bp: impl Into<Breakpoint>) -> bool {
        self.brk.contains(&bp.into())
    }
}
impl<W: Write, R: BufRead> Read for Repl<W, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cin.read(buf)
    }
}
impl<W: Write, R: BufRead> Write for Repl<W, R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.cout.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.cout.flush()
    }
}

pub fn run_repl(prog: &[u8]) -> Result<u8, VMRunError> {
    let mut repl = Repl {
        cin: BufReader::new(stdin()),
        cout: BufWriter::new(stdout()),
        exit: false,
        vm: VM::new(),
        start_addr: 0,
        brk: Default::default(),
        cmd: Default::default(),
        args: Default::default(),
    };
    repl.vm.reset();

    if let Err(e) = repl.vm.load_executable_bytes(prog) {
        eprintln!("load program error: {e:?}");
        exit(3);
    }
    repl.start_addr = repl.vm.pc;

    while !repl.exit {
        poll(&mut repl);
        run_cmd(&mut repl)?;
    }

    if repl.vm.halted() {
        Ok(repl.vm.exit_code())
    } else {
        Ok(0)
    }
}

fn poll<W: Write, R: BufRead>(repl: &mut Repl<W, R>) {
    loop {
        write!(repl, "repl> ").unwrap();
        repl.cout.flush().unwrap();

        let mut line = String::new();
        repl.cin.read_line(&mut line).unwrap();

        let mut toks = line.trim().split_whitespace();
        let Some(cmd) = toks.next() else {
            continue;
        };

        repl.cmd = cmd.to_owned();
        repl.args = toks.map(|v| v.to_owned()).collect();
        break;
    }
}

fn run_cmd<W: Write, R: BufRead>(repl: &mut Repl<W, R>) -> Result<(), VMRunError> {
    match repl.cmd.as_str() {
        "h" | "help" => Ok(help(repl)),
        "q" | "quit" => {
            repl.exit = true;
            Ok(())
        }
        "r" | "reset" => Ok(reset(repl)),
        "s" | "step" => {
            let step = if let Some(cnt_arg) = repl.args.get(0) {
                match cnt_arg.parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        writeln!(repl, "step: {e}").unwrap();
                        return Ok(());
                    }
                }
            } else {
                1
            };
            run_vm(repl, RunWith::Report, RunUntil::Next(step))
        }
        "c" | "cont" => run_vm(repl, RunWith::Report, RunUntil::Halt),
        "b" | "brk" => Ok(toggle_brk(repl)),
        _ => {
            writeln!(repl.cout, "unknown command '{}'", repl.cmd).unwrap();
            Ok(())
        }
    }
}

fn help<W: Write, R: BufRead>(repl: &mut Repl<W, R>) {
    write!(
        repl,
        r#"
h, help              -- list commands.
q, quit              -- exit REPL.
r, reset             -- reset REPL.
s, step <N>          -- advance VM for N steps (N is 1 if unspecified) until interrupted.
c, cont              -- advance VM until interrupted.
b, brk  <addr/inst>  -- toggle breakpoint for address ('0x' prefixed) or instruction.

"#
    )
    .unwrap();
}

fn reset<W: Write, R: BufRead>(repl: &mut Repl<W, R>) {
    repl.vm.reset();
    repl.vm.pc = repl.start_addr;
    repl.brk.clear();
    repl.exit = false;
    repl.cmd.clear();
    repl.args.clear();
    writeln!(repl, "resetted REPL.").unwrap();
    repl.flush().unwrap();
}

fn toggle_brk<W: Write, R: BufRead>(repl: &mut Repl<W, R>) {
    let Some(brk) = repl.args.get(0) else {
        writeln!(
            repl,
            "brk: expect address or instruction name as a breakpoint"
        )
        .unwrap();
        return;
    };
    if let Some(addr) = brk.strip_prefix("0x") {
        let addr: usize = match usize::from_str_radix(addr, 16) {
            Ok(v) => v,
            Err(e) => {
                writeln!(repl, "brk: invalid address: {e}").unwrap();
                return;
            }
        };
        let bp = Breakpoint::Addr(addr);
        if repl.brk.contains(&bp) {
            writeln!(repl, "brk: removed breakpoint at address {addr:x}").unwrap();
            repl.brk.remove(&bp);
        } else {
            writeln!(repl, "brk: added breakpoint at address {addr:x}").unwrap();
            repl.brk.insert(bp);
        }
    } else {
        if !brk.chars().all(|v| v.is_alphabetic() || v == '.') {
            let fmt = format!("brk: invalid breakpoint argument: '{brk}'");
            writeln!(repl, "{}", fmt).unwrap();
            return;
        }

        let inst = brk.to_owned();
        let bp = Breakpoint::Inst(inst.clone());
        if repl.brk.contains(&bp) {
            writeln!(repl, "brk: removed breakpoint at instruction '{inst}'").unwrap();
            repl.brk.remove(&bp);
        } else {
            writeln!(repl, "brk: added breakpoint at instruction '{inst}'").unwrap();
            repl.brk.insert(bp);
        }
    }
}

fn run_vm<W: Write, R: BufRead>(
    repl: &mut Repl<W, R>,
    with: RunWith,
    mut until: RunUntil,
) -> Result<(), VMRunError> {
    loop {
        match with {
            RunWith::Report => {
                if let Some(rep) = new_report(repl) {
                    writeln!(repl, "{}", rep).unwrap();
                }
            }
        };

        repl.vm.step()?;

        if repl.hit_breakpoint(repl.vm.pc) {
            let addr = repl.vm.pc;
            writeln!(repl, "breakpoint hit addr = {:x}", addr).unwrap();
            break;
        } else if let Some(rep) = new_report(repl) {
            if repl.hit_breakpoint(rep.inst_name.to_owned()) {
                writeln!(repl, "breakpoint hit inst = {}", rep.inst_name).unwrap();
                break;
            }
        }

        match until {
            RunUntil::Next(s) => {
                if s == 1 {
                    break;
                }
                until = RunUntil::Next(s - 1);
            }
            RunUntil::Halt => {
                if repl.vm.halted() {
                    let code = repl.vm.exit_code();
                    writeln!(repl, "VM halted with exit code {code}").unwrap();
                    break;
                }
            }
        }
    }
    Ok(())
}

fn new_report<W: Write, R: BufRead>(repl: &mut Repl<W, R>) -> Option<InstReport> {
    let raw_inst = {
        let Ok([b1, b2, b3, b4]) = repl.vm.mem_range(repl.vm.pc, 4) else {
            return None;
        };
        u32::from_le_bytes([*b1, *b2, *b3, *b4])
    };

    let format = RawFormat::parse(raw_inst).unwrap();

    let Ok(inst) = repl.vm.fetch_inst(repl.vm.pc) else {
        return None;
    };

    Some(InstReport {
        addr: repl.vm.pc,
        raw_inst,
        inst_name: inst.name(),
        format,
    })
}
