import logicmin

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

t = logicmin.TT(8, 3 * 7)

for i in range(0, 256):
    input = f'{i:0>8b}'
    hundreds = DIGITS[i // 100]
    tens = DIGITS[(i // 10) % 10]
    ones = DIGITS[i % 10]
    t.add(input, f'{hundreds}{tens}{ones}')

sols = t.solve()
text = sols.printN(
    xnames=['b16', 'b8','b4','b2','b1'],
    ynames=[
        'a3', 'b3', 'c3', 'd3', 'e3', 'f3', 'g3',
        'a2', 'b2', 'c2', 'd2', 'e2', 'f2', 'g2',
        'a1', 'b1', 'c1', 'd1', 'e1', 'f1', 'g1',
    ]
)

print(text)
