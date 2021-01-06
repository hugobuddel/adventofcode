
import numpy

def convertclaim(thes):
    aid, aat, ase, awh = thes.strip().split()
    ax, ay = [int(c) for c in ase.strip(":").split(",")]
    aw, ah = [int(c) for c in awh.strip(":").split("x")]
    return aid, ax, ay, aw, ah

ss = """
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"""
myclaims1 = ss.strip().split("\n")

myclaims2 = open("input.txt").readlines()

myclaims = myclaims2
print(myclaims)

for myclaim in myclaims:
    myid, myx, myy, myw, myh = convertclaim(myclaim)
    print(myx, myy, myw, myh )

xywhs = [convertclaim(myclaim) for myclaim in myclaims]

thegrid = numpy.zeros(shape=(1000, 1000), dtype=numpy.int)

for myid, myx, myy, myw, myh in xywhs:
    thegrid[myx:myx+myw, myy:myy+myh] += 1

#print(thegrid)

clashes = thegrid > 1

#print(clashes)

print(clashes.sum())

for myid, myx, myy, myw, myh in xywhs:
    if (thegrid[myx:myx+myw, myy:myy+myh] == 1).all():
        print(myid)
