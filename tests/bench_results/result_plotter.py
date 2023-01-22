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
ax[0].plot(ns[0:180], ts[0:180])
ax[1].plot(ns[0:180], dts[0:180])
plt.show()