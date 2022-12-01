# test data
test_data = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""

# split test data
test_cals = test_data.split("\n")

# import data
with open("./input.txt", "r") as d:
    cals = []
    for cal in d.readlines():
        cals.append(cal.replace("\n", ""))


# find the most calories
def most_calories(cals: list[str]) -> int:
    sum = 0
    res1 = 0
    for item in cals:
        if item == "":
            if res1 < sum:
                res1 = sum
            sum = 0
            continue

        sum += int(item)
    return res1



# find the most calories
def most_calories_3(cals: list[str]) -> int:
    sum = 0
    res1 = 0
    res2 = 0
    res3 = 0
    for item in cals:
        if item == "":
            if res1 <= sum:
                res3 = res2
                res2 = res1
                res1 = sum
                sum = 0
                continue
            if res2 <= sum:
                res3 = res2
                res2 = sum
                sum = 0
                continue
            if res3 <= sum:
                res3 = sum
                sum = 0
                continue
            sum = 0
            continue

        sum += int(item)
    print(res1, res2, res3)
    return res1 + res2 + res3


if __name__ == "__main__":
    # cals = test_cals
    print(most_calories_3(cals))
