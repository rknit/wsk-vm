from info import Inst, Module, Modules

IMPL_TOKEN = "$IMPL"

def read_insts() -> Modules:
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

        # finish impl
    gen("}")
    return out



def gen_impl_inst(inst: Inst) -> str:
    return f"""#[derive(Debug, Clone, Copy)]
pub struct {inst.name};
impl {inst.name} {{
    pub fn run(vm: &mut VM, {inst.run_params()}) -> Result<(), VMRunError> {{
        todo!("implement {inst.name} please!");
        Ok(())
    }}
}}"""

def get_impl_token(inst: Inst) -> str:
    return f"// {IMPL_TOKEN} {inst.name}"

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