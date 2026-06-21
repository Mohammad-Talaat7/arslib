# ARS Evolution Atlas: Final Research Study

## 1. Experimental Setup
- **Cores:** 8 | **RAM:** 15864 MB
- **PMC Instrumentation:** true (Multi-thread Inherit: Enabled)
- **Statistical Setup:** Reps=7, Seed=42

## Category: i64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 13.218µs | 10227 | 0 | 3.85% | 2.56 | 1154.39 MB/s |
| Timsort | 1000 | 21.939µs | 10588 | 0 | 0.00% | 2.04 | 695.51 MB/s |
| ARS Gen 1: Foundation | 1000 | 268.057µs | 0 | 2000 | 0.77% | 2.10 | 56.92 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 313.223µs | 0 | 2000 | 1.43% | 2.08 | 48.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 13.254µs | 10227 | 0 | 15.15% | 2.53 | 1151.26 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 13.029µs | 10227 | 0 | 17.39% | 2.58 | 1171.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 13µs | 10227 | 0 | 20.00% | 2.59 | 1173.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 19.156µs | 10588 | 0 | 4.96% | 2.04 | 796.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.728µs | 10227 | 0 | 46.15% | 2.52 | 1301.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 19.204µs | 10588 | 0 | 3.33% | 2.03 | 794.56 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.444µs | 10227 | 0 | 7.41% | 2.58 | 1333.34 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.252µs | 10227 | 0 | 0.00% | 2.61 | 1356.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.011µs | 10227 | 0 | 0.00% | 2.61 | 1524.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 107.699µs | 10227 | 2000 | 0.79% | 0.62 | 141.68 MB/s |
| Quicksort | 10000 | 117.623µs | 136654 | 0 | 0.38% | 1.03 | 1297.26 MB/s |
| Timsort | 10000 | 172.354µs | 140327 | 0 | 0.35% | 1.06 | 885.32 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.417023ms | 0 | 30000 | 0.20% | 1.73 | 34.55 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.710035ms | 0 | 30000 | 0.23% | 1.71 | 32.40 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 288.105µs | 193611 | 14351 | 1.07% | 1.07 | 529.63 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 201.626µs | 51695 | 10000 | 0.32% | 0.83 | 756.79 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 128.378µs | 51695 | 0 | 0.21% | 0.72 | 1188.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 134.134µs | 57359 | 0 | 0.36% | 0.78 | 1137.58 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 134.549µs | 59671 | 0 | 0.19% | 0.75 | 1134.07 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 163.427µs | 62214 | 0 | 0.63% | 0.75 | 933.68 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 125.23µs | 51695 | 0 | 0.17% | 0.73 | 1218.46 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 123.217µs | 51695 | 0 | 0.22% | 0.79 | 1238.37 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 135.976µs | 51695 | 0 | 0.26% | 0.79 | 1122.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 296.502µs | 51695 | 20000 | 0.41% | 0.74 | 514.63 MB/s |
| Quicksort | 100000 | 1.381516ms | 1709595 | 0 | 2.80% | 1.61 | 1104.50 MB/s |
| Timsort | 100000 | 1.995909ms | 1743505 | 0 | 2.26% | 1.57 | 764.50 MB/s |
| ARS Gen 1: Foundation | 100000 | 37.915043ms | 0 | 300000 | 1.67% | 0.95 | 40.24 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 40.259707ms | 0 | 300000 | 2.34% | 1.01 | 37.90 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 20.453559ms | 1885062 | 108703 | 18.34% | 1.29 | 74.60 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 8.632609ms | 881353 | 100000 | 15.93% | 1.12 | 176.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.12213ms | 881353 | 0 | 14.54% | 1.08 | 1359.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.189867ms | 921838 | 0 | 14.16% | 1.02 | 1282.39 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.14278ms | 955554 | 0 | 11.30% | 1.08 | 1335.23 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.367167ms | 991979 | 0 | 10.65% | 1.08 | 1116.09 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.163744ms | 881353 | 0 | 11.16% | 1.02 | 1311.18 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.084223ms | 772388 | 0 | 12.16% | 1.01 | 1407.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.034467ms | 881353 | 0 | 10.70% | 0.95 | 1475.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.381839ms | 881353 | 200000 | 14.18% | 1.00 | 1104.24 MB/s |
| Quicksort | 1000000 | 22.144748ms | 20423287 | 0 | 15.25% | 2.16 | 689.05 MB/s |
| Timsort | 1000000 | 32.483793ms | 20813246 | 0 | 19.27% | 1.89 | 469.74 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 25.80478ms | 21493355 | 1017407 | 28.25% | 1.45 | 591.32 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.503776ms | 10218658 | 1000000 | 46.67% | 1.03 | 1326.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.9221ms | 10218658 | 0 | 47.28% | 0.94 | 1537.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.598973ms | 10628212 | 0 | 47.04% | 1.05 | 1589.63 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.331498ms | 13023009 | 0 | 40.85% | 1.15 | 2409.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.061527ms | 13432511 | 0 | 37.99% | 1.08 | 2160.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.341844ms | 10218658 | 0 | 42.80% | 1.01 | 1829.19 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.834383ms | 11276404 | 0 | 47.63% | 1.10 | 1727.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.379203ms | 12320223 | 0 | 48.24% | 0.94 | 1626.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 20.931605ms | 12171637 | 2000000 | 45.71% | 0.92 | 728.98 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.742µs | 10330 | 0 | 39.57% | 0.92 | 1299.51 MB/s |
| Timsort | 1000 | 18.65µs | 10648 | 0 | 39.57% | 0.92 | 818.17 MB/s |
| ARS Gen 1: Foundation | 1000 | 190.419µs | 503 | 2000 | 39.57% | 0.92 | 80.13 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 213.661µs | 503 | 2000 | 39.56% | 0.92 | 71.42 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.564µs | 10330 | 0 | 39.57% | 0.92 | 1319.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.603µs | 10330 | 0 | 39.57% | 0.92 | 1315.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.451µs | 10330 | 0 | 39.57% | 0.92 | 1332.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.167µs | 10648 | 0 | 39.57% | 0.92 | 839.92 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.813µs | 10330 | 0 | 39.57% | 0.92 | 1291.69 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.422µs | 10648 | 0 | 39.57% | 0.92 | 828.29 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 12.039µs | 10330 | 0 | 39.57% | 0.92 | 1267.45 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.671µs | 10330 | 0 | 39.57% | 0.92 | 1307.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.696µs | 10330 | 0 | 39.57% | 0.92 | 1304.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 121.604µs | 10330 | 2000 | 39.50% | 0.92 | 125.48 MB/s |
| Quicksort | 10000 | 141.283µs | 134638 | 0 | 39.19% | 0.92 | 1080.02 MB/s |
| Timsort | 10000 | 214.338µs | 140096 | 0 | 39.18% | 0.92 | 711.90 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.252425ms | 57643 | 30000 | 38.78% | 0.92 | 121.83 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.3203ms | 57632 | 30000 | 38.72% | 0.92 | 115.57 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 325.936µs | 191358 | 14351 | 38.98% | 0.92 | 468.15 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 234.2µs | 61389 | 10000 | 38.86% | 0.92 | 651.53 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 150.48µs | 61389 | 0 | 38.78% | 0.91 | 1014.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 153.977µs | 64672 | 0 | 38.87% | 0.91 | 990.98 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 270.162µs | 58551 | 0 | 38.82% | 0.92 | 564.80 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 286.545µs | 61376 | 0 | 38.80% | 0.92 | 532.51 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 148.78µs | 61389 | 0 | 38.88% | 0.91 | 1025.59 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 150.188µs | 61389 | 0 | 38.81% | 0.91 | 1015.98 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 148.441µs | 61389 | 0 | 38.81% | 0.91 | 1027.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 331.644µs | 61389 | 20000 | 38.65% | 0.91 | 460.10 MB/s |
| Quicksort | 100000 | 1.28356ms | 1446704 | 0 | 37.78% | 0.94 | 1188.79 MB/s |
| Timsort | 100000 | 1.58769ms | 1445193 | 0 | 36.94% | 0.94 | 961.07 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.324191ms | 1387258 | 300000 | 35.47% | 0.94 | 208.33 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.154859ms | 1386968 | 300000 | 35.71% | 0.95 | 187.11 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.221699ms | 1645061 | 108703 | 37.46% | 0.93 | 686.81 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 887.558µs | 734392 | 100000 | 37.29% | 0.93 | 1719.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 718.175µs | 734392 | 0 | 37.89% | 0.92 | 2124.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 753.134µs | 735546 | 0 | 37.29% | 0.92 | 2026.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 826.669µs | 701300 | 0 | 37.40% | 0.92 | 1845.82 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 823.387µs | 706496 | 0 | 37.27% | 0.92 | 1853.17 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 580.935µs | 734392 | 0 | 36.94% | 0.92 | 2626.59 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 655.411µs | 629097 | 0 | 37.11% | 0.92 | 2328.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 712.938µs | 734392 | 0 | 37.16% | 0.92 | 2140.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 995.252µs | 734392 | 200000 | 37.45% | 0.92 | 1533.16 MB/s |
| Quicksort | 1000000 | 9.615846ms | 13567694 | 0 | 32.62% | 1.11 | 1586.84 MB/s |
| Timsort | 1000000 | 14.65876ms | 14681691 | 0 | 29.09% | 1.09 | 1040.93 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 18.257636ms | 14956001 | 1017407 | 35.29% | 1.06 | 835.75 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.779651ms | 4787996 | 1000000 | 43.43% | 0.89 | 2250.67 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.733959ms | 4787996 | 0 | 43.58% | 0.87 | 2265.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.905644ms | 4821847 | 0 | 43.46% | 0.89 | 2209.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.385971ms | 6224416 | 0 | 40.76% | 0.98 | 2833.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.673446ms | 6253879 | 0 | 39.77% | 0.97 | 2689.51 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 6.840407ms | 4757456 | 0 | 41.81% | 0.89 | 2230.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.074485ms | 2295151 | 0 | 40.93% | 0.92 | 1681.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.64072ms | 2529783 | 0 | 40.72% | 0.89 | 1765.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.943848ms | 11703949 | 2000000 | 39.75% | 0.99 | 900.55 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.112µs | 9762 | 0 | 44.92% | 1.21 | 1373.18 MB/s |
| Timsort | 1000 | 15.205µs | 9882 | 0 | 44.92% | 1.21 | 1003.54 MB/s |
| ARS Gen 1: Foundation | 1000 | 90.992µs | 9788 | 2000 | 44.92% | 1.21 | 167.69 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 98.981µs | 9815 | 2000 | 44.92% | 1.21 | 154.16 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 12.262µs | 9762 | 0 | 44.92% | 1.21 | 1244.40 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.608µs | 9762 | 0 | 44.92% | 1.21 | 1314.51 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.509µs | 9762 | 0 | 44.92% | 1.21 | 1325.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 15.444µs | 9882 | 0 | 44.92% | 1.21 | 988.01 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.543µs | 9762 | 0 | 44.92% | 1.21 | 1321.91 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 15.726µs | 9882 | 0 | 44.92% | 1.21 | 970.29 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.388µs | 9762 | 0 | 44.92% | 1.21 | 1339.90 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.323µs | 9762 | 0 | 44.92% | 1.21 | 1347.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.15µs | 9762 | 0 | 44.92% | 1.21 | 1368.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 124.642µs | 9762 | 2000 | 44.90% | 1.21 | 122.42 MB/s |
| Quicksort | 10000 | 141.345µs | 134689 | 0 | 44.80% | 1.21 | 1079.54 MB/s |
| Timsort | 10000 | 186.057µs | 132195 | 0 | 44.79% | 1.21 | 820.11 MB/s |
| ARS Gen 1: Foundation | 10000 | 853.977µs | 130386 | 30000 | 44.68% | 1.22 | 178.68 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 920.406µs | 130325 | 30000 | 44.66% | 1.21 | 165.78 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 331.554µs | 187157 | 14351 | 44.72% | 1.21 | 460.22 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 210.325µs | 45304 | 10000 | 44.68% | 1.21 | 725.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 137.551µs | 45304 | 0 | 44.67% | 1.21 | 1109.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 135.162µs | 36417 | 0 | 44.68% | 1.21 | 1128.93 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 275.627µs | 52081 | 0 | 44.69% | 1.21 | 553.60 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 276.787µs | 47021 | 0 | 44.67% | 1.21 | 551.28 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 143.985µs | 45304 | 0 | 44.68% | 1.21 | 1059.75 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 141.749µs | 45304 | 0 | 44.68% | 1.21 | 1076.47 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 142.736µs | 45304 | 0 | 44.67% | 1.21 | 1069.02 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 314.836µs | 45304 | 20000 | 44.61% | 1.21 | 484.66 MB/s |
| Quicksort | 100000 | 1.604953ms | 1716043 | 0 | 44.21% | 1.22 | 950.73 MB/s |
| Timsort | 100000 | 1.947476ms | 1660908 | 0 | 43.84% | 1.22 | 783.52 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.083121ms | 1643878 | 300000 | 43.40% | 1.23 | 215.42 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 7.755372ms | 1643640 | 300000 | 43.47% | 1.23 | 196.75 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.064878ms | 1830188 | 108703 | 44.05% | 1.22 | 738.97 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 909.646µs | 827444 | 100000 | 43.95% | 1.21 | 1677.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 620.242µs | 827444 | 0 | 43.91% | 1.21 | 2460.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 505.843µs | 410171 | 0 | 43.85% | 1.21 | 3016.51 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 695.375µs | 906132 | 0 | 43.96% | 1.21 | 2194.33 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 644.013µs | 448015 | 0 | 43.99% | 1.21 | 2369.33 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 694.531µs | 827444 | 0 | 43.91% | 1.21 | 2196.99 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 675.744µs | 718138 | 0 | 43.94% | 1.21 | 2258.07 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 651.131µs | 827444 | 0 | 43.83% | 1.21 | 2343.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.121265ms | 827444 | 200000 | 43.98% | 1.21 | 1360.85 MB/s |
| Quicksort | 1000000 | 17.707549ms | 20672771 | 0 | 41.99% | 1.33 | 861.71 MB/s |
| Timsort | 1000000 | 24.651023ms | 19775927 | 0 | 39.20% | 1.32 | 618.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.45693ms | 20984698 | 1017407 | 43.85% | 1.26 | 784.23 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.213538ms | 9742173 | 1000000 | 45.56% | 1.14 | 1656.13 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.490153ms | 9742173 | 0 | 45.62% | 1.14 | 1797.23 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.362018ms | 4127840 | 0 | 45.57% | 1.13 | 2072.64 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.068464ms | 12610499 | 0 | 44.71% | 1.22 | 2158.71 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.207499ms | 5755875 | 0 | 44.52% | 1.20 | 2458.12 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.330428ms | 9742173 | 0 | 45.19% | 1.16 | 1831.69 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.883563ms | 10843448 | 0 | 45.50% | 1.17 | 1717.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.352109ms | 11954018 | 0 | 45.40% | 1.15 | 1631.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.109407ms | 15033625 | 2000000 | 44.77% | 1.18 | 842.59 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 3.219µs | 3735 | 0 | 45.23% | 1.30 | 4740.23 MB/s |
| Timsort | 1000 | 4.61µs | 3747 | 0 | 45.23% | 1.30 | 3309.93 MB/s |
| ARS Gen 1: Foundation | 1000 | 31.524µs | 995 | 2000 | 45.23% | 1.30 | 484.04 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 42.061µs | 995 | 2000 | 45.23% | 1.30 | 362.78 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 3.127µs | 3735 | 0 | 45.23% | 1.30 | 4879.69 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 3.162µs | 3735 | 0 | 45.23% | 1.30 | 4825.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 3.285µs | 3735 | 0 | 45.23% | 1.30 | 4644.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 4.584µs | 3747 | 0 | 45.23% | 1.30 | 3328.71 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 3.306µs | 3735 | 0 | 45.23% | 1.30 | 4615.48 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 4.404µs | 3747 | 0 | 45.23% | 1.30 | 3464.76 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 3.176µs | 3735 | 0 | 45.23% | 1.30 | 4804.40 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 3.143µs | 3735 | 0 | 45.23% | 1.30 | 4854.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 3.149µs | 3735 | 0 | 45.23% | 1.30 | 4845.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 107.843µs | 3735 | 2000 | 45.21% | 1.30 | 141.49 MB/s |
| Quicksort | 10000 | 28.6µs | 36573 | 0 | 45.16% | 1.30 | 5335.24 MB/s |
| Timsort | 10000 | 37.839µs | 36775 | 0 | 45.16% | 1.30 | 4032.56 MB/s |
| ARS Gen 1: Foundation | 10000 | 198.722µs | 9995 | 30000 | 45.14% | 1.30 | 767.85 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 254.349µs | 9995 | 30000 | 45.14% | 1.30 | 599.92 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 250.438µs | 115988 | 14351 | 45.13% | 1.30 | 609.28 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 221.133µs | 9999 | 10000 | 45.11% | 1.30 | 690.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 144.84µs | 9999 | 0 | 45.11% | 1.30 | 1053.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 154.969µs | 9999 | 0 | 45.11% | 1.30 | 984.63 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 275.537µs | 9999 | 0 | 45.11% | 1.30 | 553.78 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 266.961µs | 9999 | 0 | 45.11% | 1.30 | 571.57 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 153.574µs | 9999 | 0 | 45.11% | 1.30 | 993.58 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 153.246µs | 9999 | 0 | 45.11% | 1.30 | 995.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 125.71µs | 9999 | 0 | 45.11% | 1.30 | 1213.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 307.205µs | 9999 | 20000 | 45.07% | 1.30 | 496.70 MB/s |
| Quicksort | 100000 | 294.63µs | 362094 | 0 | 44.89% | 1.30 | 5178.97 MB/s |
| Timsort | 100000 | 409.218µs | 382517 | 0 | 44.70% | 1.30 | 3728.77 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.062866ms | 99995 | 300000 | 44.89% | 1.29 | 1435.63 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.447767ms | 99995 | 300000 | 44.89% | 1.30 | 1053.95 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.81619ms | 1129938 | 108703 | 44.86% | 1.30 | 840.15 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 613.323µs | 100001 | 100000 | 44.71% | 1.30 | 2487.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 342.786µs | 100001 | 0 | 44.72% | 1.30 | 4451.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 356.209µs | 100001 | 0 | 44.72% | 1.30 | 4283.66 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 573.305µs | 100001 | 0 | 44.76% | 1.29 | 2661.55 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 610.668µs | 100001 | 0 | 44.77% | 1.29 | 2498.70 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 551.636µs | 199996 | 0 | 44.61% | 1.30 | 2766.10 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 660.8µs | 199996 | 0 | 44.83% | 1.29 | 2309.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 559.978µs | 199996 | 0 | 44.63% | 1.29 | 2724.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 789.81µs | 100001 | 200000 | 44.75% | 1.29 | 1931.96 MB/s |
| Quicksort | 1000000 | 3.209274ms | 3809528 | 0 | 44.11% | 1.31 | 4754.59 MB/s |
| Timsort | 1000000 | 6.835947ms | 4510660 | 0 | 43.90% | 1.30 | 2232.14 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.327234ms | 12062959 | 1017407 | 44.59% | 1.32 | 789.50 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.342866ms | 999999 | 1000000 | 45.89% | 1.23 | 2405.66 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.672972ms | 999999 | 0 | 45.93% | 1.24 | 2286.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.468854ms | 999999 | 0 | 45.91% | 1.24 | 2358.81 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.431407ms | 999999 | 0 | 45.28% | 1.27 | 3443.33 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.411935ms | 999999 | 0 | 45.25% | 1.26 | 3458.53 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.298333ms | 1999994 | 0 | 46.06% | 1.22 | 1838.78 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.072424ms | 1999994 | 0 | 45.83% | 1.23 | 2157.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.285784ms | 1999994 | 0 | 46.06% | 1.22 | 1841.56 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.408405ms | 5364815 | 2000000 | 43.31% | 1.28 | 929.94 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.11µs | 5508 | 0 | 43.98% | 1.27 | 2986.06 MB/s |
| Timsort | 1000 | 7.229µs | 5460 | 0 | 43.98% | 1.27 | 2110.77 MB/s |
| ARS Gen 1: Foundation | 1000 | 31.009µs | 4914 | 2000 | 43.98% | 1.27 | 492.08 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 39.008µs | 4914 | 2000 | 43.98% | 1.27 | 391.17 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 4.972µs | 5508 | 0 | 43.98% | 1.27 | 3068.94 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 6.198µs | 5508 | 0 | 43.98% | 1.27 | 2461.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.537µs | 5508 | 0 | 43.98% | 1.27 | 2755.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 8.536µs | 5460 | 0 | 43.98% | 1.27 | 1787.58 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.201µs | 5508 | 0 | 43.98% | 1.27 | 2933.82 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.004µs | 5460 | 0 | 43.98% | 1.27 | 2178.58 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 4.541µs | 5508 | 0 | 43.98% | 1.27 | 3360.23 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 4.574µs | 5508 | 0 | 43.98% | 1.27 | 3335.98 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 4.911µs | 5508 | 0 | 43.98% | 1.27 | 3107.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 101.25µs | 5508 | 2000 | 43.97% | 1.27 | 150.70 MB/s |
| Quicksort | 10000 | 37.586µs | 53621 | 0 | 43.91% | 1.27 | 4059.70 MB/s |
| Timsort | 10000 | 51.417µs | 53742 | 0 | 43.91% | 1.27 | 2967.65 MB/s |
| ARS Gen 1: Foundation | 10000 | 246.453µs | 50132 | 30000 | 43.89% | 1.27 | 619.14 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 280.602µs | 50259 | 30000 | 43.89% | 1.27 | 543.79 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 226.335µs | 124917 | 14351 | 43.89% | 1.27 | 674.17 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 257.698µs | 52500 | 10000 | 43.86% | 1.27 | 592.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 173.026µs | 52500 | 0 | 43.87% | 1.27 | 881.88 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 167.406µs | 51829 | 0 | 43.86% | 1.27 | 911.48 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 359.113µs | 42054 | 0 | 43.86% | 1.27 | 424.90 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 349.281µs | 42636 | 0 | 43.86% | 1.27 | 436.86 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 191.579µs | 16860 | 0 | 43.82% | 1.27 | 796.48 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 164.677µs | 52500 | 0 | 43.86% | 1.27 | 926.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 158.434µs | 52500 | 0 | 43.86% | 1.27 | 963.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 299.042µs | 52500 | 20000 | 43.83% | 1.27 | 510.26 MB/s |
| Quicksort | 100000 | 374.203µs | 532062 | 0 | 43.78% | 1.27 | 4077.68 MB/s |
| Timsort | 100000 | 492.981µs | 535405 | 0 | 43.61% | 1.27 | 3095.21 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.547086ms | 506805 | 300000 | 43.74% | 1.27 | 986.29 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.735791ms | 506783 | 300000 | 43.72% | 1.28 | 879.07 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.724983ms | 1174310 | 108703 | 43.74% | 1.28 | 884.58 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.369175ms | 519466 | 100000 | 43.58% | 1.27 | 1114.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 800.086µs | 519466 | 0 | 43.57% | 1.27 | 1907.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.077973ms | 520212 | 0 | 43.47% | 1.27 | 1415.51 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.052624ms | 499545 | 0 | 43.62% | 1.27 | 1449.60 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.140082ms | 502501 | 0 | 43.50% | 1.27 | 1338.39 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.171346ms | 203055 | 0 | 43.25% | 1.27 | 1302.67 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 915.32µs | 182074 | 0 | 43.38% | 1.27 | 1667.04 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.384707ms | 197448 | 0 | 43.22% | 1.27 | 1101.95 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.197403ms | 519466 | 200000 | 43.54% | 1.27 | 1274.32 MB/s |
| Quicksort | 1000000 | 4.201199ms | 5301519 | 0 | 43.17% | 1.29 | 3632.01 MB/s |
| Timsort | 1000000 | 8.41263ms | 6302942 | 0 | 42.49% | 1.28 | 1813.80 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.821628ms | 12308876 | 1017407 | 43.67% | 1.29 | 732.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 13.696018ms | 5221477 | 1000000 | 44.23% | 1.22 | 1114.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.508944ms | 5221477 | 0 | 44.18% | 1.23 | 1451.98 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.590804ms | 6004244 | 0 | 43.51% | 1.23 | 1045.78 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.943894ms | 5265586 | 0 | 43.85% | 1.27 | 1920.82 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.338468ms | 6045570 | 0 | 43.34% | 1.26 | 1236.68 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.334818ms | 1938046 | 0 | 45.27% | 1.19 | 995.04 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 22.251084ms | 2076365 | 0 | 45.77% | 1.22 | 685.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.111973ms | 2063926 | 0 | 45.89% | 1.19 | 607.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.597899ms | 9792110 | 2000000 | 43.99% | 1.23 | 919.32 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.468µs | 10296 | 0 | 44.24% | 1.37 | 1457.66 MB/s |
| Timsort | 1000 | 17.714µs | 10670 | 0 | 44.24% | 1.37 | 861.40 MB/s |
| ARS Gen 1: Foundation | 1000 | 158.546µs | 808 | 2000 | 44.24% | 1.37 | 96.24 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 184.59µs | 808 | 2000 | 44.24% | 1.37 | 82.66 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.389µs | 10296 | 0 | 44.24% | 1.37 | 1468.74 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.452µs | 10296 | 0 | 44.24% | 1.37 | 1459.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.257µs | 10296 | 0 | 44.24% | 1.37 | 1487.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.199µs | 10670 | 0 | 44.24% | 1.37 | 887.19 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 10.416µs | 10296 | 0 | 44.24% | 1.37 | 1464.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 17.161µs | 10670 | 0 | 44.24% | 1.37 | 889.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.276µs | 10296 | 0 | 44.24% | 1.37 | 1484.90 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.294µs | 10296 | 0 | 44.24% | 1.37 | 1482.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.153µs | 10296 | 0 | 44.24% | 1.37 | 1368.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 105.199µs | 10296 | 2000 | 44.23% | 1.37 | 145.05 MB/s |
| Quicksort | 10000 | 127.452µs | 134101 | 0 | 44.19% | 1.37 | 1197.22 MB/s |
| Timsort | 10000 | 193.026µs | 137729 | 0 | 44.19% | 1.37 | 790.50 MB/s |
| ARS Gen 1: Foundation | 10000 | 923.481µs | 84429 | 30000 | 44.14% | 1.37 | 165.23 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.015251ms | 84430 | 30000 | 44.14% | 1.37 | 150.30 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 323.42µs | 190005 | 14351 | 44.16% | 1.37 | 471.79 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 230.856µs | 71389 | 10000 | 44.15% | 1.37 | 660.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 140.745µs | 71389 | 0 | 44.14% | 1.37 | 1084.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 153.315µs | 73990 | 0 | 44.14% | 1.37 | 995.26 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 338.168µs | 60048 | 0 | 44.15% | 1.37 | 451.22 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 372.856µs | 62612 | 0 | 44.14% | 1.37 | 409.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 147.059µs | 71389 | 0 | 44.14% | 1.37 | 1037.60 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 150.79µs | 71389 | 0 | 44.15% | 1.37 | 1011.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 169.77µs | 71389 | 0 | 44.15% | 1.37 | 898.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 359.029µs | 71389 | 20000 | 44.13% | 1.37 | 425.00 MB/s |
| Quicksort | 100000 | 1.218516ms | 1353942 | 0 | 44.01% | 1.37 | 1252.24 MB/s |
| Timsort | 100000 | 1.49222ms | 1358979 | 0 | 43.89% | 1.37 | 1022.56 MB/s |
| ARS Gen 1: Foundation | 100000 | 5.86615ms | 1260666 | 300000 | 43.75% | 1.35 | 260.12 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 6.615326ms | 1260598 | 300000 | 43.73% | 1.35 | 230.66 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.168448ms | 1555111 | 108703 | 43.98% | 1.37 | 703.67 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 814.948µs | 735888 | 100000 | 43.94% | 1.36 | 1872.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 657.758µs | 735888 | 0 | 43.92% | 1.36 | 2319.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 741.448µs | 741765 | 0 | 43.98% | 1.36 | 2057.97 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 777.792µs | 651349 | 0 | 43.93% | 1.36 | 1961.81 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 893.203µs | 657321 | 0 | 43.91% | 1.36 | 1708.32 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 673.074µs | 710308 | 0 | 43.89% | 1.36 | 2267.03 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 707.449µs | 631417 | 0 | 43.92% | 1.36 | 2156.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 719.47µs | 735888 | 0 | 43.94% | 1.36 | 2120.84 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.087745ms | 735888 | 200000 | 43.91% | 1.36 | 1402.79 MB/s |
| Quicksort | 1000000 | 9.300531ms | 12909957 | 0 | 43.05% | 1.40 | 1640.64 MB/s |
| Timsort | 1000000 | 14.330371ms | 14007926 | 0 | 41.89% | 1.39 | 1064.79 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.13496ms | 14286900 | 1017407 | 43.35% | 1.38 | 797.43 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.564282ms | 5157050 | 1000000 | 44.73% | 1.32 | 2017.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.186526ms | 5157050 | 0 | 44.72% | 1.31 | 2123.25 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.451659ms | 5175392 | 0 | 44.58% | 1.32 | 2047.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.499517ms | 6007232 | 0 | 44.31% | 1.35 | 2774.57 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.843641ms | 6044896 | 0 | 44.10% | 1.35 | 2611.18 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.109855ms | 2361022 | 0 | 43.94% | 1.32 | 1674.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.72855ms | 1866734 | 0 | 44.15% | 1.31 | 1568.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.981273ms | 2025491 | 0 | 44.12% | 1.30 | 1698.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.232544ms | 11985588 | 2000000 | 43.95% | 1.35 | 940.01 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.2µs | 10451 | 0 | 44.47% | 1.41 | 1362.39 MB/s |
| Timsort | 1000 | 18.542µs | 10742 | 0 | 44.47% | 1.41 | 822.93 MB/s |
| ARS Gen 1: Foundation | 1000 | 100.709µs | 5331 | 2000 | 44.47% | 1.41 | 151.51 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 117.897µs | 5339 | 2000 | 44.47% | 1.41 | 129.42 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.296µs | 10451 | 0 | 44.47% | 1.41 | 1350.81 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.016µs | 10451 | 0 | 44.47% | 1.41 | 1385.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.948µs | 10451 | 0 | 44.47% | 1.41 | 1393.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.198µs | 10742 | 0 | 44.47% | 1.41 | 838.49 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.203µs | 10451 | 0 | 44.47% | 1.41 | 1362.03 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.437µs | 10742 | 0 | 44.47% | 1.41 | 827.62 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.293µs | 10451 | 0 | 44.47% | 1.41 | 1351.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.278µs | 10451 | 0 | 44.47% | 1.41 | 1352.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.995µs | 10451 | 0 | 44.47% | 1.41 | 1387.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 116.585µs | 10451 | 2000 | 44.47% | 1.41 | 130.88 MB/s |
| Quicksort | 10000 | 104.633µs | 111159 | 0 | 44.44% | 1.41 | 1458.32 MB/s |
| Timsort | 10000 | 140.938µs | 110728 | 0 | 44.43% | 1.41 | 1082.66 MB/s |
| ARS Gen 1: Foundation | 10000 | 448.253µs | 75427 | 30000 | 44.42% | 1.41 | 340.41 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 560.6µs | 74701 | 30000 | 44.41% | 1.41 | 272.19 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 284.806µs | 163143 | 14351 | 44.42% | 1.41 | 535.76 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 231.754µs | 72583 | 10000 | 44.41% | 1.41 | 658.40 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 150.522µs | 72583 | 0 | 44.41% | 1.41 | 1013.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.687µs | 72287 | 0 | 44.40% | 1.41 | 949.60 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 257.262µs | 63448 | 0 | 44.40% | 1.41 | 593.12 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 286.33µs | 63348 | 0 | 44.40% | 1.41 | 532.91 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 156.889µs | 72583 | 0 | 44.41% | 1.41 | 972.59 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 156.928µs | 72583 | 0 | 44.40% | 1.41 | 972.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 151.995µs | 72583 | 0 | 44.41% | 1.41 | 1003.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 321.549µs | 72583 | 20000 | 44.39% | 1.41 | 474.54 MB/s |
| Quicksort | 100000 | 830.547µs | 1016581 | 0 | 44.31% | 1.41 | 1837.20 MB/s |
| Timsort | 100000 | 967.039µs | 1021185 | 0 | 44.22% | 1.41 | 1577.89 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.268906ms | 680916 | 300000 | 44.34% | 1.40 | 672.52 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.600499ms | 680031 | 300000 | 44.33% | 1.40 | 586.76 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.892533ms | 1237724 | 108703 | 44.25% | 1.41 | 806.26 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 898.163µs | 631252 | 100000 | 44.26% | 1.41 | 1698.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 563.341µs | 631252 | 0 | 44.23% | 1.41 | 2708.62 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 627.69µs | 634097 | 0 | 44.24% | 1.41 | 2430.94 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 684.56µs | 555626 | 0 | 44.26% | 1.41 | 2228.99 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 706.448µs | 562372 | 0 | 44.25% | 1.41 | 2159.93 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 722.016µs | 134521 | 0 | 44.11% | 1.41 | 2113.36 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 722.323µs | 169903 | 0 | 44.16% | 1.41 | 2112.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 692.873µs | 264519 | 0 | 44.17% | 1.41 | 2202.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.106695ms | 631252 | 200000 | 44.23% | 1.41 | 1378.77 MB/s |
| Quicksort | 1000000 | 7.445777ms | 9921218 | 0 | 43.57% | 1.43 | 2049.32 MB/s |
| Timsort | 1000000 | 11.882451ms | 11000160 | 0 | 42.65% | 1.42 | 1284.14 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.526053ms | 12348632 | 1017407 | 43.67% | 1.41 | 708.85 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.26675ms | 5346522 | 1000000 | 44.73% | 1.36 | 2099.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.083828ms | 5346522 | 0 | 44.74% | 1.37 | 2154.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.956214ms | 5363683 | 0 | 44.26% | 1.36 | 1917.85 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.636314ms | 5434749 | 0 | 44.51% | 1.39 | 2707.23 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.162099ms | 5451863 | 0 | 43.97% | 1.39 | 2476.23 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.469855ms | 1070102 | 0 | 44.53% | 1.34 | 1457.40 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 12.432281ms | 1041286 | 0 | 45.15% | 1.35 | 1227.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.264673ms | 1013215 | 0 | 45.17% | 1.34 | 1354.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.429194ms | 11100982 | 2000000 | 44.47% | 1.38 | 928.76 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.267µs | 10179 | 0 | 44.35% | 1.42 | 1354.29 MB/s |
| Timsort | 1000 | 18.067µs | 10913 | 0 | 44.35% | 1.42 | 844.57 MB/s |
| ARS Gen 1: Foundation | 1000 | 249.729µs | 0 | 2000 | 44.35% | 1.42 | 61.10 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 270.234µs | 0 | 2000 | 44.35% | 1.42 | 56.47 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.215µs | 10179 | 0 | 44.35% | 1.42 | 1360.57 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.038µs | 10179 | 0 | 44.35% | 1.42 | 1382.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.183µs | 10179 | 0 | 44.35% | 1.42 | 1364.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.917µs | 10913 | 0 | 44.35% | 1.42 | 851.64 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.305µs | 10179 | 0 | 44.35% | 1.42 | 1349.74 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.711µs | 10913 | 0 | 44.35% | 1.42 | 815.50 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.319µs | 10179 | 0 | 44.35% | 1.42 | 1348.07 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.025µs | 10179 | 0 | 44.35% | 1.42 | 1384.02 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.046µs | 10179 | 0 | 44.35% | 1.42 | 1381.39 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 120.196µs | 10179 | 2000 | 44.35% | 1.42 | 126.95 MB/s |
| Quicksort | 10000 | 140.757µs | 137738 | 0 | 44.32% | 1.42 | 1084.05 MB/s |
| Timsort | 10000 | 213.184µs | 141392 | 0 | 44.32% | 1.42 | 715.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.440954ms | 0 | 30000 | 43.98% | 1.43 | 28.04 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.841911ms | 0 | 30000 | 43.98% | 1.43 | 26.12 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 338.632µs | 193231 | 14351 | 44.30% | 1.42 | 450.60 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 230.197µs | 51645 | 10000 | 44.29% | 1.42 | 662.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 162.444µs | 51645 | 0 | 44.29% | 1.42 | 939.33 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 161.978µs | 57426 | 0 | 44.28% | 1.42 | 942.03 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 287.068µs | 59080 | 0 | 44.29% | 1.42 | 531.54 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 299.367µs | 61965 | 0 | 44.29% | 1.42 | 509.70 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 156.351µs | 51645 | 0 | 44.29% | 1.42 | 975.93 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 149.02µs | 51645 | 0 | 44.28% | 1.42 | 1023.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 156.743µs | 51645 | 0 | 44.29% | 1.42 | 973.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 334.919µs | 51645 | 20000 | 44.27% | 1.42 | 455.60 MB/s |
| Quicksort | 100000 | 1.681048ms | 1704558 | 0 | 44.17% | 1.42 | 907.70 MB/s |
| Timsort | 100000 | 2.307361ms | 1748721 | 0 | 44.08% | 1.42 | 661.31 MB/s |
| ARS Gen 1: Foundation | 100000 | 42.480241ms | 6 | 300000 | 35.56% | 1.36 | 35.92 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.028435ms | 6 | 300000 | 34.69% | 1.35 | 33.15 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.437387ms | 1886207 | 108703 | 44.14% | 1.42 | 626.03 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 966.506µs | 879882 | 100000 | 44.13% | 1.42 | 1578.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 705.983µs | 879882 | 0 | 44.12% | 1.42 | 2161.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 849.338µs | 922129 | 0 | 44.11% | 1.42 | 1796.55 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 727.266µs | 954423 | 0 | 44.12% | 1.42 | 2098.10 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 898.782µs | 993675 | 0 | 44.11% | 1.42 | 1697.72 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 701.753µs | 879882 | 0 | 44.11% | 1.42 | 2174.38 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 755.312µs | 773088 | 0 | 44.10% | 1.42 | 2020.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 791.885µs | 879882 | 0 | 44.13% | 1.42 | 1926.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.110814ms | 879882 | 200000 | 44.13% | 1.42 | 1373.66 MB/s |
| Quicksort | 1000000 | 18.274521ms | 20437271 | 0 | 43.53% | 1.45 | 834.98 MB/s |
| Timsort | 1000000 | 26.46187ms | 20799465 | 0 | 42.74% | 1.45 | 576.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 23.644319ms | 21505010 | 1017407 | 43.77% | 1.43 | 645.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.462855ms | 10221412 | 1000000 | 44.48% | 1.38 | 1612.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.51233ms | 10221412 | 0 | 44.48% | 1.38 | 1792.55 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.270437ms | 10628930 | 0 | 44.48% | 1.38 | 1645.96 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.884363ms | 12929332 | 0 | 44.30% | 1.40 | 2216.44 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.733638ms | 13335182 | 0 | 44.28% | 1.40 | 1747.13 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.405216ms | 10221412 | 0 | 44.30% | 1.38 | 1815.40 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.955244ms | 11275443 | 0 | 44.52% | 1.39 | 1703.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.466928ms | 12322876 | 0 | 44.51% | 1.38 | 1611.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.352902ms | 13517886 | 2000000 | 44.58% | 1.38 | 601.86 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.006µs | 5504 | 0 | 44.87% | 1.36 | 3048.10 MB/s |
| Timsort | 1000 | 7.05µs | 5497 | 0 | 44.87% | 1.36 | 2164.37 MB/s |
| ARS Gen 1: Foundation | 1000 | 44.227µs | 984 | 2000 | 44.87% | 1.36 | 345.01 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.595µs | 984 | 2000 | 44.87% | 1.36 | 260.41 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 4.921µs | 5504 | 0 | 44.87% | 1.36 | 3100.75 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.207µs | 5504 | 0 | 44.87% | 1.36 | 2930.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.286µs | 5504 | 0 | 44.87% | 1.36 | 2886.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 6.853µs | 5497 | 0 | 44.87% | 1.36 | 2226.59 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.074µs | 5504 | 0 | 44.87% | 1.36 | 3007.25 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 6.909µs | 5497 | 0 | 44.87% | 1.36 | 2208.54 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 4.861µs | 5504 | 0 | 44.87% | 1.36 | 3139.02 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 4.853µs | 5504 | 0 | 44.87% | 1.36 | 3144.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 4.809µs | 5504 | 0 | 44.87% | 1.36 | 3172.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 114.015µs | 5504 | 2000 | 44.86% | 1.36 | 133.83 MB/s |
| Quicksort | 10000 | 41.534µs | 53753 | 0 | 44.84% | 1.36 | 3673.81 MB/s |
| Timsort | 10000 | 52.43µs | 54514 | 0 | 44.84% | 1.36 | 2910.32 MB/s |
| ARS Gen 1: Foundation | 10000 | 251.061µs | 9984 | 30000 | 44.83% | 1.36 | 607.77 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 331.133µs | 9984 | 30000 | 44.83% | 1.36 | 460.81 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 244.88µs | 121806 | 14351 | 44.83% | 1.36 | 623.11 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 200.698µs | 12063 | 10000 | 44.82% | 1.36 | 760.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 123.017µs | 12063 | 0 | 44.82% | 1.36 | 1240.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 125.674µs | 12087 | 0 | 44.82% | 1.36 | 1214.16 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 251.693µs | 12063 | 0 | 44.82% | 1.36 | 606.25 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 253.882µs | 12087 | 0 | 44.82% | 1.36 | 601.02 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 123.757µs | 12063 | 0 | 44.82% | 1.36 | 1232.96 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 124.608µs | 12063 | 0 | 44.82% | 1.36 | 1224.54 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 132.974µs | 12063 | 0 | 44.82% | 1.36 | 1147.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 310.403µs | 12063 | 20000 | 44.81% | 1.36 | 491.58 MB/s |
| Quicksort | 100000 | 430.929µs | 522910 | 0 | 44.74% | 1.36 | 3540.91 MB/s |
| Timsort | 100000 | 542.342µs | 516617 | 0 | 44.67% | 1.36 | 2813.50 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.316686ms | 99984 | 300000 | 44.76% | 1.36 | 1158.88 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.600886ms | 99984 | 300000 | 44.75% | 1.36 | 953.15 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.788269ms | 1144941 | 108703 | 44.73% | 1.36 | 853.27 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 566.295µs | 144579 | 100000 | 44.67% | 1.36 | 2694.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 366.741µs | 144579 | 0 | 44.69% | 1.36 | 4160.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 350.146µs | 145223 | 0 | 44.68% | 1.36 | 4357.84 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 506.643µs | 99988 | 0 | 44.70% | 1.36 | 3011.74 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 502.76µs | 99988 | 0 | 44.69% | 1.36 | 3035.00 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 535.065µs | 199988 | 0 | 44.65% | 1.36 | 2851.76 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 494.136µs | 199972 | 0 | 44.68% | 1.36 | 3087.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 395.317µs | 100004 | 0 | 44.67% | 1.36 | 3859.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 745.599µs | 144579 | 200000 | 44.68% | 1.36 | 2046.51 MB/s |
| Quicksort | 1000000 | 4.061813ms | 5201420 | 0 | 44.26% | 1.37 | 3756.64 MB/s |
| Timsort | 1000000 | 7.930612ms | 6174589 | 0 | 43.87% | 1.37 | 1924.04 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.523439ms | 12089713 | 1017407 | 44.60% | 1.37 | 781.56 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.166564ms | 999990 | 1000000 | 45.16% | 1.32 | 2474.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.388831ms | 999990 | 0 | 45.19% | 1.33 | 2388.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.395461ms | 999990 | 0 | 45.17% | 1.33 | 2385.88 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.325754ms | 999990 | 0 | 44.92% | 1.35 | 3527.43 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.183367ms | 999990 | 0 | 44.92% | 1.35 | 3647.49 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.826198ms | 1999974 | 0 | 45.11% | 1.32 | 1949.71 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 6.896677ms | 1999974 | 0 | 45.05% | 1.34 | 2212.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.534941ms | 1999984 | 0 | 45.11% | 1.32 | 2025.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 17.097868ms | 5706274 | 2000000 | 44.15% | 1.36 | 892.44 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.216µs | 10179 | 0 | 44.45% | 1.35 | 1360.45 MB/s |
| Timsort | 1000 | 17.713µs | 10913 | 0 | 44.45% | 1.35 | 861.45 MB/s |
| ARS Gen 1: Foundation | 1000 | 233.694µs | 0 | 2000 | 44.45% | 1.35 | 65.29 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 269.337µs | 0 | 2000 | 44.44% | 1.35 | 56.65 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.112µs | 10179 | 0 | 44.45% | 1.35 | 1373.18 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.543µs | 10179 | 0 | 44.45% | 1.35 | 1321.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.098µs | 10179 | 0 | 44.45% | 1.35 | 1374.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.018µs | 10913 | 0 | 44.45% | 1.35 | 846.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.289µs | 10179 | 0 | 44.45% | 1.35 | 1351.65 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 17.958µs | 10913 | 0 | 44.45% | 1.35 | 849.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.939µs | 10179 | 0 | 44.45% | 1.35 | 1394.90 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.049µs | 10179 | 0 | 44.45% | 1.35 | 1381.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.859µs | 10179 | 0 | 44.45% | 1.35 | 1405.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 113.904µs | 10179 | 2000 | 44.44% | 1.35 | 133.96 MB/s |
| Quicksort | 10000 | 140.488µs | 137738 | 0 | 44.42% | 1.35 | 1086.13 MB/s |
| Timsort | 10000 | 209.187µs | 141392 | 0 | 44.42% | 1.35 | 729.43 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.383685ms | 0 | 30000 | 44.18% | 1.36 | 28.34 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.779789ms | 0 | 30000 | 44.06% | 1.36 | 26.40 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 337.803µs | 193231 | 14351 | 44.41% | 1.35 | 451.71 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 236.503µs | 51645 | 10000 | 44.40% | 1.35 | 645.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 148.67µs | 51645 | 0 | 44.39% | 1.35 | 1026.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 153.492µs | 57426 | 0 | 44.39% | 1.35 | 994.11 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 299.207µs | 59080 | 0 | 44.40% | 1.35 | 509.97 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 309.833µs | 61965 | 0 | 44.39% | 1.35 | 492.48 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 161.752µs | 51645 | 0 | 44.40% | 1.35 | 943.34 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 149.35µs | 51645 | 0 | 44.39% | 1.35 | 1021.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 150.464µs | 51645 | 0 | 44.39% | 1.35 | 1014.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 346.485µs | 51645 | 20000 | 44.38% | 1.35 | 440.39 MB/s |
| Quicksort | 100000 | 1.717246ms | 1704558 | 0 | 44.31% | 1.35 | 888.56 MB/s |
| Timsort | 100000 | 2.313214ms | 1748721 | 0 | 44.25% | 1.35 | 659.64 MB/s |
| ARS Gen 1: Foundation | 100000 | 40.918996ms | 6 | 300000 | 35.78% | 1.29 | 37.29 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.520663ms | 6 | 300000 | 35.85% | 1.29 | 34.27 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.469499ms | 1886207 | 108703 | 44.30% | 1.35 | 617.89 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 939.332µs | 879882 | 100000 | 44.27% | 1.35 | 1624.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 698.955µs | 879882 | 0 | 44.27% | 1.35 | 2183.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 746.228µs | 922129 | 0 | 44.26% | 1.35 | 2044.79 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 737.045µs | 954423 | 0 | 44.28% | 1.35 | 2070.27 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 821.074µs | 993675 | 0 | 44.28% | 1.35 | 1858.39 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 642.862µs | 879882 | 0 | 44.24% | 1.35 | 2373.57 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 658.959µs | 773088 | 0 | 44.25% | 1.35 | 2315.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 683.07µs | 879882 | 0 | 44.25% | 1.35 | 2233.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.150395ms | 879882 | 200000 | 44.27% | 1.35 | 1326.40 MB/s |
| Quicksort | 1000000 | 17.802183ms | 20437271 | 0 | 43.74% | 1.38 | 857.13 MB/s |
| Timsort | 1000000 | 29.729449ms | 20799465 | 0 | 43.09% | 1.37 | 513.26 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.571468ms | 21505010 | 1017407 | 44.03% | 1.36 | 707.36 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.265205ms | 10221412 | 1000000 | 44.55% | 1.33 | 1646.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.640544ms | 10221412 | 0 | 44.55% | 1.32 | 1765.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.448346ms | 10628930 | 0 | 44.54% | 1.32 | 1614.97 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.794784ms | 12929332 | 0 | 44.43% | 1.34 | 2245.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.26995ms | 13335182 | 0 | 44.36% | 1.34 | 1845.09 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.435018ms | 10221412 | 0 | 44.43% | 1.33 | 1808.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.317061ms | 11275443 | 0 | 44.61% | 1.33 | 1637.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.486542ms | 12322876 | 0 | 44.59% | 1.33 | 1608.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.686475ms | 13517882 | 2000000 | 44.32% | 1.33 | 618.10 MB/s |

