depth = 0
pos = 0
aim = 0

with open("input.txt") as f:
    for line in f.readlines():
        s = line.split(" ")

        if s[0] == "forward":
            pos += int(s[1])
            depth += int(s[1]) * aim
        if s[0] == "down":
            aim += int(s[1])
        if s[0] == "up":
            aim -= int(s[1])
    
    print (depth * pos)