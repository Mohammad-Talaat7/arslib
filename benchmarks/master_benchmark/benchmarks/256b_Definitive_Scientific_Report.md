# ARS Evolution Atlas: Final Research Study

## 1. Experimental Setup
- **Cores:** 8 | **RAM:** 15864 MB
- **PMC Instrumentation:** true (Multi-thread Inherit: Enabled)
- **Statistical Setup:** Reps=7, Seed=42

## Category: i64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 14.147µs | 10227 | 0 | 0.00% | 2.58 | 1078.59 MB/s |
| Timsort | 1000 | 23.453µs | 10588 | 0 | 4.94% | 2.05 | 650.61 MB/s |
| ARS Gen 1: Foundation | 1000 | 289.772µs | 0 | 2000 | 1.37% | 2.09 | 52.66 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 336.964µs | 0 | 2000 | 0.25% | 2.08 | 45.28 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 14.156µs | 10227 | 0 | 0.00% | 2.57 | 1077.90 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 13.859µs | 10227 | 0 | 0.00% | 2.63 | 1101.00 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 14.071µs | 10227 | 0 | 0.00% | 2.59 | 1084.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 23.533µs | 10588 | 0 | 0.00% | 2.04 | 648.40 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 14.525µs | 10227 | 0 | 7.69% | 2.51 | 1050.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 20.367µs | 10588 | 0 | 0.00% | 2.05 | 749.19 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 12.15µs | 10227 | 0 | 0.00% | 2.61 | 1255.87 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 12.136µs | 10227 | 0 | 0.00% | 2.60 | 1257.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.884µs | 10227 | 0 | 0.00% | 2.65 | 1283.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 118.174µs | 10227 | 2000 | 0.73% | 0.66 | 129.12 MB/s |
| Quicksort | 10000 | 123.622µs | 136654 | 0 | 0.97% | 1.02 | 1234.31 MB/s |
| Timsort | 10000 | 171.39µs | 140327 | 0 | 0.24% | 1.06 | 890.30 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.551016ms | 0 | 30000 | 0.24% | 1.72 | 33.53 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.759742ms | 0 | 30000 | 0.19% | 1.70 | 32.06 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 265.777µs | 193611 | 14351 | 0.28% | 1.08 | 574.12 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 191.487µs | 51695 | 10000 | 0.14% | 0.83 | 796.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 124.351µs | 51695 | 0 | 0.16% | 0.74 | 1227.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 132.719µs | 57359 | 0 | 0.13% | 0.77 | 1149.71 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 144.448µs | 59671 | 0 | 0.22% | 0.79 | 1056.35 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 125.917µs | 62214 | 0 | 0.23% | 0.79 | 1211.81 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 132.928µs | 51695 | 0 | 0.17% | 0.76 | 1147.90 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 125.305µs | 51695 | 0 | 0.11% | 0.75 | 1217.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 121.959µs | 51695 | 0 | 0.13% | 0.79 | 1251.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 325.172µs | 51695 | 20000 | 0.24% | 0.74 | 469.25 MB/s |
| Quicksort | 100000 | 1.4206ms | 1709595 | 0 | 3.62% | 1.68 | 1074.11 MB/s |
| Timsort | 100000 | 2.040843ms | 1743505 | 0 | 3.22% | 1.62 | 747.67 MB/s |
| ARS Gen 1: Foundation | 100000 | 38.387879ms | 0 | 300000 | 2.20% | 1.00 | 39.75 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 39.709919ms | 0 | 300000 | 2.88% | 1.01 | 38.43 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.049933ms | 1885062 | 108703 | 13.12% | 1.30 | 744.36 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 872.841µs | 881353 | 100000 | 16.43% | 0.98 | 1748.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 757.607µs | 881353 | 0 | 15.98% | 0.92 | 2014.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 625.147µs | 921838 | 0 | 6.24% | 1.06 | 2440.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 724.291µs | 955554 | 0 | 11.82% | 1.07 | 2106.72 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 843.165µs | 991979 | 0 | 9.66% | 1.03 | 1809.70 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 694.839µs | 881353 | 0 | 11.39% | 0.92 | 2196.02 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 810.269µs | 772388 | 0 | 15.27% | 0.97 | 1883.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 651.901µs | 881353 | 0 | 8.62% | 0.92 | 2340.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.051936ms | 881353 | 200000 | 17.66% | 0.95 | 1450.54 MB/s |
| Quicksort | 1000000 | 17.617997ms | 20423287 | 0 | 20.01% | 1.96 | 866.09 MB/s |
| Timsort | 1000000 | 29.746972ms | 20813246 | 0 | 23.62% | 1.68 | 512.95 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 22.141819ms | 21493355 | 1017407 | 30.57% | 1.40 | 689.14 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.578097ms | 10218658 | 1000000 | 48.37% | 0.91 | 1593.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.73758ms | 10218658 | 0 | 48.87% | 0.87 | 1746.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.409319ms | 10628212 | 0 | 47.72% | 0.89 | 1621.67 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.271377ms | 13023009 | 0 | 44.16% | 1.27 | 2098.47 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.251234ms | 13432511 | 0 | 43.75% | 1.12 | 1849.27 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.205559ms | 10218658 | 0 | 45.04% | 0.91 | 1657.56 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.189651ms | 11276404 | 0 | 49.78% | 1.03 | 1497.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.20268ms | 12320223 | 0 | 53.13% | 0.96 | 1495.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 22.267227ms | 12171637 | 2000000 | 50.48% | 0.84 | 685.26 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 12.227µs | 10330 | 0 | 44.60% | 0.89 | 1247.96 MB/s |
| Timsort | 1000 | 18.929µs | 10648 | 0 | 44.60% | 0.89 | 806.11 MB/s |
| ARS Gen 1: Foundation | 1000 | 196.738µs | 503 | 2000 | 44.58% | 0.90 | 77.56 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 217.573µs | 503 | 2000 | 44.58% | 0.90 | 70.13 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 12.383µs | 10330 | 0 | 44.60% | 0.89 | 1232.24 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.999µs | 10330 | 0 | 44.60% | 0.89 | 1271.67 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.993µs | 10330 | 0 | 44.60% | 0.89 | 1272.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 19.1µs | 10648 | 0 | 44.60% | 0.89 | 798.89 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 12.269µs | 10330 | 0 | 44.60% | 0.89 | 1243.69 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 19.281µs | 10648 | 0 | 44.60% | 0.89 | 791.39 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.913µs | 10330 | 0 | 44.60% | 0.89 | 1280.85 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.905µs | 10330 | 0 | 44.60% | 0.89 | 1281.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.775µs | 10330 | 0 | 44.60% | 0.89 | 1295.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 121.853µs | 10330 | 2000 | 44.48% | 0.89 | 125.22 MB/s |
| Quicksort | 10000 | 145.156µs | 134638 | 0 | 44.10% | 0.89 | 1051.20 MB/s |
| Timsort | 10000 | 221.009µs | 140096 | 0 | 44.08% | 0.89 | 690.41 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.283778ms | 57643 | 30000 | 43.44% | 0.90 | 118.86 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.41039ms | 57632 | 30000 | 43.40% | 0.90 | 108.19 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 357.978µs | 191358 | 14351 | 43.79% | 0.89 | 426.25 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 222.347µs | 61389 | 10000 | 43.66% | 0.89 | 686.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 149.15µs | 61389 | 0 | 43.54% | 0.89 | 1023.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 158.359µs | 64672 | 0 | 43.55% | 0.89 | 963.56 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 262.285µs | 58684 | 0 | 43.64% | 0.89 | 581.76 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 266.553µs | 61459 | 0 | 43.60% | 0.89 | 572.45 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 146.006µs | 61389 | 0 | 43.55% | 0.89 | 1045.08 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 153.664µs | 61389 | 0 | 43.54% | 0.89 | 993.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 158.295µs | 61389 | 0 | 43.54% | 0.89 | 963.95 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 338.494µs | 61389 | 20000 | 43.34% | 0.89 | 450.78 MB/s |
| Quicksort | 100000 | 1.356319ms | 1446704 | 0 | 42.07% | 0.92 | 1125.01 MB/s |
| Timsort | 100000 | 1.67594ms | 1445193 | 0 | 40.88% | 0.92 | 910.46 MB/s |
| ARS Gen 1: Foundation | 100000 | 8.002504ms | 1387258 | 300000 | 39.03% | 0.92 | 190.68 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.909475ms | 1386968 | 300000 | 39.44% | 0.93 | 171.26 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.324345ms | 1645061 | 108703 | 41.65% | 0.92 | 656.48 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 888.555µs | 734392 | 100000 | 41.89% | 0.90 | 1717.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 821.943µs | 734392 | 0 | 41.98% | 0.90 | 1856.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 779.083µs | 735546 | 0 | 41.87% | 0.90 | 1958.56 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 768.646µs | 701732 | 0 | 41.81% | 0.90 | 1985.15 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 855.111µs | 707058 | 0 | 41.67% | 0.90 | 1784.42 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 711.776µs | 734392 | 0 | 41.51% | 0.90 | 2143.76 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 701.862µs | 629097 | 0 | 41.90% | 0.90 | 2174.04 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 737.275µs | 734392 | 0 | 41.48% | 0.90 | 2069.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.111758ms | 734392 | 200000 | 41.34% | 0.90 | 1372.49 MB/s |
| Quicksort | 1000000 | 9.900167ms | 13567694 | 0 | 35.40% | 1.13 | 1541.27 MB/s |
| Timsort | 1000000 | 16.173019ms | 14681691 | 0 | 31.41% | 1.10 | 943.47 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.979789ms | 14956001 | 1017407 | 39.90% | 1.06 | 727.31 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.253705ms | 4787996 | 1000000 | 48.60% | 0.89 | 2103.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.606163ms | 4787996 | 0 | 48.39% | 0.88 | 2309.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.205441ms | 4821847 | 0 | 48.38% | 0.89 | 2117.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.531849ms | 6224935 | 0 | 45.52% | 0.97 | 2758.35 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.079184ms | 6254216 | 0 | 44.41% | 0.99 | 2510.01 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.861601ms | 4757456 | 0 | 47.37% | 0.88 | 1940.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.393948ms | 2295151 | 0 | 45.73% | 0.90 | 1624.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.187857ms | 2529783 | 0 | 45.45% | 0.88 | 1660.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.012988ms | 11655372 | 2000000 | 43.58% | 1.01 | 802.55 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.553µs | 9762 | 0 | 47.43% | 1.18 | 1320.76 MB/s |
| Timsort | 1000 | 16.043µs | 9882 | 0 | 47.43% | 1.18 | 951.12 MB/s |
| ARS Gen 1: Foundation | 1000 | 95.953µs | 9788 | 2000 | 47.43% | 1.18 | 159.02 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 101.03µs | 9815 | 2000 | 47.43% | 1.18 | 151.03 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.72µs | 9762 | 0 | 47.43% | 1.18 | 1301.94 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.625µs | 9762 | 0 | 47.43% | 1.18 | 1312.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.352µs | 9762 | 0 | 47.43% | 1.18 | 1344.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 15.934µs | 9882 | 0 | 47.43% | 1.18 | 957.62 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.843µs | 9762 | 0 | 47.43% | 1.18 | 1288.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 16.279µs | 9882 | 0 | 47.43% | 1.18 | 937.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.68µs | 9762 | 0 | 47.43% | 1.18 | 1306.40 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.467µs | 9762 | 0 | 47.43% | 1.18 | 1330.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.529µs | 9762 | 0 | 47.43% | 1.18 | 1323.51 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 122.343µs | 9762 | 2000 | 47.40% | 1.18 | 124.72 MB/s |
| Quicksort | 10000 | 147.723µs | 134689 | 0 | 47.29% | 1.18 | 1032.93 MB/s |
| Timsort | 10000 | 189.929µs | 132195 | 0 | 47.28% | 1.18 | 803.39 MB/s |
| ARS Gen 1: Foundation | 10000 | 875.819µs | 130386 | 30000 | 47.15% | 1.18 | 174.22 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 904.716µs | 130325 | 30000 | 47.15% | 1.18 | 168.66 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 332.593µs | 187157 | 14351 | 47.21% | 1.18 | 458.78 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 232.817µs | 45304 | 10000 | 47.16% | 1.18 | 655.40 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 144.487µs | 45304 | 0 | 47.15% | 1.18 | 1056.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 149.147µs | 36417 | 0 | 47.15% | 1.18 | 1023.07 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 263.032µs | 52155 | 0 | 47.16% | 1.18 | 580.11 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 262.5µs | 46959 | 0 | 47.17% | 1.18 | 581.29 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 147.852µs | 45304 | 0 | 47.16% | 1.18 | 1032.03 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 141.7µs | 45304 | 0 | 47.15% | 1.18 | 1076.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 146.655µs | 45304 | 0 | 47.16% | 1.18 | 1040.45 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 329.349µs | 45304 | 20000 | 47.14% | 1.18 | 463.30 MB/s |
| Quicksort | 100000 | 1.670898ms | 1716043 | 0 | 46.80% | 1.19 | 913.21 MB/s |
| Timsort | 100000 | 2.046185ms | 1660908 | 0 | 46.44% | 1.19 | 745.72 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.614111ms | 1643878 | 300000 | 45.92% | 1.20 | 200.40 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.195033ms | 1643640 | 300000 | 45.96% | 1.20 | 186.20 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.216871ms | 1830188 | 108703 | 46.72% | 1.18 | 688.30 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.051723ms | 827444 | 100000 | 46.63% | 1.18 | 1450.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 837.046µs | 827444 | 0 | 46.68% | 1.17 | 1822.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 687.935µs | 410171 | 0 | 46.65% | 1.17 | 2218.06 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 707.647µs | 906650 | 0 | 46.65% | 1.18 | 2156.27 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 626.228µs | 447837 | 0 | 46.64% | 1.18 | 2436.62 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 885.93µs | 827444 | 0 | 46.57% | 1.17 | 1722.35 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 765.713µs | 718138 | 0 | 46.67% | 1.18 | 1992.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 776.811µs | 827444 | 0 | 46.55% | 1.17 | 1964.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.211648ms | 827444 | 200000 | 46.52% | 1.17 | 1259.34 MB/s |
| Quicksort | 1000000 | 17.683001ms | 20672771 | 0 | 45.15% | 1.30 | 862.91 MB/s |
| Timsort | 1000000 | 26.67635ms | 19775927 | 0 | 42.35% | 1.28 | 572.00 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.817051ms | 20984698 | 1017407 | 47.10% | 1.23 | 699.40 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.952519ms | 9742173 | 1000000 | 48.26% | 1.13 | 1533.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.136054ms | 9742173 | 0 | 48.37% | 1.12 | 1670.17 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.267064ms | 4127840 | 0 | 48.15% | 1.12 | 2099.72 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.021238ms | 12614329 | 0 | 47.40% | 1.19 | 1902.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.187691ms | 5756709 | 0 | 47.19% | 1.18 | 2465.99 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.269802ms | 9742173 | 0 | 47.96% | 1.13 | 1646.07 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.020579ms | 10843448 | 0 | 48.19% | 1.16 | 1522.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.34326ms | 11954018 | 0 | 48.31% | 1.13 | 1475.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 21.966572ms | 15179419 | 2000000 | 46.94% | 1.17 | 694.64 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 3.808µs | 3735 | 0 | 50.06% | 1.24 | 4007.03 MB/s |
| Timsort | 1000 | 5.091µs | 3747 | 0 | 50.06% | 1.24 | 2997.21 MB/s |
| ARS Gen 1: Foundation | 1000 | 32.376µs | 995 | 2000 | 50.06% | 1.24 | 471.30 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 43.86µs | 995 | 2000 | 50.06% | 1.24 | 347.90 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 3.719µs | 3735 | 0 | 50.06% | 1.24 | 4102.93 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 3.546µs | 3735 | 0 | 50.06% | 1.24 | 4303.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 3.539µs | 3735 | 0 | 50.06% | 1.24 | 4311.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 4.805µs | 3747 | 0 | 50.06% | 1.24 | 3175.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 3.667µs | 3735 | 0 | 50.06% | 1.24 | 4161.11 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 5.139µs | 3747 | 0 | 50.06% | 1.24 | 2969.21 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 3.522µs | 3735 | 0 | 50.06% | 1.24 | 4332.42 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 3.577µs | 3735 | 0 | 50.06% | 1.24 | 4265.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 3.528µs | 3735 | 0 | 50.06% | 1.24 | 4325.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 117.831µs | 3735 | 2000 | 50.05% | 1.24 | 129.50 MB/s |
| Quicksort | 10000 | 29.625µs | 36573 | 0 | 49.98% | 1.24 | 5150.65 MB/s |
| Timsort | 10000 | 38.54µs | 36775 | 0 | 49.98% | 1.24 | 3959.21 MB/s |
| ARS Gen 1: Foundation | 10000 | 194.262µs | 9995 | 30000 | 49.95% | 1.24 | 785.47 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 291.541µs | 9995 | 30000 | 49.96% | 1.24 | 523.38 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 257.052µs | 115988 | 14351 | 49.95% | 1.24 | 593.61 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 204.618µs | 9999 | 10000 | 49.92% | 1.24 | 745.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 122.152µs | 9999 | 0 | 49.93% | 1.24 | 1249.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 120.471µs | 9999 | 0 | 49.92% | 1.24 | 1266.59 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 241.697µs | 9999 | 0 | 49.93% | 1.24 | 631.32 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 235.389µs | 9999 | 0 | 49.93% | 1.24 | 648.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 123.907µs | 9999 | 0 | 49.92% | 1.24 | 1231.47 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 125.452µs | 9999 | 0 | 49.92% | 1.24 | 1216.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 119.746µs | 9999 | 0 | 49.92% | 1.24 | 1274.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 289.088µs | 9999 | 20000 | 49.89% | 1.24 | 527.83 MB/s |
| Quicksort | 100000 | 315.274µs | 362094 | 0 | 49.72% | 1.24 | 4839.85 MB/s |
| Timsort | 100000 | 428.735µs | 382517 | 0 | 49.54% | 1.24 | 3559.03 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.178217ms | 99995 | 300000 | 49.71% | 1.24 | 1295.07 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.442657ms | 99995 | 300000 | 49.72% | 1.24 | 1057.69 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.857563ms | 1129938 | 108703 | 49.73% | 1.25 | 821.44 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 574.725µs | 100001 | 100000 | 49.55% | 1.24 | 2654.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 397.026µs | 100001 | 0 | 49.53% | 1.24 | 3843.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 365.835µs | 100001 | 0 | 49.54% | 1.24 | 4170.95 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 538.688µs | 100001 | 0 | 49.59% | 1.24 | 2832.58 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 534.004µs | 100001 | 0 | 49.60% | 1.24 | 2857.43 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 707.674µs | 199996 | 0 | 49.45% | 1.24 | 2156.19 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 594.271µs | 199996 | 0 | 49.59% | 1.24 | 2567.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 570.701µs | 199996 | 0 | 49.48% | 1.24 | 2673.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 810.008µs | 100001 | 200000 | 49.51% | 1.24 | 1883.78 MB/s |
| Quicksort | 1000000 | 3.26972ms | 3809528 | 0 | 48.95% | 1.25 | 4666.70 MB/s |
| Timsort | 1000000 | 7.35826ms | 4510660 | 0 | 48.61% | 1.25 | 2073.70 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.182777ms | 12062959 | 1017407 | 49.63% | 1.26 | 720.34 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.445619ms | 999999 | 1000000 | 50.59% | 1.19 | 2367.31 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.592311ms | 999999 | 0 | 50.60% | 1.20 | 2314.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.583577ms | 999999 | 0 | 50.65% | 1.20 | 2317.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.705297ms | 999999 | 0 | 50.06% | 1.22 | 3242.90 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.680487ms | 999999 | 0 | 50.05% | 1.22 | 3260.09 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.751551ms | 1999994 | 0 | 50.70% | 1.18 | 1743.55 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.283357ms | 1999994 | 0 | 50.53% | 1.20 | 2095.02 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.000191ms | 1999994 | 0 | 50.80% | 1.16 | 1695.39 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 17.507542ms | 5364815 | 2000000 | 48.80% | 1.21 | 871.56 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.597µs | 5508 | 0 | 49.24% | 1.23 | 2726.24 MB/s |
| Timsort | 1000 | 7.844µs | 5460 | 0 | 49.24% | 1.23 | 1945.28 MB/s |
| ARS Gen 1: Foundation | 1000 | 30.916µs | 4914 | 2000 | 49.24% | 1.23 | 493.56 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 38.419µs | 4914 | 2000 | 49.24% | 1.23 | 397.17 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 5.135µs | 5508 | 0 | 49.24% | 1.23 | 2971.53 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.176µs | 5508 | 0 | 49.24% | 1.23 | 2947.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.003µs | 5508 | 0 | 49.24% | 1.23 | 3049.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 7.031µs | 5460 | 0 | 49.24% | 1.23 | 2170.22 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.011µs | 5508 | 0 | 49.24% | 1.23 | 3045.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.363µs | 5460 | 0 | 49.24% | 1.23 | 2072.36 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5µs | 5508 | 0 | 49.24% | 1.23 | 3051.76 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 5.009µs | 5508 | 0 | 49.24% | 1.23 | 3046.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 5.124µs | 5508 | 0 | 49.24% | 1.23 | 2977.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 111.361µs | 5508 | 2000 | 49.22% | 1.23 | 137.02 MB/s |
| Quicksort | 10000 | 39.524µs | 53621 | 0 | 49.18% | 1.23 | 3860.64 MB/s |
| Timsort | 10000 | 49.774µs | 53742 | 0 | 49.17% | 1.23 | 3065.61 MB/s |
| ARS Gen 1: Foundation | 10000 | 218.04µs | 50132 | 30000 | 49.14% | 1.23 | 699.82 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 253.964µs | 50259 | 30000 | 49.14% | 1.23 | 600.82 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 245.167µs | 124917 | 14351 | 49.15% | 1.23 | 622.38 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 271.432µs | 52500 | 10000 | 49.12% | 1.23 | 562.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 184.518µs | 52500 | 0 | 49.12% | 1.23 | 826.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 185.84µs | 51829 | 0 | 49.11% | 1.23 | 821.07 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 347.588µs | 42069 | 0 | 49.13% | 1.23 | 438.99 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 332.502µs | 42671 | 0 | 49.12% | 1.23 | 458.91 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 179.5µs | 16860 | 0 | 49.10% | 1.22 | 850.07 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 181.762µs | 52500 | 0 | 49.12% | 1.23 | 839.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 171.635µs | 52500 | 0 | 49.12% | 1.22 | 889.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 301.585µs | 52500 | 20000 | 49.08% | 1.22 | 505.95 MB/s |
| Quicksort | 100000 | 379.362µs | 532062 | 0 | 49.03% | 1.23 | 4022.22 MB/s |
| Timsort | 100000 | 473.594µs | 535405 | 0 | 48.85% | 1.23 | 3221.91 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.692278ms | 506805 | 300000 | 48.98% | 1.23 | 901.67 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.089652ms | 506783 | 300000 | 48.97% | 1.23 | 730.21 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.655898ms | 1174310 | 108703 | 49.01% | 1.23 | 921.48 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.392594ms | 519466 | 100000 | 48.91% | 1.22 | 1095.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 929.677µs | 519466 | 0 | 48.84% | 1.23 | 1641.30 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 932.784µs | 520212 | 0 | 48.72% | 1.22 | 1635.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.037253ms | 499859 | 0 | 48.86% | 1.23 | 1471.08 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.250049ms | 502934 | 0 | 48.79% | 1.23 | 1220.66 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.264392ms | 203055 | 0 | 48.48% | 1.22 | 1206.81 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 914.118µs | 182074 | 0 | 48.62% | 1.22 | 1669.24 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.470837ms | 197448 | 0 | 48.50% | 1.22 | 1037.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.298195ms | 519466 | 200000 | 48.79% | 1.23 | 1175.38 MB/s |
| Quicksort | 1000000 | 4.315826ms | 5301519 | 0 | 48.38% | 1.24 | 3535.54 MB/s |
| Timsort | 1000000 | 8.829377ms | 6302942 | 0 | 47.65% | 1.23 | 1728.18 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.629038ms | 12308876 | 1017407 | 49.12% | 1.24 | 705.48 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 15.031615ms | 5221477 | 1000000 | 49.38% | 1.19 | 1015.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.01662ms | 5221477 | 0 | 49.14% | 1.19 | 1385.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.410083ms | 6004244 | 0 | 48.72% | 1.20 | 876.43 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.229308ms | 5269244 | 0 | 48.91% | 1.22 | 1854.20 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.689836ms | 6047616 | 0 | 48.27% | 1.22 | 1202.44 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.243311ms | 1938046 | 0 | 49.95% | 1.17 | 884.91 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 23.843858ms | 2076365 | 0 | 50.81% | 1.19 | 639.95 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 30.00664ms | 2063926 | 0 | 50.98% | 1.17 | 508.51 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.774611ms | 9646390 | 2000000 | 48.84% | 1.21 | 771.64 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.886µs | 10296 | 0 | 50.04% | 1.29 | 1401.69 MB/s |
| Timsort | 1000 | 18.215µs | 10670 | 0 | 50.04% | 1.29 | 837.70 MB/s |
| ARS Gen 1: Foundation | 1000 | 165.775µs | 808 | 2000 | 50.04% | 1.29 | 92.05 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 182.065µs | 808 | 2000 | 50.04% | 1.29 | 83.81 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.091µs | 10296 | 0 | 50.04% | 1.29 | 1375.78 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 12.799µs | 10296 | 0 | 50.04% | 1.29 | 1192.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 12.783µs | 10296 | 0 | 50.04% | 1.29 | 1193.68 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.609µs | 10670 | 0 | 50.04% | 1.29 | 866.53 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 12.338µs | 10296 | 0 | 50.04% | 1.29 | 1236.73 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.158µs | 10670 | 0 | 50.04% | 1.29 | 840.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.751µs | 10296 | 0 | 50.04% | 1.29 | 1419.29 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.148µs | 10296 | 0 | 50.04% | 1.29 | 1368.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.737µs | 10296 | 0 | 50.04% | 1.29 | 1421.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.213µs | 10296 | 2000 | 50.03% | 1.29 | 132.44 MB/s |
| Quicksort | 10000 | 135.11µs | 134101 | 0 | 49.98% | 1.29 | 1129.36 MB/s |
| Timsort | 10000 | 197.006µs | 137729 | 0 | 49.98% | 1.29 | 774.53 MB/s |
| ARS Gen 1: Foundation | 10000 | 953.697µs | 84429 | 30000 | 49.93% | 1.29 | 160.00 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.063202ms | 84430 | 30000 | 49.93% | 1.29 | 143.52 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 295.892µs | 190005 | 14351 | 49.96% | 1.29 | 515.69 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 225.942µs | 71389 | 10000 | 49.94% | 1.29 | 675.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 142.042µs | 71389 | 0 | 49.94% | 1.29 | 1074.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 148.82µs | 73990 | 0 | 49.93% | 1.29 | 1025.32 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 322.195µs | 60060 | 0 | 49.94% | 1.29 | 473.59 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 331.116µs | 62651 | 0 | 49.94% | 1.29 | 460.83 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 151.672µs | 71389 | 0 | 49.93% | 1.29 | 1006.04 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 152.472µs | 71389 | 0 | 49.94% | 1.29 | 1000.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 155.755µs | 71389 | 0 | 49.94% | 1.29 | 979.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 326.28µs | 71389 | 20000 | 49.91% | 1.29 | 467.66 MB/s |
| Quicksort | 100000 | 1.243058ms | 1353942 | 0 | 49.80% | 1.30 | 1227.52 MB/s |
| Timsort | 100000 | 1.507576ms | 1358979 | 0 | 49.65% | 1.30 | 1012.14 MB/s |
| ARS Gen 1: Foundation | 100000 | 6.241252ms | 1260666 | 300000 | 49.49% | 1.28 | 244.48 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 6.785253ms | 1260598 | 300000 | 49.46% | 1.28 | 224.88 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.173038ms | 1555111 | 108703 | 49.74% | 1.30 | 702.19 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 947.624µs | 735888 | 100000 | 49.77% | 1.29 | 1610.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 700.713µs | 735888 | 0 | 49.76% | 1.29 | 2177.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 739.912µs | 741765 | 0 | 49.75% | 1.29 | 2062.24 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 814.819µs | 651623 | 0 | 49.76% | 1.29 | 1872.66 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 863.45µs | 657591 | 0 | 49.75% | 1.29 | 1767.19 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 785.823µs | 710308 | 0 | 49.72% | 1.29 | 1941.76 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 782.697µs | 631417 | 0 | 49.76% | 1.29 | 1949.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 768.038µs | 735888 | 0 | 49.72% | 1.29 | 1986.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.120538ms | 735888 | 200000 | 49.69% | 1.29 | 1361.74 MB/s |
| Quicksort | 1000000 | 10.127208ms | 12909957 | 0 | 48.82% | 1.33 | 1506.71 MB/s |
| Timsort | 1000000 | 17.027825ms | 14007926 | 0 | 47.80% | 1.32 | 896.11 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.31045ms | 14286900 | 1017407 | 49.18% | 1.31 | 751.28 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.523176ms | 5157050 | 1000000 | 50.45% | 1.26 | 2028.24 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.308154ms | 5157050 | 0 | 50.44% | 1.26 | 2087.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 8.111416ms | 5175392 | 0 | 50.30% | 1.26 | 1881.15 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.786095ms | 6007807 | 0 | 50.15% | 1.29 | 2637.15 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.493428ms | 6046248 | 0 | 49.91% | 1.28 | 2349.88 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.17373ms | 2361022 | 0 | 49.80% | 1.26 | 1499.82 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.058221ms | 1866734 | 0 | 49.79% | 1.26 | 1517.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.436672ms | 2025491 | 0 | 50.04% | 1.25 | 1462.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.688095ms | 11742720 | 2000000 | 50.60% | 1.28 | 816.50 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 12.207µs | 10451 | 0 | 51.15% | 1.32 | 1250.00 MB/s |
| Timsort | 1000 | 19.518µs | 10742 | 0 | 51.15% | 1.32 | 781.78 MB/s |
| ARS Gen 1: Foundation | 1000 | 111.646µs | 5331 | 2000 | 51.15% | 1.32 | 136.67 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 123.663µs | 5339 | 2000 | 51.15% | 1.32 | 123.39 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 12.211µs | 10451 | 0 | 51.15% | 1.32 | 1249.59 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 12.025µs | 10451 | 0 | 51.15% | 1.32 | 1268.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 12.114µs | 10451 | 0 | 51.15% | 1.32 | 1259.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 19.967µs | 10742 | 0 | 51.15% | 1.32 | 764.20 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 12.591µs | 10451 | 0 | 51.15% | 1.32 | 1211.88 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 19.434µs | 10742 | 0 | 51.15% | 1.32 | 785.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 12.281µs | 10451 | 0 | 51.15% | 1.32 | 1242.47 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 12.039µs | 10451 | 0 | 51.15% | 1.32 | 1267.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.946µs | 10451 | 0 | 51.15% | 1.32 | 1277.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.947µs | 10451 | 2000 | 51.15% | 1.32 | 131.60 MB/s |
| Quicksort | 10000 | 110.507µs | 111159 | 0 | 51.11% | 1.32 | 1380.80 MB/s |
| Timsort | 10000 | 150.012µs | 110728 | 0 | 51.11% | 1.32 | 1017.17 MB/s |
| ARS Gen 1: Foundation | 10000 | 464.552µs | 75427 | 30000 | 51.09% | 1.32 | 328.46 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 571.72µs | 74701 | 30000 | 51.09% | 1.32 | 266.89 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 295.862µs | 163143 | 14351 | 51.09% | 1.32 | 515.74 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 248.746µs | 72583 | 10000 | 51.08% | 1.32 | 613.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 155.223µs | 72583 | 0 | 51.08% | 1.32 | 983.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 171.141µs | 72287 | 0 | 51.08% | 1.32 | 891.59 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 242.228µs | 63448 | 0 | 51.08% | 1.32 | 629.93 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 261.935µs | 63348 | 0 | 51.08% | 1.32 | 582.54 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 157.474µs | 72583 | 0 | 51.08% | 1.32 | 968.97 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 155.756µs | 72583 | 0 | 51.08% | 1.32 | 979.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 160.069µs | 72583 | 0 | 51.09% | 1.32 | 953.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 341.636µs | 72583 | 20000 | 51.07% | 1.32 | 446.64 MB/s |
| Quicksort | 100000 | 879.835µs | 1016581 | 0 | 51.00% | 1.32 | 1734.28 MB/s |
| Timsort | 100000 | 1.040069ms | 1021185 | 0 | 50.88% | 1.32 | 1467.09 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.363766ms | 680916 | 300000 | 50.98% | 1.32 | 645.53 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.760974ms | 680031 | 300000 | 50.98% | 1.32 | 552.66 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.946828ms | 1237724 | 108703 | 50.94% | 1.32 | 783.78 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.009254ms | 631252 | 100000 | 50.97% | 1.32 | 1511.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 791.114µs | 631252 | 0 | 50.96% | 1.32 | 1928.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 722.513µs | 634097 | 0 | 50.96% | 1.32 | 2111.91 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 727.837µs | 555626 | 0 | 50.97% | 1.32 | 2096.46 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 731.623µs | 562372 | 0 | 50.96% | 1.32 | 2085.61 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 782.044µs | 134521 | 0 | 50.79% | 1.32 | 1951.14 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 815.769µs | 169903 | 0 | 50.84% | 1.32 | 1870.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 710.761µs | 264519 | 0 | 50.85% | 1.32 | 2146.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.141083ms | 631252 | 200000 | 50.92% | 1.32 | 1337.22 MB/s |
| Quicksort | 1000000 | 7.844941ms | 9921218 | 0 | 50.28% | 1.34 | 1945.05 MB/s |
| Timsort | 1000000 | 12.787095ms | 11000160 | 0 | 49.09% | 1.33 | 1193.30 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 23.183488ms | 12348632 | 1017407 | 50.36% | 1.33 | 658.17 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 8.01217ms | 5346522 | 1000000 | 51.35% | 1.29 | 1904.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.259254ms | 5346522 | 0 | 51.31% | 1.29 | 2101.98 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.969247ms | 5363683 | 0 | 50.90% | 1.29 | 1914.71 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.538375ms | 5434749 | 0 | 51.05% | 1.31 | 2755.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.508711ms | 5451863 | 0 | 50.55% | 1.30 | 2344.36 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.788891ms | 1070102 | 0 | 51.13% | 1.27 | 1414.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.956384ms | 1041286 | 0 | 51.51% | 1.27 | 1276.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.024192ms | 1013215 | 0 | 51.74% | 1.26 | 1171.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 17.673782ms | 11100972 | 2000000 | 51.20% | 1.30 | 863.36 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.457µs | 10179 | 0 | 51.31% | 1.33 | 1331.83 MB/s |
| Timsort | 1000 | 18.331µs | 10913 | 0 | 51.31% | 1.33 | 832.40 MB/s |
| ARS Gen 1: Foundation | 1000 | 241.963µs | 0 | 2000 | 51.31% | 1.33 | 63.06 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 278.323µs | 0 | 2000 | 51.30% | 1.33 | 54.82 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.406µs | 10179 | 0 | 51.31% | 1.33 | 1337.79 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.292µs | 10179 | 0 | 51.31% | 1.33 | 1351.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.452µs | 10179 | 0 | 51.31% | 1.33 | 1332.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.796µs | 10913 | 0 | 51.31% | 1.33 | 811.81 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.571µs | 10179 | 0 | 51.31% | 1.33 | 1318.71 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.485µs | 10913 | 0 | 51.31% | 1.33 | 825.47 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.138µs | 10179 | 0 | 51.31% | 1.33 | 1369.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.172µs | 10179 | 0 | 51.31% | 1.33 | 1365.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.511µs | 10179 | 0 | 51.31% | 1.33 | 1325.58 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 125.793µs | 10179 | 2000 | 51.30% | 1.33 | 121.30 MB/s |
| Quicksort | 10000 | 141.604µs | 137738 | 0 | 51.27% | 1.33 | 1077.57 MB/s |
| Timsort | 10000 | 214.552µs | 141392 | 0 | 51.27% | 1.33 | 711.19 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.548388ms | 0 | 30000 | 50.87% | 1.33 | 27.50 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.980986ms | 0 | 30000 | 50.82% | 1.33 | 25.51 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 340.268µs | 193231 | 14351 | 51.25% | 1.33 | 448.43 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 232.746µs | 51645 | 10000 | 51.23% | 1.33 | 655.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 148.482µs | 51645 | 0 | 51.23% | 1.33 | 1027.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 157.699µs | 57426 | 0 | 51.22% | 1.33 | 967.59 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 267.807µs | 59098 | 0 | 51.23% | 1.33 | 569.77 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 273.823µs | 62026 | 0 | 51.24% | 1.33 | 557.25 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 148.097µs | 51645 | 0 | 51.23% | 1.33 | 1030.32 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 149.767µs | 51645 | 0 | 51.23% | 1.33 | 1018.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 155.181µs | 51645 | 0 | 51.23% | 1.33 | 983.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 355.603µs | 51645 | 20000 | 51.22% | 1.33 | 429.10 MB/s |
| Quicksort | 100000 | 1.699208ms | 1704558 | 0 | 51.11% | 1.33 | 897.99 MB/s |
| Timsort | 100000 | 2.347502ms | 1748721 | 0 | 51.02% | 1.33 | 650.00 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.900703ms | 6 | 300000 | 42.66% | 1.29 | 34.76 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 48.99021ms | 6 | 300000 | 40.25% | 1.26 | 31.15 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.506195ms | 1886207 | 108703 | 51.07% | 1.33 | 608.84 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 993.247µs | 879882 | 100000 | 51.08% | 1.33 | 1536.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 772.643µs | 879882 | 0 | 51.08% | 1.32 | 1974.88 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 898.905µs | 922129 | 0 | 51.08% | 1.32 | 1697.49 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 919.058µs | 955522 | 0 | 51.07% | 1.33 | 1660.26 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.069958ms | 994838 | 0 | 51.08% | 1.32 | 1426.11 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 771.924µs | 879882 | 0 | 51.05% | 1.33 | 1976.72 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 761.748µs | 773088 | 0 | 51.06% | 1.33 | 2003.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 784.59µs | 879882 | 0 | 51.05% | 1.32 | 1944.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.195683ms | 879882 | 200000 | 51.05% | 1.33 | 1276.16 MB/s |
| Quicksort | 1000000 | 18.126904ms | 20437271 | 0 | 50.41% | 1.36 | 841.78 MB/s |
| Timsort | 1000000 | 29.304032ms | 20799465 | 0 | 49.67% | 1.35 | 520.71 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.628993ms | 21505010 | 1017407 | 50.59% | 1.33 | 619.55 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.509587ms | 10221412 | 1000000 | 51.20% | 1.30 | 1604.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.838295ms | 10221412 | 0 | 51.22% | 1.29 | 1726.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.544603ms | 10628930 | 0 | 51.19% | 1.29 | 1598.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.473361ms | 12929332 | 0 | 51.15% | 1.32 | 2041.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.799048ms | 13335182 | 0 | 51.14% | 1.32 | 1734.14 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.535709ms | 10221412 | 0 | 51.12% | 1.30 | 1600.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.258051ms | 11275443 | 0 | 51.28% | 1.31 | 1648.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.598572ms | 12322876 | 0 | 51.38% | 1.29 | 1439.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 27.075113ms | 13663613 | 2000000 | 51.07% | 1.30 | 563.57 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.443µs | 5504 | 0 | 52.33% | 1.27 | 2803.38 MB/s |
| Timsort | 1000 | 7.556µs | 5497 | 0 | 52.33% | 1.27 | 2019.43 MB/s |
| ARS Gen 1: Foundation | 1000 | 46.712µs | 984 | 2000 | 52.33% | 1.27 | 326.66 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 60.916µs | 984 | 2000 | 52.33% | 1.27 | 250.49 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 5.05µs | 5504 | 0 | 52.33% | 1.27 | 3021.54 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.074µs | 5504 | 0 | 52.33% | 1.27 | 3007.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.012µs | 5504 | 0 | 52.33% | 1.27 | 3044.45 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 7.277µs | 5497 | 0 | 52.33% | 1.27 | 2096.85 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.225µs | 5504 | 0 | 52.33% | 1.27 | 2920.34 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.536µs | 5497 | 0 | 52.33% | 1.27 | 2024.79 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5.108µs | 5504 | 0 | 52.33% | 1.27 | 2987.23 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 5.138µs | 5504 | 0 | 52.33% | 1.27 | 2969.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 5.196µs | 5504 | 0 | 52.33% | 1.27 | 2936.64 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 114.011µs | 5504 | 2000 | 52.32% | 1.27 | 133.84 MB/s |
| Quicksort | 10000 | 42.811µs | 53753 | 0 | 52.30% | 1.27 | 3564.22 MB/s |
| Timsort | 10000 | 54.179µs | 54514 | 0 | 52.30% | 1.27 | 2816.37 MB/s |
| ARS Gen 1: Foundation | 10000 | 261.828µs | 9984 | 30000 | 52.29% | 1.27 | 582.78 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 330µs | 9984 | 30000 | 52.29% | 1.27 | 462.39 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 253.715µs | 121806 | 14351 | 52.29% | 1.27 | 601.41 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 206.782µs | 12063 | 10000 | 52.28% | 1.27 | 737.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 127.83µs | 12063 | 0 | 52.27% | 1.27 | 1193.68 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 125.589µs | 12087 | 0 | 52.27% | 1.27 | 1214.98 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 224.986µs | 12063 | 0 | 52.28% | 1.27 | 678.21 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 228.359µs | 12087 | 0 | 52.28% | 1.27 | 668.19 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 127.856µs | 12063 | 0 | 52.27% | 1.27 | 1193.44 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 144.427µs | 12063 | 0 | 52.27% | 1.27 | 1056.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 134.454µs | 12063 | 0 | 52.28% | 1.27 | 1134.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 307.792µs | 12063 | 20000 | 52.26% | 1.27 | 495.75 MB/s |
| Quicksort | 100000 | 436.94µs | 522910 | 0 | 52.18% | 1.27 | 3492.19 MB/s |
| Timsort | 100000 | 547.625µs | 516617 | 0 | 52.10% | 1.27 | 2786.36 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.324128ms | 99984 | 300000 | 52.18% | 1.27 | 1152.37 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.677027ms | 99984 | 300000 | 52.17% | 1.27 | 909.87 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.87856ms | 1144941 | 108703 | 52.17% | 1.27 | 812.26 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 447.476µs | 144579 | 100000 | 52.12% | 1.27 | 3409.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 401.81µs | 144579 | 0 | 52.11% | 1.27 | 3797.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 381.498µs | 145223 | 0 | 52.12% | 1.27 | 3999.70 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 490.458µs | 99988 | 0 | 52.13% | 1.27 | 3111.13 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 591.131µs | 99988 | 0 | 52.13% | 1.27 | 2581.29 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 556.516µs | 199988 | 0 | 52.11% | 1.27 | 2741.84 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 552.603µs | 199972 | 0 | 52.11% | 1.27 | 2761.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 407.186µs | 100004 | 0 | 52.10% | 1.27 | 3747.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 819.378µs | 144579 | 200000 | 52.14% | 1.27 | 1862.24 MB/s |
| Quicksort | 1000000 | 4.257156ms | 5201420 | 0 | 51.64% | 1.28 | 3584.27 MB/s |
| Timsort | 1000000 | 8.300351ms | 6174589 | 0 | 51.07% | 1.28 | 1838.33 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.358459ms | 12089713 | 1017407 | 52.05% | 1.28 | 749.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.383127ms | 999990 | 1000000 | 52.48% | 1.24 | 2390.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.074194ms | 999990 | 0 | 52.51% | 1.25 | 2156.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.500149ms | 999990 | 0 | 52.47% | 1.25 | 2347.45 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.424659ms | 999990 | 0 | 52.28% | 1.26 | 3448.58 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.298028ms | 999990 | 0 | 52.28% | 1.26 | 3550.18 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.763612ms | 1999974 | 0 | 52.42% | 1.24 | 1965.42 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.130122ms | 1999974 | 0 | 52.36% | 1.25 | 2140.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.934044ms | 1999984 | 0 | 52.39% | 1.24 | 1923.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.671083ms | 5706274 | 2000000 | 51.94% | 1.26 | 915.28 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.021µs | 10179 | 0 | 51.69% | 1.27 | 1384.52 MB/s |
| Timsort | 1000 | 17.642µs | 10913 | 0 | 51.69% | 1.27 | 864.91 MB/s |
| ARS Gen 1: Foundation | 1000 | 227.983µs | 0 | 2000 | 51.69% | 1.27 | 66.93 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 261.508µs | 0 | 2000 | 51.69% | 1.27 | 58.35 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.841µs | 10179 | 0 | 51.69% | 1.27 | 1407.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.861µs | 10179 | 0 | 51.69% | 1.27 | 1404.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.978µs | 10179 | 0 | 51.69% | 1.27 | 1389.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.495µs | 10913 | 0 | 51.69% | 1.27 | 872.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.145µs | 10179 | 0 | 51.69% | 1.27 | 1369.12 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 17.911µs | 10913 | 0 | 51.69% | 1.27 | 851.92 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.983µs | 10179 | 0 | 51.69% | 1.27 | 1389.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.785µs | 10179 | 0 | 51.69% | 1.27 | 1414.82 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.903µs | 10179 | 0 | 51.69% | 1.27 | 1399.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.619µs | 10179 | 2000 | 51.68% | 1.27 | 139.20 MB/s |
| Quicksort | 10000 | 134.662µs | 137738 | 0 | 51.66% | 1.27 | 1133.12 MB/s |
| Timsort | 10000 | 201.928µs | 141392 | 0 | 51.66% | 1.27 | 755.65 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.432204ms | 0 | 30000 | 51.31% | 1.28 | 28.09 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.699789ms | 0 | 30000 | 51.28% | 1.28 | 26.77 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 323.919µs | 193231 | 14351 | 51.64% | 1.27 | 471.07 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 239.179µs | 51645 | 10000 | 51.63% | 1.27 | 637.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 161.454µs | 51645 | 0 | 51.63% | 1.27 | 945.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 171.877µs | 57426 | 0 | 51.63% | 1.27 | 887.77 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 284.391µs | 59098 | 0 | 51.64% | 1.27 | 536.54 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 293.374µs | 62026 | 0 | 51.63% | 1.27 | 520.11 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 166.927µs | 51645 | 0 | 51.63% | 1.27 | 914.10 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 151.794µs | 51645 | 0 | 51.63% | 1.27 | 1005.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 157.87µs | 51645 | 0 | 51.63% | 1.27 | 966.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 355.959µs | 51645 | 20000 | 51.61% | 1.27 | 428.67 MB/s |
| Quicksort | 100000 | 1.743166ms | 1704558 | 0 | 51.52% | 1.27 | 875.35 MB/s |
| Timsort | 100000 | 2.326239ms | 1748721 | 0 | 51.45% | 1.27 | 655.94 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.145838ms | 6 | 300000 | 43.54% | 1.23 | 35.37 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.091831ms | 6 | 300000 | 42.57% | 1.24 | 33.11 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.628298ms | 1886207 | 108703 | 51.50% | 1.27 | 580.56 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.197258ms | 879882 | 100000 | 51.51% | 1.27 | 1274.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 829.515µs | 879882 | 0 | 51.51% | 1.27 | 1839.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 841.917µs | 922129 | 0 | 51.48% | 1.27 | 1812.39 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 806.263µs | 955522 | 0 | 51.51% | 1.27 | 1892.53 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.023657ms | 994838 | 0 | 51.51% | 1.27 | 1490.62 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 820.034µs | 879882 | 0 | 51.49% | 1.27 | 1860.75 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 812.994µs | 773088 | 0 | 51.50% | 1.27 | 1876.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 797.074µs | 879882 | 0 | 51.49% | 1.27 | 1914.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.18743ms | 879882 | 200000 | 51.50% | 1.27 | 1285.03 MB/s |
| Quicksort | 1000000 | 18.966085ms | 20437271 | 0 | 51.04% | 1.30 | 804.53 MB/s |
| Timsort | 1000000 | 28.432772ms | 20799465 | 0 | 50.26% | 1.29 | 536.66 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.53193ms | 21505010 | 1017407 | 51.06% | 1.28 | 622.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.643517ms | 10221412 | 1000000 | 51.65% | 1.26 | 1582.28 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.246917ms | 10221412 | 0 | 51.69% | 1.25 | 1650.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.302861ms | 10628930 | 0 | 51.67% | 1.25 | 1481.02 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.450289ms | 12929332 | 0 | 51.62% | 1.27 | 1614.64 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 9.386756ms | 13335182 | 0 | 51.61% | 1.27 | 1625.57 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.082251ms | 10221412 | 0 | 51.59% | 1.26 | 1513.43 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.169182ms | 11275443 | 0 | 51.81% | 1.27 | 1500.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 12.180168ms | 12322876 | 0 | 51.80% | 1.26 | 1252.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.636574ms | 13469297 | 2000000 | 51.49% | 1.25 | 498.06 MB/s |