## Category: f64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.182µs | 10325 | 0 | 44.71% | 1.31 | 839.23 MB/s |
| Timsort | 1000 | 26.513µs | 10521 | 0 | 44.70% | 1.31 | 575.52 MB/s |
| ARS Gen 1: Foundation | 1000 | 237.103µs | 0 | 2000 | 44.70% | 1.31 | 64.36 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 276.573µs | 0 | 2000 | 44.70% | 1.31 | 55.17 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.017µs | 10325 | 0 | 44.70% | 1.31 | 896.68 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.54µs | 10325 | 0 | 44.70% | 1.31 | 869.94 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.336µs | 10325 | 0 | 44.70% | 1.31 | 880.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.419µs | 10521 | 0 | 44.70% | 1.31 | 577.57 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.151µs | 10325 | 0 | 44.71% | 1.31 | 840.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 26.201µs | 10521 | 0 | 44.70% | 1.31 | 582.37 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.315µs | 10325 | 0 | 44.70% | 1.31 | 833.13 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.397µs | 10325 | 0 | 44.70% | 1.31 | 877.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.438µs | 10325 | 0 | 44.70% | 1.31 | 875.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 118.514µs | 10325 | 2000 | 44.70% | 1.31 | 128.75 MB/s |
| Quicksort | 10000 | 226.449µs | 136464 | 0 | 44.68% | 1.31 | 673.83 MB/s |
| Timsort | 10000 | 316.721µs | 141512 | 0 | 44.68% | 1.31 | 481.77 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.451161ms | 0 | 30000 | 44.46% | 1.32 | 27.99 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.861644ms | 0 | 30000 | 44.37% | 1.32 | 26.03 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 408.602µs | 193135 | 14351 | 44.67% | 1.31 | 373.44 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 259.314µs | 73138 | 10000 | 44.66% | 1.31 | 588.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 161.636µs | 73138 | 0 | 44.66% | 1.31 | 944.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 174.718µs | 76380 | 0 | 44.66% | 1.31 | 873.34 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 229.684µs | 62698 | 0 | 44.66% | 1.31 | 664.34 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 248.088µs | 65867 | 0 | 44.66% | 1.31 | 615.06 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 168.525µs | 73138 | 0 | 44.66% | 1.31 | 905.43 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 166.758µs | 73138 | 0 | 44.66% | 1.31 | 915.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 166.524µs | 73138 | 0 | 44.66% | 1.31 | 916.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 339.921µs | 73138 | 20000 | 44.65% | 1.31 | 448.89 MB/s |
| Quicksort | 100000 | 2.598476ms | 1705718 | 0 | 44.59% | 1.32 | 587.22 MB/s |
| Timsort | 100000 | 3.42566ms | 1751732 | 0 | 44.52% | 1.32 | 445.43 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.608388ms | 0 | 300000 | 37.02% | 1.27 | 36.67 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.799965ms | 0 | 300000 | 36.44% | 1.27 | 34.06 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.164008ms | 1884272 | 108703 | 44.58% | 1.31 | 482.26 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.223864ms | 1101865 | 100000 | 44.58% | 1.31 | 1246.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 959.1µs | 1101865 | 0 | 44.56% | 1.31 | 1590.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.136692ms | 1142841 | 0 | 44.58% | 1.31 | 1342.39 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 952.78µs | 1002379 | 0 | 44.56% | 1.31 | 1601.50 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.100324ms | 1045724 | 0 | 44.56% | 1.31 | 1386.75 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 952.843µs | 1101865 | 0 | 44.56% | 1.31 | 1601.40 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 939.431µs | 999614 | 0 | 44.56% | 1.31 | 1624.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 930.704µs | 1101865 | 0 | 44.55% | 1.31 | 1639.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.373273ms | 1101865 | 200000 | 44.55% | 1.31 | 1111.13 MB/s |
| Quicksort | 1000000 | 28.821471ms | 20430901 | 0 | 44.22% | 1.34 | 529.42 MB/s |
| Timsort | 1000000 | 42.077795ms | 20822215 | 0 | 43.75% | 1.34 | 362.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.730963ms | 21498086 | 1017407 | 44.33% | 1.32 | 550.24 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.087093ms | 12665814 | 1000000 | 44.93% | 1.30 | 1376.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.108616ms | 12665814 | 0 | 44.94% | 1.30 | 1509.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.881844ms | 13081361 | 0 | 44.87% | 1.29 | 1284.21 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.057957ms | 13583765 | 0 | 44.75% | 1.32 | 1684.57 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.317338ms | 14002566 | 0 | 44.63% | 1.31 | 1348.27 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.810699ms | 6406252 | 0 | 44.58% | 1.29 | 1411.45 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.566738ms | 5861815 | 0 | 44.56% | 1.30 | 1319.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.384399ms | 7398340 | 0 | 44.63% | 1.29 | 1340.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.920791ms | 14470210 | 2000000 | 44.88% | 1.29 | 612.29 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.979µs | 10345 | 0 | 44.73% | 1.28 | 803.98 MB/s |
| Timsort | 1000 | 27.213µs | 10685 | 0 | 44.73% | 1.28 | 560.72 MB/s |
| ARS Gen 1: Foundation | 1000 | 243.619µs | 0 | 2000 | 44.72% | 1.28 | 62.63 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 289.482µs | 0 | 2000 | 44.72% | 1.28 | 52.71 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.881µs | 10345 | 0 | 44.73% | 1.28 | 853.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.177µs | 10345 | 0 | 44.73% | 1.28 | 839.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.964µs | 10345 | 0 | 44.73% | 1.28 | 849.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.07µs | 10685 | 0 | 44.73% | 1.28 | 563.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.754µs | 10345 | 0 | 44.73% | 1.28 | 813.63 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.588µs | 10685 | 0 | 44.73% | 1.28 | 553.10 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.932µs | 10345 | 0 | 44.73% | 1.28 | 805.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.875µs | 10345 | 0 | 44.73% | 1.28 | 853.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.72µs | 10345 | 0 | 44.73% | 1.28 | 861.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 130.082µs | 10345 | 2000 | 44.72% | 1.28 | 117.30 MB/s |
| Quicksort | 10000 | 235.046µs | 137462 | 0 | 44.70% | 1.28 | 649.18 MB/s |
| Timsort | 10000 | 327.843µs | 141011 | 0 | 44.70% | 1.28 | 465.43 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.5461ms | 0 | 30000 | 44.48% | 1.29 | 27.51 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.277408ms | 0 | 30000 | 44.43% | 1.29 | 24.31 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 437.255µs | 192671 | 14351 | 44.69% | 1.28 | 348.97 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 422.551µs | 125399 | 10000 | 44.69% | 1.28 | 361.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 313.78µs | 125399 | 0 | 44.69% | 1.28 | 486.29 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 342.787µs | 130052 | 0 | 44.68% | 1.28 | 445.14 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 409.387µs | 109718 | 0 | 44.69% | 1.28 | 372.72 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 435.286µs | 113881 | 0 | 44.69% | 1.28 | 350.55 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 259.386µs | 48812 | 0 | 44.66% | 1.28 | 588.27 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 311.369µs | 125399 | 0 | 44.69% | 1.28 | 490.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 307.517µs | 125399 | 0 | 44.68% | 1.28 | 496.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 488.087µs | 125399 | 20000 | 44.68% | 1.28 | 312.62 MB/s |
| Quicksort | 100000 | 2.623782ms | 1710455 | 0 | 44.64% | 1.29 | 581.56 MB/s |
| Timsort | 100000 | 3.402683ms | 1746462 | 0 | 44.58% | 1.28 | 448.43 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.016213ms | 0 | 300000 | 37.59% | 1.25 | 35.47 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 49.915956ms | 0 | 300000 | 37.45% | 1.25 | 30.57 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.03235ms | 1884751 | 108703 | 44.63% | 1.28 | 503.20 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.267376ms | 1586392 | 100000 | 44.59% | 1.29 | 672.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.677888ms | 1586392 | 0 | 44.60% | 1.28 | 909.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.165891ms | 1629438 | 0 | 44.59% | 1.28 | 704.50 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.382493ms | 1447738 | 0 | 44.62% | 1.28 | 1103.72 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.743784ms | 1487078 | 0 | 44.60% | 1.28 | 875.04 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.125641ms | 834504 | 0 | 44.53% | 1.28 | 1355.56 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.14964ms | 657220 | 0 | 44.52% | 1.28 | 1327.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.181508ms | 834504 | 0 | 44.53% | 1.28 | 1291.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.035122ms | 1586392 | 200000 | 44.59% | 1.28 | 749.77 MB/s |
| Quicksort | 1000000 | 29.078656ms | 20420624 | 0 | 44.30% | 1.31 | 524.74 MB/s |
| Timsort | 1000000 | 42.551081ms | 20810565 | 0 | 43.82% | 1.30 | 358.60 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.947484ms | 21491076 | 1017407 | 44.46% | 1.29 | 566.24 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 20.71882ms | 17729670 | 1000000 | 44.80% | 1.29 | 736.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.059403ms | 17729670 | 0 | 44.79% | 1.29 | 844.92 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 23.663816ms | 18126422 | 0 | 44.35% | 1.28 | 644.82 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 13.036866ms | 17798278 | 0 | 44.65% | 1.29 | 1170.43 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.748057ms | 18171061 | 0 | 44.36% | 1.28 | 813.89 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 14.844683ms | 9157468 | 0 | 44.84% | 1.25 | 1027.90 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.55453ms | 9620349 | 0 | 45.15% | 1.27 | 980.99 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 15.248081ms | 11727786 | 0 | 45.25% | 1.26 | 1000.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 27.44031ms | 19223480 | 2000000 | 44.83% | 1.27 | 556.07 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.565µs | 9762 | 0 | 44.71% | 1.29 | 868.70 MB/s |
| Timsort | 1000 | 23.035µs | 9882 | 0 | 44.71% | 1.29 | 662.42 MB/s |
| ARS Gen 1: Foundation | 1000 | 121.313µs | 0 | 2000 | 44.70% | 1.29 | 125.78 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 165.052µs | 0 | 2000 | 44.70% | 1.29 | 92.45 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.894µs | 9762 | 0 | 44.71% | 1.29 | 903.21 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.091µs | 9762 | 0 | 44.71% | 1.29 | 892.80 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.925µs | 9762 | 0 | 44.71% | 1.29 | 901.55 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 23.195µs | 9882 | 0 | 44.71% | 1.29 | 657.85 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.552µs | 9762 | 0 | 44.71% | 1.29 | 869.35 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 22.543µs | 9882 | 0 | 44.71% | 1.29 | 676.87 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.62µs | 9762 | 0 | 44.71% | 1.29 | 865.99 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 16.733µs | 9762 | 0 | 44.71% | 1.29 | 911.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.561µs | 9762 | 0 | 44.71% | 1.29 | 921.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 123.156µs | 9762 | 2000 | 44.70% | 1.29 | 123.90 MB/s |
| Quicksort | 10000 | 223.665µs | 134689 | 0 | 44.69% | 1.29 | 682.22 MB/s |
| Timsort | 10000 | 282.081µs | 132195 | 0 | 44.68% | 1.29 | 540.94 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.265615ms | 0 | 30000 | 44.58% | 1.29 | 67.35 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.437588ms | 0 | 30000 | 44.56% | 1.29 | 62.60 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 392.777µs | 187157 | 14351 | 44.67% | 1.29 | 388.48 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 484.216µs | 129133 | 10000 | 44.67% | 1.29 | 315.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 421.876µs | 129133 | 0 | 44.67% | 1.29 | 361.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 439.395µs | 124389 | 0 | 44.67% | 1.29 | 347.27 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 290.847µs | 112273 | 0 | 44.67% | 1.29 | 524.63 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 320.668µs | 109531 | 0 | 44.67% | 1.29 | 475.84 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 234.94µs | 51743 | 0 | 44.65% | 1.29 | 649.48 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 423.719µs | 129133 | 0 | 44.67% | 1.29 | 360.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 382.705µs | 129133 | 0 | 44.67% | 1.29 | 398.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 556.31µs | 129133 | 20000 | 44.66% | 1.29 | 274.29 MB/s |
| Quicksort | 100000 | 2.609659ms | 1716043 | 0 | 44.61% | 1.29 | 584.70 MB/s |
| Timsort | 100000 | 3.039078ms | 1660908 | 0 | 44.56% | 1.29 | 502.09 MB/s |
| ARS Gen 1: Foundation | 100000 | 18.497686ms | 0 | 300000 | 42.65% | 1.28 | 82.49 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 20.405807ms | 0 | 300000 | 42.43% | 1.29 | 74.78 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.971624ms | 1830188 | 108703 | 44.62% | 1.29 | 513.48 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.508292ms | 1653890 | 100000 | 44.59% | 1.29 | 434.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.834367ms | 1653890 | 0 | 44.56% | 1.29 | 538.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.371975ms | 1589383 | 0 | 44.53% | 1.29 | 452.52 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.615237ms | 1472393 | 0 | 44.60% | 1.29 | 944.68 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.860318ms | 1387582 | 0 | 44.57% | 1.29 | 820.22 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.095391ms | 815713 | 0 | 44.51% | 1.28 | 1393.00 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.131397ms | 631229 | 0 | 44.52% | 1.29 | 1348.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.228778ms | 815713 | 0 | 44.49% | 1.29 | 1241.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.493279ms | 1653890 | 200000 | 44.56% | 1.29 | 436.80 MB/s |
| Quicksort | 1000000 | 28.738521ms | 20672771 | 0 | 44.30% | 1.32 | 530.95 MB/s |
| Timsort | 1000000 | 38.194344ms | 19775927 | 0 | 43.76% | 1.31 | 399.50 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.521347ms | 20984698 | 1017407 | 44.62% | 1.30 | 554.43 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 20.333568ms | 18442598 | 1000000 | 44.76% | 1.29 | 750.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.536332ms | 18442598 | 0 | 44.80% | 1.29 | 870.12 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 23.268935ms | 17501336 | 0 | 44.43% | 1.28 | 655.76 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.307163ms | 18449113 | 0 | 44.57% | 1.31 | 935.71 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.355156ms | 17575612 | 0 | 44.22% | 1.30 | 714.52 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 13.831686ms | 8914015 | 0 | 45.10% | 1.27 | 1103.18 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.691066ms | 9611874 | 0 | 45.21% | 1.28 | 972.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.35745ms | 11855374 | 0 | 45.23% | 1.27 | 879.09 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.998069ms | 16617913 | 2000000 | 44.72% | 1.29 | 763.01 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 4.758µs | 3735 | 0 | 44.59% | 1.28 | 3206.98 MB/s |
| Timsort | 1000 | 6.812µs | 3747 | 0 | 44.59% | 1.28 | 2239.99 MB/s |
| ARS Gen 1: Foundation | 1000 | 32.389µs | 995 | 2000 | 44.59% | 1.28 | 471.11 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 32.322µs | 995 | 2000 | 44.59% | 1.28 | 472.09 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 4.614µs | 3735 | 0 | 44.59% | 1.28 | 3307.06 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 4.821µs | 3735 | 0 | 44.59% | 1.28 | 3165.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.026µs | 3735 | 0 | 44.59% | 1.28 | 3035.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 6.76µs | 3747 | 0 | 44.59% | 1.28 | 2257.22 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.311µs | 3735 | 0 | 44.59% | 1.28 | 2873.05 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.085µs | 3747 | 0 | 44.59% | 1.28 | 2153.68 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5.128µs | 3735 | 0 | 44.59% | 1.28 | 2975.58 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 4.92µs | 3735 | 0 | 44.59% | 1.28 | 3101.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 4.747µs | 3735 | 0 | 44.59% | 1.28 | 3214.41 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 116.279µs | 3735 | 2000 | 44.58% | 1.28 | 131.23 MB/s |
| Quicksort | 10000 | 45.088µs | 36573 | 0 | 44.57% | 1.28 | 3384.22 MB/s |
| Timsort | 10000 | 64.112µs | 36775 | 0 | 44.57% | 1.28 | 2380.02 MB/s |
| ARS Gen 1: Foundation | 10000 | 284.949µs | 9995 | 30000 | 44.56% | 1.28 | 535.49 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 347.56µs | 9995 | 30000 | 44.56% | 1.28 | 439.03 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 287.326µs | 115988 | 14351 | 44.56% | 1.28 | 531.06 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 211.48µs | 9999 | 10000 | 44.55% | 1.28 | 721.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 126.13µs | 9999 | 0 | 44.55% | 1.28 | 1209.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 128.24µs | 9999 | 0 | 44.56% | 1.28 | 1189.86 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 247.373µs | 9999 | 0 | 44.55% | 1.28 | 616.83 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 265.426µs | 9999 | 0 | 44.56% | 1.28 | 574.88 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 131.704µs | 9999 | 0 | 44.55% | 1.28 | 1158.57 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 128.858µs | 9999 | 0 | 44.56% | 1.28 | 1184.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 127.549µs | 9999 | 0 | 44.55% | 1.28 | 1196.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 308.376µs | 9999 | 20000 | 44.54% | 1.28 | 494.81 MB/s |
| Quicksort | 100000 | 465.684µs | 362094 | 0 | 44.49% | 1.28 | 3276.64 MB/s |
| Timsort | 100000 | 640.389µs | 382517 | 0 | 44.44% | 1.28 | 2382.74 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.184343ms | 99995 | 300000 | 44.49% | 1.28 | 1288.38 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.31235ms | 99995 | 300000 | 44.49% | 1.28 | 1162.71 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.162078ms | 1129938 | 108703 | 44.48% | 1.28 | 705.75 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 468.619µs | 100001 | 100000 | 44.44% | 1.28 | 3256.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 375.158µs | 100001 | 0 | 44.44% | 1.28 | 4067.30 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 396.534µs | 100001 | 0 | 44.44% | 1.28 | 3848.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 613.226µs | 100001 | 0 | 44.45% | 1.28 | 2488.28 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 602.111µs | 100001 | 0 | 44.46% | 1.28 | 2534.22 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 688.331µs | 199996 | 0 | 44.42% | 1.28 | 2216.78 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 513.802µs | 199996 | 0 | 44.44% | 1.28 | 2969.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 605.737µs | 199996 | 0 | 44.42% | 1.28 | 2519.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 822.869µs | 100001 | 200000 | 44.43% | 1.28 | 1854.34 MB/s |
| Quicksort | 1000000 | 4.304057ms | 3809528 | 0 | 44.26% | 1.28 | 3545.21 MB/s |
| Timsort | 1000000 | 8.03946ms | 4510660 | 0 | 44.22% | 1.28 | 1897.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.360466ms | 12062959 | 1017407 | 44.39% | 1.28 | 626.38 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.161849ms | 999999 | 1000000 | 44.78% | 1.26 | 2476.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.383104ms | 999999 | 0 | 44.78% | 1.26 | 2390.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.351165ms | 999999 | 0 | 44.78% | 1.27 | 2402.52 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.71145ms | 999999 | 0 | 44.57% | 1.27 | 3238.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.755938ms | 999999 | 0 | 44.57% | 1.27 | 3208.37 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.531278ms | 1999994 | 0 | 44.83% | 1.26 | 1788.57 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.12581ms | 1999994 | 0 | 44.74% | 1.27 | 2141.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.633342ms | 1999994 | 0 | 44.83% | 1.26 | 1767.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 17.954845ms | 5364815 | 2000000 | 44.22% | 1.28 | 849.84 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 8.975µs | 5508 | 0 | 44.45% | 1.28 | 1700.14 MB/s |
| Timsort | 1000 | 11.437µs | 5460 | 0 | 44.45% | 1.28 | 1334.16 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.28µs | 921 | 2000 | 44.45% | 1.28 | 322.73 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 53.123µs | 921 | 2000 | 44.45% | 1.28 | 287.24 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 8.067µs | 5508 | 0 | 44.45% | 1.28 | 1891.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 8.967µs | 5508 | 0 | 44.45% | 1.28 | 1701.66 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 8.584µs | 5508 | 0 | 44.45% | 1.28 | 1777.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.387µs | 5460 | 0 | 44.45% | 1.28 | 1340.02 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 8.073µs | 5508 | 0 | 44.45% | 1.28 | 1890.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 11.61µs | 5460 | 0 | 44.45% | 1.28 | 1314.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 8.926µs | 5508 | 0 | 44.45% | 1.28 | 1709.48 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 8.034µs | 5508 | 0 | 44.45% | 1.28 | 1899.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.798µs | 5508 | 0 | 44.45% | 1.28 | 1956.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 105.932µs | 5508 | 2000 | 44.45% | 1.28 | 144.04 MB/s |
| Quicksort | 10000 | 67.747µs | 53621 | 0 | 44.43% | 1.28 | 2252.32 MB/s |
| Timsort | 10000 | 92.95µs | 53742 | 0 | 44.43% | 1.28 | 1641.61 MB/s |
| ARS Gen 1: Foundation | 10000 | 295.291µs | 9683 | 30000 | 44.42% | 1.28 | 516.74 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 330.331µs | 9683 | 30000 | 44.42% | 1.28 | 461.92 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 292.418µs | 124917 | 14351 | 44.43% | 1.28 | 521.81 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 201.503µs | 10961 | 10000 | 44.42% | 1.28 | 757.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 121.045µs | 10961 | 0 | 44.42% | 1.28 | 1260.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 129.032µs | 11013 | 0 | 44.42% | 1.28 | 1182.56 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 318.713µs | 13929 | 0 | 44.42% | 1.28 | 478.76 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 334.118µs | 14152 | 0 | 44.42% | 1.28 | 456.69 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 127.894µs | 15115 | 0 | 44.42% | 1.28 | 1193.08 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 128.385µs | 10961 | 0 | 44.42% | 1.28 | 1188.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 126.145µs | 10961 | 0 | 44.42% | 1.28 | 1209.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 305.295µs | 10961 | 20000 | 44.41% | 1.28 | 499.80 MB/s |
| Quicksort | 100000 | 664.312µs | 532062 | 0 | 44.36% | 1.28 | 2296.93 MB/s |
| Timsort | 100000 | 908.386µs | 535405 | 0 | 44.32% | 1.28 | 1679.77 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.878012ms | 98733 | 300000 | 44.35% | 1.28 | 812.50 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.949537ms | 98733 | 300000 | 44.35% | 1.28 | 782.69 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.179562ms | 1174310 | 108703 | 44.35% | 1.28 | 700.09 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 577.395µs | 122228 | 100000 | 44.32% | 1.28 | 2642.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 387.686µs | 122228 | 0 | 44.31% | 1.28 | 3935.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 394.521µs | 122352 | 0 | 44.31% | 1.28 | 3867.67 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 641.519µs | 151498 | 0 | 44.33% | 1.28 | 2378.54 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 662.701µs | 152054 | 0 | 44.33% | 1.28 | 2302.51 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 555.266µs | 192482 | 0 | 44.30% | 1.28 | 2748.01 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 605.929µs | 182525 | 0 | 44.31% | 1.28 | 2518.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 537.424µs | 186875 | 0 | 44.30% | 1.28 | 2839.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 765.778µs | 122228 | 200000 | 44.32% | 1.28 | 1992.59 MB/s |
| Quicksort | 1000000 | 6.062283ms | 5301519 | 0 | 44.20% | 1.29 | 2517.00 MB/s |
| Timsort | 1000000 | 10.930769ms | 6302942 | 0 | 44.06% | 1.28 | 1395.95 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.792354ms | 12308876 | 1017407 | 44.34% | 1.28 | 615.46 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.147017ms | 1094612 | 1000000 | 44.72% | 1.26 | 2482.31 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.193572ms | 1094612 | 0 | 44.73% | 1.26 | 2463.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.247098ms | 1095552 | 0 | 44.72% | 1.26 | 2442.54 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.791256ms | 1534410 | 0 | 44.55% | 1.28 | 3184.72 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.731009ms | 1541577 | 0 | 44.55% | 1.28 | 3225.27 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.105177ms | 1989097 | 0 | 44.75% | 1.26 | 1882.60 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.044164ms | 2062330 | 0 | 44.69% | 1.27 | 2166.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.281943ms | 2113182 | 0 | 44.74% | 1.26 | 1842.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.506383ms | 5807618 | 2000000 | 44.01% | 1.28 | 782.25 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.827µs | 10241 | 0 | 44.25% | 1.28 | 855.94 MB/s |
| Timsort | 1000 | 25.708µs | 10555 | 0 | 44.25% | 1.28 | 593.54 MB/s |
| ARS Gen 1: Foundation | 1000 | 232.179µs | 0 | 2000 | 44.25% | 1.28 | 65.72 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 283.841µs | 0 | 2000 | 44.25% | 1.28 | 53.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.496µs | 10241 | 0 | 44.25% | 1.28 | 872.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.831µs | 10241 | 0 | 44.25% | 1.28 | 855.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.859µs | 10241 | 0 | 44.25% | 1.28 | 854.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.527µs | 10555 | 0 | 44.25% | 1.28 | 575.22 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.423µs | 10241 | 0 | 44.25% | 1.28 | 875.78 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 25.286µs | 10555 | 0 | 44.25% | 1.28 | 603.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.603µs | 10241 | 0 | 44.25% | 1.28 | 866.83 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.261µs | 10241 | 0 | 44.25% | 1.28 | 884.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.628µs | 10241 | 0 | 44.25% | 1.28 | 917.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 125.056µs | 10241 | 2000 | 44.25% | 1.28 | 122.02 MB/s |
| Quicksort | 10000 | 220.898µs | 137603 | 0 | 44.23% | 1.28 | 690.76 MB/s |
| Timsort | 10000 | 304.162µs | 140916 | 0 | 44.23% | 1.28 | 501.67 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.241815ms | 0 | 30000 | 44.06% | 1.28 | 29.11 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.585716ms | 0 | 30000 | 43.94% | 1.28 | 27.32 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 354.65µs | 192365 | 14351 | 44.22% | 1.28 | 430.25 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 225.072µs | 66763 | 10000 | 44.22% | 1.28 | 677.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 172.074µs | 66763 | 0 | 44.22% | 1.28 | 886.76 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 166.189µs | 69738 | 0 | 44.21% | 1.28 | 918.16 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 241.785µs | 61148 | 0 | 44.21% | 1.28 | 631.09 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 257.924µs | 64565 | 0 | 44.21% | 1.28 | 591.60 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 157.137µs | 66763 | 0 | 44.22% | 1.28 | 971.05 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 156.668µs | 66763 | 0 | 44.21% | 1.28 | 973.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 159.315µs | 66763 | 0 | 44.21% | 1.28 | 957.77 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 341.93µs | 66763 | 20000 | 44.21% | 1.28 | 446.25 MB/s |
| Quicksort | 100000 | 2.61842ms | 1710395 | 0 | 44.16% | 1.28 | 582.75 MB/s |
| Timsort | 100000 | 3.408277ms | 1746952 | 0 | 44.11% | 1.28 | 447.70 MB/s |
| ARS Gen 1: Foundation | 100000 | 42.611793ms | 0 | 300000 | 37.82% | 1.25 | 35.81 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 49.667011ms | 0 | 300000 | 37.03% | 1.24 | 30.72 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.026025ms | 1885598 | 108703 | 44.14% | 1.28 | 504.25 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.159708ms | 1045510 | 100000 | 44.13% | 1.28 | 1315.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 966.57µs | 1045510 | 0 | 44.15% | 1.28 | 1578.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.112265ms | 1086813 | 0 | 44.13% | 1.28 | 1371.87 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 993.868µs | 981178 | 0 | 44.13% | 1.28 | 1535.29 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.143379ms | 1020346 | 0 | 44.14% | 1.28 | 1334.53 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 907.813µs | 1045510 | 0 | 44.12% | 1.28 | 1680.83 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 857.149µs | 936769 | 0 | 44.12% | 1.28 | 1780.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 928.799µs | 1045510 | 0 | 44.13% | 1.28 | 1642.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.329047ms | 1045510 | 200000 | 44.14% | 1.28 | 1148.10 MB/s |
| Quicksort | 1000000 | 28.433441ms | 20431039 | 0 | 43.81% | 1.30 | 536.65 MB/s |
| Timsort | 1000000 | 41.972628ms | 20806652 | 0 | 43.41% | 1.29 | 363.54 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.138133ms | 21500526 | 1017407 | 43.97% | 1.28 | 562.26 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.779535ms | 12082942 | 1000000 | 44.44% | 1.27 | 1415.53 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.548615ms | 12082942 | 0 | 44.43% | 1.27 | 1446.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.647551ms | 12502702 | 0 | 44.42% | 1.27 | 1310.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.170676ms | 13210593 | 0 | 44.27% | 1.28 | 1663.87 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.061501ms | 13633723 | 0 | 44.20% | 1.27 | 1379.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.874044ms | 7768109 | 0 | 44.25% | 1.26 | 1403.23 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.600528ms | 6294342 | 0 | 44.17% | 1.27 | 1315.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.255118ms | 7082878 | 0 | 44.16% | 1.27 | 1355.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.63002ms | 13879909 | 2000000 | 44.37% | 1.26 | 595.35 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 19.057µs | 10551 | 0 | 44.37% | 1.26 | 800.69 MB/s |
| Timsort | 1000 | 27.53µs | 10537 | 0 | 44.37% | 1.26 | 554.26 MB/s |
| ARS Gen 1: Foundation | 1000 | 241.864µs | 0 | 2000 | 44.37% | 1.26 | 63.09 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 289.806µs | 0 | 2000 | 44.37% | 1.26 | 52.65 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 18.076µs | 10551 | 0 | 44.37% | 1.26 | 844.15 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.479µs | 10551 | 0 | 44.37% | 1.26 | 825.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.109µs | 10551 | 0 | 44.37% | 1.26 | 842.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.345µs | 10537 | 0 | 44.37% | 1.26 | 558.01 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.87µs | 10551 | 0 | 44.37% | 1.26 | 808.63 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.364µs | 10537 | 0 | 44.37% | 1.26 | 557.62 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.06µs | 10551 | 0 | 44.37% | 1.26 | 800.57 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 18.439µs | 10551 | 0 | 44.37% | 1.26 | 827.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 18.096µs | 10551 | 0 | 44.37% | 1.26 | 843.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 131.022µs | 10551 | 2000 | 44.37% | 1.26 | 116.46 MB/s |
| Quicksort | 10000 | 233.916µs | 136744 | 0 | 44.35% | 1.26 | 652.32 MB/s |
| Timsort | 10000 | 328.975µs | 140772 | 0 | 44.35% | 1.26 | 463.83 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.572885ms | 0 | 30000 | 44.17% | 1.26 | 27.38 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.016484ms | 0 | 30000 | 44.12% | 1.26 | 25.36 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 388.548µs | 193085 | 14351 | 44.34% | 1.26 | 392.71 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 402.069µs | 126000 | 10000 | 44.34% | 1.26 | 379.51 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 298.059µs | 126000 | 0 | 44.34% | 1.26 | 511.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 344.062µs | 130426 | 0 | 44.34% | 1.26 | 443.49 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 413.081µs | 118809 | 0 | 44.34% | 1.26 | 369.39 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 448.533µs | 123101 | 0 | 44.34% | 1.26 | 340.19 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 270.366µs | 90031 | 0 | 44.33% | 1.26 | 564.38 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 316.148µs | 126000 | 0 | 44.34% | 1.26 | 482.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 309.854µs | 126000 | 0 | 44.34% | 1.26 | 492.45 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 530.926µs | 126000 | 20000 | 44.33% | 1.26 | 287.40 MB/s |
| Quicksort | 100000 | 2.549471ms | 1704961 | 0 | 44.30% | 1.26 | 598.51 MB/s |
| Timsort | 100000 | 3.400127ms | 1748322 | 0 | 44.26% | 1.26 | 448.77 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.534948ms | 0 | 300000 | 38.15% | 1.23 | 35.05 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.550844ms | 0 | 300000 | 38.07% | 1.24 | 34.25 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.161112ms | 1885129 | 108703 | 44.30% | 1.26 | 482.70 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.194501ms | 1618379 | 100000 | 44.26% | 1.26 | 695.32 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.779293ms | 1618379 | 0 | 44.26% | 1.26 | 857.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.240323ms | 1658575 | 0 | 44.26% | 1.26 | 681.10 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.904069ms | 1529988 | 0 | 44.29% | 1.26 | 801.38 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.246913ms | 1573500 | 0 | 44.27% | 1.26 | 679.10 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.284379ms | 673827 | 0 | 44.17% | 1.26 | 1188.03 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.463553ms | 837701 | 0 | 44.22% | 1.26 | 1042.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.472076ms | 967480 | 0 | 44.22% | 1.26 | 1036.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.103853ms | 1618379 | 200000 | 44.26% | 1.26 | 725.28 MB/s |
| Quicksort | 1000000 | 31.171242ms | 20435426 | 0 | 44.03% | 1.28 | 489.51 MB/s |
| Timsort | 1000000 | 42.678925ms | 20818465 | 0 | 43.64% | 1.28 | 357.53 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.484257ms | 21488833 | 1017407 | 44.15% | 1.26 | 555.18 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 21.858212ms | 19275700 | 1000000 | 44.42% | 1.27 | 698.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 21.387982ms | 19275700 | 0 | 44.39% | 1.26 | 713.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 26.239195ms | 19658200 | 0 | 44.02% | 1.26 | 581.53 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 13.121001ms | 19275700 | 0 | 44.30% | 1.27 | 1162.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 26.837064ms | 19658200 | 0 | 43.95% | 1.26 | 568.57 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.352538ms | 6708948 | 0 | 44.40% | 1.24 | 879.34 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.149941ms | 6379291 | 0 | 44.88% | 1.25 | 840.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.76055ms | 8360463 | 0 | 44.85% | 1.24 | 813.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.978748ms | 20819818 | 2000000 | 44.50% | 1.27 | 492.56 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.487µs | 10288 | 0 | 45.02% | 1.27 | 825.38 MB/s |
| Timsort | 1000 | 26.765µs | 10450 | 0 | 45.02% | 1.27 | 570.10 MB/s |
| ARS Gen 1: Foundation | 1000 | 232.586µs | 0 | 2000 | 45.02% | 1.27 | 65.60 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 277.343µs | 0 | 2000 | 45.02% | 1.27 | 55.02 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.351µs | 10288 | 0 | 45.02% | 1.27 | 879.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.892µs | 10288 | 0 | 45.02% | 1.27 | 852.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.468µs | 10288 | 0 | 45.02% | 1.27 | 873.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.534µs | 10450 | 0 | 45.02% | 1.27 | 554.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.541µs | 10288 | 0 | 45.02% | 1.27 | 822.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 26.579µs | 10450 | 0 | 45.02% | 1.27 | 574.09 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.387µs | 10288 | 0 | 45.02% | 1.27 | 829.87 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.728µs | 10288 | 0 | 45.02% | 1.27 | 860.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.377µs | 10288 | 0 | 45.02% | 1.27 | 878.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 119.563µs | 10288 | 2000 | 45.02% | 1.27 | 127.62 MB/s |
| Quicksort | 10000 | 226.396µs | 136714 | 0 | 45.00% | 1.27 | 673.99 MB/s |
| Timsort | 10000 | 315.436µs | 140903 | 0 | 45.00% | 1.27 | 483.74 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.344989ms | 160 | 30000 | 44.83% | 1.27 | 28.55 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.718683ms | 160 | 30000 | 44.82% | 1.27 | 26.68 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 419.993µs | 193162 | 14351 | 44.99% | 1.27 | 363.31 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 239.602µs | 52333 | 10000 | 44.99% | 1.27 | 636.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 152.393µs | 52333 | 0 | 44.98% | 1.27 | 1001.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 169.957µs | 57763 | 0 | 44.98% | 1.27 | 897.80 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 210.21µs | 59057 | 0 | 44.99% | 1.27 | 725.88 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 206.974µs | 62100 | 0 | 44.99% | 1.27 | 737.23 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 164.425µs | 52333 | 0 | 44.98% | 1.27 | 928.01 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 163.571µs | 52333 | 0 | 44.98% | 1.27 | 932.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 160.953µs | 52333 | 0 | 44.98% | 1.27 | 948.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 348.117µs | 52333 | 20000 | 44.97% | 1.27 | 438.32 MB/s |
| Quicksort | 100000 | 2.716909ms | 1706033 | 0 | 44.92% | 1.27 | 561.62 MB/s |
| Timsort | 100000 | 3.447652ms | 1748408 | 0 | 44.87% | 1.27 | 442.58 MB/s |
| ARS Gen 1: Foundation | 100000 | 46.322116ms | 15822 | 300000 | 38.86% | 1.24 | 32.94 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.897234ms | 15822 | 300000 | 38.74% | 1.25 | 32.54 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.98361ms | 1885784 | 108703 | 44.91% | 1.27 | 511.42 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.13665ms | 882348 | 100000 | 44.91% | 1.27 | 1342.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 759.568µs | 882348 | 0 | 44.89% | 1.27 | 2008.88 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.045347ms | 921462 | 0 | 44.89% | 1.27 | 1459.69 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 998.283µs | 939598 | 0 | 44.89% | 1.27 | 1528.50 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.17011ms | 975983 | 0 | 44.89% | 1.27 | 1304.05 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 827.435µs | 882348 | 0 | 44.88% | 1.27 | 1844.11 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 888.64µs | 771432 | 0 | 44.88% | 1.27 | 1717.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 814.204µs | 882348 | 0 | 44.88% | 1.27 | 1874.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.23611ms | 882348 | 200000 | 44.91% | 1.27 | 1234.42 MB/s |
| Quicksort | 1000000 | 29.109308ms | 20389196 | 0 | 44.60% | 1.29 | 524.19 MB/s |
| Timsort | 1000000 | 42.570121ms | 20780417 | 0 | 44.25% | 1.29 | 358.44 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.101631ms | 21441825 | 1017407 | 44.68% | 1.27 | 563.02 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.266535ms | 10157321 | 1000000 | 45.08% | 1.26 | 1486.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.631342ms | 10157321 | 0 | 45.08% | 1.26 | 1584.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.941371ms | 10561958 | 0 | 45.09% | 1.26 | 1394.60 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.000772ms | 12859603 | 0 | 45.00% | 1.27 | 1695.28 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 10.810658ms | 13271645 | 0 | 44.95% | 1.27 | 1411.46 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.628281ms | 10157321 | 0 | 44.99% | 1.26 | 1584.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.660546ms | 11214454 | 0 | 45.10% | 1.27 | 1431.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.552714ms | 12268914 | 0 | 45.11% | 1.26 | 1445.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.631624ms | 13658791 | 2000000 | 44.92% | 1.27 | 498.14 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 7.851µs | 5797 | 0 | 45.71% | 1.27 | 1943.55 MB/s |
| Timsort | 1000 | 11.064µs | 5499 | 0 | 45.71% | 1.27 | 1379.14 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.065µs | 984 | 2000 | 45.71% | 1.27 | 324.21 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.1µs | 984 | 2000 | 45.71% | 1.27 | 262.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 7.311µs | 5797 | 0 | 45.71% | 1.27 | 2087.10 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 7.766µs | 5797 | 0 | 45.71% | 1.27 | 1964.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 7.514µs | 5797 | 0 | 45.71% | 1.27 | 2030.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.146µs | 5499 | 0 | 45.71% | 1.27 | 1368.99 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 7.704µs | 5797 | 0 | 45.71% | 1.27 | 1980.63 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 10.98µs | 5499 | 0 | 45.71% | 1.27 | 1389.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 7.929µs | 5797 | 0 | 45.71% | 1.27 | 1924.43 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 7.382µs | 5797 | 0 | 45.71% | 1.27 | 2067.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.262µs | 5797 | 0 | 45.71% | 1.27 | 2101.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 110.783µs | 5797 | 2000 | 45.70% | 1.27 | 137.74 MB/s |
| Quicksort | 10000 | 67.601µs | 53838 | 0 | 45.69% | 1.27 | 2257.18 MB/s |
| Timsort | 10000 | 89.452µs | 53843 | 0 | 45.69% | 1.27 | 1705.81 MB/s |
| ARS Gen 1: Foundation | 10000 | 272.356µs | 9984 | 30000 | 45.69% | 1.27 | 560.25 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 310.683µs | 9984 | 30000 | 45.69% | 1.27 | 491.14 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 308.98µs | 122148 | 14351 | 45.69% | 1.27 | 493.84 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 241.088µs | 12061 | 10000 | 45.68% | 1.27 | 632.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 128.869µs | 12061 | 0 | 45.68% | 1.27 | 1184.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 125.182µs | 12085 | 0 | 45.68% | 1.27 | 1218.93 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 216.615µs | 12061 | 0 | 45.68% | 1.27 | 704.42 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 241.631µs | 12085 | 0 | 45.68% | 1.27 | 631.49 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 132.571µs | 12061 | 0 | 45.68% | 1.27 | 1150.99 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 130.361µs | 12061 | 0 | 45.68% | 1.27 | 1170.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 133.239µs | 12061 | 0 | 45.68% | 1.27 | 1145.22 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 297.689µs | 12061 | 20000 | 45.67% | 1.27 | 512.57 MB/s |
| Quicksort | 100000 | 654.457µs | 529379 | 0 | 45.63% | 1.27 | 2331.52 MB/s |
| Timsort | 100000 | 893.25µs | 529674 | 0 | 45.59% | 1.27 | 1708.23 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.391688ms | 99984 | 300000 | 45.64% | 1.27 | 1096.42 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.519278ms | 99984 | 300000 | 45.64% | 1.27 | 1004.34 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.21679ms | 1143461 | 108703 | 45.63% | 1.27 | 688.33 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 535.512µs | 151116 | 100000 | 45.60% | 1.27 | 2849.38 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 370.568µs | 151116 | 0 | 45.60% | 1.27 | 4117.68 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 379.706µs | 151622 | 0 | 45.60% | 1.27 | 4018.58 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 563.371µs | 99988 | 0 | 45.61% | 1.27 | 2708.48 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 562.877µs | 99988 | 0 | 45.61% | 1.27 | 2710.86 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 747.145µs | 199984 | 0 | 45.57% | 1.27 | 2042.28 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 571.858µs | 199972 | 0 | 45.60% | 1.27 | 2668.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 560.504µs | 100000 | 0 | 45.59% | 1.27 | 2722.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 862.712µs | 151116 | 200000 | 45.60% | 1.27 | 1768.70 MB/s |
| Quicksort | 1000000 | 5.642498ms | 5138620 | 0 | 45.40% | 1.28 | 2704.26 MB/s |
| Timsort | 1000000 | 9.975587ms | 6175006 | 0 | 45.18% | 1.27 | 1529.61 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.218148ms | 12087538 | 1017407 | 45.58% | 1.27 | 630.06 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.258235ms | 999988 | 1000000 | 45.90% | 1.26 | 2438.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.323038ms | 999988 | 0 | 45.90% | 1.26 | 2413.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.520329ms | 999988 | 0 | 45.90% | 1.26 | 2340.19 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.443868ms | 999988 | 0 | 45.76% | 1.27 | 3433.67 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.391618ms | 999988 | 0 | 45.75% | 1.27 | 3474.53 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.823609ms | 1999972 | 0 | 45.86% | 1.26 | 1950.35 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.123326ms | 1999972 | 0 | 45.85% | 1.27 | 2142.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.893797ms | 1999976 | 0 | 45.89% | 1.26 | 1933.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.548772ms | 5668763 | 2000000 | 45.39% | 1.27 | 922.05 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.127µs | 10288 | 0 | 45.58% | 1.27 | 890.92 MB/s |
| Timsort | 1000 | 25.126µs | 10450 | 0 | 45.58% | 1.27 | 607.29 MB/s |
| ARS Gen 1: Foundation | 1000 | 220.374µs | 0 | 2000 | 45.58% | 1.27 | 69.24 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 262.197µs | 0 | 2000 | 45.57% | 1.27 | 58.20 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.23µs | 10288 | 0 | 45.58% | 1.27 | 940.16 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 16.901µs | 10288 | 0 | 45.58% | 1.27 | 902.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.366µs | 10288 | 0 | 45.58% | 1.27 | 932.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 24.945µs | 10450 | 0 | 45.58% | 1.27 | 611.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 16.953µs | 10288 | 0 | 45.58% | 1.27 | 900.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 24.767µs | 10450 | 0 | 45.58% | 1.27 | 616.09 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.188µs | 10288 | 0 | 45.58% | 1.27 | 887.76 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 16.711µs | 10288 | 0 | 45.58% | 1.27 | 913.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.643µs | 10288 | 0 | 45.58% | 1.27 | 916.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 121.782µs | 10288 | 2000 | 45.57% | 1.27 | 125.30 MB/s |
| Quicksort | 10000 | 218.313µs | 136714 | 0 | 45.56% | 1.27 | 698.94 MB/s |
| Timsort | 10000 | 311.718µs | 140903 | 0 | 45.56% | 1.27 | 489.51 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.187239ms | 160 | 30000 | 45.41% | 1.27 | 29.42 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.455445ms | 160 | 30000 | 45.40% | 1.27 | 27.97 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 397.281µs | 193162 | 14351 | 45.55% | 1.27 | 384.08 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 243.179µs | 52333 | 10000 | 45.55% | 1.27 | 627.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 157.177µs | 52333 | 0 | 45.54% | 1.27 | 970.80 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 162.199µs | 57763 | 0 | 45.54% | 1.27 | 940.74 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 188.839µs | 59057 | 0 | 45.55% | 1.27 | 808.03 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 181.542µs | 62100 | 0 | 45.55% | 1.27 | 840.51 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 169.907µs | 52333 | 0 | 45.54% | 1.27 | 898.07 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 156.532µs | 52333 | 0 | 45.54% | 1.27 | 974.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 165.564µs | 52333 | 0 | 45.54% | 1.27 | 921.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 354.523µs | 52333 | 20000 | 45.54% | 1.27 | 430.40 MB/s |
| Quicksort | 100000 | 2.703643ms | 1706033 | 0 | 45.49% | 1.27 | 564.38 MB/s |
| Timsort | 100000 | 3.409771ms | 1748408 | 0 | 45.45% | 1.27 | 447.50 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.473627ms | 15822 | 300000 | 39.99% | 1.25 | 36.79 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.7935ms | 15822 | 300000 | 40.00% | 1.25 | 33.32 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.038506ms | 1885784 | 108703 | 45.48% | 1.27 | 502.18 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.045382ms | 882348 | 100000 | 45.48% | 1.27 | 1459.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 873.696µs | 882348 | 0 | 45.49% | 1.27 | 1746.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.025001ms | 921462 | 0 | 45.48% | 1.27 | 1488.66 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.024752ms | 939598 | 0 | 45.48% | 1.27 | 1489.02 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.186289ms | 975983 | 0 | 45.48% | 1.27 | 1286.26 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 806.651µs | 882348 | 0 | 45.46% | 1.27 | 1891.62 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 786.566µs | 771432 | 0 | 45.46% | 1.27 | 1939.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 812.741µs | 882348 | 0 | 45.46% | 1.27 | 1877.45 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.336429ms | 882348 | 200000 | 45.46% | 1.27 | 1141.76 MB/s |
| Quicksort | 1000000 | 30.76775ms | 20389196 | 0 | 45.20% | 1.29 | 495.93 MB/s |
| Timsort | 1000000 | 45.317563ms | 20780417 | 0 | 44.85% | 1.29 | 336.71 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.234817ms | 21441825 | 1017407 | 45.32% | 1.27 | 560.27 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.347756ms | 10157321 | 1000000 | 45.64% | 1.27 | 1474.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.587617ms | 10157321 | 0 | 45.63% | 1.26 | 1591.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.169093ms | 10561958 | 0 | 45.64% | 1.26 | 1366.16 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.960548ms | 12859603 | 0 | 45.56% | 1.27 | 1702.89 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.351675ms | 13271645 | 0 | 45.52% | 1.27 | 1344.19 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.628238ms | 10157321 | 0 | 45.55% | 1.26 | 1584.80 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.686754ms | 11214454 | 0 | 45.65% | 1.27 | 1427.82 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.606181ms | 12268914 | 0 | 45.66% | 1.27 | 1438.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.441734ms | 13658796 | 2000000 | 45.55% | 1.27 | 501.25 MB/s |

