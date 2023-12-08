import itertools

out = []
for line in open("input.txt").readlines():
    inp = line.split("|")
    out.append([i.split() for i in inp])

numbers = ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]
numbers_map = dict(zip(numbers, range(10)))

sum = 0 
for i in out:
    signals, outputs = i

    mapping = {
        'a': set("abcdefg"),
        'b': set("abcdefg"),
        'c': set("abcdefg"),
        'd': set("abcdefg"),
        'e': set("abcdefg"),
        'f': set("abcdefg"),
        'g': set("abcdefg"),
    }

    for signal in signals:
        if len(signal) == 2:
            for s in signal:
                mapping[s] = mapping[s].intersection("cf")
        if len(signal) == 3:
            for s in signal:
                mapping[s] = mapping[s].intersection("acf")
        if len(signal) == 4:
            for s in signal:
                mapping[s] = mapping[s].intersection("bcdf")

    for perm in itertools.permutations("abcdefg", 7):
        pmap = dict(zip("abcdefg", perm))
        good = True
        for m, p in pmap.items():
            if p not in mapping[m]:
                good = False
                break

        if good:
            # apply the map to the input
            for signal in signals:
                mapped = "".join(sorted([pmap[s] for s in signal]))
                if mapped not in numbers:
                    good = False
                    break

        if good:
            map = pmap
            break

    sum += int("".join([str(numbers_map["".join(sorted([map[s] for s in output]))]) for output in outputs]))

print(sum)