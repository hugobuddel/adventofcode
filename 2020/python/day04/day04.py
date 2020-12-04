print(len([
    pp
    for passport in open("input.txt").read().split("\n\n")
    if len((pp := {
        field: value
        for (field, value) in [
            entry.split(":")
            for entry in passport.strip().split()
        ]
    }).keys() - {'cid'}) > 6
]))
