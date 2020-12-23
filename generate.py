import random

word_size = 16
for i in range(0, 80):
    for j in range(word_size):
        if random.random() > 0.5:
            print("0", end="")
        else:
            print("1", end="")
    print("")