## Category: String

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 65.306µs | 10370 | 0 | 46.37% | 1.27 | 934.60 MB/s |
| Timsort | 1000 | 77.307µs | 10522 | 0 | 46.37% | 1.27 | 789.52 MB/s |
| ARS Gen 1: Foundation | 1000 | 330.149µs | 0 | 2000 | 46.37% | 1.27 | 184.87 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 375.457µs | 0 | 2000 | 46.37% | 1.27 | 162.56 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 66.89µs | 10370 | 0 | 46.37% | 1.27 | 912.47 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 66.544µs | 10370 | 0 | 46.37% | 1.27 | 917.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 67.931µs | 10370 | 0 | 46.37% | 1.27 | 898.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 74.263µs | 10522 | 0 | 46.37% | 1.27 | 821.88 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 66.782µs | 10370 | 0 | 46.37% | 1.27 | 913.95 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 76.016µs | 10522 | 0 | 46.37% | 1.27 | 802.93 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 66.698µs | 10370 | 0 | 46.37% | 1.27 | 915.10 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 65.228µs | 10370 | 0 | 46.37% | 1.27 | 935.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 65.819µs | 10370 | 0 | 46.37% | 1.27 | 927.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 258.14µs | 10370 | 2000 | 46.37% | 1.27 | 236.44 MB/s |
| Quicksort | 10000 | 825.469µs | 136866 | 0 | 46.34% | 1.27 | 739.40 MB/s |
| Timsort | 10000 | 995.75µs | 141490 | 0 | 46.32% | 1.27 | 612.96 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.568099ms | 0 | 30000 | 46.19% | 1.28 | 92.93 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.783484ms | 0 | 30000 | 46.20% | 1.28 | 89.98 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.340197ms | 193846 | 14351 | 46.32% | 1.27 | 260.81 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 744.369µs | 67438 | 10000 | 46.33% | 1.27 | 819.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 267.552µs | 67438 | 0 | 46.33% | 1.27 | 2281.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 276.212µs | 70298 | 0 | 46.33% | 1.27 | 2209.72 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 430.125µs | 63043 | 0 | 46.33% | 1.27 | 1419.01 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 476.103µs | 67007 | 0 | 46.33% | 1.27 | 1281.97 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 296.064µs | 67438 | 0 | 46.33% | 1.27 | 2061.55 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 272.594µs | 67438 | 0 | 46.33% | 1.27 | 2239.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 279.834µs | 67438 | 0 | 46.33% | 1.27 | 2181.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.34498ms | 67438 | 20000 | 46.32% | 1.27 | 453.80 MB/s |
| Quicksort | 100000 | 12.716561ms | 1718762 | 0 | 45.82% | 1.28 | 479.97 MB/s |
| Timsort | 100000 | 14.082805ms | 1759891 | 0 | 45.65% | 1.28 | 433.40 MB/s |
| ARS Gen 1: Foundation | 100000 | 57.918675ms | 0 | 300000 | 41.56% | 1.26 | 105.38 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 57.607604ms | 0 | 300000 | 41.19% | 1.26 | 105.95 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.89541ms | 1895222 | 108703 | 46.26% | 1.28 | 341.07 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.124138ms | 1029722 | 100000 | 46.26% | 1.27 | 1479.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.719553ms | 1029722 | 0 | 46.26% | 1.27 | 2244.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.128115ms | 1071423 | 0 | 46.22% | 1.27 | 1951.18 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.3377ms | 978520 | 0 | 46.23% | 1.27 | 1828.66 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.509718ms | 1019338 | 0 | 46.22% | 1.27 | 1739.03 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.249064ms | 1029722 | 0 | 46.24% | 1.27 | 1878.55 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.3028ms | 961965 | 0 | 46.28% | 1.27 | 1847.98 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.40775ms | 1029722 | 0 | 46.23% | 1.27 | 1791.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 11.98196ms | 1029722 | 200000 | 45.87% | 1.27 | 509.39 MB/s |
| Quicksort | 1000000 | 300.41213ms | 20518628 | 0 | 48.12% | 1.29 | 203.17 MB/s |
| Timsort | 1000000 | 362.407718ms | 20902099 | 0 | 46.88% | 1.27 | 168.42 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 214.957241ms | 21589743 | 1017407 | 46.38% | 1.27 | 283.94 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 57.44415ms | 12256776 | 1000000 | 47.41% | 1.21 | 1062.51 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 50.39895ms | 12256776 | 0 | 47.35% | 1.22 | 1211.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.983822ms | 12679336 | 0 | 47.07% | 1.20 | 1090.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 64.026689ms | 13331493 | 0 | 47.56% | 1.20 | 953.28 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 70.375984ms | 13750405 | 0 | 47.34% | 1.20 | 867.27 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.207398ms | 9434971 | 0 | 47.63% | 1.21 | 1169.09 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 68.792971ms | 7913738 | 0 | 48.73% | 1.19 | 887.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 59.010225ms | 8719170 | 0 | 48.23% | 1.20 | 1034.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 432.096397ms | 15199661 | 2000000 | 50.98% | 1.22 | 141.25 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 56.288µs | 10370 | 0 | 45.55% | 1.25 | 1084.34 MB/s |
| Timsort | 1000 | 71.124µs | 10522 | 0 | 45.55% | 1.25 | 858.15 MB/s |
| ARS Gen 1: Foundation | 1000 | 277.123µs | 0 | 2000 | 45.55% | 1.25 | 220.25 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 300.876µs | 0 | 2000 | 45.55% | 1.25 | 202.86 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 56.672µs | 10370 | 0 | 45.55% | 1.25 | 1076.99 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 58.308µs | 10370 | 0 | 45.55% | 1.25 | 1046.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 59.024µs | 10370 | 0 | 45.55% | 1.25 | 1034.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 60.885µs | 10522 | 0 | 45.55% | 1.25 | 1002.47 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 54.728µs | 10370 | 0 | 45.55% | 1.25 | 1115.25 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 68.019µs | 10522 | 0 | 45.55% | 1.25 | 897.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 58.528µs | 10370 | 0 | 45.55% | 1.25 | 1042.84 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 54.753µs | 10370 | 0 | 45.55% | 1.25 | 1114.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 54.747µs | 10370 | 0 | 45.55% | 1.25 | 1114.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 221.258µs | 10370 | 2000 | 45.55% | 1.25 | 275.86 MB/s |
| Quicksort | 10000 | 719.032µs | 136866 | 0 | 45.52% | 1.26 | 848.85 MB/s |
| Timsort | 10000 | 878.071µs | 141490 | 0 | 45.51% | 1.26 | 695.11 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.948392ms | 0 | 30000 | 45.37% | 1.26 | 102.61 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.187197ms | 0 | 30000 | 45.35% | 1.26 | 98.65 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.023295ms | 193846 | 14351 | 45.50% | 1.26 | 301.66 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 632.483µs | 67438 | 10000 | 45.51% | 1.25 | 965.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 217.793µs | 67438 | 0 | 45.52% | 1.25 | 2802.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 218.413µs | 70298 | 0 | 45.51% | 1.25 | 2794.48 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 353.426µs | 63043 | 0 | 45.51% | 1.25 | 1726.96 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 392.763µs | 67007 | 0 | 45.51% | 1.25 | 1553.99 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 232.182µs | 67438 | 0 | 45.51% | 1.25 | 2628.76 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 224.721µs | 67438 | 0 | 45.51% | 1.25 | 2716.04 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 234.258µs | 67438 | 0 | 45.51% | 1.25 | 2605.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.003731ms | 67438 | 20000 | 45.49% | 1.25 | 608.08 MB/s |
| Quicksort | 100000 | 9.613643ms | 1718762 | 0 | 45.06% | 1.26 | 634.88 MB/s |
| Timsort | 100000 | 12.241014ms | 1759891 | 0 | 44.86% | 1.26 | 498.61 MB/s |
| ARS Gen 1: Foundation | 100000 | 58.835744ms | 0 | 300000 | 41.36% | 1.24 | 103.74 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 58.653035ms | 0 | 300000 | 41.23% | 1.24 | 104.06 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 18.557161ms | 1895222 | 108703 | 45.44% | 1.26 | 328.90 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.955309ms | 1029722 | 100000 | 45.41% | 1.25 | 1543.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.84781ms | 1029722 | 0 | 45.38% | 1.25 | 2143.23 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.147774ms | 1071423 | 0 | 45.33% | 1.25 | 1938.99 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.191086ms | 978520 | 0 | 45.37% | 1.25 | 1912.68 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.310214ms | 1019338 | 0 | 45.37% | 1.25 | 1843.84 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.911058ms | 1029722 | 0 | 45.36% | 1.25 | 2096.67 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.053683ms | 961965 | 0 | 45.43% | 1.25 | 1998.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.941711ms | 1029722 | 0 | 45.36% | 1.25 | 2074.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.488673ms | 1029722 | 200000 | 45.12% | 1.26 | 581.91 MB/s |
| Quicksort | 1000000 | 284.166528ms | 20518628 | 0 | 47.40% | 1.27 | 214.79 MB/s |
| Timsort | 1000000 | 385.175902ms | 20902099 | 0 | 46.42% | 1.25 | 158.46 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 199.510927ms | 21589743 | 1017407 | 45.58% | 1.25 | 305.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.054249ms | 12256776 | 1000000 | 46.71% | 1.20 | 999.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.855068ms | 12256776 | 0 | 46.59% | 1.19 | 1249.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.181949ms | 12679336 | 0 | 46.37% | 1.19 | 1106.07 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 62.245437ms | 13331493 | 0 | 46.79% | 1.19 | 980.56 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 69.301419ms | 13750405 | 0 | 46.67% | 1.19 | 880.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.577143ms | 9434971 | 0 | 46.86% | 1.20 | 1183.38 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 69.844915ms | 7913738 | 0 | 47.91% | 1.18 | 873.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.259409ms | 8719170 | 0 | 47.44% | 1.19 | 1065.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 431.220364ms | 15199653 | 2000000 | 50.13% | 1.20 | 141.54 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 47.237µs | 9540 | 0 | 45.05% | 1.24 | 1292.10 MB/s |
| Timsort | 1000 | 47.933µs | 9492 | 0 | 45.05% | 1.24 | 1273.34 MB/s |
| ARS Gen 1: Foundation | 1000 | 134.924µs | 9394 | 2000 | 45.05% | 1.24 | 452.37 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 134.109µs | 9417 | 2000 | 45.05% | 1.24 | 455.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 50.264µs | 9540 | 0 | 45.05% | 1.24 | 1214.29 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 48.525µs | 9540 | 0 | 45.05% | 1.24 | 1257.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 47.765µs | 9540 | 0 | 45.05% | 1.24 | 1277.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 51.633µs | 9492 | 0 | 45.05% | 1.24 | 1182.10 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 49.344µs | 9540 | 0 | 45.05% | 1.24 | 1236.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 49.705µs | 9492 | 0 | 45.05% | 1.24 | 1227.95 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 50.771µs | 9540 | 0 | 45.05% | 1.24 | 1202.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 50.504µs | 9540 | 0 | 45.05% | 1.24 | 1208.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 55.51µs | 9540 | 0 | 45.05% | 1.24 | 1099.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 200.39µs | 9540 | 2000 | 45.05% | 1.24 | 304.58 MB/s |
| Quicksort | 10000 | 714.71µs | 132500 | 0 | 45.03% | 1.24 | 853.98 MB/s |
| Timsort | 10000 | 747.205µs | 127861 | 0 | 45.02% | 1.24 | 816.85 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.885619ms | 94604 | 30000 | 45.00% | 1.24 | 323.69 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.924692ms | 94565 | 30000 | 45.00% | 1.24 | 317.12 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.100913ms | 182797 | 14351 | 45.01% | 1.24 | 290.52 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 728.316µs | 88075 | 10000 | 45.01% | 1.24 | 838.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 283.78µs | 88075 | 0 | 45.02% | 1.24 | 2150.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 268.881µs | 63479 | 0 | 45.02% | 1.24 | 2269.97 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 348.004µs | 73151 | 0 | 45.01% | 1.24 | 1753.86 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 334.204µs | 48448 | 0 | 45.01% | 1.24 | 1826.28 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 330.53µs | 88075 | 0 | 45.02% | 1.24 | 1846.58 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 282.078µs | 88075 | 0 | 45.02% | 1.24 | 2163.77 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 298.032µs | 88075 | 0 | 45.02% | 1.24 | 2047.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.102792ms | 88075 | 20000 | 45.00% | 1.24 | 553.46 MB/s |
| Quicksort | 100000 | 9.860617ms | 1695729 | 0 | 44.85% | 1.25 | 618.98 MB/s |
| Timsort | 100000 | 10.442143ms | 1618264 | 0 | 44.72% | 1.25 | 584.51 MB/s |
| ARS Gen 1: Foundation | 100000 | 23.33286ms | 958264 | 300000 | 45.07% | 1.25 | 261.58 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 22.490411ms | 958287 | 300000 | 45.06% | 1.25 | 271.38 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.971268ms | 1799629 | 108703 | 45.01% | 1.25 | 359.64 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.782623ms | 1250176 | 100000 | 44.92% | 1.24 | 1276.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.053718ms | 1250176 | 0 | 44.96% | 1.24 | 1998.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.361419ms | 957484 | 0 | 44.92% | 1.24 | 1815.76 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.986458ms | 1082137 | 0 | 44.94% | 1.24 | 2043.73 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.520163ms | 561919 | 0 | 44.92% | 1.24 | 2421.87 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.170936ms | 877121 | 0 | 44.95% | 1.24 | 1924.83 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.255654ms | 943356 | 0 | 44.92% | 1.24 | 1874.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.504453ms | 1084625 | 0 | 44.94% | 1.24 | 1741.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 11.379358ms | 1250176 | 200000 | 44.89% | 1.24 | 536.37 MB/s |
| Quicksort | 1000000 | 145.971477ms | 20467458 | 0 | 44.22% | 1.29 | 418.13 MB/s |
| Timsort | 1000000 | 181.09174ms | 19247236 | 0 | 43.98% | 1.28 | 337.04 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 221.503284ms | 20726079 | 1017407 | 46.23% | 1.26 | 275.55 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.674166ms | 14427992 | 1000000 | 46.06% | 1.21 | 989.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 41.573951ms | 14427992 | 0 | 45.11% | 1.22 | 1468.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 40.766806ms | 9562892 | 0 | 44.68% | 1.20 | 1497.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 54.903211ms | 14500857 | 0 | 45.56% | 1.21 | 1111.69 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 53.580344ms | 9781181 | 0 | 45.02% | 1.20 | 1139.13 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 43.656737ms | 10121426 | 0 | 45.92% | 1.22 | 1398.07 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 53.487204ms | 10340217 | 0 | 46.27% | 1.21 | 1141.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 50.382416ms | 11441302 | 0 | 46.01% | 1.22 | 1211.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 248.4148ms | 19075227 | 2000000 | 47.25% | 1.25 | 245.70 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 40.152µs | 5636 | 0 | 51.56% | 1.26 | 1520.10 MB/s |
| Timsort | 1000 | 50.759µs | 5782 | 0 | 51.56% | 1.26 | 1202.45 MB/s |
| ARS Gen 1: Foundation | 1000 | 136.164µs | 984 | 2000 | 51.56% | 1.26 | 448.25 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 139.693µs | 984 | 2000 | 51.56% | 1.26 | 436.92 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 39.766µs | 5636 | 0 | 51.56% | 1.26 | 1534.86 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 40.527µs | 5636 | 0 | 51.56% | 1.26 | 1506.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 39.363µs | 5636 | 0 | 51.56% | 1.26 | 1550.57 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 42.284µs | 5782 | 0 | 51.56% | 1.26 | 1443.46 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 37.643µs | 5636 | 0 | 51.56% | 1.26 | 1621.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 40.499µs | 5782 | 0 | 51.56% | 1.26 | 1507.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 35.98µs | 5636 | 0 | 51.56% | 1.26 | 1696.36 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 37.907µs | 5636 | 0 | 51.56% | 1.26 | 1610.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 37.029µs | 5636 | 0 | 51.56% | 1.26 | 1648.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 194.518µs | 5636 | 2000 | 51.55% | 1.26 | 313.78 MB/s |
| Quicksort | 10000 | 359.811µs | 53113 | 0 | 51.53% | 1.26 | 1696.31 MB/s |
| Timsort | 10000 | 470.561µs | 54714 | 0 | 51.52% | 1.26 | 1297.07 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.539296ms | 9984 | 30000 | 51.51% | 1.26 | 396.51 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.566615ms | 9984 | 30000 | 51.51% | 1.26 | 389.60 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.770959ms | 122389 | 14351 | 51.53% | 1.26 | 344.64 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 628.132µs | 14075 | 10000 | 51.53% | 1.26 | 971.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 154.631µs | 14075 | 0 | 51.53% | 1.26 | 3947.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 159.117µs | 14094 | 0 | 51.53% | 1.26 | 3835.87 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 336.129µs | 12021 | 0 | 51.53% | 1.26 | 1815.83 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 315.289µs | 12028 | 0 | 51.53% | 1.26 | 1935.85 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 225.977µs | 14075 | 0 | 51.53% | 1.26 | 2700.95 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 137.163µs | 14075 | 0 | 51.53% | 1.26 | 4449.83 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 184.593µs | 14075 | 0 | 51.53% | 1.26 | 3306.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.148515ms | 14075 | 20000 | 51.51% | 1.26 | 531.43 MB/s |
| Quicksort | 100000 | 4.132914ms | 516589 | 0 | 51.32% | 1.26 | 1476.81 MB/s |
| Timsort | 100000 | 4.727924ms | 529550 | 0 | 51.23% | 1.26 | 1290.95 MB/s |
| ARS Gen 1: Foundation | 100000 | 15.207139ms | 99984 | 300000 | 51.34% | 1.26 | 401.36 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 15.755305ms | 99984 | 300000 | 51.32% | 1.26 | 387.39 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.613564ms | 1144965 | 108703 | 51.49% | 1.26 | 367.38 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.130496ms | 151083 | 100000 | 51.51% | 1.26 | 1949.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.885447ms | 151083 | 0 | 51.50% | 1.26 | 3237.17 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.666716ms | 151309 | 0 | 51.50% | 1.26 | 3662.00 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.878799ms | 99990 | 0 | 51.48% | 1.26 | 3248.63 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.939104ms | 99990 | 0 | 51.50% | 1.26 | 3147.60 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.978884ms | 200008 | 0 | 51.41% | 1.26 | 3084.32 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.456117ms | 200008 | 0 | 51.44% | 1.26 | 2485.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.671683ms | 100024 | 0 | 51.48% | 1.26 | 3651.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 8.895053ms | 151083 | 200000 | 51.28% | 1.26 | 686.17 MB/s |
| Quicksort | 1000000 | 96.487255ms | 5202060 | 0 | 52.89% | 1.26 | 632.57 MB/s |
| Timsort | 1000000 | 156.306552ms | 6111262 | 0 | 53.02% | 1.25 | 390.48 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 172.763066ms | 12085476 | 1017407 | 51.74% | 1.27 | 353.29 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 30.264858ms | 999988 | 1000000 | 52.07% | 1.25 | 2016.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 23.280135ms | 999988 | 0 | 52.01% | 1.24 | 2621.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 22.450303ms | 999988 | 0 | 51.99% | 1.24 | 2718.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 24.940381ms | 999988 | 0 | 52.02% | 1.24 | 2447.24 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 23.979506ms | 999988 | 0 | 52.04% | 1.24 | 2545.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 34.104776ms | 1999972 | 0 | 52.18% | 1.23 | 1789.64 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 41.445365ms | 1999976 | 0 | 52.46% | 1.23 | 1472.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 34.593058ms | 1999976 | 0 | 52.22% | 1.23 | 1764.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 318.799905ms | 5709060 | 2000000 | 54.33% | 1.24 | 191.45 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 53.755µs | 10370 | 0 | 51.43% | 1.25 | 1135.43 MB/s |
| Timsort | 1000 | 61.05µs | 10522 | 0 | 51.43% | 1.25 | 999.76 MB/s |
| ARS Gen 1: Foundation | 1000 | 279.549µs | 0 | 2000 | 51.43% | 1.25 | 218.33 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 297.297µs | 0 | 2000 | 51.43% | 1.25 | 205.30 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 54.036µs | 10370 | 0 | 51.43% | 1.25 | 1129.53 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 58.236µs | 10370 | 0 | 51.43% | 1.25 | 1048.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 59.652µs | 10370 | 0 | 51.43% | 1.25 | 1023.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 68.746µs | 10522 | 0 | 51.43% | 1.25 | 887.84 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 58.231µs | 10370 | 0 | 51.43% | 1.25 | 1048.16 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 70.917µs | 10522 | 0 | 51.43% | 1.25 | 860.66 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 61.353µs | 10370 | 0 | 51.43% | 1.25 | 994.82 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 61.195µs | 10370 | 0 | 51.43% | 1.25 | 997.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 61.658µs | 10370 | 0 | 51.43% | 1.25 | 989.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 239.64µs | 10370 | 2000 | 51.43% | 1.25 | 254.70 MB/s |
| Quicksort | 10000 | 790.19µs | 136866 | 0 | 51.40% | 1.25 | 772.41 MB/s |
| Timsort | 10000 | 881.073µs | 141490 | 0 | 51.40% | 1.25 | 692.74 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.049248ms | 0 | 30000 | 51.28% | 1.25 | 100.90 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.943699ms | 0 | 30000 | 51.28% | 1.25 | 102.69 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.962971ms | 193846 | 14351 | 51.39% | 1.25 | 310.93 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 655.47µs | 67438 | 10000 | 51.40% | 1.25 | 931.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 227.146µs | 67438 | 0 | 51.40% | 1.25 | 2687.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 242.041µs | 70298 | 0 | 51.40% | 1.25 | 2521.69 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 370.833µs | 63043 | 0 | 51.40% | 1.25 | 1645.89 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 384.965µs | 67007 | 0 | 51.40% | 1.25 | 1585.47 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 234.593µs | 67438 | 0 | 51.40% | 1.25 | 2601.75 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 234.604µs | 67438 | 0 | 51.40% | 1.25 | 2601.62 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 239.804µs | 67438 | 0 | 51.40% | 1.25 | 2545.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.054454ms | 67438 | 20000 | 51.38% | 1.25 | 578.83 MB/s |
| Quicksort | 100000 | 9.815682ms | 1718762 | 0 | 51.07% | 1.26 | 621.81 MB/s |
| Timsort | 100000 | 12.411012ms | 1759891 | 0 | 50.92% | 1.25 | 491.78 MB/s |
| ARS Gen 1: Foundation | 100000 | 57.163986ms | 0 | 300000 | 48.79% | 1.25 | 106.77 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 58.891743ms | 0 | 300000 | 47.71% | 1.24 | 103.64 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.705967ms | 1895222 | 108703 | 51.32% | 1.25 | 365.35 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.905948ms | 1029722 | 100000 | 51.30% | 1.25 | 1562.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.902258ms | 1029722 | 0 | 51.27% | 1.25 | 2103.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.133384ms | 1071423 | 0 | 51.26% | 1.25 | 1947.90 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.068019ms | 978520 | 0 | 51.30% | 1.25 | 1989.40 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.504408ms | 1019338 | 0 | 51.27% | 1.25 | 1741.67 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.968453ms | 1029722 | 0 | 51.31% | 1.25 | 2056.13 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.146803ms | 961965 | 0 | 51.33% | 1.25 | 1939.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.954202ms | 1029722 | 0 | 51.29% | 1.25 | 2066.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.250014ms | 1029722 | 200000 | 51.02% | 1.25 | 595.46 MB/s |
| Quicksort | 1000000 | 252.241419ms | 20518628 | 0 | 51.97% | 1.27 | 241.97 MB/s |
| Timsort | 1000000 | 345.673397ms | 20902099 | 0 | 50.96% | 1.25 | 176.57 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 194.122185ms | 21589743 | 1017407 | 51.13% | 1.25 | 314.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.058935ms | 12256776 | 1000000 | 51.80% | 1.20 | 1016.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 49.06085ms | 12256776 | 0 | 51.85% | 1.21 | 1244.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 60.116319ms | 12679336 | 0 | 51.46% | 1.19 | 1015.28 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 62.435663ms | 13331493 | 0 | 51.95% | 1.21 | 977.57 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 74.382798ms | 13750405 | 0 | 51.69% | 1.19 | 820.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.973874ms | 9434971 | 0 | 51.95% | 1.21 | 1174.34 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 69.279417ms | 7913738 | 0 | 52.62% | 1.20 | 881.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.930789ms | 8719170 | 0 | 52.32% | 1.20 | 1053.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 421.577524ms | 15199655 | 2000000 | 53.84% | 1.21 | 144.78 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 51.616µs | 10370 | 0 | 52.52% | 1.22 | 1182.49 MB/s |
| Timsort | 1000 | 65.361µs | 10522 | 0 | 52.52% | 1.22 | 933.82 MB/s |
| ARS Gen 1: Foundation | 1000 | 281.486µs | 0 | 2000 | 52.52% | 1.22 | 216.83 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 306.83µs | 0 | 2000 | 52.52% | 1.22 | 198.92 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 57.631µs | 10370 | 0 | 52.52% | 1.22 | 1059.07 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 55.578µs | 10370 | 0 | 52.52% | 1.22 | 1098.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 58.834µs | 10370 | 0 | 52.52% | 1.22 | 1037.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 62.613µs | 10522 | 0 | 52.52% | 1.22 | 974.80 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 57.417µs | 10370 | 0 | 52.52% | 1.22 | 1063.02 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 66.125µs | 10522 | 0 | 52.52% | 1.22 | 923.03 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 59.275µs | 10370 | 0 | 52.52% | 1.22 | 1029.69 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 57.302µs | 10370 | 0 | 52.52% | 1.22 | 1065.15 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 52.942µs | 10370 | 0 | 52.52% | 1.22 | 1152.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 224.264µs | 10370 | 2000 | 52.52% | 1.22 | 272.16 MB/s |
| Quicksort | 10000 | 725.438µs | 136866 | 0 | 52.50% | 1.22 | 841.36 MB/s |
| Timsort | 10000 | 936.662µs | 141490 | 0 | 52.49% | 1.22 | 651.62 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.659826ms | 0 | 30000 | 52.39% | 1.23 | 107.84 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.710979ms | 0 | 30000 | 52.37% | 1.23 | 90.95 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.185935ms | 193846 | 14351 | 52.49% | 1.22 | 279.22 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 708.939µs | 67438 | 10000 | 52.50% | 1.22 | 860.94 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 226.291µs | 67438 | 0 | 52.50% | 1.22 | 2697.20 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 233.974µs | 70298 | 0 | 52.50% | 1.22 | 2608.63 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 364.555µs | 63043 | 0 | 52.49% | 1.22 | 1674.24 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 367.372µs | 67007 | 0 | 52.49% | 1.22 | 1661.40 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 235.322µs | 67438 | 0 | 52.50% | 1.22 | 2593.69 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 222.569µs | 67438 | 0 | 52.50% | 1.22 | 2742.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 220.453µs | 67438 | 0 | 52.50% | 1.22 | 2768.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.046343ms | 67438 | 20000 | 52.48% | 1.22 | 583.32 MB/s |
| Quicksort | 100000 | 10.001114ms | 1718762 | 0 | 52.16% | 1.23 | 610.28 MB/s |
| Timsort | 100000 | 12.778304ms | 1759891 | 0 | 52.02% | 1.23 | 477.65 MB/s |
| ARS Gen 1: Foundation | 100000 | 60.707434ms | 0 | 300000 | 49.15% | 1.21 | 100.54 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.673262ms | 0 | 300000 | 49.15% | 1.21 | 98.97 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.025164ms | 1895222 | 108703 | 52.41% | 1.23 | 358.50 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.952147ms | 1029722 | 100000 | 52.39% | 1.22 | 1544.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.668914ms | 1029722 | 0 | 52.41% | 1.22 | 2286.89 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.078717ms | 1071423 | 0 | 52.39% | 1.22 | 1982.49 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.175679ms | 978520 | 0 | 52.39% | 1.22 | 1921.96 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.300044ms | 1019338 | 0 | 52.39% | 1.22 | 1849.53 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.042373ms | 1029722 | 0 | 52.41% | 1.22 | 2006.17 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.107005ms | 961965 | 0 | 52.43% | 1.22 | 1964.44 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.941238ms | 1029722 | 0 | 52.41% | 1.22 | 2075.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.399719ms | 1029722 | 200000 | 52.17% | 1.22 | 586.89 MB/s |
| Quicksort | 1000000 | 239.268905ms | 20518628 | 0 | 52.91% | 1.25 | 255.09 MB/s |
| Timsort | 1000000 | 341.374524ms | 20902099 | 0 | 52.00% | 1.23 | 178.79 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 214.627399ms | 21589743 | 1017407 | 52.20% | 1.22 | 284.38 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.21462ms | 12256776 | 1000000 | 52.81% | 1.18 | 997.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 50.846822ms | 12256776 | 0 | 52.67% | 1.19 | 1200.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 57.770296ms | 12679336 | 0 | 52.52% | 1.18 | 1056.51 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 67.218254ms | 13331493 | 0 | 52.81% | 1.18 | 908.01 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 74.643645ms | 13750405 | 0 | 52.64% | 1.18 | 817.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 53.471149ms | 9434971 | 0 | 53.00% | 1.18 | 1141.46 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 70.568288ms | 7913738 | 0 | 53.53% | 1.17 | 864.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 60.074002ms | 8719170 | 0 | 53.33% | 1.18 | 1016.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 434.641829ms | 15199655 | 2000000 | 54.67% | 1.20 | 140.43 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 54.174µs | 10370 | 0 | 51.91% | 1.21 | 1126.65 MB/s |
| Timsort | 1000 | 64.41µs | 10522 | 0 | 51.91% | 1.21 | 947.60 MB/s |
| ARS Gen 1: Foundation | 1000 | 270.526µs | 0 | 2000 | 51.91% | 1.21 | 225.62 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 295.197µs | 0 | 2000 | 51.90% | 1.21 | 206.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 59.406µs | 10370 | 0 | 51.91% | 1.21 | 1027.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 54.8µs | 10370 | 0 | 51.91% | 1.21 | 1113.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 58.275µs | 10370 | 0 | 51.91% | 1.21 | 1047.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 66.383µs | 10522 | 0 | 51.91% | 1.21 | 919.44 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 57.665µs | 10370 | 0 | 51.91% | 1.21 | 1058.44 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 65.381µs | 10522 | 0 | 51.91% | 1.21 | 933.53 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 53.777µs | 10370 | 0 | 51.91% | 1.21 | 1134.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 58.477µs | 10370 | 0 | 51.91% | 1.21 | 1043.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 56.483µs | 10370 | 0 | 51.91% | 1.21 | 1080.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 219.744µs | 10370 | 2000 | 51.90% | 1.21 | 277.76 MB/s |
| Quicksort | 10000 | 731.374µs | 136866 | 0 | 51.88% | 1.21 | 834.53 MB/s |
| Timsort | 10000 | 896.041µs | 141490 | 0 | 51.88% | 1.21 | 681.16 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.213244ms | 0 | 30000 | 51.81% | 1.22 | 98.23 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.785562ms | 0 | 30000 | 51.81% | 1.22 | 105.50 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.126686ms | 193846 | 14351 | 51.88% | 1.21 | 287.00 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 664.865µs | 67438 | 10000 | 51.88% | 1.21 | 918.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 222.412µs | 67438 | 0 | 51.88% | 1.21 | 2744.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 227.431µs | 70298 | 0 | 51.88% | 1.21 | 2683.68 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 353.484µs | 63043 | 0 | 51.88% | 1.21 | 1726.67 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 371.854µs | 67007 | 0 | 51.88% | 1.21 | 1641.37 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 230.561µs | 67438 | 0 | 51.88% | 1.21 | 2647.25 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 224.529µs | 67438 | 0 | 51.88% | 1.21 | 2718.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 241.263µs | 67438 | 0 | 51.88% | 1.21 | 2529.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.090715ms | 67438 | 20000 | 51.87% | 1.21 | 559.59 MB/s |
| Quicksort | 100000 | 11.125267ms | 1718762 | 0 | 51.53% | 1.22 | 548.62 MB/s |
| Timsort | 100000 | 12.995038ms | 1759891 | 0 | 51.44% | 1.22 | 469.68 MB/s |
| ARS Gen 1: Foundation | 100000 | 62.381079ms | 0 | 300000 | 49.06% | 1.21 | 97.84 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.05672ms | 0 | 300000 | 48.80% | 1.20 | 99.96 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.488806ms | 1895222 | 108703 | 51.81% | 1.22 | 370.16 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.004249ms | 1029722 | 100000 | 51.79% | 1.21 | 1524.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.845317ms | 1029722 | 0 | 51.81% | 1.21 | 2145.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.019925ms | 1071423 | 0 | 51.79% | 1.21 | 2021.08 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.193671ms | 978520 | 0 | 51.79% | 1.21 | 1911.13 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.444258ms | 1019338 | 0 | 51.78% | 1.21 | 1772.08 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.930578ms | 1029722 | 0 | 51.79% | 1.21 | 2082.70 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.396595ms | 961965 | 0 | 51.82% | 1.21 | 1796.95 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.050432ms | 1029722 | 0 | 51.79% | 1.21 | 2000.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.122624ms | 1029722 | 200000 | 51.60% | 1.21 | 602.96 MB/s |
| Quicksort | 1000000 | 251.514139ms | 20518628 | 0 | 52.30% | 1.23 | 242.67 MB/s |
| Timsort | 1000000 | 363.015106ms | 20902099 | 0 | 51.55% | 1.21 | 168.13 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 201.182928ms | 21589743 | 1017407 | 51.75% | 1.22 | 303.38 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.060793ms | 12256776 | 1000000 | 52.24% | 1.18 | 999.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 53.873558ms | 12256776 | 0 | 52.13% | 1.17 | 1132.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 61.188689ms | 12679336 | 0 | 51.91% | 1.17 | 997.49 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 68.282714ms | 13331493 | 0 | 52.31% | 1.17 | 893.86 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 79.028305ms | 13750405 | 0 | 52.12% | 1.16 | 772.32 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 55.86867ms | 9434971 | 0 | 52.34% | 1.17 | 1092.48 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 72.615297ms | 7913738 | 0 | 52.99% | 1.16 | 840.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 60.91897ms | 8719170 | 0 | 52.70% | 1.17 | 1001.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 426.783708ms | 15199655 | 2000000 | 54.18% | 1.19 | 143.01 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 54.329µs | 10370 | 0 | 51.38% | 1.20 | 1123.44 MB/s |
| Timsort | 1000 | 61.177µs | 10522 | 0 | 51.38% | 1.20 | 997.68 MB/s |
| ARS Gen 1: Foundation | 1000 | 286.795µs | 0 | 2000 | 51.38% | 1.20 | 212.82 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 313.418µs | 0 | 2000 | 51.37% | 1.20 | 194.74 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 54.393µs | 10370 | 0 | 51.38% | 1.20 | 1122.11 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 56.93µs | 10370 | 0 | 51.38% | 1.20 | 1072.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 63.003µs | 10370 | 0 | 51.38% | 1.20 | 968.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 68.676µs | 10522 | 0 | 51.38% | 1.20 | 888.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 61.628µs | 10370 | 0 | 51.38% | 1.20 | 990.38 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 70.78µs | 10522 | 0 | 51.38% | 1.20 | 862.32 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 63.188µs | 10370 | 0 | 51.38% | 1.20 | 965.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 64.207µs | 10370 | 0 | 51.38% | 1.20 | 950.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 63.55µs | 10370 | 0 | 51.38% | 1.20 | 960.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 235.97µs | 10370 | 2000 | 51.37% | 1.20 | 258.66 MB/s |
| Quicksort | 10000 | 774.359µs | 136866 | 0 | 51.36% | 1.20 | 788.20 MB/s |
| Timsort | 10000 | 836.616µs | 141490 | 0 | 51.35% | 1.20 | 729.55 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.531789ms | 0 | 30000 | 51.23% | 1.21 | 93.44 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.280827ms | 0 | 30000 | 51.25% | 1.21 | 97.18 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.145052ms | 193846 | 14351 | 51.35% | 1.21 | 284.54 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 604.686µs | 67438 | 10000 | 51.35% | 1.20 | 1009.37 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 216.98µs | 67438 | 0 | 51.35% | 1.20 | 2812.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 256.109µs | 70298 | 0 | 51.35% | 1.20 | 2383.17 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 353.501µs | 63043 | 0 | 51.35% | 1.20 | 1726.59 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 365.913µs | 67007 | 0 | 51.35% | 1.20 | 1668.02 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 247.027µs | 67438 | 0 | 51.35% | 1.20 | 2470.79 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 229.658µs | 67438 | 0 | 51.35% | 1.20 | 2657.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 229.535µs | 67438 | 0 | 51.35% | 1.20 | 2659.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.024294ms | 67438 | 20000 | 51.34% | 1.20 | 595.88 MB/s |
| Quicksort | 100000 | 11.357633ms | 1718762 | 0 | 51.04% | 1.21 | 537.39 MB/s |
| Timsort | 100000 | 14.028778ms | 1759891 | 0 | 50.92% | 1.21 | 435.07 MB/s |
| ARS Gen 1: Foundation | 100000 | 61.20085ms | 0 | 300000 | 48.55% | 1.20 | 99.73 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.578864ms | 0 | 300000 | 48.63% | 1.20 | 99.12 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.635852ms | 1895222 | 108703 | 51.29% | 1.21 | 346.09 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.11873ms | 1029722 | 100000 | 51.27% | 1.20 | 1481.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.817196ms | 1029722 | 0 | 51.26% | 1.20 | 2166.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.103168ms | 1071423 | 0 | 51.26% | 1.20 | 1966.87 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.327956ms | 978520 | 0 | 51.26% | 1.20 | 1834.01 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.386288ms | 1019338 | 0 | 51.25% | 1.20 | 1802.42 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.934362ms | 1029722 | 0 | 51.25% | 1.20 | 2080.01 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.068069ms | 961965 | 0 | 51.28% | 1.20 | 1989.37 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.887199ms | 1029722 | 0 | 51.25% | 1.20 | 2113.99 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.501044ms | 1029722 | 200000 | 51.06% | 1.20 | 581.23 MB/s |
| Quicksort | 1000000 | 237.776612ms | 20518628 | 0 | 51.88% | 1.23 | 256.69 MB/s |
| Timsort | 1000000 | 352.536057ms | 20902099 | 0 | 51.09% | 1.21 | 173.13 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 201.165732ms | 21589743 | 1017407 | 51.17% | 1.20 | 303.41 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.276408ms | 12256776 | 1000000 | 51.67% | 1.17 | 1012.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 50.978774ms | 12256776 | 0 | 51.66% | 1.17 | 1197.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 61.1549ms | 12679336 | 0 | 51.41% | 1.16 | 998.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 64.067815ms | 13331493 | 0 | 51.72% | 1.17 | 952.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 76.599717ms | 13750405 | 0 | 51.56% | 1.16 | 796.81 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 53.902705ms | 9434971 | 0 | 51.86% | 1.18 | 1132.32 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 72.276905ms | 7913738 | 0 | 52.41% | 1.16 | 844.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 59.085028ms | 8719170 | 0 | 52.20% | 1.16 | 1033.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 459.424834ms | 15151080 | 2000000 | 53.40% | 1.18 | 132.85 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 36.012µs | 5636 | 0 | 50.76% | 1.20 | 1694.86 MB/s |
| Timsort | 1000 | 50.952µs | 5782 | 0 | 50.76% | 1.20 | 1197.90 MB/s |
| ARS Gen 1: Foundation | 1000 | 169.304µs | 984 | 2000 | 50.76% | 1.20 | 360.51 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 160.354µs | 984 | 2000 | 50.76% | 1.20 | 380.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 43.737µs | 5636 | 0 | 50.76% | 1.20 | 1395.50 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 51.6µs | 5636 | 0 | 50.76% | 1.20 | 1182.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 35.595µs | 5636 | 0 | 50.76% | 1.20 | 1714.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 47.916µs | 5782 | 0 | 50.76% | 1.20 | 1273.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 42.568µs | 5636 | 0 | 50.76% | 1.20 | 1433.83 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 56.096µs | 5782 | 0 | 50.76% | 1.20 | 1088.05 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 50.553µs | 5636 | 0 | 50.76% | 1.20 | 1207.35 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 41.325µs | 5636 | 0 | 50.76% | 1.20 | 1476.95 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 42.604µs | 5636 | 0 | 50.76% | 1.20 | 1432.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 291.668µs | 5636 | 2000 | 50.76% | 1.20 | 209.26 MB/s |
| Quicksort | 10000 | 379.287µs | 53113 | 0 | 50.74% | 1.20 | 1609.21 MB/s |
| Timsort | 10000 | 428.816µs | 54714 | 0 | 50.74% | 1.20 | 1423.34 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.711312ms | 9984 | 30000 | 50.72% | 1.20 | 356.66 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.722065ms | 9984 | 30000 | 50.72% | 1.20 | 354.43 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.932057ms | 122389 | 14351 | 50.74% | 1.20 | 315.91 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 828.702µs | 14075 | 10000 | 50.74% | 1.20 | 736.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 317.618µs | 14075 | 0 | 50.74% | 1.20 | 1921.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 202.536µs | 14094 | 0 | 50.74% | 1.20 | 3013.55 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 413.913µs | 12021 | 0 | 50.74% | 1.20 | 1474.59 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 383.432µs | 12028 | 0 | 50.74% | 1.20 | 1591.81 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 250.045µs | 14075 | 0 | 50.74% | 1.20 | 2440.97 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 258.22µs | 14075 | 0 | 50.74% | 1.20 | 2363.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 159.663µs | 14075 | 0 | 50.74% | 1.20 | 3822.75 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.319496ms | 14075 | 20000 | 50.72% | 1.20 | 462.56 MB/s |
| Quicksort | 100000 | 3.964419ms | 516589 | 0 | 50.57% | 1.20 | 1539.57 MB/s |
| Timsort | 100000 | 4.862805ms | 529550 | 0 | 50.51% | 1.20 | 1255.14 MB/s |
| ARS Gen 1: Foundation | 100000 | 16.908729ms | 99984 | 300000 | 50.60% | 1.20 | 360.97 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 18.047959ms | 99984 | 300000 | 50.60% | 1.20 | 338.18 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.447927ms | 1144965 | 108703 | 50.71% | 1.20 | 371.08 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.760861ms | 151083 | 100000 | 50.73% | 1.20 | 1622.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.681655ms | 151083 | 0 | 50.72% | 1.20 | 3629.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.813704ms | 151309 | 0 | 50.72% | 1.20 | 3365.22 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.147499ms | 99990 | 0 | 50.70% | 1.20 | 2842.15 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.123197ms | 99990 | 0 | 50.71% | 1.20 | 2874.68 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.273039ms | 200008 | 0 | 50.67% | 1.20 | 2685.18 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.162562ms | 200008 | 0 | 50.66% | 1.20 | 2822.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.806252ms | 100024 | 0 | 50.71% | 1.20 | 3379.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 8.912785ms | 151083 | 200000 | 50.55% | 1.20 | 684.80 MB/s |
| Quicksort | 1000000 | 90.289024ms | 5202060 | 0 | 51.81% | 1.20 | 676.00 MB/s |
| Timsort | 1000000 | 146.02849ms | 6111262 | 0 | 51.95% | 1.19 | 417.97 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 160.512913ms | 12085476 | 1017407 | 50.96% | 1.21 | 380.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 30.8004ms | 999988 | 1000000 | 51.25% | 1.18 | 1981.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 24.548708ms | 999988 | 0 | 51.15% | 1.18 | 2486.29 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 21.774638ms | 999988 | 0 | 51.11% | 1.18 | 2803.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 25.900339ms | 999988 | 0 | 51.21% | 1.18 | 2356.54 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 25.130617ms | 999988 | 0 | 51.13% | 1.18 | 2428.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 33.819381ms | 1999972 | 0 | 51.29% | 1.17 | 1804.74 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 39.501108ms | 1999976 | 0 | 51.47% | 1.18 | 1545.15 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 36.519182ms | 1999976 | 0 | 51.37% | 1.17 | 1671.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 322.894166ms | 5709060 | 2000000 | 53.28% | 1.18 | 189.03 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 57.386µs | 10308 | 0 | 50.67% | 1.19 | 1063.59 MB/s |
| Timsort | 1000 | 67.153µs | 10658 | 0 | 50.67% | 1.19 | 908.90 MB/s |
| ARS Gen 1: Foundation | 1000 | 135.824µs | 10308 | 2000 | 50.66% | 1.19 | 449.37 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 140.202µs | 10308 | 2000 | 50.66% | 1.19 | 435.34 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 54.138µs | 10308 | 0 | 50.67% | 1.19 | 1127.40 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 57.841µs | 10308 | 0 | 50.67% | 1.19 | 1055.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 53.43µs | 10308 | 0 | 50.67% | 1.19 | 1142.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 63.116µs | 10658 | 0 | 50.67% | 1.19 | 967.03 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 53.78µs | 10308 | 0 | 50.67% | 1.19 | 1134.90 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 64.135µs | 10658 | 0 | 50.67% | 1.19 | 951.67 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 52.439µs | 10308 | 0 | 50.67% | 1.19 | 1163.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 53.323µs | 10308 | 0 | 50.67% | 1.19 | 1144.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 54.824µs | 10308 | 0 | 50.67% | 1.19 | 1113.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 232.298µs | 10308 | 2000 | 50.66% | 1.19 | 262.75 MB/s |
| Quicksort | 10000 | 761.93µs | 138349 | 0 | 50.63% | 1.19 | 801.06 MB/s |
| Timsort | 10000 | 921.81µs | 142268 | 0 | 50.62% | 1.19 | 662.12 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.377889ms | 138349 | 30000 | 50.59% | 1.19 | 256.68 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.449038ms | 138349 | 30000 | 50.59% | 1.19 | 249.22 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.368552ms | 193925 | 14351 | 50.62% | 1.19 | 257.69 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.505029ms | 138355 | 10000 | 50.62% | 1.19 | 405.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 903.524µs | 138355 | 0 | 50.61% | 1.19 | 675.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.083921ms | 142274 | 0 | 50.61% | 1.19 | 563.10 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 982.524µs | 138355 | 0 | 50.61% | 1.19 | 621.21 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.35957ms | 142274 | 0 | 50.60% | 1.19 | 448.93 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 820.763µs | 138355 | 0 | 50.62% | 1.19 | 743.64 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 897.208µs | 138355 | 0 | 50.61% | 1.19 | 680.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 928.191µs | 138355 | 0 | 50.61% | 1.19 | 657.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.964782ms | 138355 | 20000 | 50.59% | 1.19 | 310.65 MB/s |
| Quicksort | 100000 | 15.077847ms | 1715173 | 0 | 50.41% | 1.19 | 404.80 MB/s |
| Timsort | 100000 | 18.347382ms | 1762853 | 0 | 50.37% | 1.19 | 332.66 MB/s |
| ARS Gen 1: Foundation | 100000 | 46.637415ms | 1715173 | 300000 | 50.34% | 1.19 | 130.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 47.269223ms | 1715173 | 300000 | 50.38% | 1.19 | 129.12 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 20.951718ms | 1895407 | 108703 | 50.64% | 1.19 | 291.31 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 19.868256ms | 1715179 | 100000 | 50.39% | 1.19 | 307.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 21.161083ms | 1715179 | 0 | 50.46% | 1.19 | 288.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 28.487486ms | 1762859 | 0 | 50.44% | 1.19 | 214.25 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 21.468875ms | 1715179 | 0 | 50.44% | 1.19 | 284.30 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 29.384453ms | 1762859 | 0 | 50.41% | 1.19 | 207.71 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 21.097437ms | 1715179 | 0 | 50.44% | 1.19 | 289.30 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 19.277717ms | 1715179 | 0 | 50.44% | 1.19 | 316.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 20.326124ms | 1715179 | 0 | 50.44% | 1.19 | 300.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 33.856137ms | 1715179 | 200000 | 50.26% | 1.19 | 180.28 MB/s |
| Quicksort | 1000000 | 516.451005ms | 20523276 | 0 | 52.81% | 1.17 | 118.18 MB/s |
| Timsort | 1000000 | 724.39774ms | 20914644 | 0 | 52.37% | 1.14 | 84.26 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 249.892887ms | 21586854 | 1017407 | 50.63% | 1.17 | 244.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 499.70238ms | 20523280 | 1000000 | 52.97% | 1.17 | 122.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 470.220601ms | 20523280 | 0 | 53.01% | 1.17 | 129.80 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 621.369809ms | 20914648 | 0 | 52.48% | 1.15 | 98.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 463.698717ms | 20523280 | 0 | 53.26% | 1.16 | 131.63 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 639.543682ms | 20914648 | 0 | 52.73% | 1.14 | 95.44 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 449.337569ms | 20523280 | 0 | 52.89% | 1.18 | 135.83 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 437.737997ms | 20523280 | 0 | 52.92% | 1.18 | 139.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 485.885143ms | 20523280 | 0 | 53.04% | 1.17 | 125.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 659.244348ms | 21960826 | 2000000 | 55.78% | 1.12 | 92.58 MB/s |

