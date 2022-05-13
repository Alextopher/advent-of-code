# # Import statistics Library
# import statistics


# file = "input.txt"
# #file = "example.txt"

# with open(file) as f:
#     # 415,863 -> 942,863
#     i = [ int(a) for a in f.readline().split(",")]
#     m = statistics.median(i)

#     sum = 0
#     for j in i:
#         sum += abs(m - j)
    
#     print(sum)

# Import statistics Library
import statistics


file = "input.txt"
#file = "example.txt"

with open(file) as f:
    # 415,863 -> 942,863
    ins = [ int(a) for a in f.readline().split(",")]
    m = max(ins)
    costs = []

    sums = 0
    for p in range(m):
        costs.append(0)
        for i in ins:
           costs[p] += sum(range(1, abs(i - p) + 1))

    for c in costs:
        print(c)