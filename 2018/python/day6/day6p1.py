import numpy

class Point:
    def __init__(self, xy):
        self.x, self.y = xy

    def __sub__(self, other):
        return abs(self.x - other.x) + abs(self.y - other.y)

    def __repr__(self):
        return str((self.x, self.y))

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

ss1 = """
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"""

ss2 = open("input.txt").read()

ss = ss2

coords = [[int(x.strip(",")) for x in l.split()] for l in ss.strip().split("\n")]

maxx = 400

points = [Point(xy) for xy in coords]

print(points)

xmin = min(p.x for p in points)
xmax = max(p.x for p in points)
ymin = min(p.y for p in points)
ymax = max(p.y for p in points)

print(xmin, xmax, ymin, ymax)

#thegrid = numpy.zeros(shape=(xmax-xmin, ymax-ymin))
thegrid = numpy.zeros(shape=(maxx, maxx), dtype='int')
thegrids = numpy.zeros((maxx, maxx), dtype='str')
thegrids.fill(".")

for xi in range(maxx):
    for yi in range(maxx):
        print(xi, yi)
        pi = Point((xi, yi))
        #dists = [p - pi for p in points if not p == pi]
        dists = [p - pi for p in points]
        idists = list(enumerate(dists))
        idistss = sorted(idists, key=lambda x: x[1])
        ibest = idistss[0][0]
        if idistss[0][1] != idistss[1][1]:
            #print("a", pi, ibest, points[ibest], idistss)
            thegrid[xi, yi] = ibest + 1
            thegrids[xi, yi] = chr(ibest + 65).lower()
        else:
            #print(".", pi, ibest, points[ibest], idistss)
            pass


for c, p in enumerate(points):
    thegrids[p.x, p.y] = chr(c + 65)

#print(thegrid)
#print(thegrids)

sg = "\n".join("".join(l) for l in thegrids.T)
#print(sg)

infinites = set(list(thegrid[0,:]) + list(thegrid[-1,:]) + list(thegrid[:,0]) + list(thegrid[:, -1]))

print(infinites)

candidates = [i for i, p in enumerate(points) if i not in infinites]

print(candidates)

candcounts = [(c, list(thegrid.ravel()).count(c)) for c in candidates]

print(candcounts)
largest = sorted(candcounts, key=lambda x: x[1])[-1]
print(largest)

