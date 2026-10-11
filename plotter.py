import traceback

import matplotlib.pyplot as plt

a = open("output.txt", encoding="utf-8").read().replace("\r", "")
a = a.split("\n\n")
a = [i.split("\n") for i in a]

time = []
data = {}

try:
    for b in a[:-1]:
        time.append(float(b[0]))
        for c in b[1:]:
            unknown, value = c.replace(" ", "").split("=")
            if unknown not in data:
                data[unknown] = []
            data[unknown].append(float(value))
except:
    traceback.print_exc()

for unknown, values in data.items():
    plt.plot(time, values, label=unknown)

plt.legend()
plt.show()