## Category: f64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 28.412µs | 10325 | 0 | 52.79% | 1.25 | 537.05 MB/s |
| Timsort | 1000 | 35.404µs | 10521 | 0 | 52.79% | 1.25 | 430.99 MB/s |
| ARS Gen 1: Foundation | 1000 | 297.41µs | 0 | 2000 | 52.79% | 1.25 | 51.31 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 354.454µs | 0 | 2000 | 52.79% | 1.25 | 43.05 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 28.259µs | 10325 | 0 | 52.79% | 1.25 | 539.96 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 22.218µs | 10325 | 0 | 52.79% | 1.25 | 686.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.896µs | 10325 | 0 | 52.79% | 1.25 | 666.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 36.327µs | 10521 | 0 | 52.79% | 1.25 | 420.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 24.448µs | 10325 | 0 | 52.79% | 1.25 | 624.13 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 34.784µs | 10521 | 0 | 52.79% | 1.25 | 438.67 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 23.847µs | 10325 | 0 | 52.79% | 1.25 | 639.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 22.514µs | 10325 | 0 | 52.79% | 1.25 | 677.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 23.104µs | 10325 | 0 | 52.79% | 1.25 | 660.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 153.266µs | 10325 | 2000 | 52.78% | 1.25 | 99.56 MB/s |
| Quicksort | 10000 | 284.432µs | 136464 | 0 | 52.76% | 1.25 | 536.47 MB/s |
| Timsort | 10000 | 416.064µs | 141512 | 0 | 52.76% | 1.25 | 366.74 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.84642ms | 0 | 30000 | 52.48% | 1.25 | 22.29 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.693284ms | 0 | 30000 | 52.45% | 1.25 | 22.80 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 463.555µs | 193135 | 14351 | 52.75% | 1.25 | 329.17 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 288.248µs | 73138 | 10000 | 52.74% | 1.25 | 529.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 191.649µs | 73138 | 0 | 52.74% | 1.25 | 796.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 348.229µs | 76380 | 0 | 52.75% | 1.25 | 438.18 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 319.502µs | 62698 | 0 | 52.75% | 1.25 | 477.58 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 386.364µs | 65867 | 0 | 52.75% | 1.25 | 394.93 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 286.75µs | 73138 | 0 | 52.75% | 1.25 | 532.13 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 361.827µs | 73138 | 0 | 52.76% | 1.25 | 421.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 196.611µs | 73138 | 0 | 52.74% | 1.25 | 776.09 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 404.211µs | 73138 | 20000 | 52.73% | 1.25 | 377.50 MB/s |
| Quicksort | 100000 | 3.296974ms | 1705718 | 0 | 52.66% | 1.25 | 462.81 MB/s |
| Timsort | 100000 | 4.320682ms | 1751732 | 0 | 52.61% | 1.25 | 353.16 MB/s |
| ARS Gen 1: Foundation | 100000 | 50.105303ms | 0 | 300000 | 46.49% | 1.23 | 30.45 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 50.885806ms | 0 | 300000 | 44.51% | 1.22 | 29.99 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.276531ms | 1884272 | 108703 | 52.64% | 1.25 | 465.70 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.271095ms | 1101865 | 100000 | 52.64% | 1.25 | 1200.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.183643ms | 1101865 | 0 | 52.64% | 1.24 | 1289.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.28584ms | 1142841 | 0 | 52.64% | 1.24 | 1186.68 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.041984ms | 1002379 | 0 | 52.64% | 1.25 | 1464.40 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.242285ms | 1045724 | 0 | 52.63% | 1.25 | 1228.28 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.081486ms | 1101865 | 0 | 52.62% | 1.25 | 1410.91 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.011062ms | 999614 | 0 | 52.64% | 1.25 | 1509.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.040528ms | 1101865 | 0 | 52.62% | 1.25 | 1466.45 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.487008ms | 1101865 | 200000 | 52.61% | 1.24 | 1026.14 MB/s |
| Quicksort | 1000000 | 29.532161ms | 20430901 | 0 | 52.26% | 1.27 | 516.68 MB/s |
| Timsort | 1000000 | 43.05464ms | 20822215 | 0 | 51.68% | 1.27 | 354.41 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.929959ms | 21498086 | 1017407 | 52.29% | 1.25 | 463.37 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 12.671473ms | 12665814 | 1000000 | 52.89% | 1.24 | 1204.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.262546ms | 12665814 | 0 | 52.86% | 1.25 | 1354.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.305201ms | 13081361 | 0 | 52.83% | 1.24 | 996.97 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 10.654628ms | 13583765 | 0 | 52.76% | 1.25 | 1432.13 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 13.277757ms | 14002566 | 0 | 52.65% | 1.25 | 1149.20 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 11.613667ms | 6406252 | 0 | 52.51% | 1.23 | 1313.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 13.475957ms | 5861815 | 0 | 52.48% | 1.24 | 1132.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 12.851216ms | 7398340 | 0 | 52.60% | 1.24 | 1187.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 27.626361ms | 14470211 | 2000000 | 52.91% | 1.23 | 552.33 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 20.036µs | 10345 | 0 | 52.62% | 1.23 | 761.57 MB/s |
| Timsort | 1000 | 29.864µs | 10685 | 0 | 52.62% | 1.23 | 510.94 MB/s |
| ARS Gen 1: Foundation | 1000 | 256.24µs | 0 | 2000 | 52.62% | 1.23 | 59.55 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 305.274µs | 0 | 2000 | 52.62% | 1.23 | 49.98 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 18.859µs | 10345 | 0 | 52.62% | 1.23 | 809.10 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.996µs | 10345 | 0 | 52.62% | 1.23 | 803.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 19.141µs | 10345 | 0 | 52.62% | 1.23 | 797.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 28.852µs | 10685 | 0 | 52.62% | 1.23 | 528.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 20.1µs | 10345 | 0 | 52.62% | 1.23 | 759.14 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.379µs | 10685 | 0 | 52.62% | 1.23 | 519.38 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.715µs | 10345 | 0 | 52.62% | 1.23 | 736.61 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 19.203µs | 10345 | 0 | 52.62% | 1.23 | 794.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.19µs | 10345 | 0 | 52.62% | 1.23 | 795.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 143.056µs | 10345 | 2000 | 52.61% | 1.23 | 106.66 MB/s |
| Quicksort | 10000 | 247.782µs | 137462 | 0 | 52.59% | 1.23 | 615.82 MB/s |
| Timsort | 10000 | 351.8µs | 141011 | 0 | 52.59% | 1.23 | 433.73 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.250324ms | 0 | 30000 | 52.33% | 1.23 | 24.41 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.249679ms | 0 | 30000 | 52.34% | 1.23 | 24.42 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 507.381µs | 192671 | 14351 | 52.58% | 1.23 | 300.74 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 734.143µs | 125399 | 10000 | 52.59% | 1.23 | 207.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 499.672µs | 125399 | 0 | 52.58% | 1.23 | 305.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 609.508µs | 130052 | 0 | 52.58% | 1.23 | 250.35 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 531.624µs | 109718 | 0 | 52.59% | 1.23 | 287.02 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 466.509µs | 113881 | 0 | 52.57% | 1.23 | 327.08 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 312.235µs | 48812 | 0 | 52.54% | 1.23 | 488.70 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 481.568µs | 125399 | 0 | 52.58% | 1.23 | 316.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 572.263µs | 125399 | 0 | 52.59% | 1.23 | 266.64 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 580.902µs | 125399 | 20000 | 52.58% | 1.23 | 262.67 MB/s |
| Quicksort | 100000 | 3.076369ms | 1710455 | 0 | 52.52% | 1.23 | 496.00 MB/s |
| Timsort | 100000 | 4.352571ms | 1746462 | 0 | 52.47% | 1.23 | 350.57 MB/s |
| ARS Gen 1: Foundation | 100000 | 54.01583ms | 0 | 300000 | 46.91% | 1.22 | 28.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 55.358386ms | 0 | 300000 | 45.80% | 1.21 | 27.56 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.442149ms | 1884751 | 108703 | 52.50% | 1.23 | 443.29 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.535463ms | 1586392 | 100000 | 52.49% | 1.23 | 601.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.858755ms | 1586392 | 0 | 52.49% | 1.23 | 820.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.709309ms | 1629438 | 0 | 52.45% | 1.23 | 563.20 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.292054ms | 1447738 | 0 | 52.50% | 1.23 | 665.73 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.973367ms | 1487078 | 0 | 52.48% | 1.23 | 773.24 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.648137ms | 834504 | 0 | 52.41% | 1.23 | 925.82 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.347597ms | 657220 | 0 | 52.38% | 1.23 | 1132.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.308542ms | 834504 | 0 | 52.41% | 1.23 | 1166.09 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.970543ms | 1586392 | 200000 | 52.50% | 1.23 | 513.67 MB/s |
| Quicksort | 1000000 | 31.241497ms | 20420624 | 0 | 52.15% | 1.26 | 488.41 MB/s |
| Timsort | 1000000 | 49.983626ms | 20810565 | 0 | 51.62% | 1.25 | 305.28 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 34.913085ms | 21491076 | 1017407 | 52.26% | 1.24 | 437.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 24.768642ms | 17729670 | 1000000 | 52.61% | 1.24 | 616.05 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.861864ms | 17729670 | 0 | 52.60% | 1.24 | 667.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 26.95756ms | 18126422 | 0 | 52.11% | 1.24 | 566.03 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.278911ms | 17798278 | 0 | 52.43% | 1.25 | 937.33 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.8011ms | 18171061 | 0 | 52.13% | 1.23 | 699.91 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.429183ms | 9157468 | 0 | 52.63% | 1.22 | 875.47 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.725152ms | 9620349 | 0 | 52.88% | 1.23 | 773.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.503504ms | 11727786 | 0 | 52.97% | 1.23 | 782.36 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 31.236686ms | 19223479 | 2000000 | 52.58% | 1.23 | 488.49 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 19.351µs | 9762 | 0 | 52.31% | 1.24 | 788.53 MB/s |
| Timsort | 1000 | 24.963µs | 9882 | 0 | 52.31% | 1.24 | 611.26 MB/s |
| ARS Gen 1: Foundation | 1000 | 127.232µs | 0 | 2000 | 52.31% | 1.24 | 119.93 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 175.674µs | 0 | 2000 | 52.31% | 1.24 | 86.86 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 18.094µs | 9762 | 0 | 52.31% | 1.24 | 843.31 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.213µs | 9762 | 0 | 52.31% | 1.24 | 837.80 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.427µs | 9762 | 0 | 52.31% | 1.24 | 828.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 24.755µs | 9882 | 0 | 52.31% | 1.24 | 616.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 19.016µs | 9762 | 0 | 52.31% | 1.24 | 802.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 24.805µs | 9882 | 0 | 52.31% | 1.24 | 615.15 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.047µs | 9762 | 0 | 52.31% | 1.24 | 801.11 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 18.112µs | 9762 | 0 | 52.31% | 1.24 | 842.47 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 18.13µs | 9762 | 0 | 52.31% | 1.24 | 841.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 128.489µs | 9762 | 2000 | 52.31% | 1.24 | 118.76 MB/s |
| Quicksort | 10000 | 237.438µs | 134689 | 0 | 52.29% | 1.24 | 642.64 MB/s |
| Timsort | 10000 | 303.645µs | 132195 | 0 | 52.29% | 1.24 | 502.52 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.458162ms | 0 | 30000 | 52.17% | 1.24 | 62.07 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.643928ms | 0 | 30000 | 52.18% | 1.24 | 57.71 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 392.348µs | 187157 | 14351 | 52.28% | 1.24 | 388.91 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 553.897µs | 129133 | 10000 | 52.27% | 1.24 | 275.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 387.64µs | 129133 | 0 | 52.27% | 1.24 | 393.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 449.064µs | 124389 | 0 | 52.27% | 1.24 | 339.79 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 277.294µs | 112273 | 0 | 52.28% | 1.24 | 550.27 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 342.605µs | 109531 | 0 | 52.28% | 1.24 | 445.38 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 251.152µs | 51743 | 0 | 52.25% | 1.23 | 607.55 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 398.053µs | 129133 | 0 | 52.27% | 1.24 | 383.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 441.309µs | 129133 | 0 | 52.27% | 1.24 | 345.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 577.288µs | 129133 | 20000 | 52.26% | 1.24 | 264.32 MB/s |
| Quicksort | 100000 | 2.69109ms | 1716043 | 0 | 52.22% | 1.24 | 567.01 MB/s |
| Timsort | 100000 | 3.285831ms | 1660908 | 0 | 52.15% | 1.24 | 464.38 MB/s |
| ARS Gen 1: Foundation | 100000 | 21.893915ms | 0 | 300000 | 49.96% | 1.24 | 69.69 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 22.495159ms | 0 | 300000 | 50.09% | 1.24 | 67.83 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.19297ms | 1830188 | 108703 | 52.20% | 1.24 | 477.89 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.898098ms | 1653890 | 100000 | 52.18% | 1.24 | 391.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.020358ms | 1653890 | 0 | 52.18% | 1.24 | 505.20 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.689246ms | 1589383 | 0 | 52.12% | 1.24 | 413.60 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.776843ms | 1472393 | 0 | 52.18% | 1.24 | 858.76 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.962781ms | 1387582 | 0 | 52.15% | 1.24 | 777.41 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.305785ms | 815713 | 0 | 52.11% | 1.23 | 1168.55 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.18736ms | 631229 | 0 | 52.10% | 1.24 | 1285.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.444727ms | 815713 | 0 | 52.09% | 1.24 | 1056.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.777763ms | 1653890 | 200000 | 52.14% | 1.24 | 403.91 MB/s |
| Quicksort | 1000000 | 32.662089ms | 20672771 | 0 | 51.93% | 1.26 | 467.17 MB/s |
| Timsort | 1000000 | 43.4548ms | 19775927 | 0 | 51.35% | 1.26 | 351.14 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.594099ms | 20984698 | 1017407 | 52.24% | 1.24 | 468.15 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 26.249979ms | 18442598 | 1000000 | 52.29% | 1.26 | 581.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 21.341777ms | 18442598 | 0 | 52.31% | 1.25 | 714.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 27.115226ms | 17501336 | 0 | 51.73% | 1.25 | 562.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 20.744513ms | 18449113 | 0 | 52.11% | 1.26 | 735.56 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 25.270371ms | 17575612 | 0 | 51.80% | 1.25 | 603.82 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 16.501742ms | 8914015 | 0 | 52.60% | 1.22 | 924.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.770966ms | 9611874 | 0 | 52.66% | 1.23 | 812.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.410514ms | 11855374 | 0 | 52.65% | 1.24 | 828.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.488147ms | 16617922 | 2000000 | 52.43% | 1.23 | 623.11 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 6.618µs | 3735 | 0 | 52.13% | 1.23 | 2305.65 MB/s |
| Timsort | 1000 | 9.277µs | 3747 | 0 | 52.13% | 1.23 | 1644.80 MB/s |
| ARS Gen 1: Foundation | 1000 | 49.461µs | 995 | 2000 | 52.13% | 1.23 | 308.50 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 62.038µs | 995 | 2000 | 52.13% | 1.23 | 245.96 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 6.236µs | 3735 | 0 | 52.13% | 1.23 | 2446.89 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 6.382µs | 3735 | 0 | 52.13% | 1.23 | 2390.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 6.284µs | 3735 | 0 | 52.13% | 1.23 | 2428.20 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 9.196µs | 3747 | 0 | 52.13% | 1.23 | 1659.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 6.549µs | 3735 | 0 | 52.13% | 1.23 | 2329.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 9.428µs | 3747 | 0 | 52.13% | 1.23 | 1618.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 6.654µs | 3735 | 0 | 52.13% | 1.23 | 2293.18 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 6.319µs | 3735 | 0 | 52.13% | 1.23 | 2414.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 6.135µs | 3735 | 0 | 52.13% | 1.23 | 2487.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 165.032µs | 3735 | 2000 | 52.12% | 1.23 | 92.46 MB/s |
| Quicksort | 10000 | 53.847µs | 36573 | 0 | 52.10% | 1.23 | 2833.73 MB/s |
| Timsort | 10000 | 73.812µs | 36775 | 0 | 52.10% | 1.23 | 2067.25 MB/s |
| ARS Gen 1: Foundation | 10000 | 343.054µs | 9995 | 30000 | 52.10% | 1.23 | 444.79 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 381.666µs | 9995 | 30000 | 52.10% | 1.23 | 399.79 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 549.748µs | 115988 | 14351 | 52.10% | 1.23 | 277.56 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 286.907µs | 9999 | 10000 | 52.10% | 1.23 | 531.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 157.371µs | 9999 | 0 | 52.09% | 1.23 | 969.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 159.562µs | 9999 | 0 | 52.09% | 1.23 | 956.29 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 290.54µs | 9999 | 0 | 52.09% | 1.23 | 525.19 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 277.395µs | 9999 | 0 | 52.09% | 1.23 | 550.07 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 153.417µs | 9999 | 0 | 52.09% | 1.23 | 994.60 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 151.518µs | 9999 | 0 | 52.09% | 1.23 | 1007.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 153.554µs | 9999 | 0 | 52.09% | 1.23 | 993.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 415.478µs | 9999 | 20000 | 52.09% | 1.23 | 367.26 MB/s |
| Quicksort | 100000 | 540.295µs | 362094 | 0 | 52.02% | 1.23 | 2824.16 MB/s |
| Timsort | 100000 | 724.114µs | 382517 | 0 | 51.96% | 1.23 | 2107.24 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.29306ms | 99995 | 300000 | 52.03% | 1.23 | 1180.05 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.61851ms | 99995 | 300000 | 52.02% | 1.23 | 942.77 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.411557ms | 1129938 | 108703 | 52.02% | 1.23 | 632.74 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 695.35µs | 100001 | 100000 | 51.99% | 1.23 | 2194.40 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 389.475µs | 100001 | 0 | 51.97% | 1.23 | 3917.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 390.578µs | 100001 | 0 | 51.97% | 1.23 | 3906.72 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 576.066µs | 100001 | 0 | 51.98% | 1.23 | 2648.79 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 641.984µs | 100001 | 0 | 51.98% | 1.23 | 2376.82 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 648.51µs | 199996 | 0 | 51.95% | 1.23 | 2352.90 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 634.498µs | 199996 | 0 | 51.97% | 1.23 | 2404.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 677.192µs | 199996 | 0 | 51.95% | 1.23 | 2253.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 848.907µs | 100001 | 200000 | 51.99% | 1.23 | 1797.46 MB/s |
| Quicksort | 1000000 | 4.684794ms | 3809528 | 0 | 51.83% | 1.23 | 3257.09 MB/s |
| Timsort | 1000000 | 9.393169ms | 4510660 | 0 | 51.77% | 1.23 | 1624.46 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.188517ms | 12062959 | 1017407 | 51.99% | 1.23 | 582.65 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.766969ms | 999999 | 1000000 | 52.27% | 1.21 | 2254.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.471486ms | 999999 | 0 | 52.28% | 1.22 | 2042.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.389176ms | 999999 | 0 | 52.29% | 1.21 | 2065.02 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.953779ms | 999999 | 0 | 52.09% | 1.22 | 3080.23 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.896792ms | 999999 | 0 | 52.09% | 1.23 | 3116.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.626808ms | 1999994 | 0 | 52.33% | 1.21 | 1585.03 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.920674ms | 1999994 | 0 | 52.22% | 1.22 | 1926.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.696097ms | 1999994 | 0 | 52.29% | 1.21 | 1754.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 21.51652ms | 5364815 | 2000000 | 52.18% | 1.23 | 709.17 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 8.772µs | 5508 | 0 | 51.90% | 1.24 | 1739.49 MB/s |
| Timsort | 1000 | 12.103µs | 5460 | 0 | 51.90% | 1.24 | 1260.74 MB/s |
| ARS Gen 1: Foundation | 1000 | 50.304µs | 921 | 2000 | 51.90% | 1.24 | 303.33 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 55.451µs | 921 | 2000 | 51.90% | 1.24 | 275.18 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 8.504µs | 5508 | 0 | 51.90% | 1.24 | 1794.31 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 8.309µs | 5508 | 0 | 51.90% | 1.24 | 1836.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 8.505µs | 5508 | 0 | 51.90% | 1.24 | 1794.10 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 12.562µs | 5460 | 0 | 51.90% | 1.24 | 1214.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 8.632µs | 5508 | 0 | 51.90% | 1.24 | 1767.70 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 12.095µs | 5460 | 0 | 51.90% | 1.24 | 1261.58 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 8.746µs | 5508 | 0 | 51.90% | 1.24 | 1744.66 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 8.435µs | 5508 | 0 | 51.90% | 1.24 | 1808.99 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 8.248µs | 5508 | 0 | 51.90% | 1.24 | 1850.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 108.671µs | 5508 | 2000 | 51.90% | 1.24 | 140.41 MB/s |
| Quicksort | 10000 | 71.82µs | 53621 | 0 | 51.88% | 1.24 | 2124.59 MB/s |
| Timsort | 10000 | 99.486µs | 53742 | 0 | 51.88% | 1.24 | 1533.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 306.548µs | 9683 | 30000 | 51.87% | 1.24 | 497.76 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 357.358µs | 9683 | 30000 | 51.87% | 1.24 | 426.99 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 315.802µs | 124917 | 14351 | 51.88% | 1.24 | 483.18 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 216.441µs | 10961 | 10000 | 51.86% | 1.24 | 704.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 167.143µs | 10961 | 0 | 51.87% | 1.24 | 912.92 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 144.976µs | 11013 | 0 | 51.87% | 1.24 | 1052.50 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 322.358µs | 14023 | 0 | 51.87% | 1.24 | 473.35 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 345.432µs | 14226 | 0 | 51.87% | 1.24 | 441.73 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 169.924µs | 15115 | 0 | 51.87% | 1.24 | 897.98 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 146.202µs | 10961 | 0 | 51.87% | 1.24 | 1043.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 142.193µs | 10961 | 0 | 51.86% | 1.24 | 1073.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 324.73µs | 10961 | 20000 | 51.86% | 1.24 | 469.89 MB/s |
| Quicksort | 100000 | 718.656µs | 532062 | 0 | 51.80% | 1.24 | 2123.24 MB/s |
| Timsort | 100000 | 966.878µs | 535405 | 0 | 51.75% | 1.24 | 1578.15 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.960222ms | 98733 | 300000 | 51.78% | 1.24 | 778.42 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.165413ms | 98733 | 300000 | 51.77% | 1.23 | 704.66 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.478835ms | 1174310 | 108703 | 51.79% | 1.24 | 615.56 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 668.77µs | 122228 | 100000 | 51.76% | 1.23 | 2281.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 436.882µs | 122228 | 0 | 51.75% | 1.24 | 3492.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 475.59µs | 122352 | 0 | 51.75% | 1.23 | 3208.39 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 661.626µs | 151812 | 0 | 51.77% | 1.24 | 2306.26 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 807.66µs | 152487 | 0 | 51.77% | 1.24 | 1889.26 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 653.87µs | 192482 | 0 | 51.75% | 1.23 | 2333.61 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 732.994µs | 182525 | 0 | 51.75% | 1.24 | 2081.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 755.714µs | 186875 | 0 | 51.75% | 1.23 | 2019.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 830.235µs | 122228 | 200000 | 51.74% | 1.24 | 1837.89 MB/s |
| Quicksort | 1000000 | 7.126035ms | 5301519 | 0 | 51.60% | 1.24 | 2141.27 MB/s |
| Timsort | 1000000 | 12.704777ms | 6302942 | 0 | 51.40% | 1.24 | 1201.03 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.222824ms | 12308876 | 1017407 | 51.86% | 1.24 | 560.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.867235ms | 1094612 | 1000000 | 52.07% | 1.22 | 2221.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.357503ms | 1094612 | 0 | 52.07% | 1.22 | 2400.12 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.650127ms | 1095552 | 0 | 52.07% | 1.22 | 2294.51 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.995212ms | 1539866 | 0 | 51.96% | 1.23 | 3054.68 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.391495ms | 1544970 | 0 | 51.95% | 1.23 | 2830.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.984544ms | 1989097 | 0 | 52.12% | 1.22 | 1528.24 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.329629ms | 2062330 | 0 | 52.06% | 1.23 | 1831.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.400155ms | 2113182 | 0 | 52.12% | 1.22 | 1623.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 21.656654ms | 5807618 | 2000000 | 51.57% | 1.24 | 704.58 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.056µs | 10241 | 0 | 51.82% | 1.24 | 845.08 MB/s |
| Timsort | 1000 | 26.206µs | 10555 | 0 | 51.82% | 1.24 | 582.26 MB/s |
| ARS Gen 1: Foundation | 1000 | 233.692µs | 0 | 2000 | 51.82% | 1.24 | 65.29 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 278.396µs | 0 | 2000 | 51.82% | 1.24 | 54.81 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.041µs | 10241 | 0 | 51.82% | 1.24 | 895.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.243µs | 10241 | 0 | 51.82% | 1.24 | 884.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.149µs | 10241 | 0 | 51.82% | 1.24 | 889.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.825µs | 10555 | 0 | 51.82% | 1.24 | 568.83 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.056µs | 10241 | 0 | 51.82% | 1.24 | 845.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 26.363µs | 10555 | 0 | 51.82% | 1.24 | 578.80 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.905µs | 10241 | 0 | 51.82% | 1.24 | 852.21 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.103µs | 10241 | 0 | 51.82% | 1.24 | 892.17 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.933µs | 10241 | 0 | 51.82% | 1.24 | 901.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 126.011µs | 10241 | 2000 | 51.82% | 1.24 | 121.09 MB/s |
| Quicksort | 10000 | 226.982µs | 137603 | 0 | 51.80% | 1.24 | 672.25 MB/s |
| Timsort | 10000 | 319.097µs | 140916 | 0 | 51.80% | 1.24 | 478.19 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.860681ms | 0 | 30000 | 51.58% | 1.24 | 26.04 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.095475ms | 0 | 30000 | 51.57% | 1.24 | 25.03 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 408.539µs | 192365 | 14351 | 51.79% | 1.23 | 373.50 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 278.913µs | 66763 | 10000 | 51.78% | 1.23 | 547.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 231.723µs | 66763 | 0 | 51.79% | 1.23 | 658.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 186.156µs | 69738 | 0 | 51.78% | 1.23 | 819.68 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 234.335µs | 61211 | 0 | 51.78% | 1.23 | 651.15 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 299.536µs | 64630 | 0 | 51.79% | 1.23 | 509.41 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 183.54µs | 66763 | 0 | 51.78% | 1.23 | 831.36 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 184.858µs | 66763 | 0 | 51.78% | 1.23 | 825.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 176.353µs | 66763 | 0 | 51.78% | 1.23 | 865.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 382.375µs | 66763 | 20000 | 51.77% | 1.23 | 399.05 MB/s |
| Quicksort | 100000 | 2.891746ms | 1710395 | 0 | 51.72% | 1.24 | 527.67 MB/s |
| Timsort | 100000 | 3.79186ms | 1746952 | 0 | 51.67% | 1.24 | 402.41 MB/s |
| ARS Gen 1: Foundation | 100000 | 49.946102ms | 0 | 300000 | 45.47% | 1.21 | 30.55 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 56.482297ms | 0 | 300000 | 45.58% | 1.22 | 27.02 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.914734ms | 1885598 | 108703 | 51.71% | 1.24 | 389.78 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.052147ms | 1045510 | 100000 | 51.72% | 1.23 | 743.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.488744ms | 1045510 | 0 | 51.71% | 1.23 | 1024.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.828025ms | 1086813 | 0 | 51.71% | 1.23 | 834.71 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.56503ms | 981284 | 0 | 51.70% | 1.24 | 974.98 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.798045ms | 1020457 | 0 | 51.70% | 1.23 | 848.63 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.35142ms | 1045510 | 0 | 51.68% | 1.23 | 1129.09 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.447837ms | 936769 | 0 | 51.70% | 1.23 | 1053.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.195779ms | 1045510 | 0 | 51.69% | 1.23 | 1276.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.527786ms | 1045510 | 200000 | 51.67% | 1.23 | 998.75 MB/s |
| Quicksort | 1000000 | 32.370076ms | 20431039 | 0 | 51.44% | 1.26 | 471.39 MB/s |
| Timsort | 1000000 | 52.460985ms | 20806652 | 0 | 51.05% | 1.25 | 290.86 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 34.418392ms | 21500526 | 1017407 | 51.41% | 1.24 | 443.33 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 12.945216ms | 12082942 | 1000000 | 51.93% | 1.23 | 1178.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.568743ms | 12082942 | 0 | 51.92% | 1.23 | 1318.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.097563ms | 12502702 | 0 | 51.93% | 1.23 | 1010.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 10.141511ms | 13210593 | 0 | 51.79% | 1.24 | 1504.59 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.441929ms | 13633723 | 0 | 51.73% | 1.24 | 1226.40 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 11.954727ms | 7768109 | 0 | 51.74% | 1.23 | 1276.38 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 13.420937ms | 6294342 | 0 | 51.65% | 1.23 | 1136.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.210123ms | 7082878 | 0 | 51.64% | 1.23 | 1155.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 26.996242ms | 13879903 | 2000000 | 51.89% | 1.22 | 565.22 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 20.076µs | 10551 | 0 | 51.94% | 1.23 | 760.05 MB/s |
| Timsort | 1000 | 29.712µs | 10537 | 0 | 51.94% | 1.23 | 513.56 MB/s |
| ARS Gen 1: Foundation | 1000 | 255.266µs | 0 | 2000 | 51.94% | 1.23 | 59.78 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 306.142µs | 0 | 2000 | 51.93% | 1.23 | 49.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 19.084µs | 10551 | 0 | 51.94% | 1.23 | 799.56 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.855µs | 10551 | 0 | 51.94% | 1.23 | 809.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.745µs | 10551 | 0 | 51.94% | 1.23 | 814.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 28.469µs | 10537 | 0 | 51.94% | 1.23 | 535.98 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 19.643µs | 10551 | 0 | 51.94% | 1.23 | 776.81 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.958µs | 10537 | 0 | 51.94% | 1.23 | 545.78 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.64µs | 10551 | 0 | 51.94% | 1.23 | 776.92 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 18.853µs | 10551 | 0 | 51.94% | 1.23 | 809.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 18.701µs | 10551 | 0 | 51.94% | 1.23 | 815.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 129.749µs | 10551 | 2000 | 51.93% | 1.23 | 117.60 MB/s |
| Quicksort | 10000 | 238.243µs | 136744 | 0 | 51.92% | 1.23 | 640.47 MB/s |
| Timsort | 10000 | 342.582µs | 140772 | 0 | 51.91% | 1.23 | 445.41 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.106938ms | 0 | 30000 | 51.69% | 1.23 | 24.99 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.074164ms | 0 | 30000 | 51.70% | 1.23 | 25.12 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 431.197µs | 193085 | 14351 | 51.90% | 1.23 | 353.87 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 454.226µs | 126000 | 10000 | 51.90% | 1.23 | 335.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 334.112µs | 126000 | 0 | 51.90% | 1.23 | 456.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 399.617µs | 130426 | 0 | 51.90% | 1.23 | 381.84 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 448.574µs | 118809 | 0 | 51.90% | 1.23 | 340.16 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 610.739µs | 123101 | 0 | 51.90% | 1.23 | 249.84 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 481.089µs | 90031 | 0 | 51.90% | 1.23 | 317.17 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 445.383µs | 126000 | 0 | 51.90% | 1.23 | 342.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 401.248µs | 126000 | 0 | 51.90% | 1.23 | 380.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 616.144µs | 126000 | 20000 | 51.90% | 1.23 | 247.65 MB/s |
| Quicksort | 100000 | 3.385017ms | 1704961 | 0 | 51.88% | 1.23 | 450.77 MB/s |
| Timsort | 100000 | 4.830981ms | 1748322 | 0 | 51.85% | 1.23 | 315.85 MB/s |
| ARS Gen 1: Foundation | 100000 | 68.279626ms | 0 | 300000 | 47.17% | 1.21 | 22.35 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 50.851568ms | 0 | 300000 | 46.20% | 1.21 | 30.01 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.338922ms | 1885129 | 108703 | 51.86% | 1.23 | 457.00 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.485526ms | 1618379 | 100000 | 51.84% | 1.23 | 613.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.713487ms | 1618379 | 0 | 51.85% | 1.23 | 890.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.45616ms | 1658575 | 0 | 51.82% | 1.23 | 621.25 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.733756ms | 1529988 | 0 | 51.85% | 1.23 | 880.10 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.120342ms | 1573500 | 0 | 51.83% | 1.23 | 719.64 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.340487ms | 673827 | 0 | 51.74% | 1.23 | 1138.30 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.487911ms | 837701 | 0 | 51.79% | 1.23 | 1025.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.636249ms | 967480 | 0 | 51.76% | 1.23 | 932.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.183955ms | 1618379 | 200000 | 51.81% | 1.23 | 698.68 MB/s |
| Quicksort | 1000000 | 34.265365ms | 20435426 | 0 | 51.54% | 1.25 | 445.31 MB/s |
| Timsort | 1000000 | 42.732156ms | 20818465 | 0 | 50.96% | 1.24 | 357.08 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.408984ms | 21488833 | 1017407 | 51.47% | 1.23 | 470.82 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 26.150877ms | 19275700 | 1000000 | 51.79% | 1.24 | 583.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 19.430232ms | 19275700 | 0 | 51.86% | 1.23 | 785.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 28.723335ms | 19658200 | 0 | 51.48% | 1.23 | 531.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 19.158668ms | 19275700 | 0 | 51.70% | 1.24 | 796.44 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 24.122765ms | 19658200 | 0 | 51.39% | 1.23 | 632.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 18.648769ms | 6708948 | 0 | 51.83% | 1.20 | 818.22 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.857525ms | 6379291 | 0 | 52.07% | 1.21 | 768.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.812134ms | 8360463 | 0 | 52.18% | 1.21 | 856.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.212138ms | 20819820 | 2000000 | 51.63% | 1.23 | 505.05 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.495µs | 10288 | 0 | 51.54% | 1.24 | 825.02 MB/s |
| Timsort | 1000 | 27.388µs | 10450 | 0 | 51.54% | 1.24 | 557.13 MB/s |
| ARS Gen 1: Foundation | 1000 | 236.981µs | 0 | 2000 | 51.54% | 1.24 | 64.39 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 279.045µs | 0 | 2000 | 51.54% | 1.24 | 54.68 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.272µs | 10288 | 0 | 51.54% | 1.24 | 883.44 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.519µs | 10288 | 0 | 51.54% | 1.24 | 870.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.707µs | 10288 | 0 | 51.54% | 1.24 | 861.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.354µs | 10450 | 0 | 51.54% | 1.24 | 557.83 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.366µs | 10288 | 0 | 51.54% | 1.24 | 830.82 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.161µs | 10450 | 0 | 51.54% | 1.24 | 523.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.591µs | 10288 | 0 | 51.54% | 1.24 | 820.76 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.542µs | 10288 | 0 | 51.54% | 1.24 | 869.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.646µs | 10288 | 0 | 51.54% | 1.24 | 864.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 124.307µs | 10288 | 2000 | 51.53% | 1.24 | 122.75 MB/s |
| Quicksort | 10000 | 218.092µs | 136714 | 0 | 51.52% | 1.24 | 699.65 MB/s |
| Timsort | 10000 | 308.411µs | 140903 | 0 | 51.52% | 1.24 | 494.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.564554ms | 160 | 30000 | 51.26% | 1.24 | 27.42 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.696292ms | 160 | 30000 | 51.25% | 1.24 | 26.79 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 411.626µs | 193162 | 14351 | 51.51% | 1.24 | 370.70 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 254.1µs | 52333 | 10000 | 51.50% | 1.24 | 600.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 155.154µs | 52333 | 0 | 51.50% | 1.24 | 983.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 172.067µs | 57763 | 0 | 51.50% | 1.24 | 886.79 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 155.387µs | 59128 | 0 | 51.50% | 1.24 | 981.99 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 225.887µs | 62231 | 0 | 51.50% | 1.24 | 675.51 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 173.195µs | 52333 | 0 | 51.50% | 1.24 | 881.02 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 166.855µs | 52333 | 0 | 51.50% | 1.24 | 914.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 167.227µs | 52333 | 0 | 51.50% | 1.24 | 912.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 367.671µs | 52333 | 20000 | 51.50% | 1.24 | 415.01 MB/s |
| Quicksort | 100000 | 2.67402ms | 1706033 | 0 | 51.44% | 1.24 | 570.63 MB/s |
| Timsort | 100000 | 3.599678ms | 1748408 | 0 | 51.39% | 1.24 | 423.89 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.755597ms | 15822 | 300000 | 45.65% | 1.22 | 34.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 48.209973ms | 15822 | 300000 | 45.35% | 1.22 | 31.65 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.007117ms | 1885784 | 108703 | 51.42% | 1.24 | 507.42 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.107282ms | 882348 | 100000 | 51.42% | 1.24 | 1378.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 891.956µs | 882348 | 0 | 51.41% | 1.24 | 1710.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.046303ms | 921462 | 0 | 51.41% | 1.24 | 1458.35 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 972.44µs | 940138 | 0 | 51.41% | 1.24 | 1569.12 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.096216ms | 976280 | 0 | 51.41% | 1.24 | 1391.95 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 871.711µs | 882348 | 0 | 51.39% | 1.24 | 1750.44 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 873.821µs | 771432 | 0 | 51.41% | 1.24 | 1746.21 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 874.165µs | 882348 | 0 | 51.42% | 1.24 | 1745.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.214139ms | 882348 | 200000 | 51.42% | 1.24 | 1256.76 MB/s |
| Quicksort | 1000000 | 30.223975ms | 20389196 | 0 | 51.05% | 1.26 | 504.86 MB/s |
| Timsort | 1000000 | 43.697413ms | 20780417 | 0 | 50.55% | 1.25 | 349.19 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 28.506508ms | 21441825 | 1017407 | 51.14% | 1.24 | 535.27 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.975302ms | 10157321 | 1000000 | 51.50% | 1.23 | 1529.66 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.495764ms | 10157321 | 0 | 51.50% | 1.23 | 1606.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.567703ms | 10561958 | 0 | 51.53% | 1.23 | 1319.09 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.915343ms | 12859603 | 0 | 51.47% | 1.24 | 1711.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 10.770132ms | 13271645 | 0 | 51.41% | 1.24 | 1416.77 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.550314ms | 10157321 | 0 | 51.42% | 1.23 | 1597.73 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.684771ms | 11214454 | 0 | 51.52% | 1.24 | 1428.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.518072ms | 12268914 | 0 | 51.54% | 1.23 | 1450.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.311806ms | 13658791 | 2000000 | 51.13% | 1.24 | 503.39 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 8.236µs | 5797 | 0 | 51.66% | 1.24 | 1852.69 MB/s |
| Timsort | 1000 | 11.352µs | 5499 | 0 | 51.66% | 1.24 | 1344.15 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.326µs | 984 | 2000 | 51.66% | 1.24 | 322.42 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.363µs | 984 | 2000 | 51.66% | 1.24 | 261.45 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 7.77µs | 5797 | 0 | 51.66% | 1.24 | 1963.81 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 8.189µs | 5797 | 0 | 51.66% | 1.24 | 1863.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 7.81µs | 5797 | 0 | 51.66% | 1.24 | 1953.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.426µs | 5499 | 0 | 51.66% | 1.24 | 1335.44 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 8.017µs | 5797 | 0 | 51.66% | 1.24 | 1903.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 11.619µs | 5499 | 0 | 51.66% | 1.24 | 1313.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 8.225µs | 5797 | 0 | 51.66% | 1.24 | 1855.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 7.823µs | 5797 | 0 | 51.66% | 1.24 | 1950.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.826µs | 5797 | 0 | 51.66% | 1.24 | 1949.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 114.478µs | 5797 | 2000 | 51.66% | 1.24 | 133.29 MB/s |
| Quicksort | 10000 | 67.777µs | 53838 | 0 | 51.65% | 1.24 | 2251.32 MB/s |
| Timsort | 10000 | 92.51µs | 53843 | 0 | 51.64% | 1.24 | 1649.42 MB/s |
| ARS Gen 1: Foundation | 10000 | 244.897µs | 9984 | 30000 | 51.64% | 1.24 | 623.07 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 315.551µs | 9984 | 30000 | 51.64% | 1.24 | 483.56 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 308.352µs | 122148 | 14351 | 51.64% | 1.24 | 494.85 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 211.317µs | 12061 | 10000 | 51.63% | 1.24 | 722.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 129.304µs | 12061 | 0 | 51.63% | 1.24 | 1180.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 127.702µs | 12085 | 0 | 51.63% | 1.24 | 1194.87 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 212.564µs | 12061 | 0 | 51.63% | 1.24 | 717.84 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 197.394µs | 12085 | 0 | 51.63% | 1.24 | 773.01 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 130.869µs | 12061 | 0 | 51.63% | 1.24 | 1165.96 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 124.882µs | 12061 | 0 | 51.63% | 1.24 | 1221.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 129.125µs | 12061 | 0 | 51.63% | 1.24 | 1181.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 312.231µs | 12061 | 20000 | 51.62% | 1.24 | 488.70 MB/s |
| Quicksort | 100000 | 664.237µs | 529379 | 0 | 51.58% | 1.24 | 2297.19 MB/s |
| Timsort | 100000 | 890.798µs | 529674 | 0 | 51.53% | 1.24 | 1712.93 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.432904ms | 99984 | 300000 | 51.57% | 1.24 | 1064.89 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.567749ms | 99984 | 300000 | 51.57% | 1.24 | 973.29 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.173217ms | 1143461 | 108703 | 51.56% | 1.24 | 702.13 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 487.028µs | 151116 | 100000 | 51.54% | 1.24 | 3133.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 419.133µs | 151116 | 0 | 51.54% | 1.24 | 3640.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 356.778µs | 151622 | 0 | 51.54% | 1.24 | 4276.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 529.368µs | 99988 | 0 | 51.55% | 1.24 | 2882.45 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 494.089µs | 99988 | 0 | 51.55% | 1.24 | 3088.27 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 667.86µs | 199984 | 0 | 51.52% | 1.24 | 2284.73 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 599.946µs | 199972 | 0 | 51.54% | 1.24 | 2543.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 482.02µs | 100000 | 0 | 51.53% | 1.24 | 3165.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 915.985µs | 151116 | 200000 | 51.54% | 1.24 | 1665.83 MB/s |
| Quicksort | 1000000 | 6.039551ms | 5138620 | 0 | 51.30% | 1.25 | 2526.48 MB/s |
| Timsort | 1000000 | 10.386311ms | 6175006 | 0 | 51.04% | 1.24 | 1469.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 25.026481ms | 12087538 | 1017407 | 51.47% | 1.24 | 609.71 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.205436ms | 999988 | 1000000 | 51.78% | 1.23 | 2458.94 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.188361ms | 999988 | 0 | 51.78% | 1.23 | 2465.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.204424ms | 999988 | 0 | 51.78% | 1.23 | 2459.34 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.170091ms | 999988 | 0 | 51.64% | 1.24 | 3659.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.185425ms | 999988 | 0 | 51.65% | 1.24 | 3645.70 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.89132ms | 1999972 | 0 | 51.73% | 1.23 | 1933.62 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.22439ms | 1999972 | 0 | 51.69% | 1.24 | 2112.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.900142ms | 1999976 | 0 | 51.75% | 1.23 | 1931.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.753752ms | 5717339 | 2000000 | 51.19% | 1.25 | 772.45 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 16.301µs | 10288 | 0 | 51.41% | 1.24 | 936.06 MB/s |
| Timsort | 1000 | 42.797µs | 10450 | 0 | 51.41% | 1.24 | 356.54 MB/s |
| ARS Gen 1: Foundation | 1000 | 405.431µs | 0 | 2000 | 51.41% | 1.24 | 37.64 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 502.572µs | 0 | 2000 | 51.41% | 1.24 | 30.36 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 35.896µs | 10288 | 0 | 51.41% | 1.24 | 425.08 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 35.224µs | 10288 | 0 | 51.41% | 1.24 | 433.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 36.291µs | 10288 | 0 | 51.41% | 1.24 | 420.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 49.965µs | 10450 | 0 | 51.41% | 1.24 | 305.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 36.383µs | 10288 | 0 | 51.41% | 1.24 | 419.39 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 48.428µs | 10450 | 0 | 51.41% | 1.24 | 315.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 35.065µs | 10288 | 0 | 51.41% | 1.24 | 435.16 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 33.741µs | 10288 | 0 | 51.41% | 1.24 | 452.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 21.497µs | 10288 | 0 | 51.41% | 1.24 | 709.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 197.525µs | 10288 | 2000 | 51.40% | 1.24 | 77.25 MB/s |
| Quicksort | 10000 | 289.145µs | 136714 | 0 | 51.39% | 1.24 | 527.72 MB/s |
| Timsort | 10000 | 404.409µs | 140903 | 0 | 51.39% | 1.24 | 377.31 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.16235ms | 160 | 30000 | 51.15% | 1.24 | 29.56 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.226551ms | 160 | 30000 | 51.13% | 1.24 | 29.19 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 366.555µs | 193162 | 14351 | 51.38% | 1.24 | 416.28 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 234.128µs | 52333 | 10000 | 51.38% | 1.24 | 651.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 148.521µs | 52333 | 0 | 51.37% | 1.24 | 1027.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.098µs | 57763 | 0 | 51.37% | 1.24 | 953.09 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 136.562µs | 59128 | 0 | 51.38% | 1.24 | 1117.35 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 149.733µs | 62231 | 0 | 51.38% | 1.24 | 1019.07 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 157.71µs | 52333 | 0 | 51.37% | 1.24 | 967.52 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 151.416µs | 52333 | 0 | 51.37% | 1.24 | 1007.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 156.566µs | 52333 | 0 | 51.37% | 1.24 | 974.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 328.438µs | 52333 | 20000 | 51.37% | 1.24 | 464.59 MB/s |
| Quicksort | 100000 | 2.875078ms | 1706033 | 0 | 51.31% | 1.24 | 530.73 MB/s |
| Timsort | 100000 | 3.780076ms | 1748408 | 0 | 51.26% | 1.24 | 403.66 MB/s |
| ARS Gen 1: Foundation | 100000 | 42.556531ms | 15822 | 300000 | 46.07% | 1.22 | 35.86 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.875548ms | 15822 | 300000 | 45.84% | 1.22 | 33.26 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.973691ms | 1885784 | 108703 | 51.29% | 1.24 | 513.13 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.089942ms | 882348 | 100000 | 51.28% | 1.24 | 1399.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 779.509µs | 882348 | 0 | 51.27% | 1.24 | 1957.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 957.833µs | 921462 | 0 | 51.27% | 1.24 | 1593.05 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 990.08µs | 940138 | 0 | 51.28% | 1.24 | 1541.17 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.124469ms | 976280 | 0 | 51.28% | 1.24 | 1356.98 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 806.976µs | 882348 | 0 | 51.26% | 1.24 | 1890.86 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 870.474µs | 771432 | 0 | 51.27% | 1.24 | 1752.93 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 842.337µs | 882348 | 0 | 51.27% | 1.24 | 1811.48 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.26208ms | 882348 | 200000 | 51.28% | 1.24 | 1209.02 MB/s |
| Quicksort | 1000000 | 30.50708ms | 20389196 | 0 | 50.91% | 1.26 | 500.17 MB/s |
| Timsort | 1000000 | 44.972209ms | 20780417 | 0 | 50.45% | 1.25 | 339.29 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 28.592911ms | 21441825 | 1017407 | 50.99% | 1.24 | 533.66 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.77474ms | 10157321 | 1000000 | 51.33% | 1.23 | 1561.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.514721ms | 10157321 | 0 | 51.33% | 1.23 | 1603.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.871375ms | 10561958 | 0 | 51.32% | 1.23 | 1403.57 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.876651ms | 12859603 | 0 | 51.29% | 1.24 | 1718.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 10.899087ms | 13271645 | 0 | 51.25% | 1.24 | 1400.01 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.963122ms | 10157321 | 0 | 51.29% | 1.23 | 1531.53 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.745028ms | 11214454 | 0 | 51.36% | 1.24 | 1420.08 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.728193ms | 12268914 | 0 | 51.37% | 1.24 | 1422.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.592553ms | 13561646 | 2000000 | 51.12% | 1.24 | 498.77 MB/s |

