class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    
def get_match_pat_from_bit_pat(bit_pat: str) -> list[tuple[int, int, str]]:
    bit_pat = bit_pat.strip().replace("_", "")[::-1].upper()
    assert len(bit_pat) == 32, "Bit pattern must be 32 bits long"
    
    start = 0
    end = 0
    ranges: list[tuple[int, int, str]] = list()
    while end < len(bit_pat):
        if bit_pat[end] == "X":
            if end - 1 >= 0:
                ranges.append((end - 1, start, bit_pat[start:end][::-1]))
            while end < len(bit_pat) and bit_pat[end] == "X":
                end += 1
            start = end
        else:
            end += 1
    
    if start < end:
        ranges.append((end - 1, start, bit_pat[start:end][::-1]))
        
    return ranges