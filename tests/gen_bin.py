hexdump = """
02400093
00102823
01002103
"""

clean = [c for c in hexdump if not c.isspace()]

assert len(clean) % 2 == 0, "byte count is not even"

bytes = bytearray()
for upper, lower in zip(clean[::2], clean[1::2]):
    bytes.append(int(f"{upper}{lower}", base=16))

with open("./bin", "wb") as file:
    file.write(bytes)
