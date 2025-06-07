from typing import Any, Optional

from info import Inst

import json

class TriNode:
    def __init__(self):
        self.children: dict[str, Optional[TriNode]] = {
            '0': None,
            '1': None,
            'X': None,
        }
        self.inst: Optional[Inst] = None
        
class TriTree:
    def __init__(self):
        self.root: TriNode = TriNode()
            
    def insert(self, inst: Inst):
        TriTree._insert(self.root, inst, len(inst.bit_pat))
        
    @staticmethod
    def _insert(node: TriNode, inst: Inst, depth: int):
        if depth == 0:
            assert node.inst is None, f"Node already has an instruction: {node.inst}, when trying to insert {inst}"
            node.inst = inst
            return
        
        bit = inst.bit_pat[depth - 1]
        assert bit in node.children, f"Invalid bit in value '{bit}'"
        
        next_node = node.children[bit]
        if next_node is None:
            next_node = TriNode()
            node.children[bit] = next_node
            
        TriTree._insert(next_node, inst, depth - 1)
        
    def __str__(self) -> str:
        return TriTree._str(self.root, 0)
    
    @staticmethod
    def _str(node: TriNode, depth: int) -> str:
        mt = TriTree.match_tree(node, depth)
        return json.dumps(mt, indent=2, ensure_ascii=True)
    
    @staticmethod
    def match_tree(node: TriNode, depth: int) -> dict[str, Any]:
        tree_dict: dict[str, Any] = dict()
        
        if node.inst is not None:
            tree_dict["inst"] = f"{node.inst}"
            return tree_dict
        
        for bit, child in node.children.items():
            if child is not None:
                if child.inst is None:
                    tree_dict[bit] = TriTree.match_tree(child, depth + 1)
                else:
                    tree_dict[bit] = f"{child.inst}"
                    
        return tree_dict
    
    def gen_match(self) -> str:
        return TriTree._gen_match(self.root, 0)
    
    @staticmethod
    def _gen_match(node: TriNode, depth: int) -> str:
        if node.inst is not None:
            return f"Inst::{node.inst.symbol}(RawInst::new(inst))"
        
        arms: dict[str, str] = dict()
        for bit, child in node.children.items():
            if child is not None:
                inner = TriTree._gen_match(child, depth + 1)
                arms[bit] = inner
                
        match_str = f"match (inst >> {depth}) & 1u32 {{"
        
        if '0' in arms:
            match_str += f"\n{'  ' * (depth + 1)}0b0 => {arms['0']},"
            
        if '1' in arms:
            match_str += f"\n{'  ' * (depth + 1)}0b1 => {arms['1']},"
            
        if 'X' in arms:
            match_str += f"\n{'  ' * (depth + 1)}_ => {arms['X']},"
        else:
            match_str += f"\n{'  ' * (depth + 1)}_ => return None,"
            
        match_str += f"\n{'  ' * depth}}}"
                    
        return match_str
    
    @staticmethod
    def _str_list(node: TriNode, depth: int) -> list[str]:
        if node.inst is not None:
            return [f" -> {node.inst}"]
        
        list_str: list[str] = []
        for bit, child in node.children.items():
            if child is not None:
                child_str = TriTree._str(child, depth + 1)
                for child_str in child_str:
                    list_str.append(bit + child_str)
                    
        return list_str