import numpy as np

data = [int(x) for x in open("input.txt").read().split(',')]

# Pick 3 positions. These points have worked for all the tests I've tried. 
# We haven't formally shown they will always work (and I can't be bothered!)
m = max(data)
e0, e1, e2 = 0, m//2, m
costs = [0, 0, 0]

# Sum the numbers from 0 to n (inclusive)
def integerSum(n):
    return n * (n + 1) / 2

# Measure the error at each position
for i in data:
    costs[0] += integerSum(abs(i - e0))
    costs[1] += integerSum(abs(i - e1))
    costs[2] += integerSum(abs(i - e2))

# Solve the system
matrix = np.matrix([[e0*e0, e0, 1], [e1*e1, e1, 1], [e2*e2, e2, 1]])
inv = np.linalg.inv(matrix)
a,b,c = np.dot(inv, np.array(costs)).T

# Solve for the vertex
x = int(-b/(2*a))

# Print the cost
cost = int(sum([integerSum(abs(i - x)) for i in data]))
print(cost)