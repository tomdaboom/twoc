import matplotlib.pyplot as plt
import numpy as np
import sys
from scipy.stats import linregress

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

linreg_model = linregress(ns, ts)
linear_pred  = [linreg_model.intercept + linreg_model.slope*i for i in ns]
r_sq = linreg_model.rvalue**2
print(f"r^2 value = {r_sq}")

fig, ax = plt.subplots(1, 2, figsize=(15, 5))

plt.rcParams.update({'font.size': 16})

ax[0].plot(ns, ts, 'b', label = f"Benchmark results")
ax[0].plot(ns, linear_pred, 'r--', label = f"Linear regression (r^2 = {r_sq.round(3)})")
ax[0].set_label("Input length")
ax[0].set_ylabel("Simulation runtime (s)")
ax[0].legend()

ax[1].plot([ns[i] for i in range(len(ns)) if dts[i] < 5], [i for i in dts if i < 5])
ax[1].set_xlabel("Input length")
ax[1].set_ylabel("dt/dn (s)")

plt.show()