#imports
import matplotlib.pyplot as plt
import sys
import numpy as np
from sklearn.metrics import r2_score

#lists to store data
ns1  = []
ts1  = []
ns2  = []
ts2  = []
ns3  = []
ts3  = []

#read result file
with open(sys.argv[1], 'r') as file:
    for line in file:
        split = line.split(",")
        ns1.append(int(split[0]))
        ts1.append(float(split[1]))

with open(sys.argv[2], 'r') as file:
    for line in file:
        split = line.split(",")
        ns2.append(int(split[0]))
        ts2.append(float(split[1]))

with open(sys.argv[3], 'r') as file:
    for line in file:
        split = line.split(",")
        ns3.append(int(split[0]))
        ts3.append(float(split[1]))

# PLOTTING


plt.rcParams.update({'font.size': 16})

plt.plot(ns1, ts1, 'b', label = f"Hashmap Implementation")
plt.plot(ns2, ts2, 'r', label = f"Array Implementation")
plt.plot(ns3, ts3, 'g', label = f"Gluek's nondeterministic algorithm")
plt.xlabel("Input length")
plt.ylabel("Simulation runtime (s)")
plt.legend()
plt.grid()


plt.show()