## Category: Custom

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 21.137µs | 10378 | 0 | 62.88% | 1.07 | 2165.70 MB/s |
| Timsort | 1000 | 28.204µs | 10965 | 0 | 62.88% | 1.07 | 1623.05 MB/s |
| ARS Gen 1: Foundation | 1000 | 206.227µs | 0 | 2000 | 62.88% | 1.07 | 221.97 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 249.724µs | 0 | 2000 | 62.88% | 1.07 | 183.31 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 23.654µs | 10378 | 0 | 62.88% | 1.07 | 1935.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 19.807µs | 10378 | 0 | 62.88% | 1.07 | 2311.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 23.795µs | 10378 | 0 | 62.88% | 1.07 | 1923.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.137µs | 10965 | 0 | 62.88% | 1.07 | 1686.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 20.153µs | 10378 | 0 | 62.88% | 1.07 | 2271.44 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 28.972µs | 10965 | 0 | 62.88% | 1.07 | 1580.02 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.917µs | 10378 | 0 | 62.88% | 1.07 | 2419.85 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 18.946µs | 10378 | 0 | 62.88% | 1.07 | 2416.15 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 18.915µs | 10378 | 0 | 62.88% | 1.07 | 2420.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 112.334µs | 10378 | 2000 | 62.88% | 1.07 | 407.50 MB/s |
| Quicksort | 10000 | 227.934µs | 138485 | 0 | 62.87% | 1.07 | 2008.32 MB/s |
| Timsort | 10000 | 326.334µs | 142802 | 0 | 62.87% | 1.07 | 1402.75 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.58117ms | 0 | 30000 | 62.84% | 1.07 | 99.92 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.631367ms | 0 | 30000 | 62.83% | 1.07 | 98.84 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 358.982µs | 194235 | 14351 | 62.86% | 1.07 | 1275.17 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 221.493µs | 53078 | 10000 | 62.86% | 1.07 | 2066.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 136.685µs | 53078 | 0 | 62.86% | 1.07 | 3349.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 143.442µs | 57974 | 0 | 62.86% | 1.07 | 3191.28 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 172.024µs | 60130 | 0 | 62.86% | 1.07 | 2661.05 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 203.972µs | 62739 | 0 | 62.86% | 1.07 | 2244.25 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 155.722µs | 53078 | 0 | 62.86% | 1.07 | 2939.62 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 135.293µs | 53078 | 0 | 62.86% | 1.07 | 3383.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 139.548µs | 53078 | 0 | 62.86% | 1.07 | 3280.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 303.065µs | 53078 | 20000 | 62.86% | 1.07 | 1510.45 MB/s |
| Quicksort | 100000 | 2.888445ms | 1716233 | 0 | 62.82% | 1.07 | 1584.81 MB/s |
| Timsort | 100000 | 4.287378ms | 1759914 | 0 | 62.77% | 1.07 | 1067.70 MB/s |
| ARS Gen 1: Foundation | 100000 | 36.22601ms | 0 | 300000 | 60.79% | 1.07 | 126.36 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 39.939793ms | 0 | 300000 | 60.58% | 1.07 | 114.61 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.760268ms | 1895170 | 108703 | 62.81% | 1.07 | 1658.40 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.510173ms | 891495 | 100000 | 62.84% | 1.07 | 3031.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.371093ms | 891495 | 0 | 62.84% | 1.07 | 3338.68 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.531299ms | 927102 | 0 | 62.84% | 1.07 | 2989.38 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.527777ms | 954799 | 0 | 62.82% | 1.07 | 2996.27 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.612655ms | 993233 | 0 | 62.82% | 1.07 | 2838.57 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.537932ms | 891495 | 0 | 62.83% | 1.07 | 2976.49 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.54314ms | 780845 | 0 | 62.83% | 1.07 | 2966.44 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.428818ms | 891495 | 0 | 62.83% | 1.07 | 3203.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.795319ms | 891495 | 200000 | 62.82% | 1.07 | 2549.76 MB/s |
| Quicksort | 1000000 | 40.210673ms | 20512439 | 0 | 62.43% | 1.08 | 1138.41 MB/s |
| Timsort | 1000000 | 72.01287ms | 20899150 | 0 | 61.92% | 1.08 | 635.67 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 34.654513ms | 21596717 | 1017407 | 62.54% | 1.07 | 1320.94 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.60104ms | 10310056 | 1000000 | 62.85% | 1.06 | 2460.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.342527ms | 10310056 | 0 | 62.85% | 1.06 | 2639.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.541281ms | 10709205 | 0 | 62.82% | 1.06 | 2468.89 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.864103ms | 13007245 | 0 | 62.79% | 1.06 | 2714.43 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 19.263006ms | 13425517 | 0 | 62.71% | 1.06 | 2376.39 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 18.841818ms | 10310056 | 0 | 62.82% | 1.06 | 2429.51 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.337972ms | 11367051 | 0 | 62.88% | 1.06 | 2250.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.597039ms | 12398342 | 0 | 62.87% | 1.07 | 2222.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.03355ms | 12262435 | 2000000 | 62.94% | 1.06 | 1039.58 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 24.413µs | 10308 | 0 | 62.91% | 1.05 | 1875.08 MB/s |
| Timsort | 1000 | 34.166µs | 10818 | 0 | 62.91% | 1.05 | 1339.82 MB/s |
| ARS Gen 1: Foundation | 1000 | 210.514µs | 458 | 2000 | 62.91% | 1.05 | 217.45 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 231.658µs | 458 | 2000 | 62.91% | 1.05 | 197.60 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 24.236µs | 10308 | 0 | 62.91% | 1.05 | 1888.78 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 24.535µs | 10308 | 0 | 62.91% | 1.05 | 1865.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 24.271µs | 10308 | 0 | 62.91% | 1.05 | 1886.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 34.291µs | 10818 | 0 | 62.91% | 1.05 | 1334.94 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 24.307µs | 10308 | 0 | 62.91% | 1.05 | 1883.26 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 34.033µs | 10818 | 0 | 62.91% | 1.05 | 1345.06 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 24.971µs | 10308 | 0 | 62.91% | 1.05 | 1833.18 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 24.181µs | 10308 | 0 | 62.91% | 1.05 | 1893.07 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 24.477µs | 10308 | 0 | 62.91% | 1.05 | 1870.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 130.024µs | 10308 | 2000 | 62.91% | 1.05 | 352.06 MB/s |
| Quicksort | 10000 | 301.086µs | 135501 | 0 | 62.91% | 1.05 | 1520.38 MB/s |
| Timsort | 10000 | 399.053µs | 140463 | 0 | 62.90% | 1.05 | 1147.12 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.46495ms | 53061 | 30000 | 62.89% | 1.05 | 312.48 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.533712ms | 53088 | 30000 | 62.89% | 1.05 | 298.47 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 440.718µs | 191553 | 14351 | 62.90% | 1.05 | 1038.68 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 256.802µs | 59910 | 10000 | 62.90% | 1.05 | 1782.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 162.036µs | 59910 | 0 | 62.90% | 1.05 | 2825.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 174.469µs | 62899 | 0 | 62.90% | 1.05 | 2623.75 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 324.424µs | 59126 | 0 | 62.90% | 1.05 | 1411.00 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 326.564µs | 61853 | 0 | 62.90% | 1.05 | 1401.76 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 176.15µs | 59910 | 0 | 62.90% | 1.05 | 2598.72 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 166.518µs | 59910 | 0 | 62.90% | 1.05 | 2749.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 171.071µs | 59910 | 0 | 62.90% | 1.05 | 2675.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 358.349µs | 59910 | 20000 | 62.90% | 1.05 | 1277.42 MB/s |
| Quicksort | 100000 | 2.521706ms | 1420515 | 0 | 62.86% | 1.05 | 1815.29 MB/s |
| Timsort | 100000 | 3.504106ms | 1424196 | 0 | 62.80% | 1.05 | 1306.36 MB/s |
| ARS Gen 1: Foundation | 100000 | 11.024901ms | 1360088 | 300000 | 62.77% | 1.05 | 415.21 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 10.611006ms | 1360044 | 300000 | 62.78% | 1.05 | 431.40 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.143825ms | 1616363 | 108703 | 62.85% | 1.05 | 1456.07 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.457913ms | 713263 | 100000 | 62.87% | 1.05 | 3139.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.291384ms | 713263 | 0 | 62.87% | 1.05 | 3544.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.500544ms | 718641 | 0 | 62.87% | 1.05 | 3050.65 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.49238ms | 681503 | 0 | 62.86% | 1.05 | 3067.34 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.749822ms | 688539 | 0 | 62.86% | 1.05 | 2616.06 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.443036ms | 713263 | 0 | 62.87% | 1.05 | 3172.23 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.531311ms | 609629 | 0 | 62.87% | 1.05 | 2989.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.488919ms | 713263 | 0 | 62.87% | 1.05 | 3074.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.897034ms | 713263 | 200000 | 62.87% | 1.05 | 2413.05 MB/s |
| Quicksort | 1000000 | 25.497462ms | 13518116 | 0 | 62.48% | 1.06 | 1795.33 MB/s |
| Timsort | 1000000 | 51.98999ms | 14666956 | 0 | 61.97% | 1.06 | 880.48 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.196703ms | 14952891 | 1017407 | 62.61% | 1.05 | 1421.77 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 16.017821ms | 4752528 | 1000000 | 62.94% | 1.04 | 2857.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.504248ms | 4752528 | 0 | 62.95% | 1.04 | 2952.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.234818ms | 4776632 | 0 | 62.87% | 1.04 | 2656.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.174549ms | 6246227 | 0 | 62.85% | 1.05 | 3016.65 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.08243ms | 6276231 | 0 | 62.69% | 1.04 | 2531.54 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.880536ms | 4706394 | 0 | 62.91% | 1.04 | 2882.55 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 21.899575ms | 2307619 | 0 | 62.84% | 1.04 | 2090.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 22.339144ms | 2550221 | 0 | 62.89% | 1.04 | 2049.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.733105ms | 11446691 | 2000000 | 62.93% | 1.04 | 1023.32 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 23.761µs | 9427 | 0 | 62.96% | 1.04 | 1926.53 MB/s |
| Timsort | 1000 | 27.532µs | 9314 | 0 | 62.96% | 1.04 | 1662.66 MB/s |
| ARS Gen 1: Foundation | 1000 | 131.68µs | 9547 | 2000 | 62.96% | 1.04 | 347.63 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 139.251µs | 9540 | 2000 | 62.96% | 1.04 | 328.73 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 23.891µs | 9427 | 0 | 62.96% | 1.04 | 1916.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 23.994µs | 9427 | 0 | 62.96% | 1.04 | 1907.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 24.294µs | 9427 | 0 | 62.96% | 1.04 | 1884.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 28.348µs | 9314 | 0 | 62.96% | 1.04 | 1614.80 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 23.716µs | 9427 | 0 | 62.96% | 1.04 | 1930.19 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.999µs | 9314 | 0 | 62.96% | 1.04 | 1634.93 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 23.864µs | 9427 | 0 | 62.96% | 1.04 | 1918.22 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 23.972µs | 9427 | 0 | 62.96% | 1.04 | 1909.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 23.744µs | 9427 | 0 | 62.96% | 1.04 | 1927.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 142.937µs | 9427 | 2000 | 62.96% | 1.04 | 320.26 MB/s |
| Quicksort | 10000 | 255.437µs | 133978 | 0 | 62.95% | 1.04 | 1792.08 MB/s |
| Timsort | 10000 | 295.758µs | 128297 | 0 | 62.95% | 1.04 | 1547.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.044436ms | 126223 | 30000 | 62.94% | 1.04 | 438.29 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.038839ms | 126108 | 30000 | 62.94% | 1.04 | 440.65 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 411.378µs | 183316 | 14351 | 62.95% | 1.04 | 1112.76 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 227.234µs | 42006 | 10000 | 62.95% | 1.04 | 2014.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 145.846µs | 42006 | 0 | 62.95% | 1.04 | 3138.68 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 145.86µs | 34856 | 0 | 62.95% | 1.04 | 3138.38 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 296.019µs | 48982 | 0 | 62.94% | 1.04 | 1546.40 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 318.281µs | 42275 | 0 | 62.94% | 1.04 | 1438.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 163.81µs | 42006 | 0 | 62.95% | 1.04 | 2794.48 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 153.549µs | 42006 | 0 | 62.95% | 1.04 | 2981.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 152.547µs | 42006 | 0 | 62.95% | 1.04 | 3000.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 354.514µs | 42006 | 20000 | 62.94% | 1.04 | 1291.24 MB/s |
| Quicksort | 100000 | 3.325485ms | 1688686 | 0 | 62.91% | 1.05 | 1376.53 MB/s |
| Timsort | 100000 | 3.971706ms | 1619959 | 0 | 62.86% | 1.05 | 1152.56 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.411629ms | 1609619 | 300000 | 62.87% | 1.05 | 439.67 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 10.522016ms | 1609452 | 300000 | 62.86% | 1.05 | 435.05 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.972648ms | 1798628 | 108703 | 62.91% | 1.05 | 1539.92 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.583859ms | 801237 | 100000 | 62.92% | 1.04 | 2890.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.349789ms | 801237 | 0 | 62.92% | 1.04 | 3391.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.110338ms | 405369 | 0 | 62.92% | 1.04 | 4122.74 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.603136ms | 871959 | 0 | 62.92% | 1.04 | 2855.43 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.469195ms | 443409 | 0 | 62.91% | 1.04 | 3115.74 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.587253ms | 801237 | 0 | 62.92% | 1.04 | 2884.00 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.462352ms | 689539 | 0 | 62.92% | 1.04 | 3130.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.483176ms | 801237 | 0 | 62.91% | 1.04 | 3086.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.712965ms | 801237 | 200000 | 62.91% | 1.04 | 2672.35 MB/s |
| Quicksort | 1000000 | 40.872949ms | 20499945 | 0 | 62.57% | 1.06 | 1119.97 MB/s |
| Timsort | 1000000 | 64.036117ms | 19254168 | 0 | 62.02% | 1.05 | 714.85 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.26163ms | 20728167 | 1017407 | 62.85% | 1.05 | 1464.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.673748ms | 9491317 | 1000000 | 62.97% | 1.04 | 2590.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.133913ms | 9491317 | 0 | 62.97% | 1.04 | 2837.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.108758ms | 4131087 | 0 | 62.97% | 1.04 | 3029.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.205863ms | 12332035 | 0 | 62.94% | 1.04 | 2660.51 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 15.506863ms | 5755621 | 0 | 62.94% | 1.04 | 2952.01 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 18.368703ms | 9491317 | 0 | 62.94% | 1.04 | 2492.08 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.335913ms | 10583380 | 0 | 62.97% | 1.04 | 2367.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.732408ms | 11681981 | 0 | 62.95% | 1.04 | 2443.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.298347ms | 14713601 | 2000000 | 62.94% | 1.04 | 1033.37 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 7.07µs | 3761 | 0 | 62.91% | 1.04 | 6474.73 MB/s |
| Timsort | 1000 | 9.062µs | 3799 | 0 | 62.91% | 1.04 | 5051.46 MB/s |
| ARS Gen 1: Foundation | 1000 | 39.669µs | 995 | 2000 | 62.91% | 1.04 | 1153.96 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 50.376µs | 995 | 2000 | 62.91% | 1.04 | 908.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 6.72µs | 3761 | 0 | 62.91% | 1.04 | 6811.96 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 6.623µs | 3761 | 0 | 62.91% | 1.04 | 6911.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 6.47µs | 3761 | 0 | 62.91% | 1.04 | 7075.17 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 8.498µs | 3799 | 0 | 62.91% | 1.04 | 5386.72 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 6.412µs | 3761 | 0 | 62.91% | 1.04 | 7139.17 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 8.227µs | 3799 | 0 | 62.91% | 1.04 | 5564.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 6.226µs | 3761 | 0 | 62.91% | 1.04 | 7352.45 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 6.144µs | 3761 | 0 | 62.91% | 1.04 | 7450.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 6.219µs | 3761 | 0 | 62.91% | 1.04 | 7360.73 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 111.608µs | 3761 | 2000 | 62.91% | 1.04 | 410.15 MB/s |
| Quicksort | 10000 | 58.51µs | 36513 | 0 | 62.91% | 1.04 | 7823.68 MB/s |
| Timsort | 10000 | 87.388µs | 36606 | 0 | 62.91% | 1.04 | 5238.29 MB/s |
| ARS Gen 1: Foundation | 10000 | 223.663µs | 9995 | 30000 | 62.90% | 1.04 | 2046.67 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 275.189µs | 9995 | 30000 | 62.90% | 1.04 | 1663.45 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 320.178µs | 115165 | 14351 | 62.90% | 1.04 | 1429.72 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 204.159µs | 10001 | 10000 | 62.90% | 1.04 | 2242.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 128.696µs | 10001 | 0 | 62.90% | 1.04 | 3556.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 129.922µs | 10001 | 0 | 62.90% | 1.04 | 3523.37 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 263.738µs | 10001 | 0 | 62.90% | 1.04 | 1735.68 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 285.817µs | 10001 | 0 | 62.90% | 1.04 | 1601.60 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 132.454µs | 10001 | 0 | 62.90% | 1.04 | 3456.02 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 132.355µs | 10001 | 0 | 62.90% | 1.04 | 3458.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 137.648µs | 10001 | 0 | 62.90% | 1.04 | 3325.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 305.302µs | 10001 | 20000 | 62.90% | 1.04 | 1499.38 MB/s |
| Quicksort | 100000 | 701.584µs | 362118 | 0 | 62.87% | 1.04 | 6524.72 MB/s |
| Timsort | 100000 | 1.158754ms | 362412 | 0 | 62.83% | 1.04 | 3950.48 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.461429ms | 99995 | 300000 | 62.87% | 1.04 | 1859.75 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.634272ms | 99995 | 300000 | 62.87% | 1.04 | 1737.72 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.0664ms | 1131774 | 108703 | 62.87% | 1.04 | 1492.84 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.048122ms | 99999 | 100000 | 62.88% | 1.04 | 4367.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.036011ms | 99999 | 0 | 62.88% | 1.04 | 4418.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 945.832µs | 99999 | 0 | 62.87% | 1.04 | 4839.80 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 945.838µs | 99999 | 0 | 62.87% | 1.04 | 4839.77 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 955.981µs | 99999 | 0 | 62.87% | 1.04 | 4788.42 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.236433ms | 199994 | 0 | 62.87% | 1.04 | 3702.29 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.157944ms | 199994 | 0 | 62.87% | 1.04 | 3953.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.225363ms | 199994 | 0 | 62.87% | 1.04 | 3735.74 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.495615ms | 99999 | 200000 | 62.86% | 1.04 | 3060.71 MB/s |
| Quicksort | 1000000 | 10.655161ms | 3806932 | 0 | 62.79% | 1.04 | 4296.17 MB/s |
| Timsort | 1000000 | 31.400674ms | 4710561 | 0 | 62.64% | 1.04 | 1457.81 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 28.272221ms | 12059635 | 1017407 | 62.88% | 1.04 | 1619.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 13.969009ms | 1000001 | 1000000 | 62.95% | 1.03 | 3276.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 14.562709ms | 1000001 | 0 | 62.95% | 1.03 | 3143.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.2311ms | 1000001 | 0 | 62.95% | 1.03 | 3216.64 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.582911ms | 1000001 | 0 | 62.95% | 1.03 | 3637.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.432459ms | 1000001 | 0 | 62.95% | 1.03 | 3682.00 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.353403ms | 1999996 | 0 | 62.96% | 1.03 | 2637.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 16.713503ms | 1999996 | 0 | 62.97% | 1.03 | 2738.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.142049ms | 1999996 | 0 | 62.95% | 1.03 | 2670.41 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.962883ms | 5365482 | 2000000 | 62.92% | 1.03 | 1041.25 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 12.106µs | 5226 | 0 | 62.97% | 1.03 | 3781.30 MB/s |
| Timsort | 1000 | 15.997µs | 5250 | 0 | 62.97% | 1.03 | 2861.56 MB/s |
| ARS Gen 1: Foundation | 1000 | 48.318µs | 4636 | 2000 | 62.97% | 1.03 | 947.40 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.619µs | 4636 | 2000 | 62.97% | 1.03 | 780.91 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.441µs | 5226 | 0 | 62.97% | 1.03 | 4001.08 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 12.078µs | 5226 | 0 | 62.97% | 1.03 | 3790.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.939µs | 5226 | 0 | 62.97% | 1.03 | 4184.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 14.338µs | 5250 | 0 | 62.97% | 1.03 | 3192.66 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 10.582µs | 5226 | 0 | 62.97% | 1.03 | 4325.87 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 13.916µs | 5250 | 0 | 62.97% | 1.03 | 3289.48 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 9.902µs | 5226 | 0 | 62.97% | 1.03 | 4622.94 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 9.681µs | 5226 | 0 | 62.97% | 1.03 | 4728.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 9.713µs | 5226 | 0 | 62.97% | 1.03 | 4712.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 103.421µs | 5226 | 2000 | 62.96% | 1.03 | 442.62 MB/s |
| Quicksort | 10000 | 83.037µs | 53591 | 0 | 62.96% | 1.03 | 5512.77 MB/s |
| Timsort | 10000 | 114.674µs | 53226 | 0 | 62.96% | 1.03 | 3991.87 MB/s |
| ARS Gen 1: Foundation | 10000 | 298.649µs | 55100 | 30000 | 62.96% | 1.03 | 1532.78 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 370.362µs | 55099 | 30000 | 62.95% | 1.03 | 1235.99 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 272.65µs | 125304 | 14351 | 62.96% | 1.03 | 1678.94 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 325.385µs | 52153 | 10000 | 62.96% | 1.03 | 1406.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 230.907µs | 52153 | 0 | 62.95% | 1.03 | 1982.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 245.523µs | 50387 | 0 | 62.95% | 1.03 | 1864.44 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 416.018µs | 42939 | 0 | 62.95% | 1.03 | 1100.35 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 433.13µs | 43078 | 0 | 62.95% | 1.03 | 1056.87 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 206.31µs | 16855 | 0 | 62.95% | 1.03 | 2218.81 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 301.975µs | 52153 | 0 | 62.95% | 1.03 | 1515.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 219.772µs | 52153 | 0 | 62.95% | 1.03 | 2082.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 336.596µs | 52153 | 20000 | 62.95% | 1.03 | 1359.98 MB/s |
| Quicksort | 100000 | 832.254µs | 529990 | 0 | 62.93% | 1.03 | 5500.29 MB/s |
| Timsort | 100000 | 1.40929ms | 531868 | 0 | 62.89% | 1.03 | 3248.19 MB/s |
| ARS Gen 1: Foundation | 100000 | 3.288558ms | 501611 | 300000 | 62.92% | 1.03 | 1391.99 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.37463ms | 501611 | 300000 | 62.92% | 1.03 | 1356.49 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.248768ms | 1172752 | 108703 | 62.93% | 1.03 | 2035.62 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.213343ms | 516727 | 100000 | 62.92% | 1.03 | 2068.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.883937ms | 516727 | 0 | 62.92% | 1.03 | 2429.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.544894ms | 519617 | 0 | 62.88% | 1.03 | 1798.75 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.056653ms | 512024 | 0 | 62.92% | 1.03 | 2225.77 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.738386ms | 502467 | 0 | 62.89% | 1.03 | 1671.66 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.413272ms | 206221 | 0 | 62.89% | 1.02 | 1896.86 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.892588ms | 182412 | 0 | 62.91% | 1.03 | 2418.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.637882ms | 200760 | 0 | 62.89% | 1.02 | 1735.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.228857ms | 516727 | 200000 | 62.91% | 1.03 | 2053.80 MB/s |
| Quicksort | 1000000 | 13.17641ms | 5281309 | 0 | 62.80% | 1.03 | 3474.12 MB/s |
| Timsort | 1000000 | 35.344063ms | 6327917 | 0 | 62.58% | 1.03 | 1295.16 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 28.9317ms | 12313781 | 1017407 | 62.94% | 1.03 | 1582.22 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 29.164647ms | 5208498 | 1000000 | 62.88% | 1.02 | 1569.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 28.5234ms | 5208498 | 0 | 62.88% | 1.02 | 1604.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 50.302605ms | 6511840 | 0 | 62.59% | 1.02 | 910.02 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 25.172491ms | 5225265 | 0 | 62.88% | 1.02 | 1818.51 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 47.453364ms | 6529655 | 0 | 62.59% | 1.02 | 964.66 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 32.272501ms | 1939650 | 0 | 63.04% | 1.02 | 1418.43 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 45.586033ms | 2064127 | 0 | 63.06% | 1.02 | 1004.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 46.680261ms | 2062304 | 0 | 63.04% | 1.02 | 980.64 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 52.292797ms | 9705779 | 2000000 | 62.94% | 1.02 | 875.39 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 27.077µs | 10133 | 0 | 62.98% | 1.03 | 1690.60 MB/s |
| Timsort | 1000 | 38.402µs | 10734 | 0 | 62.98% | 1.03 | 1192.03 MB/s |
| ARS Gen 1: Foundation | 1000 | 206.682µs | 691 | 2000 | 62.98% | 1.03 | 221.48 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 207.882µs | 691 | 2000 | 62.98% | 1.03 | 220.20 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 23.343µs | 10133 | 0 | 62.98% | 1.03 | 1961.03 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 23.036µs | 10133 | 0 | 62.98% | 1.03 | 1987.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.339µs | 10133 | 0 | 62.98% | 1.03 | 2049.17 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 31.533µs | 10734 | 0 | 62.98% | 1.03 | 1451.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.14µs | 10133 | 0 | 62.98% | 1.03 | 2067.59 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 32.271µs | 10734 | 0 | 62.98% | 1.03 | 1418.50 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 22.539µs | 10133 | 0 | 62.98% | 1.03 | 2030.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 21.045µs | 10133 | 0 | 62.98% | 1.03 | 2175.17 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 21.162µs | 10133 | 0 | 62.98% | 1.03 | 2163.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.683µs | 10133 | 2000 | 62.98% | 1.03 | 395.71 MB/s |
| Quicksort | 10000 | 238.865µs | 133996 | 0 | 62.97% | 1.03 | 1916.41 MB/s |
| Timsort | 10000 | 320.522µs | 137398 | 0 | 62.97% | 1.03 | 1428.18 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.070434ms | 77629 | 30000 | 62.96% | 1.03 | 427.64 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.062751ms | 77623 | 30000 | 62.96% | 1.03 | 430.73 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 387.533µs | 189660 | 14351 | 62.97% | 1.03 | 1181.23 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 222.923µs | 69470 | 10000 | 62.97% | 1.03 | 2053.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 148.492µs | 69470 | 0 | 62.97% | 1.03 | 3082.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.884µs | 72482 | 0 | 62.97% | 1.03 | 2845.30 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 315.926µs | 59470 | 0 | 62.97% | 1.03 | 1448.96 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 396.587µs | 62562 | 0 | 62.97% | 1.03 | 1154.26 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 171.178µs | 69470 | 0 | 62.97% | 1.03 | 2674.20 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 145.049µs | 69470 | 0 | 62.97% | 1.03 | 3155.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 137.006µs | 69470 | 0 | 62.97% | 1.03 | 3341.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 303.911µs | 69470 | 20000 | 62.96% | 1.03 | 1506.24 MB/s |
| Quicksort | 100000 | 2.239019ms | 1339911 | 0 | 62.93% | 1.03 | 2044.48 MB/s |
| Timsort | 100000 | 3.193051ms | 1340773 | 0 | 62.89% | 1.03 | 1433.62 MB/s |
| ARS Gen 1: Foundation | 100000 | 8.530388ms | 1262245 | 300000 | 62.88% | 1.03 | 536.63 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.419549ms | 1262822 | 300000 | 62.88% | 1.03 | 543.69 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.550192ms | 1543517 | 108703 | 62.93% | 1.03 | 1795.02 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.513225ms | 727700 | 100000 | 62.95% | 1.03 | 3025.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.32219ms | 727700 | 0 | 62.95% | 1.03 | 3462.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.402848ms | 737053 | 0 | 62.94% | 1.03 | 3263.10 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.390123ms | 628511 | 0 | 62.94% | 1.03 | 3292.97 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.584741ms | 634320 | 0 | 62.94% | 1.03 | 2888.57 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.343928ms | 701327 | 0 | 62.94% | 1.03 | 3406.16 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.352953ms | 628891 | 0 | 62.94% | 1.03 | 3383.44 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.312472ms | 727700 | 0 | 62.94% | 1.03 | 3487.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.618562ms | 727700 | 200000 | 62.93% | 1.03 | 2828.21 MB/s |
| Quicksort | 1000000 | 27.781833ms | 12880459 | 0 | 62.62% | 1.03 | 1647.71 MB/s |
| Timsort | 1000000 | 52.213334ms | 13984642 | 0 | 62.17% | 1.03 | 876.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.58148ms | 14266844 | 1017407 | 62.73% | 1.03 | 1449.47 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.098753ms | 5509338 | 1000000 | 62.98% | 1.02 | 2677.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.273811ms | 5509338 | 0 | 62.99% | 1.02 | 2812.89 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.581698ms | 5538714 | 0 | 62.85% | 1.02 | 2463.52 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.760047ms | 6195850 | 0 | 62.94% | 1.02 | 2904.58 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.06709ms | 6227611 | 0 | 62.76% | 1.02 | 2533.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 21.990394ms | 2169826 | 0 | 62.91% | 1.02 | 2081.65 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 26.424866ms | 1707337 | 0 | 62.94% | 1.02 | 1732.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.281356ms | 1857655 | 0 | 63.00% | 1.02 | 1810.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.100816ms | 11804109 | 2000000 | 63.00% | 1.02 | 1062.08 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 24.964µs | 9985 | 0 | 63.05% | 1.02 | 1833.70 MB/s |
| Timsort | 1000 | 33.22µs | 10392 | 0 | 63.05% | 1.02 | 1377.98 MB/s |
| ARS Gen 1: Foundation | 1000 | 123.182µs | 5421 | 2000 | 63.05% | 1.02 | 371.62 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 139.267µs | 5356 | 2000 | 63.05% | 1.02 | 328.70 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 22.374µs | 9985 | 0 | 63.05% | 1.02 | 2045.96 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 22.044µs | 9985 | 0 | 63.05% | 1.02 | 2076.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.431µs | 9985 | 0 | 63.05% | 1.02 | 2135.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.652µs | 10392 | 0 | 63.05% | 1.02 | 1543.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.535µs | 9985 | 0 | 63.05% | 1.02 | 2125.67 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 30.055µs | 10392 | 0 | 63.05% | 1.02 | 1523.09 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 21.766µs | 9985 | 0 | 63.05% | 1.02 | 2103.11 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.9µs | 9985 | 0 | 63.05% | 1.02 | 2190.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.796µs | 9985 | 0 | 63.05% | 1.02 | 2201.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 114.128µs | 9985 | 2000 | 63.05% | 1.02 | 401.10 MB/s |
| Quicksort | 10000 | 158.357µs | 107604 | 0 | 63.04% | 1.02 | 2890.71 MB/s |
| Timsort | 10000 | 231.449µs | 109657 | 0 | 63.04% | 1.02 | 1977.82 MB/s |
| ARS Gen 1: Foundation | 10000 | 520.6µs | 73762 | 30000 | 63.04% | 1.02 | 879.30 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 642.763µs | 73552 | 30000 | 63.04% | 1.02 | 712.18 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 356.058µs | 160276 | 14351 | 63.04% | 1.02 | 1285.64 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 239.762µs | 70340 | 10000 | 63.04% | 1.02 | 1909.24 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 168.408µs | 70340 | 0 | 63.04% | 1.02 | 2718.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 214.242µs | 71216 | 0 | 63.04% | 1.02 | 2136.67 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 299.423µs | 59344 | 0 | 63.04% | 1.02 | 1528.82 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 307.012µs | 60054 | 0 | 63.04% | 1.02 | 1491.03 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 207.612µs | 70340 | 0 | 63.04% | 1.02 | 2204.90 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 189.831µs | 70340 | 0 | 63.04% | 1.02 | 2411.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 181.333µs | 70340 | 0 | 63.04% | 1.02 | 2524.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 371.025µs | 70340 | 20000 | 63.04% | 1.02 | 1233.78 MB/s |
| Quicksort | 100000 | 1.530737ms | 1011458 | 0 | 63.01% | 1.02 | 2990.48 MB/s |
| Timsort | 100000 | 2.534711ms | 1014769 | 0 | 62.96% | 1.02 | 1805.98 MB/s |
| ARS Gen 1: Foundation | 100000 | 4.291615ms | 696758 | 300000 | 63.01% | 1.02 | 1066.65 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 4.28455ms | 697287 | 300000 | 63.01% | 1.02 | 1068.41 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.520357ms | 1231300 | 108703 | 63.00% | 1.02 | 1816.27 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.363143ms | 671477 | 100000 | 63.01% | 1.02 | 3358.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.354306ms | 671477 | 0 | 63.02% | 1.02 | 3380.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.47349ms | 673524 | 0 | 63.01% | 1.02 | 3106.66 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.200466ms | 554286 | 0 | 63.02% | 1.02 | 3813.22 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.461231ms | 555220 | 0 | 63.01% | 1.02 | 3132.73 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.466965ms | 105158 | 0 | 63.00% | 1.02 | 3120.48 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.594584ms | 179970 | 0 | 63.00% | 1.02 | 2870.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.309722ms | 140724 | 0 | 63.01% | 1.02 | 3495.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.868169ms | 671477 | 200000 | 63.00% | 1.02 | 2450.33 MB/s |
| Quicksort | 1000000 | 22.213539ms | 9937773 | 0 | 62.69% | 1.02 | 2060.74 MB/s |
| Timsort | 1000000 | 46.630188ms | 11004404 | 0 | 62.24% | 1.02 | 981.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.956631ms | 12334215 | 1017407 | 62.84% | 1.02 | 1432.45 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.331355ms | 4762552 | 1000000 | 63.00% | 1.01 | 2497.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 19.128842ms | 4762552 | 0 | 63.00% | 1.01 | 2393.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 27.786551ms | 4748110 | 0 | 62.86% | 1.00 | 1647.43 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.244574ms | 4888204 | 0 | 62.97% | 1.01 | 2817.95 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 26.313672ms | 4904839 | 0 | 62.84% | 1.00 | 1739.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 26.891404ms | 1096506 | 0 | 63.08% | 1.01 | 1702.27 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 30.202031ms | 1053340 | 0 | 63.13% | 1.01 | 1515.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 26.533539ms | 1036862 | 0 | 63.11% | 1.01 | 1725.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 45.709404ms | 10645205 | 2000000 | 62.97% | 1.01 | 1001.46 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 26.685µs | 10337 | 0 | 62.94% | 1.01 | 1715.43 MB/s |
| Timsort | 1000 | 37.629µs | 10667 | 0 | 62.94% | 1.01 | 1216.52 MB/s |
| ARS Gen 1: Foundation | 1000 | 290.054µs | 0 | 2000 | 62.94% | 1.01 | 157.82 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 289.805µs | 0 | 2000 | 62.94% | 1.01 | 157.96 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 22.172µs | 10337 | 0 | 62.94% | 1.01 | 2064.60 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 22.332µs | 10337 | 0 | 62.94% | 1.01 | 2049.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.249µs | 10337 | 0 | 62.94% | 1.01 | 2057.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 30.066µs | 10667 | 0 | 62.94% | 1.01 | 1522.53 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.397µs | 10337 | 0 | 62.94% | 1.01 | 2139.38 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.467µs | 10667 | 0 | 62.94% | 1.01 | 1553.48 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.701µs | 10337 | 0 | 62.94% | 1.01 | 2211.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.831µs | 10337 | 0 | 62.94% | 1.01 | 2197.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.468µs | 10337 | 0 | 62.94% | 1.01 | 2236.48 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 120.127µs | 10337 | 2000 | 62.94% | 1.01 | 381.07 MB/s |
| Quicksort | 10000 | 236.534µs | 137946 | 0 | 62.93% | 1.01 | 1935.30 MB/s |
| Timsort | 10000 | 330.32µs | 142499 | 0 | 62.93% | 1.01 | 1385.82 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.044081ms | 0 | 30000 | 62.91% | 1.01 | 90.75 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.520528ms | 0 | 30000 | 62.90% | 1.01 | 82.92 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 410.878µs | 194806 | 14351 | 62.93% | 1.01 | 1114.11 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 221.64µs | 52643 | 10000 | 62.93% | 1.01 | 2065.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 138.528µs | 52643 | 0 | 62.93% | 1.01 | 3304.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 147.473µs | 58028 | 0 | 62.93% | 1.01 | 3104.05 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 274.005µs | 60571 | 0 | 62.93% | 1.01 | 1670.64 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 181.182µs | 63560 | 0 | 62.93% | 1.01 | 2526.54 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 162.856µs | 52643 | 0 | 62.93% | 1.01 | 2810.85 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 148.138µs | 52643 | 0 | 62.93% | 1.01 | 3090.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 132.144µs | 52643 | 0 | 62.93% | 1.01 | 3464.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 293.253µs | 52643 | 20000 | 62.92% | 1.01 | 1560.99 MB/s |
| Quicksort | 100000 | 3.535951ms | 1718970 | 0 | 62.89% | 1.01 | 1294.60 MB/s |
| Timsort | 100000 | 5.260303ms | 1756228 | 0 | 62.85% | 1.01 | 870.22 MB/s |
| ARS Gen 1: Foundation | 100000 | 38.843976ms | 5 | 300000 | 61.34% | 1.01 | 117.85 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 42.619158ms | 5 | 300000 | 61.19% | 1.01 | 107.41 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.916264ms | 1893310 | 108703 | 62.89% | 1.01 | 1569.69 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.623453ms | 888976 | 100000 | 62.91% | 1.01 | 2819.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.455197ms | 888976 | 0 | 62.91% | 1.01 | 3145.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.661488ms | 929234 | 0 | 62.91% | 1.01 | 2755.14 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.570158ms | 956140 | 0 | 62.90% | 1.01 | 2915.40 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.712867ms | 992831 | 0 | 62.89% | 1.01 | 2672.50 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.625683ms | 888976 | 0 | 62.90% | 1.01 | 2815.82 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.624639ms | 780493 | 0 | 62.90% | 1.01 | 2817.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.63408ms | 888976 | 0 | 62.90% | 1.01 | 2801.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.134145ms | 888976 | 200000 | 62.90% | 1.01 | 2144.95 MB/s |
| Quicksort | 1000000 | 42.313999ms | 20525437 | 0 | 62.60% | 1.02 | 1081.83 MB/s |
| Timsort | 1000000 | 72.999895ms | 20897754 | 0 | 62.20% | 1.01 | 627.07 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 35.686687ms | 21586005 | 1017407 | 62.67% | 1.01 | 1282.73 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.779488ms | 10308690 | 1000000 | 62.92% | 1.00 | 2437.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.24376ms | 10308690 | 0 | 62.92% | 1.00 | 2654.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.24549ms | 10708698 | 0 | 62.90% | 1.00 | 2378.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.653842ms | 13010120 | 0 | 62.87% | 1.01 | 2453.99 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.589608ms | 13427133 | 0 | 62.82% | 1.01 | 2120.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.329877ms | 10308690 | 0 | 62.90% | 1.00 | 2368.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.520223ms | 11360616 | 0 | 62.94% | 1.00 | 2230.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.680341ms | 12417054 | 0 | 62.93% | 1.00 | 2213.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 48.229447ms | 13659719 | 2000000 | 62.95% | 1.00 | 949.14 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.073µs | 5628 | 0 | 63.05% | 1.00 | 4544.46 MB/s |
| Timsort | 1000 | 12.905µs | 5482 | 0 | 63.05% | 1.00 | 3547.18 MB/s |
| ARS Gen 1: Foundation | 1000 | 53.541µs | 984 | 2000 | 63.05% | 1.00 | 854.98 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 63.159µs | 984 | 2000 | 63.05% | 1.00 | 724.78 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 9.97µs | 5628 | 0 | 63.05% | 1.00 | 4591.41 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.083µs | 5628 | 0 | 63.05% | 1.00 | 4539.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.435µs | 5628 | 0 | 63.05% | 1.00 | 4386.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 12.352µs | 5482 | 0 | 63.05% | 1.00 | 3705.99 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 10.046µs | 5628 | 0 | 63.05% | 1.00 | 4556.68 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 12.775µs | 5482 | 0 | 63.05% | 1.00 | 3583.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.011µs | 5628 | 0 | 63.05% | 1.00 | 4572.61 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.099µs | 5628 | 0 | 63.05% | 1.00 | 4532.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.151µs | 5628 | 0 | 63.05% | 1.00 | 4509.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 126.483µs | 5628 | 2000 | 63.05% | 1.00 | 361.92 MB/s |
| Quicksort | 10000 | 94.001µs | 54006 | 0 | 63.05% | 1.00 | 4869.77 MB/s |
| Timsort | 10000 | 134.482µs | 53486 | 0 | 63.05% | 1.00 | 3403.90 MB/s |
| ARS Gen 1: Foundation | 10000 | 318.43µs | 9984 | 30000 | 63.05% | 1.00 | 1437.56 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 379.707µs | 9984 | 30000 | 63.05% | 1.00 | 1205.57 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 322.303µs | 122898 | 14351 | 63.05% | 1.00 | 1420.29 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 210.892µs | 9990 | 10000 | 63.05% | 1.00 | 2170.61 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 118.846µs | 9990 | 0 | 63.05% | 1.00 | 3851.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 126.51µs | 9990 | 0 | 63.05% | 1.00 | 3618.40 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 253.934µs | 9990 | 0 | 63.05% | 1.00 | 1802.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 269.216µs | 9990 | 0 | 63.05% | 1.00 | 1700.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 135.333µs | 9990 | 0 | 63.05% | 1.00 | 3382.50 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 126.662µs | 9990 | 0 | 63.05% | 1.00 | 3614.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 128.928µs | 9990 | 0 | 63.05% | 1.00 | 3550.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 311.267µs | 9990 | 20000 | 63.04% | 1.00 | 1470.65 MB/s |
| Quicksort | 100000 | 966.2µs | 522721 | 0 | 63.02% | 1.00 | 4737.77 MB/s |
| Timsort | 100000 | 1.526369ms | 535563 | 0 | 62.97% | 1.00 | 2999.04 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.77045ms | 99984 | 300000 | 63.03% | 1.00 | 1652.31 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.015017ms | 99984 | 300000 | 63.03% | 1.00 | 1518.28 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.516779ms | 1145301 | 108703 | 63.02% | 1.00 | 1818.85 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.019298ms | 119528 | 100000 | 63.02% | 1.00 | 4490.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 929.239µs | 119528 | 0 | 63.02% | 1.00 | 4926.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 942.586µs | 119779 | 0 | 63.02% | 1.00 | 4856.47 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 939.597µs | 99990 | 0 | 63.02% | 1.00 | 4871.92 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 921.749µs | 99990 | 0 | 63.02% | 1.00 | 4966.25 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.233565ms | 199986 | 0 | 63.02% | 1.00 | 3710.90 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.234845ms | 199974 | 0 | 63.02% | 1.00 | 3707.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.179284ms | 100002 | 0 | 63.03% | 1.00 | 3881.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.44904ms | 119528 | 200000 | 63.02% | 1.00 | 3159.08 MB/s |
| Quicksort | 1000000 | 13.400335ms | 5200332 | 0 | 62.87% | 1.00 | 3416.06 MB/s |
| Timsort | 1000000 | 35.672375ms | 6204510 | 0 | 62.60% | 1.00 | 1283.24 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.024006ms | 12086670 | 1017407 | 63.02% | 1.00 | 1693.91 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 14.451459ms | 999988 | 1000000 | 63.11% | 0.99 | 3167.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 14.767799ms | 999988 | 0 | 63.10% | 1.00 | 3099.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.836973ms | 999988 | 0 | 63.10% | 0.99 | 3085.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.842886ms | 999988 | 0 | 63.09% | 1.00 | 3564.34 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.859363ms | 999988 | 0 | 63.08% | 1.00 | 3559.77 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.080488ms | 1999972 | 0 | 63.09% | 1.00 | 2680.04 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 17.008439ms | 1999972 | 0 | 63.10% | 0.99 | 2691.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.538605ms | 1999972 | 0 | 63.09% | 0.99 | 2610.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.866013ms | 5484640 | 2000000 | 63.02% | 0.99 | 1043.55 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 26.84µs | 10337 | 0 | 63.05% | 0.99 | 1705.53 MB/s |
| Timsort | 1000 | 35.957µs | 10667 | 0 | 63.05% | 0.99 | 1273.09 MB/s |
| ARS Gen 1: Foundation | 1000 | 262.522µs | 0 | 2000 | 63.05% | 0.99 | 174.37 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 279.535µs | 0 | 2000 | 63.05% | 0.99 | 163.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 22.88µs | 10337 | 0 | 63.05% | 0.99 | 2000.72 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 22.705µs | 10337 | 0 | 63.05% | 0.99 | 2016.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.171µs | 10337 | 0 | 63.05% | 0.99 | 2162.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.911µs | 10667 | 0 | 63.05% | 0.99 | 1530.42 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.398µs | 10337 | 0 | 63.05% | 0.99 | 2139.28 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.108µs | 10667 | 0 | 63.05% | 0.99 | 1572.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.679µs | 10337 | 0 | 63.05% | 0.99 | 2213.66 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 21.044µs | 10337 | 0 | 63.05% | 0.99 | 2175.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.724µs | 10337 | 0 | 63.05% | 0.99 | 2208.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 118.821µs | 10337 | 2000 | 63.05% | 0.99 | 385.25 MB/s |
| Quicksort | 10000 | 241.285µs | 137946 | 0 | 63.04% | 0.99 | 1897.19 MB/s |
| Timsort | 10000 | 323.898µs | 142499 | 0 | 63.04% | 0.99 | 1413.30 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.038461ms | 0 | 30000 | 63.02% | 0.99 | 90.85 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.422445ms | 0 | 30000 | 63.02% | 0.99 | 84.42 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 410.211µs | 194806 | 14351 | 63.04% | 0.99 | 1115.92 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 240.611µs | 52643 | 10000 | 63.04% | 0.99 | 1902.51 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 135.984µs | 52643 | 0 | 63.04% | 0.99 | 3366.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 144.045µs | 58028 | 0 | 63.04% | 0.99 | 3177.92 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 232.389µs | 60571 | 0 | 63.04% | 0.99 | 1969.82 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 193.476µs | 63560 | 0 | 63.04% | 0.99 | 2366.00 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 160.713µs | 52643 | 0 | 63.04% | 0.99 | 2848.33 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 152.781µs | 52643 | 0 | 63.04% | 0.99 | 2996.21 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 135.138µs | 52643 | 0 | 63.04% | 0.99 | 3387.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 291.79µs | 52643 | 20000 | 63.04% | 0.99 | 1568.81 MB/s |
| Quicksort | 100000 | 3.269939ms | 1718970 | 0 | 63.01% | 0.99 | 1399.92 MB/s |
| Timsort | 100000 | 4.870079ms | 1756228 | 0 | 62.97% | 0.99 | 939.95 MB/s |
| ARS Gen 1: Foundation | 100000 | 38.251546ms | 5 | 300000 | 61.54% | 0.99 | 119.67 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 41.412437ms | 5 | 300000 | 61.44% | 0.99 | 110.54 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.785266ms | 1893310 | 108703 | 63.00% | 0.99 | 1643.52 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.696029ms | 888976 | 100000 | 63.02% | 0.99 | 2699.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.385886ms | 888976 | 0 | 63.02% | 0.99 | 3303.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.562598ms | 929234 | 0 | 63.02% | 0.99 | 2929.50 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.459642ms | 956140 | 0 | 63.01% | 0.99 | 3136.14 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.799795ms | 992831 | 0 | 63.01% | 0.99 | 2543.42 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.469013ms | 888976 | 0 | 63.01% | 0.99 | 3116.13 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.457914ms | 780493 | 0 | 63.01% | 0.99 | 3139.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.521634ms | 888976 | 0 | 63.01% | 0.99 | 3008.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.805531ms | 888976 | 200000 | 63.02% | 0.99 | 2535.34 MB/s |
| Quicksort | 1000000 | 45.334603ms | 20525437 | 0 | 62.74% | 1.00 | 1009.74 MB/s |
| Timsort | 1000000 | 73.345218ms | 20897754 | 0 | 62.35% | 1.00 | 624.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 36.705516ms | 21586005 | 1017407 | 62.80% | 0.99 | 1247.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.559962ms | 10308690 | 1000000 | 63.03% | 0.98 | 2466.40 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.300701ms | 10308690 | 0 | 63.04% | 0.99 | 2645.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.843343ms | 10708698 | 0 | 63.02% | 0.99 | 2429.31 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.492464ms | 13010120 | 0 | 62.98% | 0.99 | 2616.92 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.063049ms | 13427133 | 0 | 62.94% | 0.99 | 2173.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.776595ms | 10308690 | 0 | 63.02% | 0.99 | 2314.67 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.286903ms | 11360616 | 0 | 63.05% | 0.99 | 2256.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.729068ms | 12417054 | 0 | 63.05% | 0.99 | 2320.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 48.288568ms | 13562578 | 2000000 | 63.06% | 0.98 | 947.98 MB/s |
