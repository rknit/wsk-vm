class Inst:
    def __init__(self, format: str, op: str, f3: str, f7: str, name: str) -> None:
        self.format = format.lower()
        self.op = op
        self.f3 = f3
        self.f7 = f7
        self.name = name.lower().capitalize()
        self.special_match: str = ""

    def enum_fields(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd: u8, rs1: u8, rs2: u8"
        if fmt == "i":
            return "rd: u8, rs1: u8, imm: i64"
        if fmt == "s":
            return "rs1: u8, rs2: u8, offset: i64"
        if fmt == "b":
            return "rs1: u8, rs2: u8, offset: i64"
        if fmt == "u":
            return "rd: u8, imm: i64"
        if fmt == "j":
            return "rd: u8, offset: i64"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"

    def enum(self) -> str:
        s = self.enum_fields()
        return f"{self.name} {{ {s} }},"

    def enum_args(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd, rs1, rs2"
        if fmt == "i":
            return "rd, rs1, imm"
        if fmt == "s":
            return "rs1, rs2, offset"
        if fmt == "b":
            return "rs1, rs2, offset"
        if fmt == "u":
            return "rd, imm"
        if fmt == "j":
            return "rd, offset"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"

    def enum_args_assign(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd, rs1, rs2"
        if fmt == "i":
            return "rd, rs1, imm: sext(imm as u64, 11) as i64"
        if fmt == "s":
            return "rs1, rs2, offset: sext(imm as u64, 11) as i64"
        if fmt == "b":
            return "rs1, rs2, offset: sext((imm as u64) << 1, 12) as i64"
        if fmt == "u":
            return "rd, imm: ((imm as i32) << 12) as i64"
        if fmt == "j":
            return "rd, offset: sext((imm as u64) << 1, 20) as i64"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"

    def get_special_match(self) -> str:
        if not self.special_match:
            return ""
        sm = self.special_match
        
        if sm == "slli" or sm == "srli":
            return "if ext!(imm, u8; 11;6) == 0"
        if sm == "srai":
            return "if ext!(imm, u8; 11;6) == 0b010000"
        
        assert False, f"invalid special match {self.special_match} for {self.name}"

    def decode_arm(self) -> str:
        fmt = self.format
        sm = self.get_special_match()
        if fmt == "r":
            return f"(0b{self.op}, 0b{self.f3}, 0b{self.f7}){sm} => Inst::{self.name} {{ {self.enum_args_assign()} }},\n"
        if fmt == "i" or fmt == "s" or fmt == "b":
            return f"(0b{self.op}, 0b{self.f3}){sm} => Inst::{self.name} {{ {self.enum_args_assign()} }},\n"
        if fmt == "u" or fmt == "j" or fmt == "o":
            return f"0b{self.op}{sm} => Inst::{self.name} {{ {self.enum_args_assign()} }},\n"
        assert False, f"invalid format {self.format}"
        
    def run_params(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd: u8, r1: u64, r2: u64"
        if fmt == "i":
            return "rd: u8, r1: u64, imm: i64"
        if fmt == "s":
            return "r1: u64, r2: u64, offset: i64"
        if fmt == "b":
            return "r1: u64, r2: u64, offset: i64"
        if fmt == "u":
            return "rd: u8, imm: i64"
        if fmt == "j":
            return "rd: u8, offset: i64"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"
        
    def run_args(self) -> str:
        fmt = self.format
        if fmt == "r":
            return "rd, vm.x(rs1), vm.x(rs2)"
        if fmt == "i":
            return "rd, vm.x(rs1), imm"
        if fmt == "s":
            return "vm.x(rs1), vm.x(rs2), offset"
        if fmt == "b":
            return "vm.x(rs1), vm.x(rs2), offset"
        if fmt == "u":
            return "rd, imm"
        if fmt == "j":
            return "rd, offset"
        if fmt == "o":
            return ""
        assert False, f"invalid format {self.format}"
        
    def run_arm(self) -> str:
        return f"Inst::{self.name} {{ {self.enum_args()} }} => {self.name}::run(vm, {self.run_args()}),\n"
    
    def __str__(self) -> str:
        return f"Inst(format={self.format}, op={self.op}, f3={self.f3}, f7={self.f7}, name={self.name})"
    
    def __eq__(self, value: object) -> bool:
        if not isinstance(value, Inst):
            return False
        return self.name == value.name
        
    def __hash__(self) -> int:
        return hash(self.name)


class Module:
    def __init__(self, name: str) -> None:
        self.name = name
        self.file_name = name.lower()
        self.insts: list[Inst] = list()

    def append(self, inst: Inst):
        assert inst not in self.insts, "duplicate inst"
        self.insts.append(inst)

    def all_format(self, fmt: str) -> list[Inst]:
        insts: list[Inst] = list()
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