i = eval(open('input5').read(), {'I':1, 'II':2, 'III':3, 'IV':4, 'V':5, 'VI':6, 'VII':7, 'VIII':8, 'IX':9, 'X':10, 'XI':11, 'XII':12, 'XIII':13})
print(''.join(chr(a + i.pop() + 96) for a in i)) 
