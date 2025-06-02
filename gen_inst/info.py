class Inst:
    def __init__(self, name: str, format: str, bit_pat: str, pats: list[tuple[int, int, str]]) -> None:
        assert len(format) == 1, "format must be a single character"
        self.raw_name = name
        self.format = format.strip().upper()
        self.symbol = "".join([part.capitalize() if len(part) > 1 else part.upper() for part in name.strip().lower().split(".")])
        self.bit_pat = bit_pat.strip().upper()
        self.pats = pats
        self.impl = ""
        
    def decode_cond(self) -> str:
        return " && ".join([f"ext!(v, u32; {hi};{lo}) == 0b{pat}" for hi, lo, pat in self.pats])
    
    def decode_arm(self) -> str:
        return f"v if {self.decode_cond()} => Inst::{self.symbol}(inst.into()),"

    def enum_variant(self) -> str:
        return f"{self.symbol}(RawInst),"
        
    def run_arm(self) -> str:
        return f"Inst::{self.symbol}(inst) => {self.symbol}::run(RunData::new(inst, vm)),\n"
    
    def name_arm(self) -> str:
        return f"Inst::{self.symbol}(_) => \"{self.raw_name}\",\n"
    
    def format_arm(self) -> str:
        return f"Inst::{self.symbol}(_) => Format::{self.format},\n"
    
    def run_param(self) -> str:
        return f"mut data: RunData"
    
    def __str__(self) -> str:
        return f"Inst(name={self.raw_name}, symbol={self.symbol})"
    
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

    def all_inst(self) -> list[Inst]:
        return [inst for mod in self.modules.values() for inst in mod.insts]