## Category: String

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 61.911µs | 10370 | 0 | 51.70% | 1.24 | 985.85 MB/s |
| Timsort | 1000 | 75.645µs | 10522 | 0 | 51.70% | 1.24 | 806.86 MB/s |
| ARS Gen 1: Foundation | 1000 | 328.278µs | 0 | 2000 | 51.70% | 1.24 | 185.93 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 374.301µs | 0 | 2000 | 51.70% | 1.24 | 163.06 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 65.503µs | 10370 | 0 | 51.70% | 1.24 | 931.79 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 67.177µs | 10370 | 0 | 51.70% | 1.24 | 908.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 68.567µs | 10370 | 0 | 51.70% | 1.24 | 890.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 76.727µs | 10522 | 0 | 51.70% | 1.24 | 795.48 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 66.375µs | 10370 | 0 | 51.70% | 1.24 | 919.55 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 74.228µs | 10522 | 0 | 51.70% | 1.24 | 822.27 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 63.983µs | 10370 | 0 | 51.70% | 1.24 | 953.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 66.066µs | 10370 | 0 | 51.70% | 1.24 | 923.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 65.889µs | 10370 | 0 | 51.70% | 1.24 | 926.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 249.379µs | 10370 | 2000 | 51.70% | 1.24 | 244.75 MB/s |
| Quicksort | 10000 | 823.684µs | 136866 | 0 | 51.66% | 1.24 | 741.00 MB/s |
| Timsort | 10000 | 972.578µs | 141490 | 0 | 51.64% | 1.24 | 627.56 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.813318ms | 0 | 30000 | 51.42% | 1.25 | 89.58 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.991915ms | 0 | 30000 | 51.47% | 1.25 | 87.29 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.308737ms | 193846 | 14351 | 51.64% | 1.24 | 264.37 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 759.529µs | 67438 | 10000 | 51.66% | 1.24 | 803.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 255.689µs | 67438 | 0 | 51.66% | 1.24 | 2387.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 268.49µs | 70298 | 0 | 51.65% | 1.24 | 2273.27 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 407.553µs | 63043 | 0 | 51.65% | 1.24 | 1497.60 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 425.653µs | 67007 | 0 | 51.65% | 1.24 | 1433.92 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 282.04µs | 67438 | 0 | 51.66% | 1.24 | 2164.06 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 279.154µs | 67438 | 0 | 51.66% | 1.24 | 2186.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 274.616µs | 67438 | 0 | 51.66% | 1.24 | 2222.56 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.279091ms | 67438 | 20000 | 51.62% | 1.24 | 477.18 MB/s |
| Quicksort | 100000 | 11.799953ms | 1718762 | 0 | 51.05% | 1.25 | 517.25 MB/s |
| Timsort | 100000 | 14.329178ms | 1759891 | 0 | 50.77% | 1.25 | 425.95 MB/s |
| ARS Gen 1: Foundation | 100000 | 57.450311ms | 0 | 300000 | 46.37% | 1.23 | 106.24 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.372759ms | 0 | 300000 | 45.73% | 1.23 | 99.45 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 18.195882ms | 1895222 | 108703 | 51.51% | 1.25 | 335.43 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.055214ms | 1029722 | 100000 | 51.50% | 1.24 | 1505.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.886339ms | 1029722 | 0 | 51.50% | 1.24 | 2114.62 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.325449ms | 1071423 | 0 | 51.44% | 1.24 | 1835.40 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.199232ms | 978520 | 0 | 51.46% | 1.24 | 1907.81 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.479895ms | 1019338 | 0 | 51.45% | 1.24 | 1753.94 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.220623ms | 1029722 | 0 | 51.47% | 1.24 | 1895.14 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.25788ms | 961965 | 0 | 51.52% | 1.24 | 1873.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.168479ms | 1029722 | 0 | 51.47% | 1.24 | 1926.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 11.188809ms | 1029722 | 200000 | 51.16% | 1.24 | 545.50 MB/s |
| Quicksort | 1000000 | 273.947403ms | 20518628 | 0 | 52.24% | 1.26 | 222.80 MB/s |
| Timsort | 1000000 | 341.91344ms | 20902099 | 0 | 50.95% | 1.25 | 178.51 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 198.118238ms | 21589743 | 1017407 | 51.12% | 1.24 | 308.07 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.018002ms | 12256776 | 1000000 | 52.00% | 1.17 | 1016.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.45465ms | 12256776 | 0 | 52.20% | 1.19 | 1259.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 56.196275ms | 12679336 | 0 | 51.75% | 1.18 | 1086.11 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 60.309412ms | 13331493 | 0 | 52.39% | 1.18 | 1012.03 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 74.459906ms | 13750405 | 0 | 51.98% | 1.16 | 819.70 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.719068ms | 9434971 | 0 | 52.52% | 1.18 | 1157.74 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 68.927248ms | 7913738 | 0 | 53.46% | 1.16 | 885.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.864439ms | 8719170 | 0 | 52.99% | 1.17 | 1054.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 424.288845ms | 15199655 | 2000000 | 54.98% | 1.20 | 143.85 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 51.307µs | 10370 | 0 | 50.93% | 1.23 | 1189.61 MB/s |
| Timsort | 1000 | 60.957µs | 10522 | 0 | 50.93% | 1.23 | 1001.28 MB/s |
| ARS Gen 1: Foundation | 1000 | 267.966µs | 0 | 2000 | 50.93% | 1.23 | 227.77 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 310.378µs | 0 | 2000 | 50.93% | 1.23 | 196.65 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 64.034µs | 10370 | 0 | 50.93% | 1.23 | 953.17 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 57.637µs | 10370 | 0 | 50.93% | 1.23 | 1058.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 58.428µs | 10370 | 0 | 50.93% | 1.23 | 1044.62 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 62.299µs | 10522 | 0 | 50.93% | 1.23 | 979.71 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 57.285µs | 10370 | 0 | 50.93% | 1.23 | 1065.46 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 64.252µs | 10522 | 0 | 50.93% | 1.23 | 949.93 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 55.895µs | 10370 | 0 | 50.93% | 1.23 | 1091.96 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 54.452µs | 10370 | 0 | 50.93% | 1.23 | 1120.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 56.672µs | 10370 | 0 | 50.93% | 1.23 | 1076.99 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 229.756µs | 10370 | 2000 | 50.92% | 1.23 | 265.65 MB/s |
| Quicksort | 10000 | 719.226µs | 136866 | 0 | 50.89% | 1.23 | 848.62 MB/s |
| Timsort | 10000 | 847.738µs | 141490 | 0 | 50.88% | 1.23 | 719.98 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.551174ms | 0 | 30000 | 50.74% | 1.24 | 109.95 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.880972ms | 0 | 30000 | 50.74% | 1.24 | 103.78 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.377486ms | 193846 | 14351 | 50.88% | 1.23 | 256.72 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 667.498µs | 67438 | 10000 | 50.88% | 1.23 | 914.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 225.853µs | 67438 | 0 | 50.89% | 1.23 | 2702.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 247.736µs | 70298 | 0 | 50.88% | 1.23 | 2463.72 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 432.279µs | 63043 | 0 | 50.88% | 1.23 | 1411.94 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 384.967µs | 67007 | 0 | 50.87% | 1.23 | 1585.46 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 233.703µs | 67438 | 0 | 50.89% | 1.23 | 2611.65 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 234.43µs | 67438 | 0 | 50.89% | 1.23 | 2603.56 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 233.147µs | 67438 | 0 | 50.88% | 1.23 | 2617.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.118157ms | 67438 | 20000 | 50.86% | 1.23 | 545.85 MB/s |
| Quicksort | 100000 | 12.188839ms | 1718762 | 0 | 50.24% | 1.24 | 500.75 MB/s |
| Timsort | 100000 | 13.872592ms | 1759891 | 0 | 50.09% | 1.24 | 439.97 MB/s |
| ARS Gen 1: Foundation | 100000 | 59.889248ms | 0 | 300000 | 45.67% | 1.21 | 101.91 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 58.843121ms | 0 | 300000 | 46.53% | 1.22 | 103.73 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.98204ms | 1895222 | 108703 | 50.75% | 1.24 | 359.41 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.031108ms | 1029722 | 100000 | 50.74% | 1.23 | 1514.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.866814ms | 1029722 | 0 | 50.75% | 1.23 | 2129.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.07664ms | 1071423 | 0 | 50.72% | 1.23 | 1983.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.113476ms | 978520 | 0 | 50.71% | 1.23 | 1960.35 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.431196ms | 1019338 | 0 | 50.68% | 1.23 | 1778.83 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.9665ms | 1029722 | 0 | 50.69% | 1.23 | 2057.48 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.1262ms | 961965 | 0 | 50.76% | 1.23 | 1952.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.138832ms | 1029722 | 0 | 50.71% | 1.23 | 1944.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 11.559643ms | 1029722 | 200000 | 50.43% | 1.23 | 528.00 MB/s |
| Quicksort | 1000000 | 275.058045ms | 20518628 | 0 | 51.73% | 1.26 | 221.90 MB/s |
| Timsort | 1000000 | 358.788423ms | 20902099 | 0 | 50.30% | 1.23 | 170.11 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 209.318276ms | 21589743 | 1017407 | 50.33% | 1.23 | 291.59 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.812145ms | 12256776 | 1000000 | 51.28% | 1.17 | 987.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 50.479586ms | 12256776 | 0 | 51.43% | 1.17 | 1209.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 59.834535ms | 12679336 | 0 | 50.90% | 1.16 | 1020.07 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 68.48426ms | 13331493 | 0 | 51.47% | 1.16 | 891.23 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 80.696742ms | 13750405 | 0 | 51.33% | 1.14 | 756.35 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 55.538157ms | 9434971 | 0 | 51.67% | 1.17 | 1098.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 73.382452ms | 7913738 | 0 | 52.67% | 1.15 | 831.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 59.264479ms | 8719170 | 0 | 52.26% | 1.17 | 1029.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 411.878501ms | 15151080 | 2000000 | 54.31% | 1.19 | 148.19 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 50.758µs | 9540 | 0 | 49.84% | 1.22 | 1202.47 MB/s |
| Timsort | 1000 | 48.867µs | 9492 | 0 | 49.84% | 1.22 | 1249.01 MB/s |
| ARS Gen 1: Foundation | 1000 | 131.447µs | 9394 | 2000 | 49.84% | 1.22 | 464.33 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 126.075µs | 9417 | 2000 | 49.84% | 1.22 | 484.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 48.869µs | 9540 | 0 | 49.84% | 1.22 | 1248.95 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 50.329µs | 9540 | 0 | 49.84% | 1.22 | 1212.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 53.871µs | 9540 | 0 | 49.84% | 1.22 | 1132.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 53.498µs | 9492 | 0 | 49.84% | 1.22 | 1140.89 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 50.598µs | 9540 | 0 | 49.84% | 1.22 | 1206.28 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 49.523µs | 9492 | 0 | 49.84% | 1.22 | 1232.46 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 51.191µs | 9540 | 0 | 49.84% | 1.22 | 1192.30 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 50.027µs | 9540 | 0 | 49.84% | 1.22 | 1220.04 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 49.254µs | 9540 | 0 | 49.84% | 1.22 | 1239.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 209.93µs | 9540 | 2000 | 49.84% | 1.22 | 290.74 MB/s |
| Quicksort | 10000 | 675.037µs | 132500 | 0 | 49.82% | 1.22 | 904.17 MB/s |
| Timsort | 10000 | 708.121µs | 127861 | 0 | 49.81% | 1.22 | 861.93 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.039665ms | 94604 | 30000 | 49.78% | 1.22 | 299.24 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.033921ms | 94565 | 30000 | 49.78% | 1.22 | 300.09 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.037718ms | 182797 | 14351 | 49.79% | 1.22 | 299.53 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 831.18µs | 88075 | 10000 | 49.80% | 1.22 | 734.32 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 285.842µs | 88075 | 0 | 49.81% | 1.22 | 2135.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 261.073µs | 63479 | 0 | 49.81% | 1.22 | 2337.86 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 355.56µs | 73151 | 0 | 49.80% | 1.22 | 1716.59 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 338.285µs | 48448 | 0 | 49.80% | 1.22 | 1804.25 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 335.375µs | 88075 | 0 | 49.81% | 1.22 | 1819.91 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 292.286µs | 88075 | 0 | 49.81% | 1.22 | 2088.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 289.926µs | 88075 | 0 | 49.81% | 1.22 | 2105.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.143592ms | 88075 | 20000 | 49.78% | 1.22 | 533.71 MB/s |
| Quicksort | 100000 | 10.717715ms | 1695729 | 0 | 49.62% | 1.22 | 569.48 MB/s |
| Timsort | 100000 | 11.354279ms | 1618264 | 0 | 49.50% | 1.22 | 537.55 MB/s |
| ARS Gen 1: Foundation | 100000 | 23.97725ms | 958264 | 300000 | 49.81% | 1.22 | 254.55 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 21.731212ms | 958287 | 300000 | 49.81% | 1.22 | 280.86 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.83133ms | 1799629 | 108703 | 49.78% | 1.22 | 342.29 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.899384ms | 1250176 | 100000 | 49.69% | 1.22 | 1245.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.053516ms | 1250176 | 0 | 49.73% | 1.22 | 1998.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.528226ms | 957484 | 0 | 49.69% | 1.21 | 1729.91 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.983352ms | 1082137 | 0 | 49.71% | 1.22 | 2045.86 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.479428ms | 561919 | 0 | 49.69% | 1.21 | 2461.66 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.271924ms | 877121 | 0 | 49.72% | 1.21 | 1865.42 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.344748ms | 943356 | 0 | 49.70% | 1.22 | 1824.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.032404ms | 1084625 | 0 | 49.71% | 1.22 | 2012.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.975693ms | 1250176 | 200000 | 49.62% | 1.22 | 556.09 MB/s |
| Quicksort | 1000000 | 145.686262ms | 20467458 | 0 | 48.52% | 1.27 | 418.95 MB/s |
| Timsort | 1000000 | 180.657302ms | 19247236 | 0 | 48.17% | 1.26 | 337.85 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 220.720579ms | 20726079 | 1017407 | 50.66% | 1.24 | 276.53 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.097132ms | 14427992 | 1000000 | 50.40% | 1.19 | 998.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 40.18641ms | 14427992 | 0 | 49.66% | 1.20 | 1518.80 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 40.903326ms | 9562892 | 0 | 49.11% | 1.18 | 1492.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 54.287746ms | 14500857 | 0 | 49.92% | 1.19 | 1124.29 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 53.7678ms | 9781181 | 0 | 49.52% | 1.17 | 1135.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 44.562152ms | 10121426 | 0 | 50.44% | 1.19 | 1369.66 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 55.114016ms | 10340217 | 0 | 50.78% | 1.18 | 1107.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 50.92879ms | 11441302 | 0 | 50.52% | 1.19 | 1198.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 250.467405ms | 19269568 | 2000000 | 51.33% | 1.23 | 243.69 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 47.452µs | 5636 | 0 | 54.30% | 1.24 | 1286.25 MB/s |
| Timsort | 1000 | 51.332µs | 5782 | 0 | 54.30% | 1.24 | 1189.03 MB/s |
| ARS Gen 1: Foundation | 1000 | 137.93µs | 984 | 2000 | 54.30% | 1.24 | 442.51 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 138.67µs | 984 | 2000 | 54.30% | 1.24 | 440.15 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 41.606µs | 5636 | 0 | 54.30% | 1.24 | 1466.98 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 38.811µs | 5636 | 0 | 54.30% | 1.24 | 1572.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 38.5µs | 5636 | 0 | 54.30% | 1.24 | 1585.33 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 40.896µs | 5782 | 0 | 54.30% | 1.24 | 1492.45 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 35.982µs | 5636 | 0 | 54.30% | 1.24 | 1696.27 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 40.231µs | 5782 | 0 | 54.30% | 1.24 | 1517.12 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 37.308µs | 5636 | 0 | 54.30% | 1.24 | 1635.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 37.948µs | 5636 | 0 | 54.30% | 1.24 | 1608.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 36.353µs | 5636 | 0 | 54.30% | 1.24 | 1678.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 205.26µs | 5636 | 2000 | 54.29% | 1.24 | 297.36 MB/s |
| Quicksort | 10000 | 352.183µs | 53113 | 0 | 54.27% | 1.24 | 1733.05 MB/s |
| Timsort | 10000 | 398.759µs | 54714 | 0 | 54.26% | 1.24 | 1530.63 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.568532ms | 9984 | 30000 | 54.25% | 1.24 | 389.12 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.400055ms | 9984 | 30000 | 54.25% | 1.24 | 435.95 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.958241ms | 122389 | 14351 | 54.26% | 1.24 | 311.68 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 623.769µs | 14075 | 10000 | 54.27% | 1.24 | 978.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 211.303µs | 14075 | 0 | 54.27% | 1.24 | 2888.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.4µs | 14094 | 0 | 54.27% | 1.24 | 3805.18 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 306.779µs | 12021 | 0 | 54.26% | 1.24 | 1989.55 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 336.587µs | 12028 | 0 | 54.26% | 1.24 | 1813.35 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 186.187µs | 14075 | 0 | 54.27% | 1.24 | 3278.16 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 148.643µs | 14075 | 0 | 54.27% | 1.24 | 4106.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 164.018µs | 14075 | 0 | 54.27% | 1.24 | 3721.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.066912ms | 14075 | 20000 | 54.25% | 1.24 | 572.07 MB/s |
| Quicksort | 100000 | 4.060516ms | 516589 | 0 | 54.05% | 1.24 | 1503.14 MB/s |
| Timsort | 100000 | 4.854385ms | 529550 | 0 | 53.96% | 1.24 | 1257.32 MB/s |
| ARS Gen 1: Foundation | 100000 | 15.429526ms | 99984 | 300000 | 54.06% | 1.24 | 395.57 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 15.286452ms | 99984 | 300000 | 54.05% | 1.24 | 399.28 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.946669ms | 1144965 | 108703 | 54.22% | 1.24 | 360.16 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.341408ms | 151083 | 100000 | 54.25% | 1.24 | 1826.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.594294ms | 151083 | 0 | 54.23% | 1.24 | 3828.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.712517ms | 151309 | 0 | 54.24% | 1.24 | 3564.06 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.932357ms | 99990 | 0 | 54.22% | 1.24 | 3158.59 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.922681ms | 99990 | 0 | 54.21% | 1.24 | 3174.48 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.099158ms | 200008 | 0 | 54.13% | 1.24 | 2907.60 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.314314ms | 200008 | 0 | 54.16% | 1.24 | 2637.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.810808ms | 100024 | 0 | 54.22% | 1.24 | 3370.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 8.828939ms | 151083 | 200000 | 53.96% | 1.24 | 691.31 MB/s |
| Quicksort | 1000000 | 89.656641ms | 5202060 | 0 | 55.36% | 1.24 | 680.77 MB/s |
| Timsort | 1000000 | 151.016391ms | 6111262 | 0 | 55.47% | 1.23 | 404.16 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 179.291618ms | 12085476 | 1017407 | 54.33% | 1.25 | 340.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 31.695041ms | 999988 | 1000000 | 54.69% | 1.22 | 1925.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.198169ms | 999988 | 0 | 54.60% | 1.22 | 2749.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 24.219417ms | 999988 | 0 | 54.70% | 1.21 | 2520.09 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 24.246614ms | 999988 | 0 | 54.59% | 1.22 | 2517.27 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 24.489329ms | 999988 | 0 | 54.61% | 1.22 | 2492.32 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 33.908956ms | 1999972 | 0 | 54.79% | 1.21 | 1799.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 38.239383ms | 1999976 | 0 | 54.98% | 1.21 | 1596.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 36.531227ms | 1999976 | 0 | 54.78% | 1.21 | 1670.77 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 309.814586ms | 5709060 | 2000000 | 56.68% | 1.22 | 197.01 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 54.914µs | 10370 | 0 | 54.08% | 1.23 | 1111.47 MB/s |
| Timsort | 1000 | 70.64µs | 10522 | 0 | 54.08% | 1.23 | 864.03 MB/s |
| ARS Gen 1: Foundation | 1000 | 274.035µs | 0 | 2000 | 54.08% | 1.23 | 222.73 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 290.044µs | 0 | 2000 | 54.08% | 1.23 | 210.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 57.851µs | 10370 | 0 | 54.08% | 1.23 | 1055.04 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 59.992µs | 10370 | 0 | 54.08% | 1.23 | 1017.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 61.527µs | 10370 | 0 | 54.08% | 1.23 | 992.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 65.9µs | 10522 | 0 | 54.08% | 1.23 | 926.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 60.261µs | 10370 | 0 | 54.08% | 1.23 | 1012.85 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 63.842µs | 10522 | 0 | 54.08% | 1.23 | 956.03 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 61.183µs | 10370 | 0 | 54.08% | 1.23 | 997.58 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 61.289µs | 10370 | 0 | 54.08% | 1.23 | 995.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 63.889µs | 10370 | 0 | 54.08% | 1.23 | 955.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 216.967µs | 10370 | 2000 | 54.08% | 1.23 | 281.31 MB/s |
| Quicksort | 10000 | 783.852µs | 136866 | 0 | 54.05% | 1.23 | 778.66 MB/s |
| Timsort | 10000 | 908.673µs | 141490 | 0 | 54.05% | 1.23 | 671.70 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.785437ms | 0 | 30000 | 53.96% | 1.23 | 105.50 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.044143ms | 0 | 30000 | 53.93% | 1.23 | 100.98 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.987298ms | 193846 | 14351 | 54.05% | 1.23 | 307.13 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 627.716µs | 67438 | 10000 | 54.05% | 1.23 | 972.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 239.498µs | 67438 | 0 | 54.05% | 1.23 | 2548.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 245.373µs | 70298 | 0 | 54.05% | 1.23 | 2487.44 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 416.83µs | 63043 | 0 | 54.05% | 1.23 | 1464.27 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 363.7µs | 67007 | 0 | 54.05% | 1.23 | 1678.17 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 237.594µs | 67438 | 0 | 54.05% | 1.23 | 2568.88 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 228.545µs | 67438 | 0 | 54.05% | 1.23 | 2670.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 233.956µs | 67438 | 0 | 54.05% | 1.23 | 2608.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.093845ms | 67438 | 20000 | 54.03% | 1.23 | 557.99 MB/s |
| Quicksort | 100000 | 11.121146ms | 1718762 | 0 | 53.66% | 1.23 | 548.82 MB/s |
| Timsort | 100000 | 13.788407ms | 1759891 | 0 | 53.50% | 1.23 | 442.66 MB/s |
| ARS Gen 1: Foundation | 100000 | 58.200253ms | 0 | 300000 | 50.77% | 1.22 | 104.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 57.708928ms | 0 | 300000 | 50.36% | 1.22 | 105.76 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.390947ms | 1895222 | 108703 | 53.94% | 1.23 | 350.96 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.363771ms | 1029722 | 100000 | 53.94% | 1.23 | 1398.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.728281ms | 1029722 | 0 | 53.95% | 1.23 | 2237.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.041987ms | 1071423 | 0 | 53.93% | 1.23 | 2006.42 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.20598ms | 978520 | 0 | 53.91% | 1.23 | 1903.79 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.38271ms | 1019338 | 0 | 53.91% | 1.23 | 1804.33 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.028484ms | 1029722 | 0 | 53.92% | 1.23 | 2015.37 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.078764ms | 961965 | 0 | 53.96% | 1.23 | 1982.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.969707ms | 1029722 | 0 | 53.91% | 1.23 | 2055.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.909495ms | 1029722 | 200000 | 53.67% | 1.23 | 615.93 MB/s |
| Quicksort | 1000000 | 226.45443ms | 20518628 | 0 | 54.28% | 1.25 | 269.53 MB/s |
| Timsort | 1000000 | 350.184396ms | 20902099 | 0 | 53.45% | 1.23 | 174.29 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 190.70636ms | 21589743 | 1017407 | 53.60% | 1.23 | 320.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.502241ms | 12256776 | 1000000 | 54.18% | 1.18 | 992.41 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 47.764505ms | 12256776 | 0 | 54.31% | 1.19 | 1277.84 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 60.577679ms | 12679336 | 0 | 53.94% | 1.17 | 1007.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 62.96729ms | 13331493 | 0 | 54.39% | 1.18 | 969.32 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 72.956338ms | 13750405 | 0 | 54.15% | 1.17 | 836.60 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.638507ms | 9434971 | 0 | 54.48% | 1.19 | 1181.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 70.670073ms | 7913738 | 0 | 55.04% | 1.18 | 863.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.122345ms | 8719170 | 0 | 54.78% | 1.18 | 1068.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 427.836393ms | 15053938 | 2000000 | 55.92% | 1.19 | 142.66 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 61.258µs | 10370 | 0 | 53.39% | 1.22 | 996.36 MB/s |
| Timsort | 1000 | 65.317µs | 10522 | 0 | 53.39% | 1.22 | 934.45 MB/s |
| ARS Gen 1: Foundation | 1000 | 266.847µs | 0 | 2000 | 53.38% | 1.22 | 228.73 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 327.263µs | 0 | 2000 | 53.38% | 1.22 | 186.50 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 57.994µs | 10370 | 0 | 53.39% | 1.22 | 1052.44 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 61.264µs | 10370 | 0 | 53.39% | 1.22 | 996.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 54.288µs | 10370 | 0 | 53.39% | 1.22 | 1124.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 76.915µs | 10522 | 0 | 53.39% | 1.22 | 793.54 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 62.446µs | 10370 | 0 | 53.39% | 1.22 | 977.41 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 79.558µs | 10522 | 0 | 53.39% | 1.22 | 767.18 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 51.352µs | 10370 | 0 | 53.39% | 1.22 | 1188.56 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 55.692µs | 10370 | 0 | 53.39% | 1.22 | 1095.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 62.993µs | 10370 | 0 | 53.39% | 1.22 | 968.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 225.497µs | 10370 | 2000 | 53.38% | 1.22 | 270.67 MB/s |
| Quicksort | 10000 | 724.444µs | 136866 | 0 | 53.36% | 1.22 | 842.51 MB/s |
| Timsort | 10000 | 884.68µs | 141490 | 0 | 53.35% | 1.22 | 689.91 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.950374ms | 0 | 30000 | 53.26% | 1.22 | 102.57 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.791263ms | 0 | 30000 | 53.24% | 1.22 | 105.39 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.051645ms | 193846 | 14351 | 53.35% | 1.22 | 297.49 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 662.369µs | 67438 | 10000 | 53.36% | 1.22 | 921.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 245.936µs | 67438 | 0 | 53.36% | 1.22 | 2481.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 245.374µs | 70298 | 0 | 53.36% | 1.22 | 2487.43 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 351.089µs | 63043 | 0 | 53.35% | 1.22 | 1738.45 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 346.666µs | 67007 | 0 | 53.35% | 1.22 | 1760.63 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 229.871µs | 67438 | 0 | 53.36% | 1.22 | 2655.19 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 242.873µs | 67438 | 0 | 53.36% | 1.22 | 2513.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 244.111µs | 67438 | 0 | 53.36% | 1.22 | 2500.30 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.051772ms | 67438 | 20000 | 53.34% | 1.22 | 580.31 MB/s |
| Quicksort | 100000 | 10.330461ms | 1718762 | 0 | 52.99% | 1.22 | 590.83 MB/s |
| Timsort | 100000 | 12.569297ms | 1759891 | 0 | 52.85% | 1.22 | 485.59 MB/s |
| ARS Gen 1: Foundation | 100000 | 62.156065ms | 0 | 300000 | 49.77% | 1.21 | 98.20 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 63.125631ms | 0 | 300000 | 49.45% | 1.21 | 96.69 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.796824ms | 1895222 | 108703 | 53.26% | 1.22 | 342.96 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.966248ms | 1029722 | 100000 | 53.25% | 1.22 | 1538.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.078955ms | 1029722 | 0 | 53.23% | 1.22 | 1982.33 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.173208ms | 1071423 | 0 | 53.23% | 1.22 | 1923.45 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.195935ms | 978520 | 0 | 53.22% | 1.22 | 1909.77 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.459358ms | 1019338 | 0 | 53.22% | 1.22 | 1764.35 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.185483ms | 1029722 | 0 | 53.23% | 1.22 | 1916.04 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.134185ms | 961965 | 0 | 53.27% | 1.22 | 1947.40 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.160005ms | 1029722 | 0 | 53.22% | 1.22 | 1931.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.368664ms | 1029722 | 200000 | 52.96% | 1.22 | 588.65 MB/s |
| Quicksort | 1000000 | 253.118818ms | 20518628 | 0 | 53.72% | 1.24 | 241.13 MB/s |
| Timsort | 1000000 | 354.609214ms | 20902099 | 0 | 52.77% | 1.23 | 172.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 195.120099ms | 21589743 | 1017407 | 52.95% | 1.22 | 312.81 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 62.014439ms | 12256776 | 1000000 | 53.54% | 1.18 | 984.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.627395ms | 12256776 | 0 | 53.67% | 1.19 | 1255.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.374233ms | 12679336 | 0 | 53.44% | 1.17 | 1102.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 61.93414ms | 13331493 | 0 | 53.78% | 1.18 | 985.48 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 72.833958ms | 13750405 | 0 | 53.60% | 1.17 | 838.00 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.091777ms | 9434971 | 0 | 53.88% | 1.18 | 1171.69 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 67.865428ms | 7913738 | 0 | 54.46% | 1.17 | 899.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 56.979263ms | 8719170 | 0 | 54.24% | 1.18 | 1071.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 424.122313ms | 15199654 | 2000000 | 55.32% | 1.19 | 143.91 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 53.052µs | 10370 | 0 | 53.23% | 1.22 | 1150.48 MB/s |
| Timsort | 1000 | 60.311µs | 10522 | 0 | 53.23% | 1.22 | 1012.01 MB/s |
| ARS Gen 1: Foundation | 1000 | 289.947µs | 0 | 2000 | 53.23% | 1.22 | 210.50 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 301.184µs | 0 | 2000 | 53.23% | 1.22 | 202.65 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 50.976µs | 10370 | 0 | 53.23% | 1.22 | 1197.33 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 50.908µs | 10370 | 0 | 53.23% | 1.22 | 1198.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 52.362µs | 10370 | 0 | 53.23% | 1.22 | 1165.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 61.003µs | 10522 | 0 | 53.23% | 1.22 | 1000.53 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 54.889µs | 10370 | 0 | 53.23% | 1.22 | 1111.97 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 66.561µs | 10522 | 0 | 53.23% | 1.22 | 916.98 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 51.347µs | 10370 | 0 | 53.23% | 1.22 | 1188.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 54.757µs | 10370 | 0 | 53.23% | 1.22 | 1114.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 60.27µs | 10370 | 0 | 53.23% | 1.22 | 1012.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 217.842µs | 10370 | 2000 | 53.23% | 1.22 | 280.18 MB/s |
| Quicksort | 10000 | 801.142µs | 136866 | 0 | 53.21% | 1.22 | 761.85 MB/s |
| Timsort | 10000 | 940.126µs | 141490 | 0 | 53.20% | 1.22 | 649.22 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.903287ms | 0 | 30000 | 53.12% | 1.22 | 88.41 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.838811ms | 0 | 30000 | 53.11% | 1.22 | 104.53 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.964721ms | 193846 | 14351 | 53.20% | 1.22 | 310.66 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 625.54µs | 67438 | 10000 | 53.20% | 1.22 | 975.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 221.495µs | 67438 | 0 | 53.20% | 1.22 | 2755.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 241.461µs | 70298 | 0 | 53.20% | 1.22 | 2527.74 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 336.5µs | 63043 | 0 | 53.20% | 1.22 | 1813.82 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 367.859µs | 67007 | 0 | 53.20% | 1.22 | 1659.20 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 230.322µs | 67438 | 0 | 53.20% | 1.22 | 2649.99 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 221.745µs | 67438 | 0 | 53.20% | 1.22 | 2752.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 228.746µs | 67438 | 0 | 53.20% | 1.22 | 2668.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.037697ms | 67438 | 20000 | 53.19% | 1.22 | 588.18 MB/s |
| Quicksort | 100000 | 10.364695ms | 1718762 | 0 | 52.86% | 1.22 | 588.88 MB/s |
| Timsort | 100000 | 11.89897ms | 1759891 | 0 | 52.74% | 1.22 | 512.94 MB/s |
| ARS Gen 1: Foundation | 100000 | 60.696377ms | 0 | 300000 | 50.23% | 1.22 | 100.56 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 60.37734ms | 0 | 300000 | 49.67% | 1.21 | 101.09 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.111241ms | 1895222 | 108703 | 53.12% | 1.22 | 356.70 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.269304ms | 1029722 | 100000 | 53.12% | 1.22 | 1429.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.943583ms | 1029722 | 0 | 53.11% | 1.22 | 2073.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.158544ms | 1071423 | 0 | 53.08% | 1.22 | 1932.38 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.231459ms | 978520 | 0 | 53.09% | 1.22 | 1888.78 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.339618ms | 1019338 | 0 | 53.08% | 1.22 | 1827.61 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.17453ms | 1029722 | 0 | 53.09% | 1.22 | 1922.65 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.177015ms | 961965 | 0 | 53.12% | 1.22 | 1921.15 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.039598ms | 1029722 | 0 | 53.10% | 1.22 | 2008.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.319731ms | 1029722 | 200000 | 52.84% | 1.22 | 591.44 MB/s |
| Quicksort | 1000000 | 239.170899ms | 20518628 | 0 | 53.55% | 1.24 | 255.19 MB/s |
| Timsort | 1000000 | 394.443685ms | 20902099 | 0 | 52.58% | 1.22 | 154.74 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 194.530017ms | 21589743 | 1017407 | 52.81% | 1.22 | 313.76 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.8797ms | 12256776 | 1000000 | 53.38% | 1.18 | 986.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.069632ms | 12256776 | 0 | 53.52% | 1.18 | 1269.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 57.117987ms | 12679336 | 0 | 53.28% | 1.18 | 1068.58 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 60.208055ms | 13331493 | 0 | 53.53% | 1.18 | 1013.74 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 73.758504ms | 13750405 | 0 | 53.38% | 1.17 | 827.50 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.270347ms | 9434971 | 0 | 53.74% | 1.18 | 1167.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 70.255542ms | 7913738 | 0 | 54.28% | 1.18 | 868.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.753554ms | 8719170 | 0 | 54.05% | 1.18 | 1056.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 432.232147ms | 15199655 | 2000000 | 55.14% | 1.19 | 141.21 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 51.233µs | 10370 | 0 | 52.74% | 1.21 | 1191.33 MB/s |
| Timsort | 1000 | 63.687µs | 10522 | 0 | 52.74% | 1.21 | 958.36 MB/s |
| ARS Gen 1: Foundation | 1000 | 268.377µs | 0 | 2000 | 52.74% | 1.21 | 227.42 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 287.202µs | 0 | 2000 | 52.74% | 1.21 | 212.52 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 74.915µs | 10370 | 0 | 52.74% | 1.21 | 814.73 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 57.43µs | 10370 | 0 | 52.74% | 1.21 | 1062.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 54.833µs | 10370 | 0 | 52.74% | 1.21 | 1113.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 63.292µs | 10522 | 0 | 52.74% | 1.21 | 964.34 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 66.785µs | 10370 | 0 | 52.74% | 1.21 | 913.91 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 65.949µs | 10522 | 0 | 52.74% | 1.21 | 925.49 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 66.532µs | 10370 | 0 | 52.74% | 1.21 | 917.38 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 63.165µs | 10370 | 0 | 52.74% | 1.21 | 966.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 65.506µs | 10370 | 0 | 52.74% | 1.21 | 931.75 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 221.512µs | 10370 | 2000 | 52.74% | 1.21 | 275.54 MB/s |
| Quicksort | 10000 | 835.195µs | 136866 | 0 | 52.72% | 1.21 | 730.79 MB/s |
| Timsort | 10000 | 911.076µs | 141490 | 0 | 52.71% | 1.21 | 669.92 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.765905ms | 0 | 30000 | 52.66% | 1.22 | 105.86 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.078004ms | 0 | 30000 | 52.65% | 1.22 | 100.42 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.937206ms | 193846 | 14351 | 52.71% | 1.21 | 315.07 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 633.578µs | 67438 | 10000 | 52.71% | 1.21 | 963.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 231.432µs | 67438 | 0 | 52.71% | 1.21 | 2637.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 241.869µs | 70298 | 0 | 52.71% | 1.21 | 2523.48 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 381.852µs | 63043 | 0 | 52.71% | 1.21 | 1598.40 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 409.212µs | 67007 | 0 | 52.71% | 1.21 | 1491.53 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 278.013µs | 67438 | 0 | 52.71% | 1.21 | 2195.41 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 221.246µs | 67438 | 0 | 52.71% | 1.21 | 2758.70 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 224.286µs | 67438 | 0 | 52.71% | 1.21 | 2721.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.166752ms | 67438 | 20000 | 52.70% | 1.21 | 523.12 MB/s |
| Quicksort | 100000 | 10.513942ms | 1718762 | 0 | 52.36% | 1.22 | 580.52 MB/s |
| Timsort | 100000 | 12.097984ms | 1759891 | 0 | 52.24% | 1.22 | 504.51 MB/s |
| ARS Gen 1: Foundation | 100000 | 59.076567ms | 0 | 300000 | 49.82% | 1.21 | 103.32 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 58.171371ms | 0 | 300000 | 49.52% | 1.21 | 104.92 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.583808ms | 1895222 | 108703 | 52.63% | 1.22 | 368.04 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.988687ms | 1029722 | 100000 | 52.63% | 1.21 | 1530.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.781766ms | 1029722 | 0 | 52.63% | 1.21 | 2194.12 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.141564ms | 1071423 | 0 | 52.61% | 1.21 | 1942.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.129852ms | 978520 | 0 | 52.61% | 1.21 | 1950.10 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.305719ms | 1019338 | 0 | 52.60% | 1.21 | 1846.35 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.188645ms | 1029722 | 0 | 52.62% | 1.21 | 1914.14 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.167599ms | 961965 | 0 | 52.65% | 1.21 | 1926.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.923174ms | 1029722 | 0 | 52.62% | 1.21 | 2087.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.988039ms | 1029722 | 200000 | 52.40% | 1.21 | 611.08 MB/s |
| Quicksort | 1000000 | 247.754973ms | 20518628 | 0 | 53.02% | 1.23 | 246.35 MB/s |
| Timsort | 1000000 | 350.64015ms | 20902099 | 0 | 52.16% | 1.22 | 174.07 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 206.835909ms | 21589743 | 1017407 | 52.39% | 1.21 | 295.09 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 62.438689ms | 12256776 | 1000000 | 52.96% | 1.17 | 977.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 50.002741ms | 12256776 | 0 | 53.05% | 1.18 | 1220.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 61.326489ms | 12679336 | 0 | 52.75% | 1.16 | 995.25 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 66.199828ms | 13331493 | 0 | 53.10% | 1.17 | 921.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 75.82351ms | 13750405 | 0 | 52.92% | 1.16 | 804.96 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.350896ms | 9434971 | 0 | 53.25% | 1.18 | 1165.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 70.621949ms | 7913738 | 0 | 53.81% | 1.17 | 864.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 58.130475ms | 8719170 | 0 | 53.57% | 1.17 | 1049.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 433.604374ms | 15151080 | 2000000 | 54.73% | 1.18 | 140.76 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 31.032µs | 5636 | 0 | 52.52% | 1.21 | 1966.85 MB/s |
| Timsort | 1000 | 38.873µs | 5782 | 0 | 52.52% | 1.21 | 1570.12 MB/s |
| ARS Gen 1: Foundation | 1000 | 113.348µs | 984 | 2000 | 52.52% | 1.21 | 538.48 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 126.568µs | 984 | 2000 | 52.52% | 1.21 | 482.23 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 37.368µs | 5636 | 0 | 52.52% | 1.21 | 1633.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 35.473µs | 5636 | 0 | 52.52% | 1.21 | 1720.61 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 35.199µs | 5636 | 0 | 52.52% | 1.21 | 1734.00 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 42.496µs | 5782 | 0 | 52.52% | 1.21 | 1436.26 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 35.458µs | 5636 | 0 | 52.52% | 1.21 | 1721.34 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 38.527µs | 5782 | 0 | 52.52% | 1.21 | 1584.22 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 34.609µs | 5636 | 0 | 52.52% | 1.21 | 1763.56 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 37.392µs | 5636 | 0 | 52.52% | 1.21 | 1632.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 37.544µs | 5636 | 0 | 52.52% | 1.21 | 1625.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 210.065µs | 5636 | 2000 | 52.52% | 1.21 | 290.55 MB/s |
| Quicksort | 10000 | 321.036µs | 53113 | 0 | 52.50% | 1.21 | 1901.19 MB/s |
| Timsort | 10000 | 373.914µs | 54714 | 0 | 52.49% | 1.21 | 1632.33 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.4233ms | 9984 | 30000 | 52.48% | 1.21 | 428.83 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.407054ms | 9984 | 30000 | 52.48% | 1.21 | 433.78 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.780106ms | 122389 | 14351 | 52.49% | 1.21 | 342.87 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 590.955µs | 14075 | 10000 | 52.50% | 1.21 | 1032.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 149.62µs | 14075 | 0 | 52.50% | 1.21 | 4079.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 141.51µs | 14094 | 0 | 52.50% | 1.21 | 4313.13 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 275.523µs | 12021 | 0 | 52.49% | 1.21 | 2215.25 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 294.857µs | 12028 | 0 | 52.49% | 1.21 | 2069.99 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 173.324µs | 14075 | 0 | 52.50% | 1.21 | 3521.45 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 210.202µs | 14075 | 0 | 52.49% | 1.21 | 2903.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 132.991µs | 14075 | 0 | 52.49% | 1.21 | 4589.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 974.078µs | 14075 | 20000 | 52.48% | 1.21 | 626.59 MB/s |
| Quicksort | 100000 | 3.980719ms | 516589 | 0 | 52.29% | 1.21 | 1533.27 MB/s |
| Timsort | 100000 | 4.692271ms | 529550 | 0 | 52.21% | 1.21 | 1300.76 MB/s |
| ARS Gen 1: Foundation | 100000 | 15.761171ms | 99984 | 300000 | 52.32% | 1.21 | 387.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 15.53786ms | 99984 | 300000 | 52.30% | 1.21 | 392.82 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 14.216705ms | 1144965 | 108703 | 52.44% | 1.21 | 429.32 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.166568ms | 151083 | 100000 | 52.46% | 1.21 | 1927.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.672434ms | 151083 | 0 | 52.46% | 1.21 | 3649.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.60765ms | 151309 | 0 | 52.45% | 1.21 | 3796.55 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.929704ms | 99990 | 0 | 52.44% | 1.21 | 3162.93 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.957296ms | 99990 | 0 | 52.44% | 1.21 | 3118.34 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.230587ms | 200008 | 0 | 52.37% | 1.21 | 2736.28 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.994262ms | 200008 | 0 | 52.39% | 1.21 | 3060.54 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.651214ms | 100024 | 0 | 52.45% | 1.21 | 3696.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.239766ms | 151083 | 200000 | 52.24% | 1.21 | 660.57 MB/s |
| Quicksort | 1000000 | 91.807181ms | 5202060 | 0 | 53.57% | 1.21 | 664.82 MB/s |
| Timsort | 1000000 | 149.926985ms | 6111262 | 0 | 53.69% | 1.20 | 407.10 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 161.414251ms | 12085476 | 1017407 | 52.63% | 1.23 | 378.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 31.031652ms | 999988 | 1000000 | 52.95% | 1.20 | 1966.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 23.154022ms | 999988 | 0 | 52.89% | 1.19 | 2636.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 22.568585ms | 999988 | 0 | 52.91% | 1.19 | 2704.43 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 24.596605ms | 999988 | 0 | 52.93% | 1.19 | 2481.45 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 23.46595ms | 999988 | 0 | 52.88% | 1.19 | 2601.01 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 33.145994ms | 1999972 | 0 | 53.03% | 1.19 | 1841.40 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 36.715342ms | 1999976 | 0 | 53.03% | 1.18 | 1662.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 37.167446ms | 1999976 | 0 | 53.02% | 1.18 | 1642.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 332.5884ms | 5709060 | 2000000 | 55.31% | 1.19 | 183.52 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 56.273µs | 10308 | 0 | 52.49% | 1.20 | 1084.63 MB/s |
| Timsort | 1000 | 65.047µs | 10658 | 0 | 52.49% | 1.20 | 938.32 MB/s |
| ARS Gen 1: Foundation | 1000 | 127.034µs | 10308 | 2000 | 52.49% | 1.20 | 480.46 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 145.346µs | 10308 | 2000 | 52.49% | 1.20 | 419.93 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 52.888µs | 10308 | 0 | 52.49% | 1.20 | 1154.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 54.434µs | 10308 | 0 | 52.49% | 1.20 | 1121.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 57.788µs | 10308 | 0 | 52.49% | 1.20 | 1056.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 67.558µs | 10658 | 0 | 52.49% | 1.20 | 903.45 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 59.078µs | 10308 | 0 | 52.49% | 1.20 | 1033.13 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 72.12µs | 10658 | 0 | 52.49% | 1.20 | 846.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 55.837µs | 10308 | 0 | 52.49% | 1.20 | 1093.10 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 56.033µs | 10308 | 0 | 52.49% | 1.20 | 1089.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 57.527µs | 10308 | 0 | 52.49% | 1.20 | 1060.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 227.814µs | 10308 | 2000 | 52.49% | 1.20 | 267.92 MB/s |
| Quicksort | 10000 | 770.038µs | 138349 | 0 | 52.45% | 1.20 | 792.63 MB/s |
| Timsort | 10000 | 989.601µs | 142268 | 0 | 52.44% | 1.20 | 616.77 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.284098ms | 138349 | 30000 | 52.41% | 1.20 | 267.22 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.184335ms | 138349 | 30000 | 52.40% | 1.20 | 279.42 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.321629ms | 193925 | 14351 | 52.44% | 1.20 | 262.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.603057ms | 138355 | 10000 | 52.44% | 1.20 | 380.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.049367ms | 138355 | 0 | 52.43% | 1.20 | 581.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.109414ms | 142274 | 0 | 52.42% | 1.20 | 550.16 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 975.636µs | 138355 | 0 | 52.42% | 1.20 | 625.59 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.169319ms | 142274 | 0 | 52.42% | 1.20 | 521.97 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 801.363µs | 138355 | 0 | 52.44% | 1.20 | 761.64 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.012152ms | 138355 | 0 | 52.43% | 1.20 | 603.02 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 947.805µs | 138355 | 0 | 52.43% | 1.20 | 643.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.901921ms | 138355 | 20000 | 52.40% | 1.20 | 320.91 MB/s |
| Quicksort | 100000 | 16.167123ms | 1715173 | 0 | 52.21% | 1.20 | 377.53 MB/s |
| Timsort | 100000 | 20.131615ms | 1762853 | 0 | 52.20% | 1.20 | 303.18 MB/s |
| ARS Gen 1: Foundation | 100000 | 48.741934ms | 1715173 | 300000 | 52.13% | 1.20 | 125.22 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.15164ms | 1715173 | 300000 | 52.11% | 1.20 | 132.25 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 21.34006ms | 1895407 | 108703 | 52.45% | 1.20 | 286.01 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 20.038461ms | 1715179 | 100000 | 52.16% | 1.20 | 304.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 20.765159ms | 1715179 | 0 | 52.22% | 1.19 | 293.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 27.904193ms | 1762859 | 0 | 52.18% | 1.19 | 218.73 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 21.557806ms | 1715179 | 0 | 52.23% | 1.19 | 283.12 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 29.282502ms | 1762859 | 0 | 52.22% | 1.19 | 208.44 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 18.660902ms | 1715179 | 0 | 52.20% | 1.20 | 327.08 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 19.193485ms | 1715179 | 0 | 52.19% | 1.20 | 318.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 19.041354ms | 1715179 | 0 | 52.21% | 1.20 | 320.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 33.785742ms | 1715179 | 200000 | 52.03% | 1.20 | 180.65 MB/s |
| Quicksort | 1000000 | 630.391619ms | 20523276 | 0 | 54.40% | 1.15 | 96.82 MB/s |
| Timsort | 1000000 | 830.409685ms | 20914644 | 0 | 53.96% | 1.12 | 73.50 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 262.033063ms | 21586854 | 1017407 | 52.10% | 1.17 | 232.93 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 529.07507ms | 20523280 | 1000000 | 54.74% | 1.16 | 115.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 576.405708ms | 20523280 | 0 | 54.81% | 1.16 | 105.89 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 707.307599ms | 20914648 | 0 | 54.26% | 1.14 | 86.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 479.774016ms | 20523280 | 0 | 55.04% | 1.16 | 127.22 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 702.206907ms | 20914648 | 0 | 54.31% | 1.13 | 86.92 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 524.170443ms | 20523280 | 0 | 54.69% | 1.17 | 116.44 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 505.831878ms | 20523280 | 0 | 54.57% | 1.17 | 120.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 479.812012ms | 20523280 | 0 | 54.65% | 1.17 | 127.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 674.447386ms | 21815105 | 2000000 | 57.45% | 1.12 | 90.50 MB/s |

