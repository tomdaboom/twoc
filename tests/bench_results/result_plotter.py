#imports
import matplotlib.pyplot as plt
import sys
import numpy as np
from sklearn.metrics import r2_score

#lists to store data
ns  = []
ts  = []
dts = []

#read result file
with open(sys.argv[2], 'r') as file:
    for line in file:
        split = line.split(",")
        ns.append(int(split[0]))
        ts.append(float(split[1]))
        dts.append(float(split[2]))
dts[0] = 0

if sys.argv[1] == 'lin':
    deg = 1
elif sys.argv[1] == 'cub':
    deg = 3
else:
    raise Exception("argv[1] should be lin or cub")

# regression on (n, t)
linreg_model = np.poly1d(np.polyfit(ns, ts, deg))
linear_pred  = [linreg_model(i) for i in ns]
r_sq = r2_score(ts, linear_pred)
print(f"r^2 value = {r_sq}")

# regression on (n, dt)
poly_model = np.poly1d(np.polyfit(ns, dts, deg-1))
poly_pred  = [poly_model(i) for i in ns] 

# PLOTTING

fig, ax = plt.subplots(1, 2, figsize=(15, 5))

plt.rcParams.update({'font.size': 16})

ax[0].plot(ns, ts, 'b', label = f"Benchmark results")
ax[0].plot(ns, linear_pred, 'r--', label = f"Regression (r^2 = {r_sq.round(3)})")
ax[0].set_label("Input length")
ax[0].set_ylabel("Simulation runtime (s)")
ax[0].legend()

ax[1].plot([ns[i] for i in range(len(ns)) if dts[i] < 5], [i for i in dts if i < 5], 'b', label = f"Benchmark derivative")
ax[1].plot(ns, poly_pred, 'r--', label = f"Prediction")
ax[1].set_xlabel("Input length")
ax[1].set_ylabel("dt/dn (s)")
ax[1].legend()

plt.show()