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
t_model = np.poly1d(np.polyfit(ns, ts, deg))
ts_pred  = t_model(ns)
r_sq = r2_score(ts, ts_pred)
print(f"t: r^2 value = {r_sq}")

# regression on (n, dt)
dt_model = np.poly1d(np.polyfit(ns, dts, deg-1))
dts_pred  = dt_model(ns)
dr_sq = r2_score(dts, dts_pred)
print(f"dt: r^2 value = {dr_sq}")

# PLOTTING

fig, ax = plt.subplots(1, 2, figsize=(15, 5))

plt.rcParams.update({'font.size': 16})

ax[0].plot(ns, ts, 'b', label = f"Benchmark results")
ax[0].plot(ns, ts_pred, 'r--', label = f"Regression (r^2 = {r_sq.round(3)})")
ax[0].set_label("Input length")
ax[0].set_ylabel("Simulation runtime (s)")
ax[0].legend()

ax[1].plot([ns[i] for i in range(len(ns)) if dts[i] < 5], [i for i in dts if i < 5], 'b', label = f"Benchmark derivative")
ax[1].plot(ns, dts_pred, 'r--', label = f"Regression (r^2 = {dr_sq.round(3)})")
ax[1].set_xlabel("Input length")
ax[1].set_ylabel("dt/dn (s)")
ax[1].legend()

plt.show()