## Category: Custom

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 20.308µs | 10378 | 0 | 64.65% | 1.08 | 2254.11 MB/s |
| Timsort | 1000 | 28.499µs | 10965 | 0 | 64.65% | 1.08 | 1606.24 MB/s |
| ARS Gen 1: Foundation | 1000 | 207.253µs | 0 | 2000 | 64.65% | 1.08 | 220.87 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 238.412µs | 0 | 2000 | 64.65% | 1.08 | 192.01 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 18.763µs | 10378 | 0 | 64.65% | 1.08 | 2439.71 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 19.204µs | 10378 | 0 | 64.65% | 1.08 | 2383.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.579µs | 10378 | 0 | 64.65% | 1.08 | 2463.88 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.142µs | 10965 | 0 | 64.65% | 1.08 | 1751.07 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.92µs | 10378 | 0 | 64.65% | 1.08 | 1997.22 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.015µs | 10965 | 0 | 64.65% | 1.08 | 1577.68 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 21.063µs | 10378 | 0 | 64.65% | 1.08 | 2173.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 19.324µs | 10378 | 0 | 64.65% | 1.08 | 2368.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.39µs | 10378 | 0 | 64.65% | 1.08 | 2360.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 114.009µs | 10378 | 2000 | 64.64% | 1.08 | 401.52 MB/s |
| Quicksort | 10000 | 228.59µs | 138485 | 0 | 64.64% | 1.08 | 2002.55 MB/s |
| Timsort | 10000 | 320.734µs | 142802 | 0 | 64.64% | 1.08 | 1427.24 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.363795ms | 0 | 30000 | 64.39% | 1.08 | 85.34 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.591128ms | 0 | 30000 | 64.39% | 1.08 | 81.87 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 408.866µs | 194235 | 14351 | 64.63% | 1.08 | 1119.59 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 235.716µs | 53078 | 10000 | 64.63% | 1.07 | 1942.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 135.076µs | 53078 | 0 | 64.63% | 1.07 | 3388.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 143.894µs | 57974 | 0 | 64.63% | 1.07 | 3181.26 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 134.632µs | 60130 | 0 | 64.63% | 1.08 | 3400.11 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 153.139µs | 62739 | 0 | 64.63% | 1.07 | 2989.20 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 153.556µs | 53078 | 0 | 64.63% | 1.07 | 2981.09 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 144.425µs | 53078 | 0 | 64.63% | 1.08 | 3169.56 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 135.4µs | 53078 | 0 | 64.63% | 1.07 | 3380.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 329.053µs | 53078 | 20000 | 64.63% | 1.07 | 1391.15 MB/s |
| Quicksort | 100000 | 2.913515ms | 1716233 | 0 | 64.58% | 1.08 | 1571.17 MB/s |
| Timsort | 100000 | 4.840763ms | 1759914 | 0 | 64.52% | 1.08 | 945.64 MB/s |
| ARS Gen 1: Foundation | 100000 | 40.185125ms | 0 | 300000 | 62.29% | 1.07 | 113.91 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 40.32417ms | 0 | 300000 | 62.20% | 1.07 | 113.52 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.960305ms | 1895170 | 108703 | 64.57% | 1.08 | 1546.34 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.523066ms | 891495 | 100000 | 64.60% | 1.07 | 3005.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.375923ms | 891495 | 0 | 64.60% | 1.07 | 3326.96 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.505285ms | 927102 | 0 | 64.60% | 1.07 | 3041.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.407018ms | 954799 | 0 | 64.59% | 1.07 | 3253.43 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.757072ms | 993233 | 0 | 64.59% | 1.07 | 2605.26 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.430969ms | 891495 | 0 | 64.59% | 1.07 | 3198.98 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.506027ms | 780845 | 0 | 64.59% | 1.07 | 3039.54 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.512716ms | 891495 | 0 | 64.60% | 1.07 | 3026.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.769277ms | 891495 | 200000 | 64.59% | 1.07 | 2587.29 MB/s |
| Quicksort | 1000000 | 40.404714ms | 20512439 | 0 | 64.13% | 1.09 | 1132.95 MB/s |
| Timsort | 1000000 | 70.919751ms | 20899150 | 0 | 63.56% | 1.08 | 645.47 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 35.152684ms | 21596717 | 1017407 | 64.25% | 1.08 | 1302.22 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.465781ms | 10310056 | 1000000 | 64.64% | 1.06 | 2478.98 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.184564ms | 10310056 | 0 | 64.64% | 1.07 | 2663.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.955002ms | 10709205 | 0 | 64.61% | 1.06 | 2415.00 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.767429ms | 13016047 | 0 | 64.58% | 1.07 | 2576.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.870284ms | 13434584 | 0 | 64.44% | 1.07 | 2193.38 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.176158ms | 10310056 | 0 | 64.60% | 1.07 | 2387.15 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.718849ms | 11367051 | 0 | 64.66% | 1.07 | 2209.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.633615ms | 12398342 | 0 | 64.65% | 1.07 | 2331.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.603638ms | 12262439 | 2000000 | 64.69% | 1.06 | 1049.83 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.014µs | 10308 | 0 | 64.66% | 1.06 | 1830.03 MB/s |
| Timsort | 1000 | 35.509µs | 10818 | 0 | 64.66% | 1.06 | 1289.15 MB/s |
| ARS Gen 1: Foundation | 1000 | 215.822µs | 458 | 2000 | 64.66% | 1.06 | 212.10 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 242.209µs | 458 | 2000 | 64.66% | 1.06 | 189.00 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 24.944µs | 10308 | 0 | 64.66% | 1.06 | 1835.17 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 25.075µs | 10308 | 0 | 64.66% | 1.06 | 1825.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 25.085µs | 10308 | 0 | 64.66% | 1.06 | 1824.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 35.277µs | 10818 | 0 | 64.66% | 1.06 | 1297.63 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 25.663µs | 10308 | 0 | 64.66% | 1.06 | 1783.75 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 35.904µs | 10818 | 0 | 64.66% | 1.06 | 1274.97 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 25.245µs | 10308 | 0 | 64.66% | 1.06 | 1813.28 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 25.053µs | 10308 | 0 | 64.66% | 1.06 | 1827.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 25.097µs | 10308 | 0 | 64.66% | 1.06 | 1823.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 147.249µs | 10308 | 2000 | 64.66% | 1.06 | 310.88 MB/s |
| Quicksort | 10000 | 311.595µs | 135501 | 0 | 64.65% | 1.06 | 1469.10 MB/s |
| Timsort | 10000 | 387.11µs | 140463 | 0 | 64.65% | 1.06 | 1182.52 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.365517ms | 53061 | 30000 | 64.64% | 1.06 | 335.23 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.89361ms | 53088 | 30000 | 64.63% | 1.06 | 241.74 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 466.935µs | 191553 | 14351 | 64.65% | 1.06 | 980.36 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 267.573µs | 59910 | 10000 | 64.65% | 1.06 | 1710.80 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 159.994µs | 59910 | 0 | 64.65% | 1.06 | 2861.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 169.142µs | 62899 | 0 | 64.64% | 1.06 | 2706.39 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 281.829µs | 59134 | 0 | 64.64% | 1.06 | 1624.26 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 307.027µs | 61884 | 0 | 64.64% | 1.06 | 1490.96 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 176.661µs | 59910 | 0 | 64.65% | 1.06 | 2591.20 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 167.686µs | 59910 | 0 | 64.64% | 1.06 | 2729.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 166.643µs | 59910 | 0 | 64.64% | 1.06 | 2746.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 354.327µs | 59910 | 20000 | 64.64% | 1.06 | 1291.92 MB/s |
| Quicksort | 100000 | 2.596562ms | 1420515 | 0 | 64.59% | 1.06 | 1762.96 MB/s |
| Timsort | 100000 | 3.459742ms | 1424196 | 0 | 64.53% | 1.06 | 1323.12 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.56896ms | 1360088 | 300000 | 64.50% | 1.06 | 433.12 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 11.053462ms | 1360044 | 300000 | 64.50% | 1.06 | 414.14 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.252488ms | 1616363 | 108703 | 64.58% | 1.06 | 1407.43 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.539921ms | 713263 | 100000 | 64.61% | 1.06 | 2972.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.251332ms | 713263 | 0 | 64.61% | 1.06 | 3658.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.474848ms | 718641 | 0 | 64.61% | 1.06 | 3103.80 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.453327ms | 681744 | 0 | 64.60% | 1.06 | 3149.76 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.574634ms | 688930 | 0 | 64.60% | 1.06 | 2907.11 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.408296ms | 713263 | 0 | 64.61% | 1.06 | 3250.48 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.504788ms | 609629 | 0 | 64.61% | 1.06 | 3042.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.559219ms | 713263 | 0 | 64.61% | 1.06 | 2935.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.825285ms | 713263 | 200000 | 64.60% | 1.06 | 2507.90 MB/s |
| Quicksort | 1000000 | 26.604251ms | 13518116 | 0 | 64.13% | 1.07 | 1720.64 MB/s |
| Timsort | 1000000 | 51.294574ms | 14666956 | 0 | 63.57% | 1.06 | 892.42 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.946548ms | 14952891 | 1017407 | 64.27% | 1.06 | 1432.90 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 15.773515ms | 4752528 | 1000000 | 64.68% | 1.05 | 2902.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.391455ms | 4752528 | 0 | 64.68% | 1.05 | 2974.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.082145ms | 4776632 | 0 | 64.61% | 1.04 | 2679.78 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.780426ms | 6258981 | 0 | 64.57% | 1.05 | 2900.83 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 17.766273ms | 6290267 | 0 | 64.38% | 1.05 | 2576.59 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.819927ms | 4706394 | 0 | 64.64% | 1.05 | 2893.59 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 22.115337ms | 2307619 | 0 | 64.54% | 1.04 | 2069.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 21.702829ms | 2550221 | 0 | 64.61% | 1.05 | 2109.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.300298ms | 11495264 | 2000000 | 64.62% | 1.05 | 1033.32 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 22.173µs | 9427 | 0 | 64.66% | 1.04 | 2064.51 MB/s |
| Timsort | 1000 | 24.085µs | 9314 | 0 | 64.66% | 1.04 | 1900.62 MB/s |
| ARS Gen 1: Foundation | 1000 | 109.345µs | 9547 | 2000 | 64.66% | 1.04 | 418.64 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 117.41µs | 9540 | 2000 | 64.66% | 1.04 | 389.88 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 19.592µs | 9427 | 0 | 64.66% | 1.04 | 2336.48 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 19.205µs | 9427 | 0 | 64.66% | 1.04 | 2383.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.772µs | 9427 | 0 | 64.66% | 1.04 | 2438.55 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 21.997µs | 9314 | 0 | 64.66% | 1.04 | 2081.03 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.992µs | 9427 | 0 | 64.66% | 1.04 | 2410.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 21.741µs | 9314 | 0 | 64.66% | 1.04 | 2105.53 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.841µs | 9427 | 0 | 64.66% | 1.04 | 2196.46 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.992µs | 9427 | 0 | 64.66% | 1.04 | 2180.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.379µs | 9427 | 0 | 64.66% | 1.04 | 2246.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 113.322µs | 9427 | 2000 | 64.66% | 1.04 | 403.95 MB/s |
| Quicksort | 10000 | 230.854µs | 133978 | 0 | 64.66% | 1.04 | 1982.91 MB/s |
| Timsort | 10000 | 280.286µs | 128297 | 0 | 64.65% | 1.04 | 1633.20 MB/s |
| ARS Gen 1: Foundation | 10000 | 920.837µs | 126223 | 30000 | 64.65% | 1.04 | 497.12 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 936.991µs | 126108 | 30000 | 64.65% | 1.04 | 488.55 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 416.437µs | 183316 | 14351 | 64.65% | 1.04 | 1099.24 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 216.262µs | 42006 | 10000 | 64.65% | 1.04 | 2116.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 135.304µs | 42006 | 0 | 64.65% | 1.04 | 3383.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 132.916µs | 34856 | 0 | 64.65% | 1.04 | 3444.01 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 251.44µs | 49067 | 0 | 64.65% | 1.04 | 1820.57 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 258.601µs | 42432 | 0 | 64.65% | 1.04 | 1770.15 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 155.06µs | 42006 | 0 | 64.65% | 1.04 | 2952.17 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 153.404µs | 42006 | 0 | 64.65% | 1.04 | 2984.04 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 143.715µs | 42006 | 0 | 64.65% | 1.04 | 3185.22 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 346.469µs | 42006 | 20000 | 64.65% | 1.04 | 1321.23 MB/s |
| Quicksort | 100000 | 3.278155ms | 1688686 | 0 | 64.61% | 1.04 | 1396.41 MB/s |
| Timsort | 100000 | 3.951179ms | 1619959 | 0 | 64.55% | 1.04 | 1158.55 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.623899ms | 1609619 | 300000 | 64.56% | 1.05 | 430.88 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 10.331608ms | 1609452 | 300000 | 64.56% | 1.05 | 443.07 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.031263ms | 1798628 | 108703 | 64.61% | 1.04 | 1510.14 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.593069ms | 801237 | 100000 | 64.62% | 1.04 | 2873.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.419964ms | 801237 | 0 | 64.62% | 1.04 | 3223.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.094919ms | 405369 | 0 | 64.62% | 1.04 | 4180.80 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.578181ms | 872624 | 0 | 64.62% | 1.04 | 2900.58 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.335012ms | 443300 | 0 | 64.61% | 1.04 | 3428.91 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.504078ms | 801237 | 0 | 64.61% | 1.04 | 3043.48 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.529929ms | 689539 | 0 | 64.61% | 1.04 | 2992.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.4748ms | 801237 | 0 | 64.61% | 1.04 | 3103.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.865473ms | 801237 | 200000 | 64.61% | 1.04 | 2453.87 MB/s |
| Quicksort | 1000000 | 42.24976ms | 20499945 | 0 | 64.21% | 1.06 | 1083.47 MB/s |
| Timsort | 1000000 | 63.892283ms | 19254168 | 0 | 63.63% | 1.05 | 716.46 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 30.952447ms | 20728167 | 1017407 | 64.54% | 1.05 | 1478.93 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.687636ms | 9491317 | 1000000 | 64.68% | 1.04 | 2588.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.48289ms | 9491317 | 0 | 64.69% | 1.04 | 2777.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.193887ms | 4131087 | 0 | 64.69% | 1.03 | 3012.81 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.993322ms | 12339609 | 0 | 64.63% | 1.04 | 2693.79 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 14.485208ms | 5760283 | 0 | 64.60% | 1.04 | 3160.21 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 18.256026ms | 9491317 | 0 | 64.65% | 1.04 | 2507.47 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.098297ms | 10583380 | 0 | 64.67% | 1.04 | 2396.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.50588ms | 11681981 | 0 | 64.65% | 1.04 | 2473.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 45.100733ms | 14859474 | 2000000 | 64.60% | 1.04 | 1014.98 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 7.058µs | 3761 | 0 | 64.56% | 1.04 | 6485.74 MB/s |
| Timsort | 1000 | 8.895µs | 3799 | 0 | 64.56% | 1.04 | 5146.30 MB/s |
| ARS Gen 1: Foundation | 1000 | 40.237µs | 995 | 2000 | 64.56% | 1.04 | 1137.67 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 51.275µs | 995 | 2000 | 64.56% | 1.04 | 892.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 6.933µs | 3761 | 0 | 64.56% | 1.04 | 6602.68 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 7.345µs | 3761 | 0 | 64.56% | 1.04 | 6232.32 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 6.83µs | 3761 | 0 | 64.56% | 1.04 | 6702.25 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 9.139µs | 3799 | 0 | 64.56% | 1.04 | 5008.90 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 6.816µs | 3761 | 0 | 64.56% | 1.04 | 6716.02 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 9.147µs | 3799 | 0 | 64.56% | 1.04 | 5004.52 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 6.832µs | 3761 | 0 | 64.56% | 1.04 | 6700.29 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 6.861µs | 3761 | 0 | 64.56% | 1.04 | 6671.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 6.849µs | 3761 | 0 | 64.56% | 1.04 | 6683.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 124.162µs | 3761 | 2000 | 64.55% | 1.04 | 368.68 MB/s |
| Quicksort | 10000 | 66.885µs | 36513 | 0 | 64.55% | 1.04 | 6844.04 MB/s |
| Timsort | 10000 | 87.499µs | 36606 | 0 | 64.55% | 1.04 | 5231.64 MB/s |
| ARS Gen 1: Foundation | 10000 | 246.131µs | 9995 | 30000 | 64.55% | 1.04 | 1859.84 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 292.886µs | 9995 | 30000 | 64.54% | 1.04 | 1562.94 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 307.296µs | 115165 | 14351 | 64.55% | 1.04 | 1489.65 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 213.298µs | 10001 | 10000 | 64.54% | 1.04 | 2146.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 132.311µs | 10001 | 0 | 64.54% | 1.04 | 3459.76 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 123.707µs | 10001 | 0 | 64.54% | 1.04 | 3700.39 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 257.895µs | 10001 | 0 | 64.54% | 1.04 | 1775.00 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 262.382µs | 10001 | 0 | 64.54% | 1.04 | 1744.65 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 139.899µs | 10001 | 0 | 64.54% | 1.04 | 3272.10 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 126.254µs | 10001 | 0 | 64.54% | 1.04 | 3625.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 126.054µs | 10001 | 0 | 64.54% | 1.04 | 3631.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 299.416µs | 10001 | 20000 | 64.54% | 1.04 | 1528.86 MB/s |
| Quicksort | 100000 | 670.34µs | 362118 | 0 | 64.51% | 1.04 | 6828.83 MB/s |
| Timsort | 100000 | 1.140276ms | 362412 | 0 | 64.47% | 1.04 | 4014.50 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.525961ms | 99995 | 300000 | 64.51% | 1.04 | 1812.24 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.739699ms | 99995 | 300000 | 64.51% | 1.04 | 1670.85 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.533256ms | 1131774 | 108703 | 64.51% | 1.04 | 1807.02 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.031048ms | 99999 | 100000 | 64.51% | 1.04 | 4439.79 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 929.077µs | 99999 | 0 | 64.51% | 1.03 | 4927.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 935.73µs | 99999 | 0 | 64.51% | 1.03 | 4892.05 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 997.669µs | 99999 | 0 | 64.51% | 1.04 | 4588.33 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 938.117µs | 99999 | 0 | 64.51% | 1.04 | 4879.60 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.189469ms | 199994 | 0 | 64.51% | 1.03 | 3848.47 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.155815ms | 199994 | 0 | 64.51% | 1.03 | 3960.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.367201ms | 199994 | 0 | 64.51% | 1.03 | 3348.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.38355ms | 99999 | 200000 | 64.50% | 1.04 | 3308.62 MB/s |
| Quicksort | 1000000 | 11.018547ms | 3806932 | 0 | 64.42% | 1.04 | 4154.48 MB/s |
| Timsort | 1000000 | 30.377869ms | 4710561 | 0 | 64.21% | 1.03 | 1506.90 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.961376ms | 12059635 | 1017407 | 64.51% | 1.04 | 1637.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 14.020977ms | 1000001 | 1000000 | 64.57% | 1.03 | 3264.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 13.931369ms | 1000001 | 0 | 64.57% | 1.03 | 3285.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.001624ms | 1000001 | 0 | 64.57% | 1.03 | 3269.36 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.751927ms | 1000001 | 0 | 64.57% | 1.03 | 3589.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.344318ms | 1000001 | 0 | 64.57% | 1.03 | 3708.29 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.115212ms | 1999996 | 0 | 64.58% | 1.03 | 2674.60 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 16.490439ms | 1999996 | 0 | 64.59% | 1.03 | 2775.93 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 16.838391ms | 1999996 | 0 | 64.58% | 1.03 | 2718.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.722133ms | 5316906 | 2000000 | 64.49% | 1.03 | 1046.98 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 12.136µs | 5226 | 0 | 64.45% | 1.03 | 3771.95 MB/s |
| Timsort | 1000 | 16.958µs | 5250 | 0 | 64.45% | 1.03 | 2699.40 MB/s |
| ARS Gen 1: Foundation | 1000 | 49.19µs | 4636 | 2000 | 64.45% | 1.03 | 930.60 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.302µs | 4636 | 2000 | 64.45% | 1.03 | 785.16 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.19µs | 5226 | 0 | 64.45% | 1.03 | 4090.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.441µs | 5226 | 0 | 64.45% | 1.03 | 4001.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.812µs | 5226 | 0 | 64.45% | 1.03 | 4233.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 14.602µs | 5250 | 0 | 64.45% | 1.03 | 3134.94 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 10.472µs | 5226 | 0 | 64.45% | 1.03 | 4371.31 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 13.893µs | 5250 | 0 | 64.45% | 1.03 | 3294.92 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 9.954µs | 5226 | 0 | 64.45% | 1.03 | 4598.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.286µs | 5226 | 0 | 64.45% | 1.03 | 4450.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.289µs | 5226 | 0 | 64.45% | 1.03 | 4449.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 114.124µs | 5226 | 2000 | 64.45% | 1.03 | 401.11 MB/s |
| Quicksort | 10000 | 82.912µs | 53591 | 0 | 64.45% | 1.03 | 5521.08 MB/s |
| Timsort | 10000 | 117.824µs | 53226 | 0 | 64.45% | 1.03 | 3885.15 MB/s |
| ARS Gen 1: Foundation | 10000 | 307.264µs | 55100 | 30000 | 64.44% | 1.03 | 1489.81 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 361.953µs | 55099 | 30000 | 64.44% | 1.03 | 1264.70 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 274.19µs | 125304 | 14351 | 64.44% | 1.03 | 1669.51 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 307.929µs | 52153 | 10000 | 64.44% | 1.03 | 1486.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 212.103µs | 52153 | 0 | 64.44% | 1.03 | 2158.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 233.261µs | 50387 | 0 | 64.44% | 1.03 | 1962.45 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 404.938µs | 42939 | 0 | 64.44% | 1.03 | 1130.45 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 379.912µs | 43078 | 0 | 64.44% | 1.03 | 1204.92 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 216.124µs | 16855 | 0 | 64.44% | 1.03 | 2118.06 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 211.537µs | 52153 | 0 | 64.44% | 1.03 | 2163.99 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 218.267µs | 52153 | 0 | 64.44% | 1.03 | 2097.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 354.234µs | 52153 | 20000 | 64.44% | 1.03 | 1292.26 MB/s |
| Quicksort | 100000 | 824.641µs | 529990 | 0 | 64.42% | 1.03 | 5551.07 MB/s |
| Timsort | 100000 | 1.399976ms | 531868 | 0 | 64.37% | 1.03 | 3269.80 MB/s |
| ARS Gen 1: Foundation | 100000 | 3.22182ms | 501611 | 300000 | 64.41% | 1.03 | 1420.82 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.361187ms | 501611 | 300000 | 64.40% | 1.03 | 1361.91 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.32833ms | 1172752 | 108703 | 64.42% | 1.03 | 1966.06 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.23017ms | 516727 | 100000 | 64.41% | 1.03 | 2052.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.878301ms | 516727 | 0 | 64.40% | 1.03 | 2437.12 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.494856ms | 519617 | 0 | 64.36% | 1.03 | 1834.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.67358ms | 512169 | 0 | 64.40% | 1.03 | 2735.24 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.498539ms | 502679 | 0 | 64.37% | 1.03 | 1832.13 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.302305ms | 206221 | 0 | 64.37% | 1.02 | 1988.28 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.898904ms | 182412 | 0 | 64.39% | 1.03 | 2410.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.615792ms | 200760 | 0 | 64.37% | 1.03 | 1750.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.276274ms | 516727 | 200000 | 64.39% | 1.03 | 2011.02 MB/s |
| Quicksort | 1000000 | 13.776924ms | 5281309 | 0 | 64.28% | 1.03 | 3322.68 MB/s |
| Timsort | 1000000 | 34.507042ms | 6327917 | 0 | 63.99% | 1.03 | 1326.58 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 29.897112ms | 12313781 | 1017407 | 64.41% | 1.03 | 1531.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 29.876495ms | 5208498 | 1000000 | 64.35% | 1.02 | 1532.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 27.894347ms | 5208498 | 0 | 64.33% | 1.02 | 1641.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 50.253911ms | 6511840 | 0 | 64.01% | 1.02 | 910.90 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 27.223738ms | 5225265 | 0 | 64.36% | 1.02 | 1681.49 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 47.571805ms | 6529655 | 0 | 64.02% | 1.02 | 962.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 32.038818ms | 1939650 | 0 | 64.51% | 1.01 | 1428.78 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 45.348879ms | 2064127 | 0 | 64.53% | 1.02 | 1009.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 47.213792ms | 2062304 | 0 | 64.52% | 1.01 | 969.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 52.932811ms | 9705768 | 2000000 | 64.35% | 1.02 | 864.80 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 26.134µs | 10133 | 0 | 64.33% | 1.02 | 1751.60 MB/s |
| Timsort | 1000 | 35.652µs | 10734 | 0 | 64.33% | 1.02 | 1283.98 MB/s |
| ARS Gen 1: Foundation | 1000 | 192.743µs | 691 | 2000 | 64.33% | 1.02 | 237.50 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 212.346µs | 691 | 2000 | 64.33% | 1.02 | 215.57 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 24.008µs | 10133 | 0 | 64.33% | 1.02 | 1906.71 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 23.393µs | 10133 | 0 | 64.33% | 1.02 | 1956.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.503µs | 10133 | 0 | 64.33% | 1.02 | 2034.23 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 31.089µs | 10734 | 0 | 64.33% | 1.02 | 1472.43 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.881µs | 10133 | 0 | 64.33% | 1.02 | 2092.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.458µs | 10734 | 0 | 64.33% | 1.02 | 1553.95 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.8µs | 10133 | 0 | 64.33% | 1.02 | 2200.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.812µs | 10133 | 0 | 64.33% | 1.02 | 2199.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.419µs | 10133 | 0 | 64.33% | 1.02 | 2241.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 122.446µs | 10133 | 2000 | 64.33% | 1.02 | 373.85 MB/s |
| Quicksort | 10000 | 236.302µs | 133996 | 0 | 64.33% | 1.02 | 1937.20 MB/s |
| Timsort | 10000 | 326.383µs | 137398 | 0 | 64.33% | 1.02 | 1402.54 MB/s |
| ARS Gen 1: Foundation | 10000 | 969.105µs | 77629 | 30000 | 64.32% | 1.02 | 472.36 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.173162ms | 77623 | 30000 | 64.32% | 1.02 | 390.20 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 438.33µs | 189660 | 14351 | 64.32% | 1.02 | 1044.34 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 237.355µs | 69470 | 10000 | 64.32% | 1.02 | 1928.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 165.183µs | 69470 | 0 | 64.32% | 1.02 | 2771.25 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 153.579µs | 72482 | 0 | 64.32% | 1.02 | 2980.64 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 315.413µs | 59544 | 0 | 64.32% | 1.02 | 1451.32 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 330.23µs | 62683 | 0 | 64.32% | 1.02 | 1386.20 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 162.162µs | 69470 | 0 | 64.32% | 1.02 | 2822.88 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 139.904µs | 69470 | 0 | 64.32% | 1.02 | 3271.98 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 154.718µs | 69470 | 0 | 64.32% | 1.02 | 2958.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 360.214µs | 69470 | 20000 | 64.32% | 1.02 | 1270.81 MB/s |
| Quicksort | 100000 | 2.038869ms | 1339911 | 0 | 64.29% | 1.03 | 2245.18 MB/s |
| Timsort | 100000 | 3.185595ms | 1340773 | 0 | 64.24% | 1.03 | 1436.98 MB/s |
| ARS Gen 1: Foundation | 100000 | 8.352989ms | 1262245 | 300000 | 64.23% | 1.02 | 548.02 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.74054ms | 1262822 | 300000 | 64.23% | 1.02 | 523.72 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.812357ms | 1543517 | 108703 | 64.28% | 1.02 | 1627.69 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.558158ms | 727700 | 100000 | 64.30% | 1.02 | 2937.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.296227ms | 727700 | 0 | 64.30% | 1.02 | 3531.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.386538ms | 737053 | 0 | 64.30% | 1.02 | 3301.49 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.385458ms | 628770 | 0 | 64.29% | 1.02 | 3304.06 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.572449ms | 634537 | 0 | 64.29% | 1.02 | 2911.15 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.344639ms | 701327 | 0 | 64.30% | 1.02 | 3404.36 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.349792ms | 628891 | 0 | 64.30% | 1.02 | 3391.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.294113ms | 727700 | 0 | 64.30% | 1.02 | 3537.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.658169ms | 727700 | 200000 | 64.29% | 1.02 | 2760.66 MB/s |
| Quicksort | 1000000 | 28.399757ms | 12880459 | 0 | 63.91% | 1.03 | 1611.86 MB/s |
| Timsort | 1000000 | 52.100619ms | 13984642 | 0 | 63.43% | 1.03 | 878.61 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 33.195737ms | 14266844 | 1017407 | 64.04% | 1.03 | 1378.98 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 16.946445ms | 5509338 | 1000000 | 64.35% | 1.02 | 2701.24 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.774108ms | 5509338 | 0 | 64.35% | 1.02 | 2728.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.11065ms | 5538714 | 0 | 64.18% | 1.01 | 2527.59 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.071148ms | 6197232 | 0 | 64.28% | 1.02 | 3037.35 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 17.990256ms | 6228989 | 0 | 64.10% | 1.01 | 2544.51 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.543604ms | 2169826 | 0 | 64.26% | 1.02 | 2030.57 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.377182ms | 1707337 | 0 | 64.28% | 1.01 | 1803.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 24.224439ms | 1857655 | 0 | 64.34% | 1.02 | 1889.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.098892ms | 11949839 | 2000000 | 64.35% | 1.02 | 1038.04 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 24.165µs | 9985 | 0 | 64.37% | 1.02 | 1894.33 MB/s |
| Timsort | 1000 | 33.253µs | 10392 | 0 | 64.37% | 1.02 | 1376.61 MB/s |
| ARS Gen 1: Foundation | 1000 | 123.391µs | 5421 | 2000 | 64.37% | 1.02 | 370.99 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 138.517µs | 5356 | 2000 | 64.37% | 1.02 | 330.47 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 22.751µs | 9985 | 0 | 64.37% | 1.02 | 2012.06 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 23.312µs | 9985 | 0 | 64.37% | 1.02 | 1963.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.078µs | 9985 | 0 | 64.37% | 1.02 | 2073.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.874µs | 10392 | 0 | 64.37% | 1.02 | 1532.31 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.35µs | 9985 | 0 | 64.37% | 1.02 | 2144.09 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.196µs | 10392 | 0 | 64.37% | 1.02 | 1567.90 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.801µs | 9985 | 0 | 64.37% | 1.02 | 2200.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.364µs | 9985 | 0 | 64.37% | 1.02 | 2247.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.119µs | 9985 | 0 | 64.37% | 1.02 | 2275.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 118.728µs | 9985 | 2000 | 64.36% | 1.02 | 385.56 MB/s |
| Quicksort | 10000 | 159.296µs | 107604 | 0 | 64.36% | 1.02 | 2873.67 MB/s |
| Timsort | 10000 | 211.041µs | 109657 | 0 | 64.36% | 1.02 | 2169.07 MB/s |
| ARS Gen 1: Foundation | 10000 | 528.136µs | 73762 | 30000 | 64.35% | 1.02 | 866.75 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 607.783µs | 73552 | 30000 | 64.35% | 1.02 | 753.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 332.551µs | 160276 | 14351 | 64.35% | 1.02 | 1376.52 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 237.826µs | 70340 | 10000 | 64.35% | 1.02 | 1924.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 168.22µs | 70340 | 0 | 64.35% | 1.02 | 2721.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 202.96µs | 71216 | 0 | 64.35% | 1.02 | 2255.44 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 310.902µs | 59344 | 0 | 64.35% | 1.02 | 1472.37 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 292.472µs | 60054 | 0 | 64.35% | 1.02 | 1565.15 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 185.075µs | 70340 | 0 | 64.35% | 1.02 | 2473.40 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 166.848µs | 70340 | 0 | 64.35% | 1.02 | 2743.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 168.221µs | 70340 | 0 | 64.35% | 1.02 | 2721.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 376.44µs | 70340 | 20000 | 64.35% | 1.02 | 1216.03 MB/s |
| Quicksort | 100000 | 1.560835ms | 1011458 | 0 | 64.32% | 1.02 | 2932.81 MB/s |
| Timsort | 100000 | 2.39271ms | 1014769 | 0 | 64.27% | 1.02 | 1913.16 MB/s |
| ARS Gen 1: Foundation | 100000 | 4.423705ms | 696758 | 300000 | 64.32% | 1.02 | 1034.80 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 4.608078ms | 697287 | 300000 | 64.33% | 1.02 | 993.39 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.888395ms | 1231300 | 108703 | 64.32% | 1.02 | 1584.84 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.488138ms | 671477 | 100000 | 64.33% | 1.02 | 3076.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.233749ms | 671477 | 0 | 64.33% | 1.02 | 3710.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.560819ms | 673524 | 0 | 64.32% | 1.01 | 2932.84 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.294322ms | 554286 | 0 | 64.33% | 1.02 | 3536.71 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.643863ms | 555220 | 0 | 64.32% | 1.02 | 2784.68 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.684805ms | 105158 | 0 | 64.32% | 1.01 | 2717.01 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.688707ms | 179970 | 0 | 64.32% | 1.01 | 2710.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.646129ms | 140724 | 0 | 64.32% | 1.01 | 2780.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.833644ms | 671477 | 200000 | 64.32% | 1.02 | 2496.47 MB/s |
| Quicksort | 1000000 | 21.753578ms | 9937773 | 0 | 63.95% | 1.02 | 2104.31 MB/s |
| Timsort | 1000000 | 46.173752ms | 11004404 | 0 | 63.48% | 1.02 | 991.39 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.933667ms | 12334215 | 1017407 | 64.10% | 1.02 | 1433.48 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.586064ms | 4762552 | 1000000 | 64.28% | 1.01 | 2462.94 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.503374ms | 4762552 | 0 | 64.29% | 1.00 | 2473.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 27.620802ms | 4748110 | 0 | 64.14% | 1.00 | 1657.31 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.128005ms | 4888204 | 0 | 64.26% | 1.01 | 2672.60 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 25.610033ms | 4904839 | 0 | 64.09% | 1.00 | 1787.44 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 27.112938ms | 1096506 | 0 | 64.38% | 1.01 | 1688.36 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 29.707156ms | 1053340 | 0 | 64.43% | 1.00 | 1540.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 27.109861ms | 1036862 | 0 | 64.43% | 1.00 | 1688.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.783595ms | 10693783 | 2000000 | 64.24% | 1.01 | 1022.17 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.037µs | 10337 | 0 | 64.24% | 1.01 | 1828.35 MB/s |
| Timsort | 1000 | 36.107µs | 10667 | 0 | 64.24% | 1.01 | 1267.80 MB/s |
| ARS Gen 1: Foundation | 1000 | 255.45µs | 0 | 2000 | 64.24% | 1.01 | 179.20 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 278.528µs | 0 | 2000 | 64.24% | 1.01 | 164.35 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 21.988µs | 10337 | 0 | 64.24% | 1.01 | 2081.88 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 22.131µs | 10337 | 0 | 64.24% | 1.01 | 2068.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.882µs | 10337 | 0 | 64.24% | 1.01 | 2091.96 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 31.309µs | 10667 | 0 | 64.24% | 1.01 | 1462.08 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.7µs | 10337 | 0 | 64.24% | 1.01 | 2016.58 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 31.856µs | 10667 | 0 | 64.24% | 1.01 | 1436.98 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 23.475µs | 10337 | 0 | 64.24% | 1.01 | 1950.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 23.258µs | 10337 | 0 | 64.24% | 1.01 | 1968.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 23.743µs | 10337 | 0 | 64.24% | 1.01 | 1927.99 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 134.879µs | 10337 | 2000 | 64.24% | 1.01 | 339.39 MB/s |
| Quicksort | 10000 | 290.556µs | 137946 | 0 | 64.24% | 1.01 | 1575.47 MB/s |
| Timsort | 10000 | 416.891µs | 142499 | 0 | 64.24% | 1.01 | 1098.04 MB/s |
| ARS Gen 1: Foundation | 10000 | 7.004904ms | 0 | 30000 | 64.05% | 1.01 | 65.35 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.694503ms | 0 | 30000 | 64.04% | 1.01 | 80.39 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 377.723µs | 194806 | 14351 | 64.23% | 1.01 | 1211.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 235.193µs | 52643 | 10000 | 64.23% | 1.01 | 1946.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 153.19µs | 52643 | 0 | 64.23% | 1.01 | 2988.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 150.958µs | 58028 | 0 | 64.23% | 1.01 | 3032.39 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 259.227µs | 60571 | 0 | 64.23% | 1.01 | 1765.88 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 163.881µs | 63560 | 0 | 64.23% | 1.01 | 2793.27 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 160.862µs | 52643 | 0 | 64.23% | 1.01 | 2845.69 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 148.9µs | 52643 | 0 | 64.23% | 1.01 | 3074.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 147.044µs | 52643 | 0 | 64.23% | 1.01 | 3113.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 333.347µs | 52643 | 20000 | 64.23% | 1.01 | 1373.23 MB/s |
| Quicksort | 100000 | 3.397834ms | 1718970 | 0 | 64.19% | 1.01 | 1347.22 MB/s |
| Timsort | 100000 | 5.077513ms | 1756228 | 0 | 64.15% | 1.01 | 901.55 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.88372ms | 5 | 300000 | 62.39% | 1.01 | 104.31 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 47.01145ms | 5 | 300000 | 62.19% | 1.01 | 97.37 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.962572ms | 1893310 | 108703 | 64.19% | 1.01 | 1545.16 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.584819ms | 888976 | 100000 | 64.21% | 1.01 | 2888.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.452477ms | 888976 | 0 | 64.21% | 1.01 | 3151.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.698694ms | 929234 | 0 | 64.21% | 1.01 | 2694.80 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.48681ms | 956140 | 0 | 64.20% | 1.01 | 3078.83 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.687328ms | 992831 | 0 | 64.20% | 1.01 | 2712.95 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.58523ms | 888976 | 0 | 64.20% | 1.01 | 2887.68 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.680833ms | 780493 | 0 | 64.20% | 1.01 | 2723.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.583537ms | 888976 | 0 | 64.20% | 1.01 | 2890.77 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.115399ms | 888976 | 200000 | 64.20% | 1.01 | 2163.96 MB/s |
| Quicksort | 1000000 | 49.71617ms | 20525437 | 0 | 63.83% | 1.02 | 920.75 MB/s |
| Timsort | 1000000 | 79.649683ms | 20897754 | 0 | 63.40% | 1.02 | 574.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 41.513011ms | 21586005 | 1017407 | 63.92% | 1.01 | 1102.70 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.866725ms | 10308690 | 1000000 | 64.22% | 1.01 | 2426.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.529413ms | 10308690 | 0 | 64.22% | 1.00 | 2611.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.069705ms | 10708698 | 0 | 64.19% | 1.01 | 2400.48 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.747294ms | 13018570 | 0 | 64.17% | 1.01 | 2579.34 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.063845ms | 13434605 | 0 | 64.06% | 1.01 | 2173.22 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 20.197545ms | 10308690 | 0 | 64.19% | 1.01 | 2266.43 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.777133ms | 11360616 | 0 | 64.24% | 1.01 | 2314.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.582328ms | 12417054 | 0 | 64.23% | 1.01 | 2337.64 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 47.6074ms | 13756863 | 2000000 | 64.23% | 1.00 | 961.54 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.269µs | 5628 | 0 | 64.31% | 1.00 | 4457.72 MB/s |
| Timsort | 1000 | 12.904µs | 5482 | 0 | 64.31% | 1.00 | 3547.46 MB/s |
| ARS Gen 1: Foundation | 1000 | 51.875µs | 984 | 2000 | 64.31% | 1.00 | 882.44 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.796µs | 984 | 2000 | 64.31% | 1.00 | 778.56 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.385µs | 5628 | 0 | 64.31% | 1.00 | 4407.93 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 9.273µs | 5628 | 0 | 64.31% | 1.00 | 4936.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 9.903µs | 5628 | 0 | 64.31% | 1.00 | 4622.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 12.178µs | 5482 | 0 | 64.31% | 1.00 | 3758.94 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 9.202µs | 5628 | 0 | 64.31% | 1.00 | 4974.61 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 12.037µs | 5482 | 0 | 64.31% | 1.00 | 3802.97 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 9.253µs | 5628 | 0 | 64.31% | 1.00 | 4947.19 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 9.283µs | 5628 | 0 | 64.31% | 1.00 | 4931.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.34µs | 5628 | 0 | 64.31% | 1.00 | 4427.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 118.123µs | 5628 | 2000 | 64.31% | 1.00 | 387.53 MB/s |
| Quicksort | 10000 | 86.336µs | 54006 | 0 | 64.31% | 1.00 | 5302.12 MB/s |
| Timsort | 10000 | 123.642µs | 53486 | 0 | 64.31% | 1.00 | 3702.33 MB/s |
| ARS Gen 1: Foundation | 10000 | 301.247µs | 9984 | 30000 | 64.30% | 1.00 | 1519.56 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 379.352µs | 9984 | 30000 | 64.30% | 1.00 | 1206.70 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 326.141µs | 122898 | 14351 | 64.30% | 1.00 | 1403.58 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 212.945µs | 9990 | 10000 | 64.30% | 1.00 | 2149.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 152.517µs | 9990 | 0 | 64.30% | 1.00 | 3001.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 129.899µs | 9990 | 0 | 64.30% | 1.00 | 3524.00 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 238.617µs | 9990 | 0 | 64.30% | 1.00 | 1918.40 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 228.121µs | 9990 | 0 | 64.30% | 1.00 | 2006.67 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 142.815µs | 9990 | 0 | 64.30% | 1.00 | 3205.29 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 136.74µs | 9990 | 0 | 64.30% | 1.00 | 3347.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 128.37µs | 9990 | 0 | 64.30% | 1.00 | 3565.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 321.534µs | 9990 | 20000 | 64.30% | 1.00 | 1423.69 MB/s |
| Quicksort | 100000 | 986.129µs | 522721 | 0 | 64.27% | 1.00 | 4642.03 MB/s |
| Timsort | 100000 | 1.612104ms | 535563 | 0 | 64.22% | 1.00 | 2839.54 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.820701ms | 99984 | 300000 | 64.28% | 1.00 | 1622.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.997508ms | 99984 | 300000 | 64.28% | 1.00 | 1527.15 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.600909ms | 1145301 | 108703 | 64.28% | 1.00 | 1760.01 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.012756ms | 119528 | 100000 | 64.28% | 1.00 | 4519.98 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 856.321µs | 119528 | 0 | 64.28% | 1.00 | 5345.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 859.602µs | 119779 | 0 | 64.28% | 1.00 | 5325.30 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.096287ms | 99990 | 0 | 64.28% | 1.00 | 4175.58 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 900.45µs | 99990 | 0 | 64.28% | 1.00 | 5083.72 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.255258ms | 199986 | 0 | 64.27% | 1.00 | 3646.77 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.188429ms | 199974 | 0 | 64.28% | 1.00 | 3851.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 986.117µs | 100002 | 0 | 64.28% | 1.00 | 4642.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.455095ms | 119528 | 200000 | 64.26% | 1.00 | 3145.94 MB/s |
| Quicksort | 1000000 | 14.057496ms | 5200332 | 0 | 64.11% | 1.00 | 3256.37 MB/s |
| Timsort | 1000000 | 36.158804ms | 6204510 | 0 | 63.80% | 1.00 | 1265.98 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.287208ms | 12086670 | 1017407 | 64.27% | 1.00 | 1677.58 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 15.061323ms | 999988 | 1000000 | 64.36% | 0.99 | 3039.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.08317ms | 999988 | 0 | 64.36% | 0.99 | 3034.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.866642ms | 999988 | 0 | 64.36% | 0.99 | 3079.13 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.48116ms | 999988 | 0 | 64.34% | 1.00 | 3667.64 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.19316ms | 999988 | 0 | 64.34% | 1.00 | 3754.27 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.258648ms | 1999972 | 0 | 64.34% | 0.99 | 2652.37 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 16.551079ms | 1999972 | 0 | 64.35% | 0.99 | 2765.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 16.791041ms | 1999972 | 0 | 64.34% | 1.00 | 2726.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 41.784688ms | 5484640 | 2000000 | 64.26% | 0.99 | 1095.53 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.863µs | 10337 | 0 | 64.25% | 0.99 | 1769.96 MB/s |
| Timsort | 1000 | 36.685µs | 10667 | 0 | 64.25% | 0.99 | 1247.82 MB/s |
| ARS Gen 1: Foundation | 1000 | 264.773µs | 0 | 2000 | 64.25% | 0.99 | 172.89 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 289.973µs | 0 | 2000 | 64.24% | 0.99 | 157.86 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 21.359µs | 10337 | 0 | 64.25% | 0.99 | 2143.19 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 21.518µs | 10337 | 0 | 64.25% | 0.99 | 2127.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.243µs | 10337 | 0 | 64.25% | 0.99 | 2058.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 31.324µs | 10667 | 0 | 64.25% | 0.99 | 1461.38 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.52µs | 10337 | 0 | 64.25% | 0.99 | 2127.15 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.406µs | 10667 | 0 | 64.25% | 0.99 | 1556.70 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.888µs | 10337 | 0 | 64.25% | 0.99 | 2191.52 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.395µs | 10337 | 0 | 64.25% | 0.99 | 2244.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.77µs | 10337 | 0 | 64.25% | 0.99 | 2315.45 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 116.26µs | 10337 | 2000 | 64.24% | 0.99 | 393.74 MB/s |
| Quicksort | 10000 | 253.977µs | 137946 | 0 | 64.24% | 0.99 | 1802.38 MB/s |
| Timsort | 10000 | 403.554µs | 142499 | 0 | 64.24% | 0.99 | 1134.33 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.506927ms | 0 | 30000 | 64.06% | 0.99 | 70.35 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.515942ms | 0 | 30000 | 64.05% | 0.99 | 70.25 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 421.61µs | 194806 | 14351 | 64.24% | 0.99 | 1085.75 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 246.145µs | 52643 | 10000 | 64.24% | 0.99 | 1859.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 159.974µs | 52643 | 0 | 64.24% | 0.99 | 2861.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 152.721µs | 58028 | 0 | 64.24% | 0.99 | 2997.39 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 259.553µs | 60571 | 0 | 64.23% | 0.99 | 1763.66 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 239.836µs | 63560 | 0 | 64.23% | 0.99 | 1908.65 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 157.175µs | 52643 | 0 | 64.24% | 0.99 | 2912.45 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 146.104µs | 52643 | 0 | 64.24% | 0.99 | 3133.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 152.143µs | 52643 | 0 | 64.23% | 0.99 | 3008.77 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 314.128µs | 52643 | 20000 | 64.23% | 0.99 | 1457.25 MB/s |
| Quicksort | 100000 | 3.264975ms | 1718970 | 0 | 64.20% | 0.99 | 1402.04 MB/s |
| Timsort | 100000 | 4.868687ms | 1756228 | 0 | 64.16% | 0.99 | 940.22 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.78191ms | 5 | 300000 | 62.36% | 0.99 | 109.56 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 43.756241ms | 5 | 300000 | 62.27% | 0.99 | 104.62 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.057828ms | 1893310 | 108703 | 64.19% | 0.99 | 1497.02 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.550312ms | 888976 | 100000 | 64.21% | 0.99 | 2952.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.282266ms | 888976 | 0 | 64.21% | 0.99 | 3569.96 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.567163ms | 929234 | 0 | 64.21% | 0.99 | 2920.97 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.405479ms | 956140 | 0 | 64.21% | 0.99 | 3256.99 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.617431ms | 992831 | 0 | 64.20% | 0.99 | 2830.19 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.461664ms | 888976 | 0 | 64.21% | 0.99 | 3131.80 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.474573ms | 780493 | 0 | 64.21% | 0.99 | 3104.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.397213ms | 888976 | 0 | 64.21% | 0.99 | 3276.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.910772ms | 888976 | 200000 | 64.21% | 0.99 | 2395.70 MB/s |
| Quicksort | 1000000 | 47.90963ms | 20525437 | 0 | 63.88% | 1.00 | 955.47 MB/s |
| Timsort | 1000000 | 78.305095ms | 20897754 | 0 | 63.47% | 1.00 | 584.59 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 37.0756ms | 21586005 | 1017407 | 63.97% | 1.00 | 1234.68 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 19.031459ms | 10308690 | 1000000 | 64.23% | 0.99 | 2405.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.118065ms | 10308690 | 0 | 64.23% | 0.99 | 2674.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.544156ms | 10708698 | 0 | 64.21% | 0.99 | 2468.51 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.833019ms | 13018570 | 0 | 64.19% | 0.99 | 2566.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.572277ms | 13434605 | 0 | 64.09% | 0.99 | 2225.15 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.144856ms | 10308690 | 0 | 64.21% | 0.99 | 2391.05 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.209364ms | 11360616 | 0 | 64.25% | 0.99 | 2265.11 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.862638ms | 12417054 | 0 | 64.24% | 0.99 | 2304.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 49.256701ms | 13756866 | 2000000 | 64.24% | 0.99 | 929.34 MB/s |
