from info import Inst, Module, Modules
import toml

IMPL_TOKEN = "$IMPL"

def read_insts(path: str) -> Modules:
    modules = Modules()

    with open(path, "r") as file:
        data = toml.load(file)
        
        for mod_name, format in data.items():
            module = Module(mod_name)
            for format, inst in format.items():
                for inst_name, inst_data in inst.items():
                    inst = Inst(
                        format, 
                        inst_data["opc"], 
                        inst_data.get("f3", "-"), 
                        inst_data.get("f7", "-"),
                        inst_name,
                    )
                    
                    # other configs
                    inst.special_match = inst_data.get("special_match", "")
                    inst.imm = inst_data.get("imm", "")
                    
                    module.append(inst)
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
        }})
    }}""")

    # run function
    gen(f"""
    pub fn run(self, vm: &mut VM) -> Result<(), VMRunError> {{
        match self {{
            {"            ".join([inst.run_arm() for inst in modules.all_inst()])}
            #[allow(unreachable_patterns)]
            _ => return Err(VMRunError {{
                err_addr: vm.pc,
                kind: VMRunErrorKind::Other(format!("{{:?}}", self)),
                info: "unimplemented inst",
            }}),
        }}
    }}""")
    
    # name function
    gen(f"""
    pub fn name(self) -> &'static str {{
        match self {{
            {"            ".join([inst.name_arm() for inst in modules.all_inst()])}
            #[allow(unreachable_patterns)]
            _ => "unknown",
        }}
    }}""")

    # finish impl
    gen("}")
    return out



def gen_impl_inst(inst: Inst) -> str:
    return f"""#[derive(Debug, Clone, Copy)]
pub struct {inst.symbol};
impl {inst.symbol} {{
    pub fn run(vm: &mut VM, {inst.run_params()}) -> Result<(), VMRunError> {{
        todo!("implement {inst.symbol} please!");
        Ok(())
    }}
}}"""

def get_impl_token(inst: Inst) -> str:
    return f"// {IMPL_TOKEN} {inst.symbol}"

def get_impl_token_full(inst: Inst) -> str:
    return f"{get_impl_token(inst)} {inst.format}"

def gen_inst(inst: Inst) -> str:
    return f"""
{get_impl_token_full(inst)}
{gen_impl_inst(inst)}
"""

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