from typing import Any

class Inst:
    def __init__(self, name: str, format: str, data: dict[str, Any]) -> None:
        assert len(format) == 1, "format must be a single character"
        self.raw_name = name
        self.format = format.lower()
        self.data = data
        self.symbol = "".join([part.capitalize() if len(part) > 1 else part.upper() for part in name.strip().lower().split(".")])
        self.impl = ""
        
    def decode_data(self) -> dict[str, str]:
        assert "decode" in self.data
        return self.data["decode"]
        
    @staticmethod
    def decode_args(fmt: str) -> list[tuple[str, str]]:
        if fmt == "r":
            return [("opc", "u8"), ("f3", "u8"), ("f7", "u8"), ("rd", "u8"), ("rs1", "u8"), ("rs2", "u8")]
        if fmt == "i":
            return [("opc", "u8"), ("f3", "u8"), ("rd", "u8"), ("rs1", "u8"), ("imm", "i16")]
        if fmt == "s":
            return [("opc", "u8"), ("f3", "u8"), ("rs1", "u8"), ("rs2", "u8"), ("imm", "i16")]
        if fmt == "b":
            return [("opc", "u8"), ("f3", "u8"), ("rs1", "u8"), ("rs2", "u8"), ("imm", "i16")]
        if fmt == "u":
            return [("opc", "u8"), ("rd", "u8"), ("imm", "i32")]
        if fmt == "j":
            return [("opc", "u8"), ("rd", "u8"), ("imm", "i32")]
        assert False, f"invalid format {fmt}"
        
    @staticmethod
    def decode_pat(fmt: str) -> str:
        return ",".join([arg[0] for arg in Inst.decode_args(fmt)])
    
    def enum_fields(self) -> list[tuple[str, str]]:
        fmt = self.format
        if fmt == "r":
            return [("rd", "u8"), ("rs1", "u8"), ("rs2", "u8")]
        if fmt == "i":
            return [("rd", "u8"), ("rs1", "u8"), ("imm", "i16")]
        if fmt == "s":
            return [("rs1", "u8"), ("rs2", "u8"), ("imm", "i16")]
        if fmt == "b":
            return [("rs1", "u8"), ("rs2", "u8"), ("imm", "i16")]
        if fmt == "u":
            return [("rd", "u8"), ("imm", "i32")]
        if fmt == "j":
            return [("rd", "u8"), ("imm", "i32")]
        assert False, f"invalid format {self.format}"
        
    def enum_args_assign(self) -> str:
        return ",".join([field[0] for field in self.enum_fields()])
    
    def enum_pat(self) -> str:
        return ",".join([field[0] for field in self.enum_fields()])
    
    def decode_arm(self) -> str:
        params = self.decode_args(self.format)
        dec_data = self.decode_data()
        args: list[str] = list()
        conds: list[str] = list()
        for param in params:
            if param[0] not in dec_data:
                args.append("_")
                continue
            data = dec_data[param[0]]
            if "if" in data:
                parts = data.split("if")
                args.append(parts[0].strip())
                conds.append(parts[1].strip())
            else:
                args.append(data.strip())
        if len(args) < len(params):
            args += ["_"] * (len(params) - len(args))
        if len(conds) == 0:
            return f"({ ','.join(args) }) => Inst::{self.symbol} {{ {self.enum_args_assign()} }},\n"
        else:
            return f"({ ','.join(args) }) if { ' && '.join(conds) } => Inst::{self.symbol} {{ {self.enum_args_assign()} }},\n"

    def enum_fields_decl(self) -> str:
        return ",".join([":".join(field) for field in self.enum_fields()])

    def enum_variant(self) -> str:
        s = self.enum_fields_decl()
        return f"{self.symbol} {{ {s} }},"
    
    def data_args(self) -> str:
        def data_field(s: str) -> str:
            if s == "imm":
                return "raw_imm"
            return s
        return ",".join([
            f"{data_field(field[0])}: {field[0]}" if data_field(field[0]) != field[0] else field[0]
            for field in self.enum_fields()
        ])
    
    def create_data(self) -> str:
        return f"Data{self.format.upper()} {{ vm, {self.data_args()} }}"
        
    def run_arm(self) -> str:
        return f"Inst::{self.symbol} {{ {self.enum_pat()} }} => {self.symbol}::run({self.create_data()}),\n"
    
    def name_arm(self) -> str:
        return f"Inst::{self.symbol} {{ .. }} => \"{self.raw_name}\",\n"
    
    def run_param(self) -> str:
        return f"mut data: Data{self.format.upper()}"
    
    def __str__(self) -> str:
        return f"Inst(name={self.raw_name}, format={self.format}, symbol={self.symbol})"
    
    def __eq__(self, value: object) -> bool:
        if not isinstance(value, Inst):
            return False
        return self.raw_name == value.raw_name
        
    def __hash__(self) -> int:
        return hash(self.raw_name)


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