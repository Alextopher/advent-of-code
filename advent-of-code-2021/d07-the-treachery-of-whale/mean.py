def integerSum(n):
    return n * (n + 1) / 2

data = [int(x) for x in open("input.txt").read().split(',')]
location = 0.5 + sum(data) / len(data)
print(location)
sum([integerSum(abs(i - location)) for i in data])