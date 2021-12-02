depth = 0
pos = 0

with open("input.txt") as f:
    for line in f.readlines():
        s = line.split(" ")

        if s[0] == "forward":
            pos += int(s[1])
        if s[0] == "down":
            depth += int(s[1])
        if s[0] == "up":
            depth -= int(s[1])
    
    print (depth * pos)