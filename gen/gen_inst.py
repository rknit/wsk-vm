#! /usr/bin/env python

OUTPUT_DIR = "./output"


class Inst:
    def __init__(self, format: str, op: str, f3: str, f7: str, name: str) -> None:
        self.format = format.lower()
        self.op = op
        self.f3 = f3
        self.f7 = f7
        self.name = name.lower().capitalize()

    def run_params(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd: u8, r1: u64, r2: u64"
        if fmt == "i":
            return "rd: u8, r1: u64, imm: i64"
        if fmt == "s":
            return "rd: u8, r1: u64, rs2: u64, offset: i64"
        if fmt == "b":
            return "r1: u64, rs2: u64, offset: i64"
        if fmt == "u":
            return "rd: u8, imm: i64"
        if fmt == "j":
            return "rd: u8, offset: i64"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"

    def enum(self) -> str:
        s = self.run_params()
        return f"{self.name} {{ {s} }},"

    def enum_args(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd, r1, r2"
        if fmt == "i":
            return "rd, r1, imm"
        if fmt == "s":
            return "rd, r1, rs2, offset"
        if fmt == "b":
            return "r1, rs2, offset"
        if fmt == "u":
            return "rd, imm"
        if fmt == "j":
            return "rd, offset"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"

    def run_arm(self) -> str:
        return f"Inst::{self.name} {{ {self.enum_args()} }} => {self.name}::run(vm, {self.enum_args()}),\n"

    def decode_arm(self) -> str:
        fmt = self.format
        if fmt == "r":
            return f"(0b{self.op}, 0b{self.f3}, 0b{self.f7}) => Inst::{self.name} {{ {self.enum_args()} }},\n"
        if fmt == "i" or fmt == "s" or fmt == "b":
            return f"(0b{self.op}, 0b{self.f3}) => Inst::{self.name} {{ {self.enum_args()} }},\n"
        if fmt == "u" or fmt == "j" or fmt == "o":
            return f"0b{self.op} => Inst::{self.name} {{ {self.enum_args()} }},\n"
        assert False, f"invalid format {self.format}"


class Module:
    def __init__(self, name: str) -> None:
        self.name = name
        self.file_name = name.lower()
        self.insts: list[Inst] = list()

    def append(self, inst: Inst):
        self.insts.append(inst)

    def validate(self):
        s = set()
        for inst in self.insts:
            v = (inst.op, inst.f3, inst.f7)
            assert v not in s, f"duplicate inst '{inst.name}'"
            s.add(v)

    def all_format(self, fmt: str) -> list[Inst]:
        insts = list()
        for inst in self.insts:
            if inst.format == fmt:
                insts.append(inst)
        return insts


class Modules:
    def __init__(self) -> None:
        self.modules: dict[str, Module] = dict()

    def append(self, mod: Module):
        assert mod.name not in self.modules, "duplicate module"
        self.modules[mod.name] = mod

    def get(self, name: str) -> Module:
        return self.modules[name]

    def mods(self):
        return self.modules.values()

    def len(self) -> int:
        return len(self.modules)

    def all_format(self, fmt: str) -> list[Inst]:
        return [inst for inst in self.all_inst() if inst.format == fmt]

    def all_inst(self) -> list[Inst]:
        return [inst for mod in self.modules.values() for inst in mod.insts]


def gen_main(modules: Modules):
    with open("./inst.rs", "w") as _gen:

        def gen(s="", endl="\n"):
            _gen.write(s + endl)

        gen("use crate::{VM, VMRunError, VMRunErrorKind};\n")
        gen("use super::format::RawFormat;\n")

        # include all modules
        for mod in modules.mods():
            mod_name = mod.name.lower()
            gen(f"mod {mod_name};")
            gen(f"use {mod_name}::*;")
            gen()

        # create Inst enum
        gen("pub enum Inst {")
        for mod_idx, mod in enumerate(modules.mods()):
            gen(f"    // {mod.name.upper()}")
            for inst in mod.insts:
                gen(f"    {inst.enum()}")
            if mod_idx + 1 != modules.len():
                gen()
        gen("}")

        # decode function
        gen(f"""
impl Inst {{
    pub fn decode(fmt: RawFormat) -> Option<Self> {{
        use RawFormat::*;
        Some(match fmt {{
            R {{
                opc,
                funct3,
                funct7,
                rd,
                rs1,
                rs2,
            }} => match (opc, funct3, funct7) {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("r")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
            I {{
                opc,
                funct3,
                rd,
                rs1,
                imm,
            }} => match (opc, funct3) {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("i")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
            S {{
                opc,
                funct3,
                rs1,
                rs2,
                imm,
            }} => match (opc, funct3) {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("s")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
            B {{
                opc,
                funct3,
                rs1,
                rs2,
                imm,
            }} => match (opc, funct3) {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("b")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
            U {{
                opc,
                rd,
                imm,
            }} => match opc {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("u")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
            J {{
                opc,
                rd,
                imm,
            }} => match opc {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("j")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
            Other {{
                opc,
            }} => match opc {{
                {"                ".join([inst.decode_arm() for inst in modules.all_format("o")])}
                #[allow(unreachable_patterns)]
                _ => return None,
            }},
        }})
    }}""")

        # run function
        gen(f"""
    pub fn run(&self, vm: &mut VM) -> Result<(), VMRunError> {{
        match self {{
            {"            ".join([inst.run_arm() for inst in modules.all_inst()])}
            #[allow(unreachable_patterns)]
            _ => return VMRunError {{
                addr: vm.pc,
                kind: VMRunErrorKind::Other(format!("{{:?}}", self)),
                info: "unimplemented inst",
            }},
        }}
    }}""")

        # finish impl
        gen("}")


if __name__ == "__main__":
    modules = Modules()
    cur_mod = None

    with open("./insts.txt", "r") as file:
        for line in file:
            toks = line.split()
            if len(toks) == 0:
                continue
            elif len(toks) == 1:
                line = line.strip()
                assert line[0] == "[" and line[-1] == "]", "not a module declaration"
                mod_name = line.lstrip("[").rstrip("]")
                modules.append(Module(mod_name))
                cur_mod = mod_name
            elif len(toks) == 5:
                assert cur_mod is not None, "no module to insert to"
                mod = modules.get(cur_mod)
                mod.append(Inst(toks[0], toks[1], toks[2], toks[3], toks[4]))
            else:
                assert False, "invalid line"

    for mod in modules.mods():
        mod.validate()

    gen_main(modules)

    # create instruction structs and their run methods
    for mod in modules.mods():
        with open(f"./{mod.file_name}.rs", "w") as _gen:

            def gen(s="", endl="\n"):
                _gen.write(s + endl)

            gen("use crate::{VM, VMRunError}\n")

            for inst in mod.insts:
                gen(f"""pub struct {inst.name};
impl {inst.name} {{
    pub fn run(vm: &mut VM, {inst.run_params()}) -> Result<(), VMRunError> {{
        todo!("implement {inst.name} please!");
        Ok(())
    }}
}}
""")
