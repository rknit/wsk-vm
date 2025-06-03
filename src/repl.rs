use std::{
    collections::HashSet,
    io::{BufRead, BufReader, BufWriter, Read, Write, stdin, stdout},
    process::exit,
};

use crate::{InstReport, VM, VMRunError, byte, uarch, x_name};

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
    Addr(uarch),
    Inst(String),
}
impl From<uarch> for Breakpoint {
    fn from(value: uarch) -> Self {
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
    start_addr: uarch,
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

pub fn run_repl(prog: &[byte]) -> Result<byte, VMRunError> {
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

        let mut toks = line.split_whitespace();
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
        "i" | "info" => Ok(info(repl)),
        _ => {
            writeln!(
                repl.cout,
                "unknown command '{}', run 'help' to list commands.",
                repl.cmd
            )
            .unwrap();
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
s, step <N>          -- advance VM for N steps or until interrupted (N is 1 if unspecified).
c, cont              -- advance VM until interrupted.
b, brk  <addr/inst>  -- toggle breakpoint at address or instruction (address must be '0x' prefixed).
i, info <arg>        -- show information about the VM status.
                        -- <arg> can be:
                        --   'pc' for program counter.
                        --   'm <start>.<end>' for memory dump (end is exclusive).
                        --   'x <mode>' for all registers where <mode> = 'hex'(default) / 'dec'.
                        --   'x <index> <mode>' for specific register where <mode> = 'hex'(default) / 'dec'.

"#
    )
    .unwrap();
}

fn info<W: Write, R: BufRead>(repl: &mut Repl<W, R>) {
    if repl.args.is_empty() {
        writeln!(repl, "i: no argument provided").unwrap();
        return;
    }

    match repl.args[0].as_str() {
        "pc" => {
            let pc = repl.vm.pc;
            writeln!(repl, "pc = {pc:x}").unwrap()
        }
        "m" => {
            if repl.args.len() < 2 {
                writeln!(repl, "i: m: expect range <start>.<end>").unwrap();
                return;
            }
            let range: Vec<&str> = repl.args[1].split(".").collect();
            if range.len() != 2 {
                writeln!(repl, "i: m: invalid range format").unwrap();
                return;
            }
            let Ok(start) = uarch::from_str_radix(range[0], 16) else {
                writeln!(repl, "i: m: invalid start address").unwrap();
                return;
            };
            let Ok(end) = uarch::from_str_radix(range[1], 16) else {
                writeln!(repl, "i: m: invalid end address").unwrap();
                return;
            };
            let mut addr = start;
            while addr < end {
                let Ok(bytes) = repl.vm.mem_range(addr, 16) else {
                    writeln!(repl, "{addr:04x}: <invalid>").unwrap();
                    break;
                };

                let hex = bytes
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<_>>()
                    .join(" ");

                writeln!(repl, "{addr:04x}: {hex}").unwrap();
                addr += 16;
            }
        }
        "x" => {
            if repl.args.len() < 2 {
                display_regs(repl, true);
                return;
            }
            let reg_index: u8 = match repl.args[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    match repl.args[1].as_str() {
                        "dec" => {
                            display_regs(repl, false);
                            return;
                        }
                        "hex" => {
                            display_regs(repl, true);
                            return;
                        }
                        _ => (),
                    }

                    let arg = repl.args[1].to_owned();
                    writeln!(repl, "i: x: unknown argument '{arg}'").unwrap();
                    return;
                }
            };
            let reg_name = x_name(reg_index);
            let val = repl.vm.x(reg_index);

            if let Some("dec") = repl.args.get(2).map(|s| s.as_str()) {
                writeln!(repl, "{reg_name} = {val}").unwrap();
            } else if matches!(repl.args.get(2).map(|s| s.as_str()), Some("hex") | None) {
                writeln!(repl, "{reg_name} = {val:x}").unwrap();
            } else {
                let arg = repl.args[2].to_owned();
                writeln!(repl, "i: x: unknown argument '{arg}'").unwrap();
            }
        }
        _ => {
            let s = format!("i: unknown argument '{}'", repl.args[0]);
            writeln!(repl, "{s}").unwrap()
        }
    }
}

fn display_regs<W: Write, R: BufRead>(repl: &mut Repl<W, R>, in_hex: bool) {
    for j in 0..8 {
        for i in 0..4 {
            let idx = i * 8 + j;
            if in_hex {
                write!(repl, "{}:\t{:16x} | ", x_name(idx), repl.vm.x(idx)).unwrap();
            } else {
                write!(repl, "{}:\t{:16} | ", x_name(idx), repl.vm.x(idx)).unwrap();
            }
        }
        writeln!(repl).unwrap();
    }
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
        let addr = match uarch::from_str_radix(addr, 16) {
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
            if repl.hit_breakpoint(rep.inst.name().to_owned()) {
                writeln!(repl, "breakpoint hit inst = {}", rep.inst.name()).unwrap();
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
    let Ok(inst) = repl.vm.fetch_inst(repl.vm.pc) else {
        return None;
    };

    Some(InstReport {
        addr: repl.vm.pc,
        inst,
    })
}
