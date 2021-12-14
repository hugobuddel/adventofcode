"""--- Part Two ---

After reviewing the available paths, you realize you might have time to visit a
single small cave twice. Specifically, big caves can be visited any number of
times, a single small cave can be visited at most twice, and the remaining
small caves can be visited at most once. However, the caves named start and end
can only be visited exactly once each: once you leave the start cave, you may
not return to it, and once you reach the end cave, the path must end
immediately.

Now, the 36 possible paths through the first example above are:

start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end

The slightly larger example above now has 103 paths through it, and the even
larger example now has 3509 paths through it.

Given these new rules, how many paths through this cave system are there?
"""

from day12a import find_paths, cave
from copy import deepcopy

def modify_cave(graph):
    smalls = [
        c for c in graph
        if c.lower() == c
           and c not in ['start', 'end']
    ]
    print(smalls)
    for small in smalls:
        graphdouble = deepcopy(graph)
        graphdouble[small + "2"] = graphdouble[small]
        for k, v in graphdouble.items():
            if small in v:
                v.add(small + "2")
        yield graphdouble


paths = set()
for graphdouble in modify_cave(cave):
    paths = paths | set(tuple(
        c.replace("2", "") for c in p
    ) for p in find_paths(graphdouble, 'start', {'start'}))

print(len(paths))
# for p in paths:
#     print(p)
