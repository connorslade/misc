import logicmin

t = logicmin.TT(4, 7)

t.add("0000","1111110") # 0
t.add("0001","0110000") # 1
t.add("0010","1101101") # 2
t.add("0011","1111001") # 3
t.add("0100","0110011") # 4
t.add("0101","1011011") # 5
t.add("0110","1011111") # 6
t.add("0111","1110000") # 7
t.add("1000","1111111") # 8
t.add("1001","1110011") # 9

sols = t.solve()
text = sols.printN(
    xnames=['b8', 'b4', 'b2', 'b1'],
    ynames=['a', 'b', 'c', 'd', 'e', 'f', 'g']
)

dot = text.count('.')
plus = text.count('+')

print(text)

print(f'Relays ≈ {min(dot, plus + 1)}')
