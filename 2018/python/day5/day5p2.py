import itertools

def destroyfully(poly):
    polyold = poly + " "
    while len(poly) < len(polyold):
        polyold = poly
        for destroy in destroyers:
            poly = poly.replace(destroy, "")
    return poly

mypoly1 = "dabAcCaCBAcCcaDA"
mypoly2 = open("input.txt").read().strip()
mypoly = mypoly2

alphabet = [chr(i) for i in range(65, 65+26)]
destroyers = [a.lower() + a for a in alphabet] + [a + a.lower() for a in alphabet]

print(destroyers)

polys = destroyfully(mypoly)
print(len(polys), polys)

mypolysclean = [mypoly.replace(l, "").replace(l.lower(), "") for l in alphabet]
mypolyscleanshort = [destroyfully(p) for p in mypolysclean]
letter_shortest, poly_shortest = sorted(zip(alphabet, mypolyscleanshort), key=lambda x: len(x[1]))[0]
print(letter_shortest, len(poly_shortest), poly_shortest)

