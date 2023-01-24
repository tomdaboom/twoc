import matplotlib.pyplot as plt
import numpy as np
import sys

ns  = []
ts  = []
dts = []

with open(sys.argv[1], 'r') as file:
    for line in file:
        split = line.split(",")
        ns.append(int(split[0]))
        ts.append(float(split[1]))
        dts.append(float(split[2]))

dts[0] = 0

fig, ax = plt.subplots(1, 2, figsize=(15, 5))
ax[0].plot(ns, ts)
ax[1].plot([ns[i] for i in range(len(ns)) if dts[i] < 5], [i for i in dts if i < 5])
plt.show()