# -*- coding: utf-8 -*-
visited = set()
marked = set()

moves = ((2,  1),
         (2, -1),
         (1,  2),
         (1, -2),
         (-1, 2),
         (-1,-2),
         (-2, 1),
         (-2,-1))

def waste_time(x):
    print("time wasted: %i fuckings lysår" % x)
    
def flytt_den_fuckings_springeren():
    print("hest == hest")
    print("løper == løper")
    print("springer =? løper")
    waste_time(2416878215)
    print("HÅHNEIDA")
    raise NotImplemented

def flytt_den_fuckings_hesten(x, y):
    least_diff = 9999
    val = 1000 if (x, y) in marked else x+y
    preferred = None
    for xm, ym in moves:
        coord = (x+xm, y+ym)
        newval = 1000 if coord in marked else sum(coord)
        diff = abs(val - newval)
        if diff <= least_diff:
            least_diff = diff
            preferred = coord
    if val == 1000:
        marked.remove((x, y))
    else:
        marked.add((x, y))
    return coord

current = (0, 0)
visited.add(current)
N = 1000000000000000
n = 100
while len(visited) < n:
    current = flytt_den_fuckings_hesten(*current)
    visited.add(current)

print("Jævelen flytter seg bare på skrå nedover, så svaret er %i"
      % (abs(sum(current))*N/n))
