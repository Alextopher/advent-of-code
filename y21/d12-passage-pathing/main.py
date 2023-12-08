inputs = [line[:-1].split("-") for line in open("input.txt")]

graph = {}
for s, t in inputs:
    if s not in graph:
        graph[s] = []
    if t not in graph:
        graph[t] = []

    graph[s].append(t)
    graph[t].append(s)

def part1(visted, node):
    if node == "end":
        return 1

    visted.append(node)

    s = sum([part1(visted, n) for n in graph[node] if not n.islower() or n not in visted])

    visted.pop()
    return s

def part2(visted, twiced, node):
    if node == "end":
        return 1

    if node == "start" and visted != []:
        return 0

    visted.append(node)

    s = 0
    for n in graph[node]:
        if n.islower():
            if visted.count(n) == 0:
                s += part2(visted, twiced, n)
            elif visted.count(n) == 1 and not twiced:
                s += part2(visted, True, n)
            elif visted.count(n) == 2 and not twiced:
                s += part2(visted, True, n)
        else:
            s += part2(visted, twiced, n)
    
    visted.pop()
    return s

print(part1([], "start"))
print(part2([], False, "start"))