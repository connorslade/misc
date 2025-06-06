import math

import logicmin

OUTPUT_DIGITS = 1
DIGITS = [
    "1111110",
    "0110000",
    "1101101",
    "1111001",
    "0110011",
    "1011011",
    "0011111",
    "1110000",
    "1111111",
    "1110011"
]

max_number = 10 ** OUTPUT_DIGITS
input_bits = math.ceil(math.log2(max_number))

t = logicmin.TT(input_bits, OUTPUT_DIGITS * 7)

for i in range(0, 2 ** input_bits):
    input = f'{i:0>{input_bits}b}'

    if i > max_number:
        t.add(input, "0000001" * OUTPUT_DIGITS)
        continue

    output = ""
    for _ in range(OUTPUT_DIGITS):
        output += DIGITS[i % 10]
        i //= 10

    t.add(input, output)

sols = t.solve()
text = sols.printN(
    xnames=[f'b{2 ** i}' for i in reversed(range(input_bits))],
    ynames=[f'{x}{i}' for x in "abcdefg" for i in range(1, OUTPUT_DIGITS + 1)]
)

print(text)
