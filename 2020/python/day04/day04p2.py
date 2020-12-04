def validate(passport):
    return all([
        1920 <= int(passport['byr']) <= 2002,
        2010 <= int(passport['iyr']) <= 2020,
        2020 <= int(passport['eyr']) <= 2030,
        passport['hgt'][-2:] in ('cm', 'in') and \
        (
            (150 <= int(passport['hgt'][:-2]) <= 193)
            if (passport['hgt'][-2:] == 'cm')
            else (59 <= int(passport['hgt'][:-2]) <= 76)
        ),
        len(passport['hcl']) == 7 and \
            passport['hcl'][0] == '#' and \
            all(c in '0123456789abcdef' for c in passport['hcl'][1:]),
        passport['ecl'] in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'},
        len(passport['pid']) == 9 and \
            all(c in '0123456789' for c in passport['pid']),
    ])

print(len([
    pp
    # for passport in open("inputexamplep2.txt").read().split("\n\n")
    for passport in open("input.txt").read().split("\n\n")
    if len((pp := {
        field: value
        for (field, value) in [
            entry.split(":")
            for entry in passport.strip().split()
        ]
    }).keys() - {'cid'}) > 6 and validate(pp)
]))
