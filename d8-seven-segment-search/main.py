out = []
for line in open("input.txt").readlines():
    inp = line.split("|")
    out.append([i.split() for i in inp])

count = 0 
for i in out:
    signals, outputs = i
    # for signal in signals:
    #     if len(signal) == 2:
    #         print(signal)
    #         count += 1
    #     elif len(signal) == 3:
    #         print(signal)
    #         count += 1
    #     elif len(signal) == 4:
    #         print(signal)
    #         count += 1
    #     elif len(signal) == 7:
    #         print(signal)
    #         count += 1

    for output in outputs:
        if len(output) == 2:
            print(output)
            count += 1
        elif len(output) == 3:
            print(output)
            count += 1
        elif len(output) == 4:
            print(output)
            count += 1
        elif len(output) == 7:
            print(output)
            count += 1

print(count)