import itertools

mypoly1 = "dabAcCaCBAcCcaDA"
mypoly2 = open("input.txt").read().strip()
mypoly = mypoly2

alphabet = [chr(i) for i in range(65, 65+26)]
destroyers = [a.lower() + a for a in alphabet] + [a + a.lower() for a in alphabet]

print(destroyers)
polyold = mypoly + " "
while len(mypoly) < len(polyold):
    polyold = mypoly
    for destroy in destroyers:
        mypoly = mypoly.replace(destroy, "")

print(len(mypoly), mypoly)
