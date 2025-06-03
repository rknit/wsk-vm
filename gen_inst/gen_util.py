from util import get_match_pat_from_bit_pat
from info import Inst, Module, Modules
import toml

IMPL_START_TOKEN = "$IMPL_START"
IMPL_END_TOKEN = "$IMPL_END"

def read_insts(path: str) -> Modules:
    modules = Modules()

    with open(path, "r") as file:
        data = toml.load(file)
        
        for mod_name, mod_data in data.items():
            module = Module(mod_name)
            
            module.bit_len = mod_data.get("bit_len", 32)
            assert module.bit_len in {16, 32}, "bit_len must be either 16 or 32"
            
            for tag, tag_data in mod_data.items():
                if Module.is_module_info(tag):
                    continue
                format, inst_data = tag, tag_data
                
                for inst_name, bit_pat in inst_data.items():
                    if module.bit_len != 32:
                        bit_pat = "0" * (32 - module.bit_len) + bit_pat
                    
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
    gen("#[repr(u8)]")
    gen("pub enum Inst {")
    i = 0
    for mod_idx, mod in enumerate(modules.mods()):
        gen(f"    // {mod.name.upper()}")
        for inst in mod.insts:
            gen(f"    {inst.enum_variant()} = {i},")
            i += 1
        if mod_idx + 1 != modules.len():
            gen()
    gen("}")

    # decode function
    gen(f"""
impl Inst {{
    #[inline]
    pub const fn decode(inst: Word) -> Option<Self> {{
        Some(match inst {{
            {"            ".join([inst.decode_arm() for inst in modules.all_inst()])}
            #[allow(unreachable_patterns)]
            _ => return None,
        }})
    }}""")

    # run function
    gen(f"""
    #[inline]
    pub fn run(self, vm: &mut VM) -> Result<(), VMRunError> {{
        static RUN_TABLE: [fn(RawInst, &mut VM) -> Result<(), VMRunError>; {len(modules.all_inst())}] = [
            {"            ".join([inst.jump_table_entry() for inst in modules.all_inst()])}
        ];
        
        let id = self.discriminant() as usize;
        let raw_inst = self.inner();
        unsafe {{
            RUN_TABLE.get_unchecked(id)(raw_inst, vm)
        }}
    }}""")
    
    # name function
    gen(f"""
    #[inline]
    pub const fn name(self) -> &'static str {{
        match self {{
            {"            ".join([inst.name_arm() for inst in modules.all_inst()])}
        }}
    }}""")
    
    # format function
    gen(f"""
    #[inline]
    pub const fn format(self) -> Format {{
        match self {{
            {"            ".join([inst.format_arm() for inst in modules.all_inst()])}
        }}
    }}""")
    
    # util functions
    gen(f"""
    #[inline]
    pub const fn inner(self) -> RawInst {{
        match self {{
            {"            ".join([f"Inst::{inst.symbol}(v) => v," for inst in modules.all_inst()])}
        }}
    }}""")
    
    gen(f"""
    #[inline]
    pub const fn raw(self) -> Word {{
        self.inner().raw()
    }}""")
    
    gen(f"""
    #[inline]
    pub const fn discriminant(&self) -> u8 {{
        unsafe {{ *(self as *const Self as *const u8) }}
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