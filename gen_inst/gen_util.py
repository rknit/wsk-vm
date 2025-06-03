from util import get_match_pat_from_bit_pat
from info import Inst, Module, Modules
import toml

IMPL_START_TOKEN = "$IMPL_START"
IMPL_END_TOKEN = "$IMPL_END"

def read_insts(path: str) -> Modules:
    modules = Modules()

    with open(path, "r") as file:
        data = toml.load(file)
        
        for mod_name, format in data.items():
            module = Module(mod_name)
            for format, inst_data in format.items():
                for inst_name, bit_pat in inst_data.items():
                    pats = get_match_pat_from_bit_pat(bit_pat)
                    module.append(Inst(name=inst_name, format=format, bit_pat=bit_pat, pats=pats))
            modules.append(module)
        
    return modules

def gen_main(modules: Modules) -> str:
    out: str = str()
    def gen(s: str = "", endl: str = "\n"):
        nonlocal out
        out += s + endl

    # include all modules
    for mod in modules.mods():
        mod_name = mod.name.lower()
        gen(f"mod {mod_name};")
        gen(f"use {mod_name}::*;")
        gen()

    # create Inst enum
    gen("#[derive(Debug, Clone, Copy)]")
    gen("pub enum Inst {")
    for mod_idx, mod in enumerate(modules.mods()):
        gen(f"    // {mod.name.upper()}")
        for inst in mod.insts:
            gen(f"    {inst.enum_variant()}")
        if mod_idx + 1 != modules.len():
            gen()
    gen("}")

    # decode function
    gen(f"""
impl Inst {{
    pub fn decode(inst: word) -> Option<Self> {{
        Some(match inst {{
            {"            ".join([inst.decode_arm() for inst in modules.all_inst()])}
            #[allow(unreachable_patterns)]
            _ => return None,
        }})
    }}""")

    # run function
    gen(f"""
    pub fn run(self, vm: &mut VM) -> Result<(), VMRunError> {{
        match self {{
            {"            ".join([inst.run_arm() for inst in modules.all_inst()])}
        }}
    }}""")
    
    # name function
    gen(f"""
    pub fn name(self) -> &'static str {{
        match self {{
            {"            ".join([inst.name_arm() for inst in modules.all_inst()])}
        }}
    }}""")
    
    # format function
    gen(f"""
    pub fn format(self) -> Format {{
        match self {{
            {"            ".join([inst.format_arm() for inst in modules.all_inst()])}
        }}
    }}""")
    
    # util functions
    gen(f"""
    pub fn inner(self) -> RawInst {{
        match self {{
            {"            ".join([f"Inst::{inst.symbol}(v) => v," for inst in modules.all_inst()])}
        }}
    }}""")
    gen(f"""
    pub fn raw(self) -> word {{
        self.inner().raw()
    }}""")

    # finish impl
    gen("}")
    return out

def get_impl_start_token(inst: Inst) -> str:
    return f"// {IMPL_START_TOKEN} {inst.symbol}$"

def get_impl_end_token(inst: Inst) -> str:
    return f"// {IMPL_END_TOKEN} {inst.symbol}$"

def get_default_impl(inst: Inst) -> str:
    return f"todo!(\"implement {inst.symbol} please!\");\nOk(())"

def gen_inst_impl(inst: Inst) -> str:
    return f"""
pub struct {inst.symbol};
impl {inst.symbol} {{
    pub fn run({inst.run_param()}) -> Result<(), VMRunError> {{
        {get_impl_start_token(inst)}
        {inst.impl if inst.impl else get_default_impl(inst)}
        {get_impl_end_token(inst)}
    }}
}}"""

def gen_inst(inst: Inst) -> str:
    return gen_inst_impl(inst)

def gen_mod(mod: Module) -> dict[Inst, str]:
    out: dict[Inst, str] = dict()
    for inst in mod.insts:
        out[inst] = gen_inst(inst)        
    return out

def gen_modules(modules: Modules) -> dict[str, dict[Inst, str]]:
    mout: dict[str, dict[Inst, str]] = dict()
    for mod in modules.mods():
        mout[mod.file_name] = gen_mod(mod)
    return mout