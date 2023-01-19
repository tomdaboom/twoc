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

plt.plot(ns, ts)
plt.xlabel('n')
plt.ylabel('t')
plt.xticks(np.arange(min(ns), max(ns), (max(ns)-min(ns))/5))
plt.yticks(np.arange(min(ts), max(ts), (max(ts)-min(ts))/5))
plt.show()

plt.plot(ns, dts)
plt.xlabel('n')
plt.ylabel('dt')
plt.xticks(np.arange(min(ns), max(ns), (max(ns)-min(ns))/5))
plt.yticks(np.arange(min(dts), max(dts), (max(dts)-min(dts))/5))
plt.show()