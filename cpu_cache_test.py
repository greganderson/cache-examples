import os
import time


def construct(size):
    lst = []
    for i in range(size):
        row = []
        for j in range(size):
            row.append(j)
        lst.append(row)
    return lst

def iterate1(lst):
    for row in range(len(lst)):
        for col in range(len(lst[row])):
            temp = lst[row][col]

def iterate2(lst):
    for row in range(len(lst)):
        for col in range(len(lst[row])):
            temp = lst[col][row]

def main():
    # Example 2D list
    #lst = [
    #    ["a1", "a2", "a3"],
    #    ["b1", "b2", "b3"],
    #    ["c1", "c2", "c3"]
    #]

    os.system("clear")
    print()
    lst = construct(20000)

    print("=======Iterators===========")
    start = time.time()
    iterate1(lst)
    end = time.time()
    elapsed = end - start
    print(f"{elapsed} s")

    start = time.time()
    iterate2(lst)
    end = time.time()
    elapsed = end - start
    print(f"{elapsed} s")
    print("=======Iterators===========")

if __name__ == "__main__":
    main()
