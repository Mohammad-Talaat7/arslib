# ARS Evolution Atlas: Final Research Study

## 1. Experimental Setup
- **Cores:** 8 | **RAM:** 15864 MB
- **PMC Instrumentation:** true (Multi-thread Inherit: Enabled)
- **Statistical Setup:** Reps=7, Seed=42

## Category: i64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 13.405µs | 10227 | 0 | 16.67% | 2.54 | 1138.29 MB/s |
| Timsort | 1000 | 21.813µs | 10588 | 0 | 28.12% | 2.04 | 699.53 MB/s |
| ARS Gen 1: Foundation | 1000 | 267.594µs | 0 | 2000 | 6.99% | 2.09 | 57.02 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 311.976µs | 0 | 2000 | 1.46% | 2.08 | 48.91 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 13.261µs | 10227 | 0 | 25.00% | 2.56 | 1150.65 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 13.155µs | 10227 | 0 | 0.00% | 2.58 | 1159.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 13.341µs | 10227 | 0 | 18.18% | 2.53 | 1143.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 21.866µs | 10588 | 0 | 41.28% | 2.04 | 697.83 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 13.362µs | 10227 | 0 | 61.54% | 2.53 | 1141.95 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 21.831µs | 10588 | 0 | 25.00% | 2.05 | 698.95 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 13.024µs | 10227 | 0 | 42.11% | 2.59 | 1171.59 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 13.018µs | 10227 | 0 | 11.11% | 2.60 | 1172.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 12.872µs | 10227 | 0 | 9.09% | 2.63 | 1185.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 116.302µs | 10227 | 2000 | 2.44% | 0.65 | 131.20 MB/s |
| Quicksort | 10000 | 119.463µs | 136654 | 0 | 1.02% | 1.03 | 1277.28 MB/s |
| Timsort | 10000 | 171.276µs | 140327 | 0 | 1.00% | 1.07 | 890.89 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.697421ms | 0 | 30000 | 0.12% | 1.68 | 32.48 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.589185ms | 0 | 30000 | 0.41% | 1.75 | 33.25 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 296.691µs | 193611 | 14351 | 1.07% | 1.06 | 514.30 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 204.921µs | 51695 | 10000 | 0.69% | 0.85 | 744.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 122.828µs | 51695 | 0 | 0.51% | 0.77 | 1242.29 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 135.564µs | 57359 | 0 | 0.57% | 0.78 | 1125.58 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 160.715µs | 59671 | 0 | 0.52% | 0.73 | 949.43 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 185.48µs | 62214 | 0 | 0.56% | 0.72 | 822.66 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 126.935µs | 51695 | 0 | 0.49% | 0.73 | 1202.09 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 122.33µs | 51695 | 0 | 0.53% | 0.75 | 1247.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 122.564µs | 51695 | 0 | 0.50% | 0.77 | 1244.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 290.126µs | 51695 | 20000 | 2.39% | 0.79 | 525.94 MB/s |
| Quicksort | 100000 | 1.37044ms | 1709595 | 0 | 3.08% | 1.67 | 1113.42 MB/s |
| Timsort | 100000 | 2.03944ms | 1743505 | 0 | 2.37% | 1.61 | 748.19 MB/s |
| ARS Gen 1: Foundation | 100000 | 39.89723ms | 0 | 300000 | 1.59% | 1.01 | 38.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 37.404811ms | 0 | 300000 | 2.15% | 1.02 | 40.79 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.0133ms | 1885062 | 108703 | 7.28% | 1.34 | 757.90 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 869.06µs | 881353 | 100000 | 9.30% | 1.02 | 1755.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 547.83µs | 881353 | 0 | 5.71% | 1.01 | 2785.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 629.478µs | 921838 | 0 | 5.23% | 1.04 | 2424.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 768.922µs | 955554 | 0 | 12.86% | 1.01 | 1984.44 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 863.768µs | 991979 | 0 | 8.66% | 1.02 | 1766.54 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 555.464µs | 881353 | 0 | 4.00% | 1.08 | 2747.03 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 644.137µs | 772388 | 0 | 6.02% | 1.11 | 2368.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 547.011µs | 881353 | 0 | 3.62% | 1.04 | 2789.48 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 928.676µs | 881353 | 200000 | 11.79% | 0.95 | 1643.07 MB/s |
| Quicksort | 1000000 | 16.317585ms | 20423287 | 0 | 13.74% | 2.22 | 935.11 MB/s |
| Timsort | 1000000 | 25.834376ms | 20813246 | 0 | 17.81% | 1.87 | 590.64 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.622367ms | 21493355 | 1017407 | 28.12% | 1.45 | 739.91 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 8.920375ms | 10218658 | 1000000 | 47.78% | 0.95 | 1710.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.440906ms | 10218658 | 0 | 48.21% | 0.82 | 1807.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.247249ms | 10628212 | 0 | 47.84% | 0.94 | 1650.09 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.115156ms | 13023009 | 0 | 41.40% | 1.16 | 2144.55 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.818278ms | 13432511 | 0 | 37.04% | 1.12 | 1951.68 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.417366ms | 10218658 | 0 | 43.16% | 0.93 | 1812.77 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.144995ms | 11276404 | 0 | 49.93% | 1.09 | 1668.54 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.298139ms | 12320223 | 0 | 50.28% | 1.02 | 1641.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 20.818378ms | 12171640 | 2000000 | 46.05% | 0.88 | 732.95 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.766µs | 10330 | 0 | 44.01% | 0.92 | 1296.85 MB/s |
| Timsort | 1000 | 18.526µs | 10648 | 0 | 44.01% | 0.92 | 823.64 MB/s |
| ARS Gen 1: Foundation | 1000 | 191.07µs | 503 | 2000 | 44.00% | 0.92 | 79.86 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 222.967µs | 503 | 2000 | 44.00% | 0.92 | 68.44 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.545µs | 10330 | 0 | 44.01% | 0.92 | 1321.68 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.447µs | 10330 | 0 | 44.01% | 0.92 | 1332.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.555µs | 10330 | 0 | 44.01% | 0.92 | 1320.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.439µs | 10648 | 0 | 44.01% | 0.92 | 827.53 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.877µs | 10330 | 0 | 44.01% | 0.92 | 1284.73 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.417µs | 10648 | 0 | 44.01% | 0.92 | 828.52 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.453µs | 10330 | 0 | 44.01% | 0.92 | 1332.30 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.722µs | 10330 | 0 | 44.01% | 0.92 | 1301.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 12.142µs | 10330 | 0 | 44.01% | 0.92 | 1256.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 116.564µs | 10330 | 2000 | 43.92% | 0.92 | 130.90 MB/s |
| Quicksort | 10000 | 140.717µs | 134638 | 0 | 43.58% | 0.92 | 1084.36 MB/s |
| Timsort | 10000 | 212.864µs | 140096 | 0 | 43.56% | 0.92 | 716.83 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.245203ms | 57643 | 30000 | 43.08% | 0.93 | 122.54 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.349277ms | 57632 | 30000 | 43.03% | 0.93 | 113.09 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 337.224µs | 191358 | 14351 | 43.34% | 0.92 | 452.48 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 240.08µs | 61389 | 10000 | 43.24% | 0.92 | 635.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 147.136µs | 61389 | 0 | 43.12% | 0.92 | 1037.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 159.181µs | 64672 | 0 | 43.16% | 0.92 | 958.58 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 307.095µs | 58551 | 0 | 43.11% | 0.92 | 496.88 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 318.707µs | 61376 | 0 | 43.10% | 0.92 | 478.77 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 160.416µs | 61389 | 0 | 43.13% | 0.92 | 951.20 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 142.01µs | 61389 | 0 | 43.13% | 0.92 | 1074.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 147.178µs | 61389 | 0 | 43.11% | 0.92 | 1036.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 335.208µs | 61389 | 20000 | 42.92% | 0.92 | 455.20 MB/s |
| Quicksort | 100000 | 1.331674ms | 1446704 | 0 | 41.98% | 0.95 | 1145.84 MB/s |
| Timsort | 100000 | 1.609948ms | 1445193 | 0 | 41.01% | 0.95 | 947.78 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.222102ms | 1387258 | 300000 | 38.86% | 0.94 | 211.28 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 7.871471ms | 1386968 | 300000 | 38.90% | 0.95 | 193.85 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.167397ms | 1645061 | 108703 | 41.67% | 0.94 | 704.01 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 834.673µs | 734392 | 100000 | 41.31% | 0.93 | 1828.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 642.334µs | 734392 | 0 | 41.27% | 0.93 | 2375.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 828.983µs | 735546 | 0 | 41.37% | 0.93 | 1840.66 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 737.999µs | 701300 | 0 | 41.33% | 0.93 | 2067.59 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 782.138µs | 706496 | 0 | 41.23% | 0.93 | 1950.91 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 628.224µs | 734392 | 0 | 41.12% | 0.93 | 2428.88 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 635.858µs | 629097 | 0 | 41.24% | 0.93 | 2399.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 610.809µs | 734392 | 0 | 41.01% | 0.93 | 2498.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.011047ms | 734392 | 200000 | 41.45% | 0.93 | 1509.21 MB/s |
| Quicksort | 1000000 | 9.527864ms | 13567694 | 0 | 35.52% | 1.13 | 1601.49 MB/s |
| Timsort | 1000000 | 14.563382ms | 14681691 | 0 | 31.30% | 1.11 | 1047.75 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 18.426227ms | 14956001 | 1017407 | 38.70% | 1.08 | 828.10 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.66334ms | 4787996 | 1000000 | 46.45% | 0.89 | 2289.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.606786ms | 4787996 | 0 | 46.58% | 0.89 | 2309.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.84368ms | 4821847 | 0 | 46.63% | 0.89 | 2229.62 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.424923ms | 6224416 | 0 | 43.52% | 0.99 | 2812.72 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.759393ms | 6253879 | 0 | 42.55% | 1.00 | 2649.37 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 6.76361ms | 4757456 | 0 | 44.80% | 0.89 | 2256.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.951571ms | 2295151 | 0 | 44.10% | 0.95 | 1704.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.517214ms | 2529783 | 0 | 42.78% | 0.88 | 1791.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 17.165933ms | 11606802 | 2000000 | 37.78% | 1.03 | 888.90 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.988µs | 9762 | 0 | 41.98% | 1.19 | 1388.68 MB/s |
| Timsort | 1000 | 14.886µs | 9882 | 0 | 41.98% | 1.19 | 1025.04 MB/s |
| ARS Gen 1: Foundation | 1000 | 90.647µs | 9788 | 2000 | 41.98% | 1.19 | 168.33 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 99.247µs | 9815 | 2000 | 41.98% | 1.19 | 153.75 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.149µs | 9762 | 0 | 41.98% | 1.19 | 1368.62 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.304µs | 9762 | 0 | 41.98% | 1.19 | 1349.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.086µs | 9762 | 0 | 41.98% | 1.19 | 1376.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 16.166µs | 9882 | 0 | 41.98% | 1.19 | 943.88 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.206µs | 9762 | 0 | 41.98% | 1.19 | 1361.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 15.844µs | 9882 | 0 | 41.98% | 1.19 | 963.06 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.906µs | 9762 | 0 | 41.98% | 1.19 | 1399.12 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.901µs | 9762 | 0 | 41.98% | 1.19 | 1399.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.849µs | 9762 | 0 | 41.98% | 1.19 | 1406.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 112.923µs | 9762 | 2000 | 41.96% | 1.19 | 135.13 MB/s |
| Quicksort | 10000 | 137.897µs | 134689 | 0 | 41.87% | 1.19 | 1106.54 MB/s |
| Timsort | 10000 | 177.669µs | 132195 | 0 | 41.86% | 1.19 | 858.83 MB/s |
| ARS Gen 1: Foundation | 10000 | 801.77µs | 130386 | 30000 | 41.74% | 1.19 | 190.31 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 807.244µs | 130325 | 30000 | 41.74% | 1.19 | 189.02 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 308.445µs | 187157 | 14351 | 41.80% | 1.19 | 494.70 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 214.498µs | 45304 | 10000 | 41.75% | 1.18 | 711.37 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 135.512µs | 45304 | 0 | 41.74% | 1.18 | 1126.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 144.294µs | 36417 | 0 | 41.74% | 1.18 | 1057.48 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 291.643µs | 52081 | 0 | 41.72% | 1.18 | 523.20 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 288.14µs | 47021 | 0 | 41.72% | 1.18 | 529.56 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 142.054µs | 45304 | 0 | 41.73% | 1.18 | 1074.15 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 135.111µs | 45304 | 0 | 41.74% | 1.18 | 1129.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 140.371µs | 45304 | 0 | 41.74% | 1.18 | 1087.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 329.12µs | 45304 | 20000 | 41.69% | 1.18 | 463.62 MB/s |
| Quicksort | 100000 | 1.628453ms | 1716043 | 0 | 41.31% | 1.19 | 937.01 MB/s |
| Timsort | 100000 | 1.900954ms | 1660908 | 0 | 40.97% | 1.20 | 802.69 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.425422ms | 1643878 | 300000 | 40.58% | 1.20 | 205.49 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 7.515122ms | 1643640 | 300000 | 40.81% | 1.20 | 203.04 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.118075ms | 1830188 | 108703 | 41.25% | 1.19 | 720.41 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 763.565µs | 827444 | 100000 | 40.99% | 1.19 | 1998.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 657.402µs | 827444 | 0 | 41.09% | 1.18 | 2321.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 501.35µs | 410171 | 0 | 40.96% | 1.18 | 3043.54 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 697.54µs | 906132 | 0 | 41.19% | 1.18 | 2187.51 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 751.452µs | 448015 | 0 | 41.03% | 1.18 | 2030.57 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 614.358µs | 827444 | 0 | 40.94% | 1.18 | 2483.70 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 735.85µs | 718138 | 0 | 41.13% | 1.18 | 2073.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 653.216µs | 827444 | 0 | 40.95% | 1.18 | 2335.95 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.094468ms | 827444 | 200000 | 41.24% | 1.18 | 1394.17 MB/s |
| Quicksort | 1000000 | 16.977817ms | 20672771 | 0 | 39.11% | 1.31 | 898.75 MB/s |
| Timsort | 1000000 | 24.324909ms | 19775927 | 0 | 36.70% | 1.29 | 627.29 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.028179ms | 20984698 | 1017407 | 41.21% | 1.24 | 761.87 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.325263ms | 9742173 | 1000000 | 42.95% | 1.14 | 1636.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.467021ms | 9742173 | 0 | 43.01% | 1.11 | 1802.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.2768ms | 4127840 | 0 | 42.97% | 1.10 | 2096.91 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.203681ms | 12610499 | 0 | 41.85% | 1.19 | 2118.19 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.013744ms | 5755875 | 0 | 41.57% | 1.17 | 2537.32 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.316919ms | 9742173 | 0 | 42.48% | 1.14 | 1834.67 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.040149ms | 10843448 | 0 | 42.93% | 1.17 | 1687.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.083511ms | 11954018 | 0 | 42.73% | 1.13 | 1679.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.001147ms | 14936471 | 2000000 | 41.73% | 1.17 | 803.05 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 3.19µs | 3735 | 0 | 44.44% | 1.25 | 4783.32 MB/s |
| Timsort | 1000 | 4.438µs | 3747 | 0 | 44.44% | 1.25 | 3438.21 MB/s |
| ARS Gen 1: Foundation | 1000 | 31.2µs | 995 | 2000 | 44.44% | 1.25 | 489.06 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 42.101µs | 995 | 2000 | 44.44% | 1.25 | 362.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 3.127µs | 3735 | 0 | 44.44% | 1.25 | 4879.69 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 3.24µs | 3735 | 0 | 44.44% | 1.25 | 4709.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 3.293µs | 3735 | 0 | 44.44% | 1.25 | 4633.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 4.666µs | 3747 | 0 | 44.44% | 1.25 | 3270.21 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 3.282µs | 3735 | 0 | 44.44% | 1.25 | 4649.23 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 4.501µs | 3747 | 0 | 44.44% | 1.25 | 3390.09 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 3.21µs | 3735 | 0 | 44.44% | 1.25 | 4753.52 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 3.249µs | 3735 | 0 | 44.44% | 1.25 | 4696.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 3.163µs | 3735 | 0 | 44.44% | 1.25 | 4824.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 107.413µs | 3735 | 2000 | 44.43% | 1.25 | 142.06 MB/s |
| Quicksort | 10000 | 28.396µs | 36573 | 0 | 44.37% | 1.25 | 5373.57 MB/s |
| Timsort | 10000 | 37.317µs | 36775 | 0 | 44.37% | 1.25 | 4088.96 MB/s |
| ARS Gen 1: Foundation | 10000 | 195.628µs | 9995 | 30000 | 44.35% | 1.25 | 779.99 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 248.482µs | 9995 | 30000 | 44.35% | 1.25 | 614.08 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 248.32µs | 115988 | 14351 | 44.34% | 1.25 | 614.48 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 200.583µs | 9999 | 10000 | 44.31% | 1.25 | 760.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 118.377µs | 9999 | 0 | 44.32% | 1.25 | 1289.00 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 116.915µs | 9999 | 0 | 44.32% | 1.25 | 1305.12 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 280.742µs | 9999 | 0 | 44.31% | 1.25 | 543.52 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 266.186µs | 9999 | 0 | 44.31% | 1.25 | 573.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 124.608µs | 9999 | 0 | 44.31% | 1.25 | 1224.54 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 122.975µs | 9999 | 0 | 44.32% | 1.25 | 1240.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 121.653µs | 9999 | 0 | 44.32% | 1.25 | 1254.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 288.831µs | 9999 | 20000 | 44.28% | 1.25 | 528.29 MB/s |
| Quicksort | 100000 | 297.24µs | 362094 | 0 | 44.06% | 1.25 | 5133.49 MB/s |
| Timsort | 100000 | 381.993µs | 382517 | 0 | 43.87% | 1.25 | 3994.52 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.007552ms | 99995 | 300000 | 44.08% | 1.25 | 1514.44 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.363645ms | 99995 | 300000 | 44.07% | 1.25 | 1118.97 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.741607ms | 1129938 | 108703 | 44.05% | 1.25 | 876.13 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 547.12µs | 100001 | 100000 | 43.90% | 1.25 | 2788.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 353.656µs | 100001 | 0 | 43.89% | 1.25 | 4314.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 350.627µs | 100001 | 0 | 43.87% | 1.25 | 4351.86 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 536.675µs | 100001 | 0 | 43.92% | 1.25 | 2843.21 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 568.419µs | 100001 | 0 | 43.93% | 1.25 | 2684.43 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 554.388µs | 199996 | 0 | 43.82% | 1.25 | 2752.37 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 586.029µs | 199996 | 0 | 43.88% | 1.25 | 2603.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 542.617µs | 199996 | 0 | 43.79% | 1.25 | 2812.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 747.626µs | 100001 | 200000 | 43.86% | 1.25 | 2040.97 MB/s |
| Quicksort | 1000000 | 2.978068ms | 3809528 | 0 | 43.22% | 1.26 | 5123.72 MB/s |
| Timsort | 1000000 | 6.616352ms | 4510660 | 0 | 43.00% | 1.25 | 2306.22 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.244578ms | 12062959 | 1017407 | 43.73% | 1.27 | 792.89 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.471612ms | 999999 | 1000000 | 45.17% | 1.18 | 2357.80 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.499012ms | 999999 | 0 | 45.18% | 1.19 | 2347.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.562918ms | 999999 | 0 | 45.19% | 1.19 | 2325.00 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.392035ms | 999999 | 0 | 44.45% | 1.22 | 3474.20 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.392979ms | 999999 | 0 | 44.46% | 1.22 | 3473.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.377344ms | 1999994 | 0 | 45.34% | 1.17 | 1821.44 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 6.977059ms | 1999994 | 0 | 45.10% | 1.19 | 2186.99 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.100667ms | 1999994 | 0 | 45.34% | 1.18 | 1883.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.240092ms | 5364815 | 2000000 | 42.66% | 1.24 | 939.58 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 4.896µs | 5508 | 0 | 42.95% | 1.22 | 3116.58 MB/s |
| Timsort | 1000 | 6.99µs | 5460 | 0 | 42.95% | 1.22 | 2182.95 MB/s |
| ARS Gen 1: Foundation | 1000 | 29.83µs | 4914 | 2000 | 42.95% | 1.22 | 511.52 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 39.511µs | 4914 | 2000 | 42.95% | 1.22 | 386.19 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 4.941µs | 5508 | 0 | 42.95% | 1.22 | 3088.20 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 4.835µs | 5508 | 0 | 42.95% | 1.22 | 3155.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.913µs | 5508 | 0 | 42.95% | 1.22 | 2580.55 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 7.455µs | 5460 | 0 | 42.95% | 1.22 | 2046.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.218µs | 5508 | 0 | 42.95% | 1.22 | 2924.26 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.329µs | 5460 | 0 | 42.95% | 1.22 | 2081.97 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5.611µs | 5508 | 0 | 42.95% | 1.22 | 2719.44 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 5.607µs | 5508 | 0 | 42.95% | 1.22 | 2721.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 4.884µs | 5508 | 0 | 42.95% | 1.22 | 3124.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 94.497µs | 5508 | 2000 | 42.94% | 1.22 | 161.47 MB/s |
| Quicksort | 10000 | 37.589µs | 53621 | 0 | 42.89% | 1.22 | 4059.38 MB/s |
| Timsort | 10000 | 55.864µs | 53742 | 0 | 42.88% | 1.22 | 2731.42 MB/s |
| ARS Gen 1: Foundation | 10000 | 285.161µs | 50132 | 30000 | 42.86% | 1.22 | 535.09 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 259.519µs | 50259 | 30000 | 42.86% | 1.22 | 587.96 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 212.018µs | 124917 | 14351 | 42.86% | 1.22 | 719.69 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 249.534µs | 52500 | 10000 | 42.83% | 1.22 | 611.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 163.873µs | 52500 | 0 | 42.84% | 1.22 | 931.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 175.744µs | 51829 | 0 | 42.83% | 1.22 | 868.24 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 354.2µs | 42054 | 0 | 42.82% | 1.22 | 430.80 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 394.172µs | 42636 | 0 | 42.82% | 1.22 | 387.11 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 229.961µs | 16860 | 0 | 42.79% | 1.22 | 663.54 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 206.649µs | 52500 | 0 | 42.85% | 1.22 | 738.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 165.001µs | 52500 | 0 | 42.83% | 1.22 | 924.77 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 320.274µs | 52500 | 20000 | 42.79% | 1.22 | 476.43 MB/s |
| Quicksort | 100000 | 376.394µs | 532062 | 0 | 42.74% | 1.22 | 4053.94 MB/s |
| Timsort | 100000 | 509.354µs | 535405 | 0 | 42.57% | 1.22 | 2995.71 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.565411ms | 506805 | 300000 | 42.70% | 1.22 | 974.75 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.784838ms | 506783 | 300000 | 42.68% | 1.22 | 854.91 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.690868ms | 1174310 | 108703 | 42.69% | 1.22 | 902.42 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.301464ms | 519466 | 100000 | 42.54% | 1.22 | 1172.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 825.284µs | 519466 | 0 | 42.54% | 1.22 | 1848.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 939.921µs | 520212 | 0 | 42.39% | 1.22 | 1623.41 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 975.335µs | 499545 | 0 | 42.57% | 1.22 | 1564.47 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.059808ms | 502501 | 0 | 42.47% | 1.22 | 1439.77 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.250685ms | 203055 | 0 | 42.21% | 1.21 | 1220.03 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 950.527µs | 182074 | 0 | 42.32% | 1.22 | 1605.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.435199ms | 197448 | 0 | 42.16% | 1.22 | 1063.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.239662ms | 519466 | 200000 | 42.52% | 1.22 | 1230.88 MB/s |
| Quicksort | 1000000 | 4.24562ms | 5301519 | 0 | 42.20% | 1.23 | 3594.01 MB/s |
| Timsort | 1000000 | 8.158009ms | 6302942 | 0 | 41.45% | 1.23 | 1870.41 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.793337ms | 12308876 | 1017407 | 42.76% | 1.24 | 733.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 13.554987ms | 5221477 | 1000000 | 43.27% | 1.19 | 1125.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.339239ms | 5221477 | 0 | 43.25% | 1.18 | 1475.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.665639ms | 6004244 | 0 | 42.66% | 1.19 | 1040.44 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.128761ms | 5265586 | 0 | 42.88% | 1.21 | 1877.14 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.075545ms | 6045570 | 0 | 42.39% | 1.21 | 1263.61 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.444127ms | 1938046 | 0 | 44.39% | 1.15 | 988.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 21.811301ms | 2076365 | 0 | 45.16% | 1.16 | 699.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.410182ms | 2063926 | 0 | 45.28% | 1.15 | 600.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.631713ms | 9792115 | 2000000 | 43.43% | 1.20 | 917.45 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.481µs | 10296 | 0 | 44.96% | 1.29 | 1455.85 MB/s |
| Timsort | 1000 | 17.27µs | 10670 | 0 | 44.96% | 1.29 | 883.54 MB/s |
| ARS Gen 1: Foundation | 1000 | 159.364µs | 808 | 2000 | 44.95% | 1.29 | 95.75 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 180.406µs | 808 | 2000 | 44.96% | 1.29 | 84.58 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.589µs | 10296 | 0 | 44.96% | 1.29 | 1316.66 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.439µs | 10296 | 0 | 44.96% | 1.29 | 1333.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.525µs | 10296 | 0 | 44.96% | 1.29 | 1323.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.048µs | 10670 | 0 | 44.96% | 1.29 | 895.05 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 10.586µs | 10296 | 0 | 44.96% | 1.29 | 1441.41 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 17.134µs | 10670 | 0 | 44.96% | 1.29 | 890.56 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.281µs | 10296 | 0 | 44.96% | 1.29 | 1484.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.302µs | 10296 | 0 | 44.96% | 1.29 | 1481.15 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.315µs | 10296 | 0 | 44.96% | 1.29 | 1479.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 105.038µs | 10296 | 2000 | 44.95% | 1.29 | 145.27 MB/s |
| Quicksort | 10000 | 127.822µs | 134101 | 0 | 44.90% | 1.29 | 1193.75 MB/s |
| Timsort | 10000 | 193.128µs | 137729 | 0 | 44.90% | 1.29 | 790.09 MB/s |
| ARS Gen 1: Foundation | 10000 | 978.524µs | 84429 | 30000 | 44.85% | 1.29 | 155.94 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 975.995µs | 84430 | 30000 | 44.84% | 1.29 | 156.34 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 293.019µs | 190005 | 14351 | 44.88% | 1.29 | 520.74 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 216.884µs | 71389 | 10000 | 44.86% | 1.29 | 703.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 140.114µs | 71389 | 0 | 44.85% | 1.29 | 1089.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 149.078µs | 73990 | 0 | 44.86% | 1.29 | 1023.54 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 340.847µs | 60048 | 0 | 44.84% | 1.29 | 447.67 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 358.692µs | 62612 | 0 | 44.85% | 1.29 | 425.40 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 145.179µs | 71389 | 0 | 44.85% | 1.29 | 1051.03 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 151.185µs | 71389 | 0 | 44.86% | 1.29 | 1009.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 174.658µs | 71389 | 0 | 44.85% | 1.29 | 873.64 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 332.927µs | 71389 | 20000 | 44.82% | 1.29 | 458.32 MB/s |
| Quicksort | 100000 | 1.18204ms | 1353942 | 0 | 44.68% | 1.29 | 1290.89 MB/s |
| Timsort | 100000 | 1.45068ms | 1358979 | 0 | 44.55% | 1.29 | 1051.84 MB/s |
| ARS Gen 1: Foundation | 100000 | 6.040788ms | 1260666 | 300000 | 44.37% | 1.27 | 252.60 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 6.181187ms | 1260598 | 300000 | 44.33% | 1.28 | 246.86 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.133953ms | 1555111 | 108703 | 44.62% | 1.29 | 715.05 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 840.994µs | 735888 | 100000 | 44.61% | 1.29 | 1814.38 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 583.161µs | 735888 | 0 | 44.58% | 1.29 | 2616.57 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 694.399µs | 741765 | 0 | 44.60% | 1.29 | 2197.41 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 810.481µs | 651349 | 0 | 44.59% | 1.29 | 1882.68 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 875.966µs | 657321 | 0 | 44.61% | 1.29 | 1741.94 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 679.926µs | 710308 | 0 | 44.60% | 1.29 | 2244.18 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 608.582µs | 631417 | 0 | 44.59% | 1.29 | 2507.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 635.234µs | 735888 | 0 | 44.55% | 1.29 | 2402.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.025061ms | 735888 | 200000 | 44.61% | 1.29 | 1488.57 MB/s |
| Quicksort | 1000000 | 9.064784ms | 12909957 | 0 | 43.71% | 1.33 | 1683.30 MB/s |
| Timsort | 1000000 | 14.072253ms | 14007926 | 0 | 42.49% | 1.32 | 1084.32 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 18.10872ms | 14286900 | 1017407 | 44.07% | 1.31 | 842.62 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.371532ms | 5157050 | 1000000 | 45.46% | 1.25 | 2069.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.244863ms | 5157050 | 0 | 45.46% | 1.24 | 2106.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.395771ms | 5175392 | 0 | 45.31% | 1.24 | 2063.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.453343ms | 6007232 | 0 | 45.03% | 1.28 | 2798.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.910711ms | 6044896 | 0 | 44.69% | 1.28 | 2581.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.029197ms | 2361022 | 0 | 44.56% | 1.24 | 1689.94 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.666734ms | 1866734 | 0 | 44.83% | 1.26 | 1578.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.218936ms | 2025491 | 0 | 44.92% | 1.24 | 1655.16 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.292445ms | 11888425 | 2000000 | 45.21% | 1.27 | 936.56 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.742µs | 10451 | 0 | 45.64% | 1.34 | 1299.51 MB/s |
| Timsort | 1000 | 18.724µs | 10742 | 0 | 45.64% | 1.34 | 814.93 MB/s |
| ARS Gen 1: Foundation | 1000 | 102.527µs | 5331 | 2000 | 45.64% | 1.34 | 148.83 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 119.322µs | 5339 | 2000 | 45.64% | 1.34 | 127.88 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.771µs | 10451 | 0 | 45.64% | 1.34 | 1296.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.601µs | 10451 | 0 | 45.64% | 1.34 | 1315.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.414µs | 10451 | 0 | 45.64% | 1.34 | 1336.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.833µs | 10742 | 0 | 45.64% | 1.34 | 810.22 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 12.09µs | 10451 | 0 | 45.64% | 1.34 | 1262.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.377µs | 10742 | 0 | 45.64% | 1.34 | 830.32 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.415µs | 10451 | 0 | 45.64% | 1.34 | 1336.73 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.403µs | 10451 | 0 | 45.64% | 1.34 | 1338.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.204µs | 10451 | 0 | 45.64% | 1.34 | 1361.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 117.544µs | 10451 | 2000 | 45.63% | 1.34 | 129.81 MB/s |
| Quicksort | 10000 | 108.903µs | 111159 | 0 | 45.60% | 1.34 | 1401.14 MB/s |
| Timsort | 10000 | 145.819µs | 110728 | 0 | 45.59% | 1.34 | 1046.42 MB/s |
| ARS Gen 1: Foundation | 10000 | 435.574µs | 75427 | 30000 | 45.57% | 1.34 | 350.31 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 503.217µs | 74701 | 30000 | 45.57% | 1.34 | 303.22 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 291.932µs | 163143 | 14351 | 45.57% | 1.34 | 522.68 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 238.128µs | 72583 | 10000 | 45.56% | 1.34 | 640.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 153.48µs | 72583 | 0 | 45.56% | 1.34 | 994.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 157.763µs | 72287 | 0 | 45.56% | 1.34 | 967.20 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 292.347µs | 63448 | 0 | 45.56% | 1.34 | 521.94 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 295.317µs | 63348 | 0 | 45.56% | 1.34 | 516.69 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 160.696µs | 72583 | 0 | 45.56% | 1.34 | 949.54 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 150.503µs | 72583 | 0 | 45.56% | 1.34 | 1013.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 161.709µs | 72583 | 0 | 45.56% | 1.34 | 943.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 319.035µs | 72583 | 20000 | 45.54% | 1.34 | 478.28 MB/s |
| Quicksort | 100000 | 812.997µs | 1016581 | 0 | 45.46% | 1.34 | 1876.86 MB/s |
| Timsort | 100000 | 980.89µs | 1021185 | 0 | 45.36% | 1.34 | 1555.61 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.294275ms | 680916 | 300000 | 45.47% | 1.34 | 665.08 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.585284ms | 680031 | 300000 | 45.47% | 1.34 | 590.22 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.019728ms | 1237724 | 108703 | 45.41% | 1.34 | 755.49 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 832.253µs | 631252 | 100000 | 45.37% | 1.34 | 1833.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 598.542µs | 631252 | 0 | 45.39% | 1.34 | 2549.33 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 609.401µs | 634097 | 0 | 45.38% | 1.34 | 2503.90 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 703.132µs | 555626 | 0 | 45.40% | 1.34 | 2170.12 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 743.926µs | 562372 | 0 | 45.39% | 1.34 | 2051.12 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 718.368µs | 134521 | 0 | 45.26% | 1.34 | 2124.09 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 790.344µs | 169903 | 0 | 45.29% | 1.34 | 1930.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 646.269µs | 264519 | 0 | 45.32% | 1.34 | 2361.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.026265ms | 631252 | 200000 | 45.38% | 1.34 | 1486.83 MB/s |
| Quicksort | 1000000 | 6.995685ms | 9921218 | 0 | 44.67% | 1.36 | 2181.17 MB/s |
| Timsort | 1000000 | 11.869203ms | 11000160 | 0 | 43.69% | 1.35 | 1285.58 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.709337ms | 12348632 | 1017407 | 44.89% | 1.35 | 702.87 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.228203ms | 5346522 | 1000000 | 45.90% | 1.30 | 2111.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.303089ms | 5346522 | 0 | 45.96% | 1.30 | 2089.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 8.160624ms | 5363683 | 0 | 45.42% | 1.28 | 1869.81 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.206935ms | 5434749 | 0 | 45.63% | 1.33 | 2930.47 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.227198ms | 5451863 | 0 | 45.04% | 1.32 | 2450.35 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.412178ms | 1070102 | 0 | 45.82% | 1.28 | 1465.48 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 12.318857ms | 1041286 | 0 | 46.38% | 1.27 | 1238.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.38461ms | 1013215 | 0 | 46.18% | 1.27 | 1340.30 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.636428ms | 10955258 | 2000000 | 45.00% | 1.32 | 917.19 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.092µs | 10179 | 0 | 46.15% | 1.36 | 1375.66 MB/s |
| Timsort | 1000 | 18.342µs | 10913 | 0 | 46.15% | 1.36 | 831.90 MB/s |
| ARS Gen 1: Foundation | 1000 | 234.545µs | 0 | 2000 | 46.15% | 1.36 | 65.06 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 269.846µs | 0 | 2000 | 46.15% | 1.36 | 56.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.925µs | 10179 | 0 | 46.15% | 1.36 | 1396.69 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.951µs | 10179 | 0 | 46.15% | 1.36 | 1393.37 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.143µs | 10179 | 0 | 46.15% | 1.36 | 1369.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.843µs | 10913 | 0 | 46.15% | 1.36 | 855.17 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.11µs | 10179 | 0 | 46.15% | 1.36 | 1373.43 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 17.976µs | 10913 | 0 | 46.15% | 1.36 | 848.84 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.057µs | 10179 | 0 | 46.15% | 1.36 | 1380.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.075µs | 10179 | 0 | 46.15% | 1.36 | 1377.77 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.813µs | 10179 | 0 | 46.15% | 1.36 | 1411.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.553µs | 10179 | 2000 | 46.14% | 1.36 | 139.28 MB/s |
| Quicksort | 10000 | 137.772µs | 137738 | 0 | 46.12% | 1.36 | 1107.54 MB/s |
| Timsort | 10000 | 209.867µs | 141392 | 0 | 46.12% | 1.36 | 727.07 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.320954ms | 0 | 30000 | 45.88% | 1.37 | 28.68 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.619282ms | 0 | 30000 | 45.72% | 1.37 | 27.15 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 318.958µs | 193231 | 14351 | 46.10% | 1.36 | 478.39 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 230.201µs | 51645 | 10000 | 46.09% | 1.36 | 662.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 148.083µs | 51645 | 0 | 46.08% | 1.36 | 1030.42 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 176.481µs | 57426 | 0 | 46.08% | 1.36 | 864.61 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 343.117µs | 59080 | 0 | 46.08% | 1.36 | 444.71 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 320.666µs | 61965 | 0 | 46.07% | 1.36 | 475.85 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 159.734µs | 51645 | 0 | 46.08% | 1.36 | 955.26 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 153.995µs | 51645 | 0 | 46.09% | 1.36 | 990.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 150.507µs | 51645 | 0 | 46.08% | 1.36 | 1013.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 328.052µs | 51645 | 20000 | 46.06% | 1.36 | 465.13 MB/s |
| Quicksort | 100000 | 1.678187ms | 1704558 | 0 | 45.94% | 1.36 | 909.24 MB/s |
| Timsort | 100000 | 2.306481ms | 1748721 | 0 | 45.87% | 1.36 | 661.56 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.146801ms | 6 | 300000 | 35.63% | 1.29 | 37.08 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.740662ms | 6 | 300000 | 35.63% | 1.30 | 34.10 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.554463ms | 1886207 | 108703 | 45.92% | 1.36 | 597.34 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 969.273µs | 879882 | 100000 | 45.87% | 1.36 | 1574.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 667.212µs | 879882 | 0 | 45.88% | 1.36 | 2286.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 796.475µs | 922129 | 0 | 45.90% | 1.36 | 1915.79 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 749.216µs | 954423 | 0 | 45.87% | 1.36 | 2036.63 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 807.474µs | 993675 | 0 | 45.89% | 1.36 | 1889.69 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 663.874µs | 879882 | 0 | 45.88% | 1.36 | 2298.45 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 704.777µs | 773088 | 0 | 45.89% | 1.36 | 2165.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 650.648µs | 879882 | 0 | 45.86% | 1.36 | 2345.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.047708ms | 879882 | 200000 | 45.89% | 1.36 | 1456.40 MB/s |
| Quicksort | 1000000 | 17.329934ms | 20437271 | 0 | 45.17% | 1.40 | 880.49 MB/s |
| Timsort | 1000000 | 28.184591ms | 20799465 | 0 | 44.40% | 1.39 | 541.39 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.461469ms | 21505010 | 1017407 | 45.41% | 1.37 | 710.99 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.281497ms | 10221412 | 1000000 | 46.13% | 1.33 | 1644.00 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.460469ms | 10221412 | 0 | 46.15% | 1.32 | 1803.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.362414ms | 10628930 | 0 | 46.18% | 1.33 | 1629.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.038592ms | 12929332 | 0 | 45.92% | 1.35 | 2167.88 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.874234ms | 13335182 | 0 | 45.76% | 1.34 | 1937.81 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.3329ms | 10221412 | 0 | 45.98% | 1.33 | 1831.15 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.361386ms | 11275443 | 0 | 46.25% | 1.34 | 1629.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.407974ms | 12322876 | 0 | 46.20% | 1.33 | 1621.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.398473ms | 13517883 | 2000000 | 45.63% | 1.33 | 625.40 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.089µs | 5504 | 0 | 46.61% | 1.33 | 2998.39 MB/s |
| Timsort | 1000 | 7.148µs | 5497 | 0 | 46.61% | 1.33 | 2134.69 MB/s |
| ARS Gen 1: Foundation | 1000 | 45.422µs | 984 | 2000 | 46.61% | 1.33 | 335.93 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 57.72µs | 984 | 2000 | 46.61% | 1.33 | 264.36 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 4.889µs | 5504 | 0 | 46.61% | 1.33 | 3121.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.291µs | 5504 | 0 | 46.61% | 1.33 | 2883.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 4.912µs | 5504 | 0 | 46.61% | 1.33 | 3106.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 7.194µs | 5497 | 0 | 46.61% | 1.33 | 2121.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.113µs | 5504 | 0 | 46.61% | 1.33 | 2984.31 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.168µs | 5497 | 0 | 46.61% | 1.33 | 2128.74 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 4.919µs | 5504 | 0 | 46.61% | 1.33 | 3102.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 4.885µs | 5504 | 0 | 46.61% | 1.33 | 3123.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 4.82µs | 5504 | 0 | 46.61% | 1.33 | 3165.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.217µs | 5504 | 2000 | 46.60% | 1.33 | 139.71 MB/s |
| Quicksort | 10000 | 42.009µs | 53753 | 0 | 46.58% | 1.33 | 3632.27 MB/s |
| Timsort | 10000 | 52.628µs | 54514 | 0 | 46.58% | 1.33 | 2899.37 MB/s |
| ARS Gen 1: Foundation | 10000 | 257.823µs | 9984 | 30000 | 46.57% | 1.33 | 591.83 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 310.374µs | 9984 | 30000 | 46.57% | 1.33 | 491.63 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 279.376µs | 121806 | 14351 | 46.57% | 1.33 | 546.17 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 203.86µs | 12063 | 10000 | 46.55% | 1.33 | 748.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 124.59µs | 12063 | 0 | 46.56% | 1.33 | 1224.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 119.666µs | 12087 | 0 | 46.56% | 1.33 | 1275.11 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 256.468µs | 12063 | 0 | 46.55% | 1.33 | 594.96 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 255.971µs | 12087 | 0 | 46.55% | 1.33 | 596.11 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 127.075µs | 12063 | 0 | 46.56% | 1.33 | 1200.77 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 119.063µs | 12063 | 0 | 46.56% | 1.33 | 1281.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 124.519µs | 12063 | 0 | 46.56% | 1.33 | 1225.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 291.106µs | 12063 | 20000 | 46.54% | 1.33 | 524.17 MB/s |
| Quicksort | 100000 | 418.52µs | 522910 | 0 | 46.46% | 1.33 | 3645.89 MB/s |
| Timsort | 100000 | 515.686µs | 516617 | 0 | 46.39% | 1.33 | 2958.93 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.298957ms | 99984 | 300000 | 46.47% | 1.33 | 1174.70 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.62576ms | 99984 | 300000 | 46.47% | 1.33 | 938.56 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.79174ms | 1144941 | 108703 | 46.47% | 1.33 | 851.62 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 499.724µs | 144579 | 100000 | 46.41% | 1.33 | 3053.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 361.144µs | 144579 | 0 | 46.41% | 1.33 | 4225.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 418.32µs | 145223 | 0 | 46.42% | 1.33 | 3647.64 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 531.591µs | 99988 | 0 | 46.42% | 1.33 | 2870.40 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 520.517µs | 99988 | 0 | 46.41% | 1.33 | 2931.47 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 439.985µs | 199988 | 0 | 46.38% | 1.33 | 3468.02 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 533.696µs | 199972 | 0 | 46.40% | 1.33 | 2859.08 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 393.617µs | 100004 | 0 | 46.39% | 1.33 | 3876.56 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 742.519µs | 144579 | 200000 | 46.40% | 1.33 | 2055.00 MB/s |
| Quicksort | 1000000 | 4.077593ms | 5201420 | 0 | 45.94% | 1.34 | 3742.11 MB/s |
| Timsort | 1000000 | 7.600192ms | 6174589 | 0 | 45.45% | 1.33 | 2007.68 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.284285ms | 12089713 | 1017407 | 46.29% | 1.34 | 791.26 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.26413ms | 999990 | 1000000 | 46.87% | 1.29 | 2435.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.463575ms | 999990 | 0 | 46.87% | 1.30 | 2360.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.333135ms | 999990 | 0 | 46.86% | 1.30 | 2409.36 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.196795ms | 999990 | 0 | 46.62% | 1.32 | 3635.82 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.408316ms | 999990 | 0 | 46.62% | 1.32 | 3461.36 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.941223ms | 1999974 | 0 | 46.82% | 1.29 | 1921.47 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.096648ms | 1999974 | 0 | 46.74% | 1.31 | 2150.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.651218ms | 1999984 | 0 | 46.75% | 1.29 | 1994.30 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.567899ms | 5706274 | 2000000 | 45.91% | 1.32 | 920.99 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.656µs | 10179 | 0 | 46.02% | 1.32 | 1431.94 MB/s |
| Timsort | 1000 | 16.853µs | 10913 | 0 | 46.02% | 1.32 | 905.40 MB/s |
| ARS Gen 1: Foundation | 1000 | 222.298µs | 0 | 2000 | 46.01% | 1.32 | 68.64 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 265.024µs | 0 | 2000 | 46.01% | 1.32 | 57.58 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.574µs | 10179 | 0 | 46.02% | 1.32 | 1443.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.738µs | 10179 | 0 | 46.02% | 1.32 | 1421.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.627µs | 10179 | 0 | 46.02% | 1.32 | 1435.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.187µs | 10913 | 0 | 46.02% | 1.32 | 887.81 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.581µs | 10179 | 0 | 46.02% | 1.32 | 1317.57 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 17.416µs | 10913 | 0 | 46.02% | 1.32 | 876.14 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.81µs | 10179 | 0 | 46.02% | 1.32 | 1411.54 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.738µs | 10179 | 0 | 46.02% | 1.32 | 1421.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.656µs | 10179 | 0 | 46.02% | 1.32 | 1431.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 110.744µs | 10179 | 2000 | 46.01% | 1.32 | 137.78 MB/s |
| Quicksort | 10000 | 133.892µs | 137738 | 0 | 45.99% | 1.32 | 1139.63 MB/s |
| Timsort | 10000 | 202.701µs | 141392 | 0 | 45.99% | 1.32 | 752.77 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.021804ms | 0 | 30000 | 45.79% | 1.33 | 30.39 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.762864ms | 0 | 30000 | 45.66% | 1.33 | 26.48 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 315.527µs | 193231 | 14351 | 45.97% | 1.32 | 483.60 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 225.239µs | 51645 | 10000 | 45.96% | 1.32 | 677.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 137.208µs | 51645 | 0 | 45.96% | 1.32 | 1112.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 158.97µs | 57426 | 0 | 45.96% | 1.32 | 959.85 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 302.869µs | 59080 | 0 | 45.96% | 1.32 | 503.81 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 310.799µs | 61965 | 0 | 45.96% | 1.32 | 490.95 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 160.397µs | 51645 | 0 | 45.96% | 1.32 | 951.31 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 154.703µs | 51645 | 0 | 45.96% | 1.32 | 986.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 154.75µs | 51645 | 0 | 45.96% | 1.32 | 986.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 335.798µs | 51645 | 20000 | 45.94% | 1.32 | 454.40 MB/s |
| Quicksort | 100000 | 1.660695ms | 1704558 | 0 | 45.86% | 1.32 | 918.82 MB/s |
| Timsort | 100000 | 2.31954ms | 1748721 | 0 | 45.78% | 1.32 | 657.84 MB/s |
| ARS Gen 1: Foundation | 100000 | 44.343854ms | 6 | 300000 | 36.04% | 1.25 | 34.41 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.972599ms | 6 | 300000 | 36.76% | 1.27 | 33.93 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.362792ms | 1886207 | 108703 | 45.83% | 1.32 | 645.79 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 944.949µs | 879882 | 100000 | 45.82% | 1.32 | 1614.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 805.164µs | 879882 | 0 | 45.80% | 1.32 | 1895.12 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 783.08µs | 922129 | 0 | 45.81% | 1.32 | 1948.56 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 839.422µs | 954423 | 0 | 45.81% | 1.32 | 1817.77 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 888.704µs | 993675 | 0 | 45.83% | 1.32 | 1716.97 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 691.211µs | 879882 | 0 | 45.79% | 1.32 | 2207.54 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 688.684µs | 773088 | 0 | 45.79% | 1.32 | 2215.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 742.322µs | 879882 | 0 | 45.79% | 1.32 | 2055.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.092899ms | 879882 | 200000 | 45.81% | 1.32 | 1396.18 MB/s |
| Quicksort | 1000000 | 18.213854ms | 20437271 | 0 | 45.24% | 1.35 | 837.76 MB/s |
| Timsort | 1000000 | 29.04334ms | 20799465 | 0 | 44.53% | 1.34 | 525.38 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 22.043036ms | 21505010 | 1017407 | 45.45% | 1.33 | 692.23 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.578292ms | 10221412 | 1000000 | 46.07% | 1.30 | 1593.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.46414ms | 10221412 | 0 | 46.06% | 1.29 | 1802.76 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.115506ms | 10628930 | 0 | 46.06% | 1.29 | 1673.94 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.490042ms | 12929332 | 0 | 45.84% | 1.31 | 2351.11 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.264982ms | 13335182 | 0 | 45.74% | 1.31 | 1846.20 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.405497ms | 10221412 | 0 | 45.92% | 1.30 | 1815.33 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.159473ms | 11275443 | 0 | 46.10% | 1.31 | 1665.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.231156ms | 12322876 | 0 | 46.09% | 1.29 | 1652.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.034495ms | 13663621 | 2000000 | 45.92% | 1.30 | 609.51 MB/s |

## Category: f64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.163µs | 10325 | 0 | 46.69% | 1.28 | 840.10 MB/s |
| Timsort | 1000 | 26.295µs | 10521 | 0 | 46.69% | 1.28 | 580.29 MB/s |
| ARS Gen 1: Foundation | 1000 | 234.084µs | 0 | 2000 | 46.69% | 1.28 | 65.19 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 275.218µs | 0 | 2000 | 46.69% | 1.28 | 55.44 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.218µs | 10325 | 0 | 46.69% | 1.28 | 886.21 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.438µs | 10325 | 0 | 46.69% | 1.28 | 875.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.345µs | 10325 | 0 | 46.69% | 1.28 | 879.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.193µs | 10521 | 0 | 46.69% | 1.28 | 582.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.098µs | 10325 | 0 | 46.69% | 1.28 | 843.12 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 26.486µs | 10521 | 0 | 46.69% | 1.28 | 576.11 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.308µs | 10325 | 0 | 46.69% | 1.28 | 833.45 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.305µs | 10325 | 0 | 46.69% | 1.28 | 881.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.461µs | 10325 | 0 | 46.69% | 1.28 | 873.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 119.688µs | 10325 | 2000 | 46.69% | 1.28 | 127.49 MB/s |
| Quicksort | 10000 | 225.73µs | 136464 | 0 | 46.67% | 1.28 | 675.98 MB/s |
| Timsort | 10000 | 317.634µs | 141512 | 0 | 46.67% | 1.28 | 480.39 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.343106ms | 0 | 30000 | 46.50% | 1.29 | 28.56 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.865925ms | 0 | 30000 | 46.27% | 1.29 | 26.01 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 394.327µs | 193135 | 14351 | 46.65% | 1.28 | 386.96 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 244.868µs | 73138 | 10000 | 46.65% | 1.28 | 623.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 157.05µs | 73138 | 0 | 46.64% | 1.28 | 971.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 181.186µs | 76380 | 0 | 46.64% | 1.28 | 842.16 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 252.178µs | 62698 | 0 | 46.64% | 1.28 | 605.08 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 268.525µs | 65867 | 0 | 46.65% | 1.28 | 568.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 176.196µs | 73138 | 0 | 46.64% | 1.28 | 866.01 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 161.869µs | 73138 | 0 | 46.64% | 1.28 | 942.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 170.658µs | 73138 | 0 | 46.65% | 1.28 | 894.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 363.695µs | 73138 | 20000 | 46.63% | 1.28 | 419.55 MB/s |
| Quicksort | 100000 | 2.689509ms | 1705718 | 0 | 46.56% | 1.28 | 567.34 MB/s |
| Timsort | 100000 | 3.408736ms | 1751732 | 0 | 46.50% | 1.28 | 447.64 MB/s |
| ARS Gen 1: Foundation | 100000 | 39.124542ms | 0 | 300000 | 39.08% | 1.25 | 39.00 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 49.883509ms | 0 | 300000 | 38.10% | 1.25 | 30.59 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.133472ms | 1884272 | 108703 | 46.55% | 1.28 | 486.96 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.171425ms | 1101865 | 100000 | 46.55% | 1.28 | 1302.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 965.852µs | 1101865 | 0 | 46.56% | 1.28 | 1579.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.16164ms | 1142841 | 0 | 46.55% | 1.28 | 1313.56 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.009182ms | 1002379 | 0 | 46.54% | 1.28 | 1512.00 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.156446ms | 1045724 | 0 | 46.53% | 1.28 | 1319.46 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 981.582µs | 1101865 | 0 | 46.51% | 1.28 | 1554.51 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 881.21µs | 999614 | 0 | 46.53% | 1.28 | 1731.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.026226ms | 1101865 | 0 | 46.53% | 1.28 | 1486.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.322112ms | 1101865 | 200000 | 46.53% | 1.28 | 1154.12 MB/s |
| Quicksort | 1000000 | 29.258779ms | 20430901 | 0 | 46.08% | 1.31 | 521.51 MB/s |
| Timsort | 1000000 | 42.463337ms | 20822215 | 0 | 45.54% | 1.31 | 359.34 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.304012ms | 21498086 | 1017407 | 46.27% | 1.29 | 558.85 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.75094ms | 12665814 | 1000000 | 46.89% | 1.27 | 1419.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.178911ms | 12665814 | 0 | 46.88% | 1.27 | 1499.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 12.067716ms | 13081361 | 0 | 46.77% | 1.27 | 1264.43 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.052871ms | 13583765 | 0 | 46.66% | 1.28 | 1685.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.278472ms | 14002566 | 0 | 46.56% | 1.28 | 1352.91 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.755274ms | 6406252 | 0 | 46.47% | 1.26 | 1418.73 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.539999ms | 5861815 | 0 | 46.47% | 1.27 | 1322.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.071012ms | 7398340 | 0 | 46.58% | 1.26 | 1378.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.530657ms | 14470213 | 2000000 | 46.84% | 1.26 | 597.67 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.781µs | 10345 | 0 | 46.67% | 1.25 | 812.46 MB/s |
| Timsort | 1000 | 27.06µs | 10685 | 0 | 46.67% | 1.25 | 563.89 MB/s |
| ARS Gen 1: Foundation | 1000 | 242.213µs | 0 | 2000 | 46.67% | 1.25 | 63.00 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 284.474µs | 0 | 2000 | 46.67% | 1.25 | 53.64 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 18.04µs | 10345 | 0 | 46.67% | 1.25 | 845.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.004µs | 10345 | 0 | 46.67% | 1.25 | 847.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.208µs | 10345 | 0 | 46.67% | 1.25 | 838.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.44µs | 10685 | 0 | 46.67% | 1.25 | 556.08 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.785µs | 10345 | 0 | 46.67% | 1.25 | 812.29 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.09µs | 10685 | 0 | 46.67% | 1.25 | 563.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.971µs | 10345 | 0 | 46.67% | 1.25 | 804.32 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.988µs | 10345 | 0 | 46.67% | 1.25 | 848.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.902µs | 10345 | 0 | 46.67% | 1.25 | 852.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 123.89µs | 10345 | 2000 | 46.67% | 1.25 | 123.16 MB/s |
| Quicksort | 10000 | 232.78µs | 137462 | 0 | 46.65% | 1.25 | 655.50 MB/s |
| Timsort | 10000 | 326.76µs | 141011 | 0 | 46.65% | 1.25 | 466.97 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.466285ms | 0 | 30000 | 46.50% | 1.26 | 27.91 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.800645ms | 0 | 30000 | 46.39% | 1.26 | 26.31 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 404.584µs | 192671 | 14351 | 46.64% | 1.25 | 377.15 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 387.637µs | 125399 | 10000 | 46.63% | 1.25 | 393.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 295.906µs | 125399 | 0 | 46.63% | 1.25 | 515.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 328.384µs | 130052 | 0 | 46.63% | 1.25 | 464.66 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 388.189µs | 109718 | 0 | 46.63% | 1.25 | 393.08 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 433.102µs | 113881 | 0 | 46.63% | 1.25 | 352.31 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 254.346µs | 48812 | 0 | 46.61% | 1.25 | 599.92 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 295.641µs | 125399 | 0 | 46.63% | 1.25 | 516.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 297.256µs | 125399 | 0 | 46.63% | 1.25 | 513.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 502.757µs | 125399 | 20000 | 46.63% | 1.25 | 303.50 MB/s |
| Quicksort | 100000 | 2.614399ms | 1710455 | 0 | 46.57% | 1.25 | 583.64 MB/s |
| Timsort | 100000 | 3.433132ms | 1746462 | 0 | 46.53% | 1.25 | 444.46 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.487835ms | 0 | 300000 | 39.02% | 1.22 | 36.78 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.04822ms | 0 | 300000 | 38.59% | 1.22 | 33.87 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.065722ms | 1884751 | 108703 | 46.56% | 1.25 | 497.72 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.054501ms | 1586392 | 100000 | 46.52% | 1.26 | 742.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.70305ms | 1586392 | 0 | 46.56% | 1.25 | 895.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.167314ms | 1629438 | 0 | 46.51% | 1.25 | 704.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.42671ms | 1447738 | 0 | 46.56% | 1.25 | 1069.51 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.739303ms | 1487078 | 0 | 46.54% | 1.25 | 877.29 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.113322ms | 834504 | 0 | 46.45% | 1.25 | 1370.56 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.151081ms | 657220 | 0 | 46.44% | 1.25 | 1325.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.182545ms | 834504 | 0 | 46.45% | 1.25 | 1290.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.145714ms | 1586392 | 200000 | 46.52% | 1.25 | 711.13 MB/s |
| Quicksort | 1000000 | 29.990613ms | 20420624 | 0 | 46.15% | 1.28 | 508.79 MB/s |
| Timsort | 1000000 | 43.626329ms | 20810565 | 0 | 45.59% | 1.28 | 349.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 28.447853ms | 21491076 | 1017407 | 46.28% | 1.26 | 536.38 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 20.854795ms | 17729670 | 1000000 | 46.64% | 1.26 | 731.67 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.767099ms | 17729670 | 0 | 46.69% | 1.26 | 813.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 24.281693ms | 18126422 | 0 | 46.21% | 1.26 | 628.41 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 14.041098ms | 17798278 | 0 | 46.48% | 1.27 | 1086.72 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.044336ms | 18171061 | 0 | 46.20% | 1.26 | 845.63 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 14.1594ms | 9157468 | 0 | 46.69% | 1.24 | 1077.64 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.721283ms | 9620349 | 0 | 47.00% | 1.24 | 970.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 15.667053ms | 11727786 | 0 | 47.07% | 1.24 | 973.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 27.554287ms | 19223473 | 2000000 | 46.61% | 1.25 | 553.77 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.672µs | 9762 | 0 | 46.29% | 1.25 | 863.44 MB/s |
| Timsort | 1000 | 22.975µs | 9882 | 0 | 46.29% | 1.25 | 664.15 MB/s |
| ARS Gen 1: Foundation | 1000 | 120.491µs | 0 | 2000 | 46.29% | 1.25 | 126.64 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 162.807µs | 0 | 2000 | 46.29% | 1.25 | 93.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.648µs | 9762 | 0 | 46.29% | 1.25 | 916.55 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 16.828µs | 9762 | 0 | 46.29% | 1.25 | 906.75 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.894µs | 9762 | 0 | 46.29% | 1.25 | 903.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 22.716µs | 9882 | 0 | 46.29% | 1.25 | 671.72 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.716µs | 9762 | 0 | 46.29% | 1.25 | 861.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 23.454µs | 9882 | 0 | 46.29% | 1.25 | 650.58 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.947µs | 9762 | 0 | 46.29% | 1.25 | 850.21 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.11µs | 9762 | 0 | 46.29% | 1.25 | 891.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.933µs | 9762 | 0 | 46.29% | 1.25 | 901.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 126.756µs | 9762 | 2000 | 46.29% | 1.25 | 120.38 MB/s |
| Quicksort | 10000 | 222.708µs | 134689 | 0 | 46.27% | 1.25 | 685.15 MB/s |
| Timsort | 10000 | 285.342µs | 132195 | 0 | 46.27% | 1.25 | 534.75 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.208816ms | 0 | 30000 | 46.20% | 1.25 | 69.08 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.454186ms | 0 | 30000 | 46.15% | 1.25 | 62.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 387.151µs | 187157 | 14351 | 46.26% | 1.25 | 394.13 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 485.214µs | 129133 | 10000 | 46.25% | 1.25 | 314.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 382.902µs | 129133 | 0 | 46.25% | 1.25 | 398.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 419.01µs | 124389 | 0 | 46.25% | 1.25 | 364.16 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 303.919µs | 112273 | 0 | 46.25% | 1.25 | 502.07 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 330.557µs | 109531 | 0 | 46.25% | 1.25 | 461.61 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 232.597µs | 51743 | 0 | 46.24% | 1.25 | 656.02 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 386.183µs | 129133 | 0 | 46.25% | 1.25 | 395.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 394.018µs | 129133 | 0 | 46.25% | 1.25 | 387.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 567.701µs | 129133 | 20000 | 46.24% | 1.25 | 268.78 MB/s |
| Quicksort | 100000 | 2.590189ms | 1716043 | 0 | 46.21% | 1.25 | 589.10 MB/s |
| Timsort | 100000 | 3.041399ms | 1660908 | 0 | 46.15% | 1.25 | 501.70 MB/s |
| ARS Gen 1: Foundation | 100000 | 18.724948ms | 0 | 300000 | 44.09% | 1.25 | 81.49 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 21.193746ms | 0 | 300000 | 44.05% | 1.25 | 72.00 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.799159ms | 1830188 | 108703 | 46.21% | 1.25 | 545.12 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.409013ms | 1653890 | 100000 | 46.15% | 1.25 | 447.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.892768ms | 1653890 | 0 | 46.16% | 1.25 | 527.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.355453ms | 1589383 | 0 | 46.12% | 1.25 | 454.75 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.681736ms | 1472393 | 0 | 46.19% | 1.25 | 907.32 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.930035ms | 1387582 | 0 | 46.15% | 1.25 | 790.60 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.097059ms | 815713 | 0 | 46.10% | 1.25 | 1390.88 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.056682ms | 631229 | 0 | 46.10% | 1.25 | 1444.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.154296ms | 815713 | 0 | 46.09% | 1.25 | 1321.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.481935ms | 1653890 | 200000 | 46.15% | 1.25 | 438.23 MB/s |
| Quicksort | 1000000 | 30.748802ms | 20672771 | 0 | 45.88% | 1.28 | 496.24 MB/s |
| Timsort | 1000000 | 40.053191ms | 19775927 | 0 | 45.28% | 1.28 | 380.96 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.270166ms | 20984698 | 1017407 | 46.20% | 1.26 | 580.84 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 20.060786ms | 18442598 | 1000000 | 46.36% | 1.26 | 760.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.188362ms | 18442598 | 0 | 46.36% | 1.26 | 887.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 22.367652ms | 17501336 | 0 | 45.95% | 1.25 | 682.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.265012ms | 18449113 | 0 | 46.17% | 1.27 | 938.14 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.428597ms | 17575612 | 0 | 45.78% | 1.26 | 712.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 14.135247ms | 8914015 | 0 | 46.34% | 1.23 | 1079.49 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.359234ms | 9611874 | 0 | 46.72% | 1.24 | 993.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 16.292934ms | 11855374 | 0 | 46.75% | 1.23 | 936.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.657572ms | 16617914 | 2000000 | 46.41% | 1.25 | 776.23 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 4.869µs | 3735 | 0 | 45.91% | 1.25 | 3133.87 MB/s |
| Timsort | 1000 | 6.834µs | 3747 | 0 | 45.91% | 1.25 | 2232.78 MB/s |
| ARS Gen 1: Foundation | 1000 | 32.684µs | 995 | 2000 | 45.91% | 1.25 | 466.86 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 30.46µs | 995 | 2000 | 45.91% | 1.25 | 500.95 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 4.667µs | 3735 | 0 | 45.91% | 1.25 | 3269.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 4.728µs | 3735 | 0 | 45.91% | 1.25 | 3227.32 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 4.765µs | 3735 | 0 | 45.91% | 1.25 | 3202.26 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 6.898µs | 3747 | 0 | 45.91% | 1.25 | 2212.06 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 4.828µs | 3735 | 0 | 45.91% | 1.25 | 3160.48 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 6.841µs | 3747 | 0 | 45.91% | 1.25 | 2230.49 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 4.83µs | 3735 | 0 | 45.91% | 1.25 | 3159.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 4.745µs | 3735 | 0 | 45.91% | 1.25 | 3215.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 4.695µs | 3735 | 0 | 45.91% | 1.25 | 3250.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 108.669µs | 3735 | 2000 | 45.91% | 1.25 | 140.42 MB/s |
| Quicksort | 10000 | 43.719µs | 36573 | 0 | 45.89% | 1.25 | 3490.20 MB/s |
| Timsort | 10000 | 61.702µs | 36775 | 0 | 45.89% | 1.25 | 2472.98 MB/s |
| ARS Gen 1: Foundation | 10000 | 198.296µs | 9995 | 30000 | 45.89% | 1.25 | 769.50 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 240.106µs | 9995 | 30000 | 45.89% | 1.25 | 635.50 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 283.329µs | 115988 | 14351 | 45.88% | 1.25 | 538.55 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 198.622µs | 9999 | 10000 | 45.88% | 1.25 | 768.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 119.312µs | 9999 | 0 | 45.88% | 1.25 | 1278.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 115.823µs | 9999 | 0 | 45.88% | 1.25 | 1317.42 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 268.907µs | 9999 | 0 | 45.88% | 1.25 | 567.44 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 264.227µs | 9999 | 0 | 45.87% | 1.25 | 577.49 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 126.947µs | 9999 | 0 | 45.88% | 1.25 | 1201.98 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 120.238µs | 9999 | 0 | 45.88% | 1.25 | 1269.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 121.411µs | 9999 | 0 | 45.88% | 1.25 | 1256.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 296.37µs | 9999 | 20000 | 45.87% | 1.25 | 514.86 MB/s |
| Quicksort | 100000 | 455.669µs | 362094 | 0 | 45.81% | 1.25 | 3348.66 MB/s |
| Timsort | 100000 | 628.222µs | 382517 | 0 | 45.76% | 1.25 | 2428.88 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.196728ms | 99995 | 300000 | 45.82% | 1.25 | 1275.04 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.328026ms | 99995 | 300000 | 45.81% | 1.25 | 1148.98 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.153012ms | 1129938 | 108703 | 45.80% | 1.25 | 708.72 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 531.761µs | 100001 | 100000 | 45.76% | 1.25 | 2869.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 359.196µs | 100001 | 0 | 45.76% | 1.25 | 4248.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 361.811µs | 100001 | 0 | 45.76% | 1.25 | 4217.34 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 623.64µs | 100001 | 0 | 45.78% | 1.25 | 2446.73 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 643.604µs | 100001 | 0 | 45.78% | 1.25 | 2370.84 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 603.326µs | 199996 | 0 | 45.74% | 1.25 | 2529.11 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 618.619µs | 199996 | 0 | 45.76% | 1.25 | 2466.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 570.84µs | 199996 | 0 | 45.74% | 1.25 | 2673.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 839.409µs | 100001 | 200000 | 45.77% | 1.25 | 1817.80 MB/s |
| Quicksort | 1000000 | 4.192662ms | 3809528 | 0 | 45.59% | 1.25 | 3639.40 MB/s |
| Timsort | 1000000 | 7.772944ms | 4510660 | 0 | 45.50% | 1.25 | 1963.06 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 23.334575ms | 12062959 | 1017407 | 45.72% | 1.25 | 653.91 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.34914ms | 999999 | 1000000 | 46.10% | 1.23 | 2403.28 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.319041ms | 999999 | 0 | 46.11% | 1.23 | 2414.73 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.271858ms | 999999 | 0 | 46.10% | 1.23 | 2432.90 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.698152ms | 999999 | 0 | 45.90% | 1.24 | 3247.83 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.699568ms | 999999 | 0 | 45.90% | 1.24 | 3246.85 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.095322ms | 1999994 | 0 | 46.15% | 1.23 | 1884.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.100674ms | 1999994 | 0 | 46.06% | 1.24 | 2148.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.491162ms | 1999994 | 0 | 46.15% | 1.23 | 1797.02 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.934647ms | 5364815 | 2000000 | 45.28% | 1.25 | 805.87 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 8.378µs | 5508 | 0 | 45.37% | 1.24 | 1821.29 MB/s |
| Timsort | 1000 | 11.372µs | 5460 | 0 | 45.37% | 1.24 | 1341.79 MB/s |
| ARS Gen 1: Foundation | 1000 | 49.165µs | 921 | 2000 | 45.37% | 1.24 | 310.36 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.033µs | 921 | 2000 | 45.37% | 1.24 | 262.93 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 8.827µs | 5508 | 0 | 45.37% | 1.24 | 1728.65 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 8.14µs | 5508 | 0 | 45.37% | 1.24 | 1874.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 8.683µs | 5508 | 0 | 45.37% | 1.24 | 1757.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.545µs | 5460 | 0 | 45.37% | 1.24 | 1321.68 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 8.197µs | 5508 | 0 | 45.37% | 1.24 | 1861.51 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 11.354µs | 5460 | 0 | 45.37% | 1.24 | 1343.91 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 8.239µs | 5508 | 0 | 45.37% | 1.24 | 1852.02 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 7.862µs | 5508 | 0 | 45.37% | 1.24 | 1940.83 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.842µs | 5508 | 0 | 45.37% | 1.24 | 1945.78 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 104.029µs | 5508 | 2000 | 45.37% | 1.24 | 146.68 MB/s |
| Quicksort | 10000 | 67.574µs | 53621 | 0 | 45.35% | 1.24 | 2258.09 MB/s |
| Timsort | 10000 | 94.017µs | 53742 | 0 | 45.35% | 1.24 | 1622.98 MB/s |
| ARS Gen 1: Foundation | 10000 | 279.069µs | 9683 | 30000 | 45.34% | 1.24 | 546.77 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 316.459µs | 9683 | 30000 | 45.34% | 1.24 | 482.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 285.9µs | 124917 | 14351 | 45.34% | 1.24 | 533.71 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 199.769µs | 10961 | 10000 | 45.33% | 1.24 | 763.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 122.947µs | 10961 | 0 | 45.33% | 1.24 | 1241.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 124.231µs | 11013 | 0 | 45.33% | 1.24 | 1228.26 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 335.149µs | 13929 | 0 | 45.33% | 1.24 | 455.28 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 342.306µs | 14152 | 0 | 45.33% | 1.24 | 445.76 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 129.091µs | 15115 | 0 | 45.33% | 1.24 | 1182.02 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 124.68µs | 10961 | 0 | 45.33% | 1.24 | 1223.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 131.176µs | 10961 | 0 | 45.33% | 1.24 | 1163.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 288.513µs | 10961 | 20000 | 45.32% | 1.24 | 528.88 MB/s |
| Quicksort | 100000 | 659.729µs | 532062 | 0 | 45.28% | 1.24 | 2312.89 MB/s |
| Timsort | 100000 | 881.867µs | 535405 | 0 | 45.23% | 1.24 | 1730.28 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.769539ms | 98733 | 300000 | 45.26% | 1.24 | 862.30 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.929637ms | 98733 | 300000 | 45.26% | 1.24 | 790.76 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.264215ms | 1174310 | 108703 | 45.27% | 1.24 | 673.91 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 574.938µs | 122228 | 100000 | 45.23% | 1.24 | 2653.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 395.575µs | 122228 | 0 | 45.22% | 1.24 | 3857.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 385.592µs | 122352 | 0 | 45.23% | 1.24 | 3957.24 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 686.72µs | 151498 | 0 | 45.24% | 1.24 | 2221.98 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 696.831µs | 152054 | 0 | 45.24% | 1.24 | 2189.74 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 544.844µs | 192482 | 0 | 45.22% | 1.24 | 2800.58 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 574.097µs | 182525 | 0 | 45.23% | 1.24 | 2657.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 529.224µs | 186875 | 0 | 45.21% | 1.24 | 2883.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 802.406µs | 122228 | 200000 | 45.22% | 1.24 | 1901.63 MB/s |
| Quicksort | 1000000 | 6.221439ms | 5301519 | 0 | 45.10% | 1.25 | 2452.61 MB/s |
| Timsort | 1000000 | 10.541953ms | 6302942 | 0 | 44.87% | 1.24 | 1447.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 25.694414ms | 12308876 | 1017407 | 45.25% | 1.24 | 593.86 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.117635ms | 1094612 | 1000000 | 45.62% | 1.22 | 2494.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.181726ms | 1094612 | 0 | 45.63% | 1.23 | 2468.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.126939ms | 1095552 | 0 | 45.62% | 1.22 | 2490.44 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.65638ms | 1534410 | 0 | 45.47% | 1.24 | 3276.96 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.815273ms | 1541577 | 0 | 45.46% | 1.24 | 3168.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.202233ms | 1989097 | 0 | 45.63% | 1.22 | 1860.32 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 6.925351ms | 2062330 | 0 | 45.59% | 1.23 | 2203.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.890208ms | 2113182 | 0 | 45.64% | 1.22 | 1933.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.52632ms | 5807618 | 2000000 | 44.94% | 1.25 | 781.45 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 16.791µs | 10241 | 0 | 45.13% | 1.23 | 908.75 MB/s |
| Timsort | 1000 | 24.15µs | 10555 | 0 | 45.13% | 1.23 | 631.83 MB/s |
| ARS Gen 1: Foundation | 1000 | 214.379µs | 0 | 2000 | 45.13% | 1.23 | 71.18 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 258.722µs | 0 | 2000 | 45.13% | 1.23 | 58.98 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.876µs | 10241 | 0 | 45.13% | 1.23 | 904.17 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 16.991µs | 10241 | 0 | 45.13% | 1.23 | 898.05 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.848µs | 10241 | 0 | 45.13% | 1.23 | 905.67 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 25.599µs | 10555 | 0 | 45.13% | 1.23 | 596.07 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.123µs | 10241 | 0 | 45.13% | 1.23 | 891.13 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 24.979µs | 10555 | 0 | 45.13% | 1.23 | 610.86 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.198µs | 10241 | 0 | 45.13% | 1.23 | 887.24 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 16.634µs | 10241 | 0 | 45.13% | 1.23 | 917.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.245µs | 10241 | 0 | 45.13% | 1.23 | 939.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.934µs | 10241 | 2000 | 45.13% | 1.23 | 131.62 MB/s |
| Quicksort | 10000 | 214.613µs | 137603 | 0 | 45.11% | 1.23 | 710.99 MB/s |
| Timsort | 10000 | 297.696µs | 140916 | 0 | 45.11% | 1.23 | 512.56 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.073845ms | 0 | 30000 | 44.97% | 1.24 | 30.07 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.665283ms | 0 | 30000 | 44.88% | 1.24 | 26.93 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 383.692µs | 192365 | 14351 | 45.10% | 1.23 | 397.68 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 238.178µs | 66763 | 10000 | 45.09% | 1.23 | 640.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 153.165µs | 66763 | 0 | 45.09% | 1.23 | 996.23 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 206.4µs | 69738 | 0 | 45.09% | 1.23 | 739.28 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 255.995µs | 61148 | 0 | 45.09% | 1.23 | 596.06 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 283.995µs | 64565 | 0 | 45.09% | 1.23 | 537.29 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 166.419µs | 66763 | 0 | 45.09% | 1.23 | 916.89 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 160.765µs | 66763 | 0 | 45.09% | 1.23 | 949.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 166.604µs | 66763 | 0 | 45.09% | 1.23 | 915.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 349.857µs | 66763 | 20000 | 45.08% | 1.23 | 436.14 MB/s |
| Quicksort | 100000 | 2.687966ms | 1710395 | 0 | 45.02% | 1.24 | 567.67 MB/s |
| Timsort | 100000 | 3.410789ms | 1746952 | 0 | 44.97% | 1.24 | 447.37 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.324557ms | 0 | 300000 | 38.35% | 1.21 | 35.22 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.175585ms | 0 | 300000 | 38.43% | 1.21 | 33.78 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.009309ms | 1885598 | 108703 | 45.02% | 1.23 | 507.05 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.30499ms | 1045510 | 100000 | 45.02% | 1.23 | 1169.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.013242ms | 1045510 | 0 | 45.02% | 1.23 | 1505.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.130281ms | 1086813 | 0 | 45.02% | 1.23 | 1350.00 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 993.922µs | 981178 | 0 | 45.00% | 1.23 | 1535.21 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.151166ms | 1020346 | 0 | 44.99% | 1.23 | 1325.51 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 937.517µs | 1045510 | 0 | 44.98% | 1.23 | 1627.57 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 857.414µs | 936769 | 0 | 45.00% | 1.23 | 1779.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 898.943µs | 1045510 | 0 | 45.00% | 1.23 | 1697.41 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.328228ms | 1045510 | 200000 | 45.00% | 1.23 | 1148.81 MB/s |
| Quicksort | 1000000 | 28.515979ms | 20431039 | 0 | 44.68% | 1.26 | 535.10 MB/s |
| Timsort | 1000000 | 42.037004ms | 20806652 | 0 | 44.27% | 1.25 | 362.98 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.496698ms | 21500526 | 1017407 | 44.81% | 1.24 | 575.88 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.761573ms | 12082942 | 1000000 | 45.33% | 1.23 | 1417.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.970987ms | 12082942 | 0 | 45.33% | 1.22 | 1530.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.652939ms | 12502702 | 0 | 45.31% | 1.23 | 1309.44 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.941524ms | 13210593 | 0 | 45.13% | 1.24 | 1706.51 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 10.909238ms | 13633723 | 0 | 45.04% | 1.23 | 1398.70 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.428468ms | 7768109 | 0 | 45.11% | 1.22 | 1463.19 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.233596ms | 6294342 | 0 | 45.03% | 1.23 | 1358.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.030037ms | 7082878 | 0 | 45.00% | 1.22 | 1383.39 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.966221ms | 13879904 | 2000000 | 45.29% | 1.22 | 611.18 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 19.06µs | 10551 | 0 | 45.05% | 1.22 | 800.57 MB/s |
| Timsort | 1000 | 27.336µs | 10537 | 0 | 45.05% | 1.22 | 558.19 MB/s |
| ARS Gen 1: Foundation | 1000 | 243.541µs | 0 | 2000 | 45.05% | 1.22 | 62.65 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 291.503µs | 0 | 2000 | 45.05% | 1.22 | 52.35 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.9µs | 10551 | 0 | 45.05% | 1.22 | 852.45 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.24µs | 10551 | 0 | 45.05% | 1.22 | 836.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.112µs | 10551 | 0 | 45.05% | 1.22 | 842.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.96µs | 10537 | 0 | 45.05% | 1.22 | 545.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 19.102µs | 10551 | 0 | 45.05% | 1.22 | 798.81 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.294µs | 10537 | 0 | 45.05% | 1.22 | 559.05 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.028µs | 10551 | 0 | 45.05% | 1.22 | 801.91 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 18.371µs | 10551 | 0 | 45.05% | 1.22 | 830.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.956µs | 10551 | 0 | 45.05% | 1.22 | 849.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 124.405µs | 10551 | 2000 | 45.05% | 1.22 | 122.65 MB/s |
| Quicksort | 10000 | 231.903µs | 136744 | 0 | 45.04% | 1.22 | 657.98 MB/s |
| Timsort | 10000 | 315.421µs | 140772 | 0 | 45.03% | 1.22 | 483.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.318214ms | 0 | 30000 | 44.91% | 1.22 | 28.69 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.956847ms | 0 | 30000 | 44.71% | 1.22 | 25.62 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 410.488µs | 193085 | 14351 | 45.02% | 1.22 | 371.72 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 406.629µs | 126000 | 10000 | 45.02% | 1.22 | 375.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 285.737µs | 126000 | 0 | 45.02% | 1.22 | 534.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 365.93µs | 130426 | 0 | 45.02% | 1.22 | 416.99 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 478.68µs | 118809 | 0 | 45.02% | 1.22 | 318.77 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 493.748µs | 123101 | 0 | 45.02% | 1.22 | 309.04 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 278.061µs | 90031 | 0 | 45.01% | 1.22 | 548.76 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 372.234µs | 126000 | 0 | 45.02% | 1.22 | 409.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 301.719µs | 126000 | 0 | 45.02% | 1.22 | 505.73 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 491.823µs | 126000 | 20000 | 45.02% | 1.22 | 310.25 MB/s |
| Quicksort | 100000 | 2.52143ms | 1704961 | 0 | 44.99% | 1.22 | 605.16 MB/s |
| Timsort | 100000 | 3.399263ms | 1748322 | 0 | 44.94% | 1.22 | 448.89 MB/s |
| ARS Gen 1: Foundation | 100000 | 39.252111ms | 0 | 300000 | 39.36% | 1.21 | 38.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 47.183801ms | 0 | 300000 | 37.83% | 1.20 | 32.34 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.119021ms | 1885129 | 108703 | 44.97% | 1.22 | 489.22 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.271208ms | 1618379 | 100000 | 44.95% | 1.22 | 671.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.826002ms | 1618379 | 0 | 44.97% | 1.22 | 835.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.144364ms | 1658575 | 0 | 44.92% | 1.22 | 711.58 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.771605ms | 1529988 | 0 | 44.96% | 1.22 | 861.30 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.128119ms | 1573500 | 0 | 44.95% | 1.22 | 717.01 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.380471ms | 673827 | 0 | 44.87% | 1.22 | 1105.33 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.42194ms | 837701 | 0 | 44.90% | 1.22 | 1073.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.557565ms | 967480 | 0 | 44.90% | 1.22 | 979.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.054914ms | 1618379 | 200000 | 44.93% | 1.22 | 742.55 MB/s |
| Quicksort | 1000000 | 28.257621ms | 20435426 | 0 | 44.68% | 1.24 | 539.99 MB/s |
| Timsort | 1000000 | 40.829868ms | 20818465 | 0 | 44.25% | 1.24 | 373.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.133801ms | 21488833 | 1017407 | 44.77% | 1.23 | 583.87 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 22.67653ms | 19275700 | 1000000 | 45.07% | 1.23 | 672.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 19.283608ms | 19275700 | 0 | 45.05% | 1.23 | 791.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 25.810787ms | 19658200 | 0 | 44.69% | 1.22 | 591.18 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.946695ms | 19275700 | 0 | 44.93% | 1.23 | 956.86 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 24.472476ms | 19658200 | 0 | 44.62% | 1.23 | 623.51 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.062852ms | 6708948 | 0 | 45.00% | 1.20 | 894.27 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.674215ms | 6379291 | 0 | 45.42% | 1.21 | 817.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.81937ms | 8360463 | 0 | 45.48% | 1.20 | 856.30 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 31.195978ms | 20771260 | 2000000 | 45.04% | 1.23 | 489.13 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.844µs | 10288 | 0 | 45.78% | 1.23 | 855.12 MB/s |
| Timsort | 1000 | 25.842µs | 10450 | 0 | 45.78% | 1.23 | 590.46 MB/s |
| ARS Gen 1: Foundation | 1000 | 229.559µs | 0 | 2000 | 45.78% | 1.23 | 66.47 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 268.757µs | 0 | 2000 | 45.78% | 1.23 | 56.78 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.164µs | 10288 | 0 | 45.78% | 1.23 | 889.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.083µs | 10288 | 0 | 45.78% | 1.23 | 893.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.945µs | 10288 | 0 | 45.78% | 1.23 | 900.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.157µs | 10450 | 0 | 45.78% | 1.23 | 583.35 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.901µs | 10288 | 0 | 45.78% | 1.23 | 852.40 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.102µs | 10450 | 0 | 45.78% | 1.23 | 563.01 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.989µs | 10288 | 0 | 45.78% | 1.23 | 848.23 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.143µs | 10288 | 0 | 45.78% | 1.23 | 890.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.06µs | 10288 | 0 | 45.78% | 1.23 | 894.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.157µs | 10288 | 2000 | 45.78% | 1.23 | 132.50 MB/s |
| Quicksort | 10000 | 218.251µs | 136714 | 0 | 45.76% | 1.23 | 699.14 MB/s |
| Timsort | 10000 | 308.66µs | 140903 | 0 | 45.76% | 1.23 | 494.36 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.259092ms | 160 | 30000 | 45.62% | 1.24 | 29.01 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.354202ms | 160 | 30000 | 45.60% | 1.24 | 28.50 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 382.741µs | 193162 | 14351 | 45.75% | 1.23 | 398.67 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 229.513µs | 52333 | 10000 | 45.75% | 1.23 | 664.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 152.147µs | 52333 | 0 | 45.74% | 1.23 | 1002.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.748µs | 57763 | 0 | 45.74% | 1.23 | 949.24 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 202.264µs | 59057 | 0 | 45.74% | 1.23 | 754.40 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 198.987µs | 62100 | 0 | 45.74% | 1.23 | 766.82 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 164.159µs | 52333 | 0 | 45.74% | 1.23 | 929.51 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 165.26µs | 52333 | 0 | 45.74% | 1.23 | 923.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 157.852µs | 52333 | 0 | 45.74% | 1.23 | 966.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 347.627µs | 52333 | 20000 | 45.73% | 1.23 | 438.94 MB/s |
| Quicksort | 100000 | 2.616085ms | 1706033 | 0 | 45.68% | 1.23 | 583.27 MB/s |
| Timsort | 100000 | 3.397532ms | 1748408 | 0 | 45.63% | 1.23 | 449.11 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.097297ms | 15822 | 300000 | 39.33% | 1.21 | 37.13 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.285964ms | 15822 | 300000 | 40.09% | 1.22 | 33.69 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.158782ms | 1885784 | 108703 | 45.66% | 1.23 | 483.06 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.064045ms | 882348 | 100000 | 45.67% | 1.23 | 1434.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 783.903µs | 882348 | 0 | 45.65% | 1.23 | 1946.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 925.888µs | 921462 | 0 | 45.65% | 1.23 | 1648.02 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.104756ms | 939598 | 0 | 45.65% | 1.23 | 1381.19 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.259649ms | 975983 | 0 | 45.66% | 1.23 | 1211.35 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 809.902µs | 882348 | 0 | 45.65% | 1.23 | 1884.03 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 872.673µs | 771432 | 0 | 45.64% | 1.23 | 1748.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 808.544µs | 882348 | 0 | 45.64% | 1.23 | 1887.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.286258ms | 882348 | 200000 | 45.66% | 1.23 | 1186.29 MB/s |
| Quicksort | 1000000 | 29.688454ms | 20389196 | 0 | 45.34% | 1.25 | 513.96 MB/s |
| Timsort | 1000000 | 44.543396ms | 20780417 | 0 | 44.96% | 1.25 | 342.56 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.707753ms | 21441825 | 1017407 | 45.46% | 1.24 | 571.32 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.157198ms | 10157321 | 1000000 | 45.85% | 1.23 | 1502.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.611299ms | 10157321 | 0 | 45.84% | 1.23 | 1587.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.013147ms | 10561958 | 0 | 45.84% | 1.22 | 1385.51 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 8.958061ms | 12859603 | 0 | 45.70% | 1.24 | 1703.36 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.202353ms | 13271645 | 0 | 45.63% | 1.23 | 1362.11 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.54783ms | 10157321 | 0 | 45.75% | 1.23 | 1598.14 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.755146ms | 11214454 | 0 | 45.85% | 1.23 | 1418.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.516346ms | 12268914 | 0 | 45.86% | 1.23 | 1450.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.429767ms | 13561646 | 2000000 | 45.79% | 1.23 | 518.48 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 7.679µs | 5797 | 0 | 46.26% | 1.24 | 1987.08 MB/s |
| Timsort | 1000 | 10.987µs | 5499 | 0 | 46.26% | 1.24 | 1388.80 MB/s |
| ARS Gen 1: Foundation | 1000 | 46.167µs | 984 | 2000 | 46.26% | 1.24 | 330.51 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 57.541µs | 984 | 2000 | 46.26% | 1.24 | 265.18 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 7.259µs | 5797 | 0 | 46.26% | 1.24 | 2102.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 7.165µs | 5797 | 0 | 46.26% | 1.24 | 2129.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 7.193µs | 5797 | 0 | 46.26% | 1.24 | 2121.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 10.741µs | 5499 | 0 | 46.26% | 1.24 | 1420.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 7.615µs | 5797 | 0 | 46.26% | 1.24 | 2003.78 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 10.814µs | 5499 | 0 | 46.26% | 1.24 | 1411.02 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 7.634µs | 5797 | 0 | 46.26% | 1.24 | 1998.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 7.365µs | 5797 | 0 | 46.26% | 1.24 | 2071.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.309µs | 5797 | 0 | 46.26% | 1.24 | 2087.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.762µs | 5797 | 2000 | 46.25% | 1.24 | 139.02 MB/s |
| Quicksort | 10000 | 66.653µs | 53838 | 0 | 46.24% | 1.24 | 2289.29 MB/s |
| Timsort | 10000 | 87.948µs | 53843 | 0 | 46.24% | 1.24 | 1734.98 MB/s |
| ARS Gen 1: Foundation | 10000 | 257.739µs | 9984 | 30000 | 46.24% | 1.24 | 592.02 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 302.237µs | 9984 | 30000 | 46.24% | 1.24 | 504.86 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 306.3µs | 122148 | 14351 | 46.24% | 1.24 | 498.16 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 200.642µs | 12061 | 10000 | 46.23% | 1.24 | 760.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 121.192µs | 12061 | 0 | 46.23% | 1.24 | 1259.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 120.408µs | 12085 | 0 | 46.23% | 1.24 | 1267.26 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 237.65µs | 12061 | 0 | 46.23% | 1.24 | 642.07 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 228.943µs | 12085 | 0 | 46.23% | 1.24 | 666.49 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 130.424µs | 12061 | 0 | 46.23% | 1.24 | 1169.94 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 131.788µs | 12061 | 0 | 46.23% | 1.24 | 1157.83 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 120.582µs | 12061 | 0 | 46.23% | 1.24 | 1265.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 318.036µs | 12061 | 20000 | 46.22% | 1.24 | 479.78 MB/s |
| Quicksort | 100000 | 648.154µs | 529379 | 0 | 46.17% | 1.24 | 2354.19 MB/s |
| Timsort | 100000 | 854.326µs | 529674 | 0 | 46.13% | 1.24 | 1786.06 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.345279ms | 99984 | 300000 | 46.18% | 1.24 | 1134.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.60493ms | 99984 | 300000 | 46.18% | 1.24 | 950.74 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.292797ms | 1143461 | 108703 | 46.17% | 1.24 | 665.51 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 584.609µs | 151116 | 100000 | 46.14% | 1.24 | 2610.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 346.914µs | 151116 | 0 | 46.14% | 1.24 | 4398.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 495.933µs | 151622 | 0 | 46.14% | 1.24 | 3076.78 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 571.635µs | 99988 | 0 | 46.15% | 1.24 | 2669.32 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 566.12µs | 99988 | 0 | 46.15% | 1.24 | 2695.33 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 517.852µs | 199984 | 0 | 46.12% | 1.24 | 2946.55 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 487.338µs | 199972 | 0 | 46.13% | 1.24 | 3131.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 403.718µs | 100000 | 0 | 46.13% | 1.24 | 3779.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 774.681µs | 151116 | 200000 | 46.14% | 1.24 | 1969.69 MB/s |
| Quicksort | 1000000 | 5.67041ms | 5138620 | 0 | 45.91% | 1.24 | 2690.95 MB/s |
| Timsort | 1000000 | 9.984949ms | 6175006 | 0 | 45.69% | 1.24 | 1528.18 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 23.544804ms | 12087538 | 1017407 | 46.12% | 1.24 | 648.07 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.11717ms | 999988 | 1000000 | 46.43% | 1.22 | 2494.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.471202ms | 999988 | 0 | 46.44% | 1.23 | 2357.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.358118ms | 999988 | 0 | 46.44% | 1.22 | 2399.89 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.521029ms | 999988 | 0 | 46.28% | 1.23 | 3375.07 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.539612ms | 999988 | 0 | 46.28% | 1.23 | 3361.25 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.026939ms | 1999972 | 0 | 46.39% | 1.23 | 1900.95 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.529917ms | 1999972 | 0 | 46.36% | 1.23 | 2026.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.921968ms | 1999976 | 0 | 46.41% | 1.22 | 1926.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.678642ms | 5717339 | 2000000 | 46.00% | 1.24 | 816.91 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.04µs | 10288 | 0 | 46.05% | 1.24 | 895.47 MB/s |
| Timsort | 1000 | 25.234µs | 10450 | 0 | 46.05% | 1.24 | 604.69 MB/s |
| ARS Gen 1: Foundation | 1000 | 219.901µs | 0 | 2000 | 46.05% | 1.24 | 69.39 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 263.148µs | 0 | 2000 | 46.05% | 1.24 | 57.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.359µs | 10288 | 0 | 46.05% | 1.24 | 932.75 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 16.371µs | 10288 | 0 | 46.05% | 1.24 | 932.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.319µs | 10288 | 0 | 46.05% | 1.24 | 935.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 24.807µs | 10450 | 0 | 46.05% | 1.24 | 615.10 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 16.934µs | 10288 | 0 | 46.05% | 1.24 | 901.07 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 24.703µs | 10450 | 0 | 46.05% | 1.24 | 617.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.086µs | 10288 | 0 | 46.05% | 1.24 | 893.06 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 16.434µs | 10288 | 0 | 46.05% | 1.24 | 928.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.018µs | 10288 | 0 | 46.05% | 1.24 | 896.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 110.225µs | 10288 | 2000 | 46.05% | 1.24 | 138.43 MB/s |
| Quicksort | 10000 | 210.841µs | 136714 | 0 | 46.04% | 1.24 | 723.71 MB/s |
| Timsort | 10000 | 297.334µs | 140903 | 0 | 46.04% | 1.24 | 513.19 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.065748ms | 160 | 30000 | 45.91% | 1.24 | 30.12 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.4077ms | 160 | 30000 | 45.89% | 1.24 | 28.22 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 400.428µs | 193162 | 14351 | 46.03% | 1.24 | 381.06 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 247.059µs | 52333 | 10000 | 46.02% | 1.24 | 617.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 152.547µs | 52333 | 0 | 46.02% | 1.24 | 1000.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 168.584µs | 57763 | 0 | 46.02% | 1.24 | 905.11 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 200.677µs | 59057 | 0 | 46.02% | 1.24 | 760.37 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 210.947µs | 62100 | 0 | 46.02% | 1.24 | 723.35 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 177.758µs | 52333 | 0 | 46.02% | 1.24 | 858.40 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 161.442µs | 52333 | 0 | 46.02% | 1.24 | 945.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 158.016µs | 52333 | 0 | 46.02% | 1.24 | 965.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 333.929µs | 52333 | 20000 | 46.01% | 1.24 | 456.95 MB/s |
| Quicksort | 100000 | 2.698762ms | 1706033 | 0 | 45.96% | 1.24 | 565.40 MB/s |
| Timsort | 100000 | 3.48211ms | 1748408 | 0 | 45.91% | 1.24 | 438.21 MB/s |
| ARS Gen 1: Foundation | 100000 | 45.583776ms | 15822 | 300000 | 39.76% | 1.21 | 33.47 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 50.078264ms | 15822 | 300000 | 39.73% | 1.21 | 30.47 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.995844ms | 1885784 | 108703 | 45.95% | 1.24 | 509.33 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 961.075µs | 882348 | 100000 | 45.93% | 1.24 | 1587.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 877.013µs | 882348 | 0 | 45.93% | 1.24 | 1739.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 935.767µs | 921462 | 0 | 45.93% | 1.24 | 1630.62 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.065127ms | 939598 | 0 | 45.95% | 1.24 | 1432.58 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.216597ms | 975983 | 0 | 45.94% | 1.24 | 1254.22 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 791.759µs | 882348 | 0 | 45.93% | 1.24 | 1927.20 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 816.231µs | 771432 | 0 | 45.93% | 1.24 | 1869.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 782.378µs | 882348 | 0 | 45.92% | 1.24 | 1950.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.186171ms | 882348 | 200000 | 45.93% | 1.24 | 1286.39 MB/s |
| Quicksort | 1000000 | 30.592408ms | 20389196 | 0 | 45.62% | 1.26 | 498.78 MB/s |
| Timsort | 1000000 | 45.164337ms | 20780417 | 0 | 45.28% | 1.25 | 337.85 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.324367ms | 21441825 | 1017407 | 45.72% | 1.24 | 558.43 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.109432ms | 10157321 | 1000000 | 46.06% | 1.23 | 1509.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.466736ms | 10157321 | 0 | 46.06% | 1.23 | 1611.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.97031ms | 10561958 | 0 | 46.07% | 1.23 | 1390.92 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.147414ms | 12859603 | 0 | 45.95% | 1.24 | 1668.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.190946ms | 13271645 | 0 | 45.90% | 1.24 | 1363.49 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.591118ms | 10157321 | 0 | 45.99% | 1.23 | 1590.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.65992ms | 11214454 | 0 | 46.08% | 1.23 | 1431.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.816698ms | 12268914 | 0 | 46.09% | 1.23 | 1410.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.913357ms | 13464491 | 2000000 | 45.92% | 1.24 | 510.10 MB/s |

## Category: String

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 59.864µs | 10370 | 0 | 46.51% | 1.24 | 1019.56 MB/s |
| Timsort | 1000 | 74.694µs | 10522 | 0 | 46.51% | 1.24 | 817.14 MB/s |
| ARS Gen 1: Foundation | 1000 | 332.543µs | 0 | 2000 | 46.51% | 1.24 | 183.54 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 366.474µs | 0 | 2000 | 46.51% | 1.24 | 166.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 66.878µs | 10370 | 0 | 46.51% | 1.24 | 912.63 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 68.67µs | 10370 | 0 | 46.51% | 1.24 | 888.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 70.484µs | 10370 | 0 | 46.51% | 1.24 | 865.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 79.805µs | 10522 | 0 | 46.51% | 1.24 | 764.80 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 68.286µs | 10370 | 0 | 46.51% | 1.24 | 893.82 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 78.826µs | 10522 | 0 | 46.51% | 1.24 | 774.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 67.185µs | 10370 | 0 | 46.51% | 1.24 | 908.46 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 70.215µs | 10370 | 0 | 46.51% | 1.24 | 869.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 67.304µs | 10370 | 0 | 46.51% | 1.24 | 906.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 253.759µs | 10370 | 2000 | 46.51% | 1.24 | 240.52 MB/s |
| Quicksort | 10000 | 862.727µs | 136866 | 0 | 46.47% | 1.24 | 707.47 MB/s |
| Timsort | 10000 | 988.621µs | 141490 | 0 | 46.46% | 1.24 | 617.38 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.602164ms | 0 | 30000 | 46.35% | 1.25 | 92.45 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.86189ms | 0 | 30000 | 46.33% | 1.25 | 88.95 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.322794ms | 193846 | 14351 | 46.46% | 1.24 | 262.77 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 712.398µs | 67438 | 10000 | 46.47% | 1.24 | 856.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 243.679µs | 67438 | 0 | 46.47% | 1.24 | 2504.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 291.572µs | 70298 | 0 | 46.47% | 1.24 | 2093.31 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 427.835µs | 63043 | 0 | 46.46% | 1.24 | 1426.61 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 452.257µs | 67007 | 0 | 46.46% | 1.24 | 1349.57 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 275.245µs | 67438 | 0 | 46.47% | 1.24 | 2217.48 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 280.818µs | 67438 | 0 | 46.47% | 1.24 | 2173.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 274µs | 67438 | 0 | 46.47% | 1.24 | 2227.56 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.317524ms | 67438 | 20000 | 46.44% | 1.24 | 463.26 MB/s |
| Quicksort | 100000 | 9.753449ms | 1718762 | 0 | 45.96% | 1.25 | 625.78 MB/s |
| Timsort | 100000 | 12.735212ms | 1759891 | 0 | 45.78% | 1.25 | 479.26 MB/s |
| ARS Gen 1: Foundation | 100000 | 57.35581ms | 0 | 300000 | 41.27% | 1.23 | 106.41 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 59.772878ms | 0 | 300000 | 41.28% | 1.23 | 102.11 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 18.510781ms | 1895222 | 108703 | 46.37% | 1.25 | 329.73 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.232135ms | 1029722 | 100000 | 46.35% | 1.24 | 1442.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.234744ms | 1029722 | 0 | 46.35% | 1.24 | 1886.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.196695ms | 1071423 | 0 | 46.32% | 1.24 | 1909.32 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.443394ms | 978520 | 0 | 46.32% | 1.24 | 1772.53 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.665865ms | 1019338 | 0 | 46.33% | 1.24 | 1664.96 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.334582ms | 1029722 | 0 | 46.32% | 1.24 | 1830.37 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.409422ms | 961965 | 0 | 46.38% | 1.24 | 1790.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.263882ms | 1029722 | 0 | 46.32% | 1.24 | 1870.02 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 11.666232ms | 1029722 | 200000 | 45.93% | 1.24 | 523.18 MB/s |
| Quicksort | 1000000 | 282.485763ms | 20518628 | 0 | 48.30% | 1.27 | 216.06 MB/s |
| Timsort | 1000000 | 351.628797ms | 20902099 | 0 | 47.10% | 1.24 | 173.58 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 198.935082ms | 21589743 | 1017407 | 46.50% | 1.24 | 306.81 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.794019ms | 12256776 | 1000000 | 47.50% | 1.18 | 1020.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 50.368851ms | 12256776 | 0 | 47.48% | 1.18 | 1211.76 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.421044ms | 12679336 | 0 | 47.14% | 1.18 | 1101.30 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 64.013472ms | 13331493 | 0 | 47.64% | 1.19 | 953.47 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 70.592591ms | 13750405 | 0 | 47.46% | 1.19 | 864.61 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.45587ms | 9434971 | 0 | 47.79% | 1.18 | 1186.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 69.687771ms | 7913738 | 0 | 48.88% | 1.17 | 875.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 56.839592ms | 8719170 | 0 | 48.36% | 1.17 | 1073.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 414.426909ms | 15199655 | 2000000 | 51.17% | 1.20 | 147.28 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 57.97µs | 10370 | 0 | 45.80% | 1.23 | 1052.87 MB/s |
| Timsort | 1000 | 64.392µs | 10522 | 0 | 45.80% | 1.23 | 947.87 MB/s |
| ARS Gen 1: Foundation | 1000 | 275.099µs | 0 | 2000 | 45.80% | 1.23 | 221.87 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 304.362µs | 0 | 2000 | 45.79% | 1.23 | 200.53 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 57.897µs | 10370 | 0 | 45.80% | 1.23 | 1054.20 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 57.52µs | 10370 | 0 | 45.80% | 1.23 | 1061.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 53.325µs | 10370 | 0 | 45.80% | 1.23 | 1144.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 63.175µs | 10522 | 0 | 45.80% | 1.23 | 966.13 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 55.557µs | 10370 | 0 | 45.80% | 1.23 | 1098.60 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 64.905µs | 10522 | 0 | 45.80% | 1.23 | 940.38 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 55.22µs | 10370 | 0 | 45.80% | 1.23 | 1105.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 62.415µs | 10370 | 0 | 45.80% | 1.23 | 977.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 55.3µs | 10370 | 0 | 45.80% | 1.23 | 1103.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 216.171µs | 10370 | 2000 | 45.79% | 1.23 | 282.35 MB/s |
| Quicksort | 10000 | 748.138µs | 136866 | 0 | 45.76% | 1.23 | 815.83 MB/s |
| Timsort | 10000 | 873.402µs | 141490 | 0 | 45.75% | 1.23 | 698.82 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.847482ms | 0 | 30000 | 45.56% | 1.23 | 104.38 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.083172ms | 0 | 30000 | 45.55% | 1.23 | 100.33 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.938613ms | 193846 | 14351 | 45.75% | 1.23 | 314.84 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 633.012µs | 67438 | 10000 | 45.76% | 1.23 | 964.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 220.947µs | 67438 | 0 | 45.76% | 1.23 | 2762.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 216.869µs | 70298 | 0 | 45.76% | 1.23 | 2814.38 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 359.151µs | 63043 | 0 | 45.75% | 1.23 | 1699.43 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 387.704µs | 67007 | 0 | 45.75% | 1.23 | 1574.27 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 240.4µs | 67438 | 0 | 45.76% | 1.23 | 2538.90 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 221.683µs | 67438 | 0 | 45.76% | 1.23 | 2753.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 2.457425ms | 67438 | 0 | 45.76% | 1.23 | 248.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 11.218142ms | 67438 | 20000 | 45.75% | 1.23 | 54.41 MB/s |
| Quicksort | 100000 | 91.4389ms | 1718762 | 0 | 45.50% | 1.24 | 66.75 MB/s |
| Timsort | 100000 | 16.572085ms | 1759891 | 0 | 45.10% | 1.24 | 368.30 MB/s |
| ARS Gen 1: Foundation | 100000 | 82.795448ms | 0 | 300000 | 40.67% | 1.22 | 73.72 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 82.967606ms | 0 | 300000 | 40.62% | 1.22 | 73.57 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 22.900105ms | 1895222 | 108703 | 45.70% | 1.24 | 266.53 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 7.085324ms | 1029722 | 100000 | 45.68% | 1.23 | 861.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.627927ms | 1029722 | 0 | 45.68% | 1.23 | 1682.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 4.329729ms | 1071423 | 0 | 45.66% | 1.23 | 1409.68 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.373259ms | 978520 | 0 | 45.67% | 1.23 | 1395.64 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.726823ms | 1019338 | 0 | 45.65% | 1.23 | 1291.25 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.028278ms | 1029722 | 0 | 45.66% | 1.23 | 1515.17 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.182738ms | 961965 | 0 | 45.73% | 1.23 | 1459.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.201372ms | 1029722 | 0 | 45.68% | 1.23 | 1452.74 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 12.654054ms | 1029722 | 200000 | 45.34% | 1.23 | 482.34 MB/s |
| Quicksort | 1000000 | 235.575208ms | 20518628 | 0 | 47.52% | 1.27 | 259.09 MB/s |
| Timsort | 1000000 | 345.548437ms | 20902099 | 0 | 46.58% | 1.24 | 176.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 188.385511ms | 21589743 | 1017407 | 45.86% | 1.23 | 323.99 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.423777ms | 12256776 | 1000000 | 46.80% | 1.17 | 1027.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 47.13196ms | 12256776 | 0 | 46.81% | 1.18 | 1294.98 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.721059ms | 12679336 | 0 | 46.62% | 1.18 | 1095.37 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 62.355452ms | 13331493 | 0 | 46.97% | 1.17 | 978.83 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 69.854257ms | 13750405 | 0 | 46.83% | 1.17 | 873.75 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.58723ms | 9434971 | 0 | 47.06% | 1.18 | 1183.14 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 67.111794ms | 7913738 | 0 | 48.08% | 1.16 | 909.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 55.603109ms | 8719170 | 0 | 47.68% | 1.17 | 1097.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 417.698682ms | 15151080 | 2000000 | 50.47% | 1.19 | 146.12 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 44.925µs | 9540 | 0 | 45.15% | 1.22 | 1358.60 MB/s |
| Timsort | 1000 | 50.606µs | 9492 | 0 | 45.15% | 1.22 | 1206.09 MB/s |
| ARS Gen 1: Foundation | 1000 | 123.684µs | 9394 | 2000 | 45.15% | 1.22 | 493.48 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 127.919µs | 9417 | 2000 | 45.15% | 1.22 | 477.14 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 51.997µs | 9540 | 0 | 45.15% | 1.22 | 1173.82 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 52.917µs | 9540 | 0 | 45.15% | 1.22 | 1153.41 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 49.446µs | 9540 | 0 | 45.15% | 1.22 | 1234.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 54.557µs | 9492 | 0 | 45.15% | 1.22 | 1118.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 59.371µs | 9540 | 0 | 45.15% | 1.22 | 1028.03 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 53.849µs | 9492 | 0 | 45.15% | 1.22 | 1133.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 54.727µs | 9540 | 0 | 45.15% | 1.22 | 1115.27 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 55.057µs | 9540 | 0 | 45.15% | 1.22 | 1108.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 56.556µs | 9540 | 0 | 45.15% | 1.22 | 1079.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 209.982µs | 9540 | 2000 | 45.15% | 1.22 | 290.67 MB/s |
| Quicksort | 10000 | 601.361µs | 132500 | 0 | 45.13% | 1.22 | 1014.95 MB/s |
| Timsort | 10000 | 674.441µs | 127861 | 0 | 45.12% | 1.22 | 904.97 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.799149ms | 94604 | 30000 | 45.10% | 1.22 | 339.24 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.890041ms | 94565 | 30000 | 45.10% | 1.22 | 322.93 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.014785ms | 182797 | 14351 | 45.10% | 1.22 | 302.94 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 774.581µs | 88075 | 10000 | 45.12% | 1.22 | 787.98 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 262.205µs | 88075 | 0 | 45.12% | 1.22 | 2327.76 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 251.941µs | 63479 | 0 | 45.12% | 1.22 | 2422.60 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 373.126µs | 73151 | 0 | 45.11% | 1.22 | 1635.78 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 365.364µs | 48448 | 0 | 45.11% | 1.22 | 1670.53 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 284.222µs | 88075 | 0 | 45.12% | 1.22 | 2147.45 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 253.828µs | 88075 | 0 | 45.12% | 1.22 | 2404.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 274.17µs | 88075 | 0 | 45.12% | 1.22 | 2226.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.235859ms | 88075 | 20000 | 45.10% | 1.22 | 493.87 MB/s |
| Quicksort | 100000 | 9.321218ms | 1695729 | 0 | 44.96% | 1.23 | 654.80 MB/s |
| Timsort | 100000 | 10.137887ms | 1618264 | 0 | 44.84% | 1.23 | 602.05 MB/s |
| ARS Gen 1: Foundation | 100000 | 21.96508ms | 958264 | 300000 | 45.20% | 1.23 | 277.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 21.875011ms | 958287 | 300000 | 45.19% | 1.23 | 279.02 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.248743ms | 1799629 | 108703 | 45.11% | 1.23 | 353.85 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.991274ms | 1250176 | 100000 | 45.03% | 1.22 | 1222.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.066997ms | 1250176 | 0 | 45.06% | 1.22 | 1990.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.441647ms | 957484 | 0 | 45.02% | 1.22 | 1773.43 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.930252ms | 1082137 | 0 | 45.03% | 1.22 | 2082.93 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.538254ms | 561919 | 0 | 45.01% | 1.22 | 2404.61 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.899962ms | 877121 | 0 | 45.03% | 1.22 | 2104.69 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.086778ms | 943356 | 0 | 45.01% | 1.22 | 1977.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.205243ms | 1084625 | 0 | 45.04% | 1.22 | 1904.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.907352ms | 1250176 | 200000 | 44.98% | 1.22 | 559.58 MB/s |
| Quicksort | 1000000 | 146.988443ms | 20467458 | 0 | 44.23% | 1.27 | 415.24 MB/s |
| Timsort | 1000000 | 176.65051ms | 19247236 | 0 | 43.97% | 1.26 | 345.51 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 224.140099ms | 20726079 | 1017407 | 46.37% | 1.24 | 272.31 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.764842ms | 14427992 | 1000000 | 46.15% | 1.20 | 1021.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 40.92593ms | 14427992 | 0 | 45.20% | 1.21 | 1491.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 40.253905ms | 9562892 | 0 | 44.75% | 1.18 | 1516.25 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 54.498358ms | 14500857 | 0 | 45.59% | 1.21 | 1119.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 51.226262ms | 9781181 | 0 | 45.18% | 1.18 | 1191.48 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 43.050053ms | 10121426 | 0 | 45.96% | 1.20 | 1417.77 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 55.523905ms | 10340217 | 0 | 46.32% | 1.19 | 1099.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 49.107888ms | 11441302 | 0 | 46.08% | 1.19 | 1242.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 253.099003ms | 19075213 | 2000000 | 47.34% | 1.23 | 241.15 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 40.997µs | 5636 | 0 | 51.12% | 1.23 | 1488.77 MB/s |
| Timsort | 1000 | 44.849µs | 5782 | 0 | 51.11% | 1.23 | 1360.90 MB/s |
| ARS Gen 1: Foundation | 1000 | 125.562µs | 984 | 2000 | 51.11% | 1.23 | 486.10 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 137.137µs | 984 | 2000 | 51.11% | 1.23 | 445.07 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 38.976µs | 5636 | 0 | 51.11% | 1.23 | 1565.97 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 39.366µs | 5636 | 0 | 51.11% | 1.23 | 1550.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 36.193µs | 5636 | 0 | 51.11% | 1.23 | 1686.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 39.893µs | 5782 | 0 | 51.11% | 1.23 | 1529.97 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 33.985µs | 5636 | 0 | 51.11% | 1.23 | 1795.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 37.684µs | 5782 | 0 | 51.11% | 1.23 | 1619.66 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 39.204µs | 5636 | 0 | 51.11% | 1.23 | 1556.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 37.166µs | 5636 | 0 | 51.11% | 1.23 | 1642.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 33.716µs | 5636 | 0 | 51.11% | 1.23 | 1810.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 215.4µs | 5636 | 2000 | 51.11% | 1.23 | 283.36 MB/s |
| Quicksort | 10000 | 334.264µs | 53113 | 0 | 51.09% | 1.23 | 1825.96 MB/s |
| Timsort | 10000 | 447.164µs | 54714 | 0 | 51.08% | 1.23 | 1364.94 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.381991ms | 9984 | 30000 | 51.06% | 1.23 | 441.65 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.469026ms | 9984 | 30000 | 51.06% | 1.23 | 415.48 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.903186ms | 122389 | 14351 | 51.08% | 1.23 | 320.70 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 685.74µs | 14075 | 10000 | 51.08% | 1.23 | 890.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 140.422µs | 14075 | 0 | 51.09% | 1.23 | 4346.55 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 136.859µs | 14094 | 0 | 51.08% | 1.23 | 4459.71 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 302.806µs | 12021 | 0 | 51.08% | 1.23 | 2015.65 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 286.739µs | 12028 | 0 | 51.08% | 1.23 | 2128.60 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 150.351µs | 14075 | 0 | 51.08% | 1.23 | 4059.51 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 136.614µs | 14075 | 0 | 51.08% | 1.23 | 4467.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 158.828µs | 14075 | 0 | 51.08% | 1.23 | 3842.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 988.968µs | 14075 | 20000 | 51.06% | 1.23 | 617.16 MB/s |
| Quicksort | 100000 | 3.857325ms | 516589 | 0 | 50.86% | 1.23 | 1582.32 MB/s |
| Timsort | 100000 | 4.603117ms | 529550 | 0 | 50.76% | 1.23 | 1325.95 MB/s |
| ARS Gen 1: Foundation | 100000 | 14.834734ms | 99984 | 300000 | 50.86% | 1.23 | 411.43 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 15.207175ms | 99984 | 300000 | 50.87% | 1.23 | 401.36 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.58368ms | 1144965 | 108703 | 51.04% | 1.23 | 368.04 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.986607ms | 151083 | 100000 | 51.07% | 1.22 | 2043.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.798188ms | 151083 | 0 | 51.06% | 1.22 | 3394.26 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.145046ms | 151309 | 0 | 51.07% | 1.22 | 2845.40 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.891201ms | 99990 | 0 | 51.04% | 1.22 | 3227.32 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.954705ms | 99990 | 0 | 51.04% | 1.22 | 3122.47 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.159609ms | 200008 | 0 | 50.97% | 1.22 | 2826.21 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.176537ms | 200008 | 0 | 50.98% | 1.22 | 2804.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.822934ms | 100024 | 0 | 51.04% | 1.22 | 3348.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.292882ms | 151083 | 200000 | 50.78% | 1.23 | 656.79 MB/s |
| Quicksort | 1000000 | 89.347584ms | 5202060 | 0 | 52.53% | 1.23 | 683.12 MB/s |
| Timsort | 1000000 | 148.398426ms | 6111262 | 0 | 52.72% | 1.22 | 411.29 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 159.25817ms | 12085476 | 1017407 | 51.33% | 1.24 | 383.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 29.343815ms | 999988 | 1000000 | 51.69% | 1.21 | 2080.00 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 28.127811ms | 999988 | 0 | 51.73% | 1.20 | 2169.92 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 25.36094ms | 999988 | 0 | 51.65% | 1.20 | 2406.66 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 27.822674ms | 999988 | 0 | 51.66% | 1.21 | 2193.72 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 32.168731ms | 999988 | 0 | 51.74% | 1.21 | 1897.34 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 34.540822ms | 1999972 | 0 | 51.87% | 1.19 | 1767.04 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 34.545227ms | 1999976 | 0 | 51.79% | 1.19 | 1766.82 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 35.109664ms | 1999976 | 0 | 51.87% | 1.19 | 1738.41 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 314.849791ms | 5709060 | 2000000 | 54.45% | 1.20 | 193.85 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 54.899µs | 10370 | 0 | 51.00% | 1.22 | 1111.77 MB/s |
| Timsort | 1000 | 62.282µs | 10522 | 0 | 51.00% | 1.22 | 979.98 MB/s |
| ARS Gen 1: Foundation | 1000 | 256.107µs | 0 | 2000 | 51.00% | 1.22 | 238.32 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 309.646µs | 0 | 2000 | 51.00% | 1.22 | 197.11 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 52.634µs | 10370 | 0 | 51.00% | 1.22 | 1159.61 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 61.917µs | 10370 | 0 | 51.00% | 1.22 | 985.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 57.095µs | 10370 | 0 | 51.00% | 1.22 | 1069.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 61.256µs | 10522 | 0 | 51.00% | 1.22 | 996.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 56.233µs | 10370 | 0 | 51.00% | 1.22 | 1085.40 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 64.023µs | 10522 | 0 | 51.00% | 1.22 | 953.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 60.234µs | 10370 | 0 | 51.00% | 1.22 | 1013.30 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 53.312µs | 10370 | 0 | 51.00% | 1.22 | 1144.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 56.054µs | 10370 | 0 | 51.00% | 1.22 | 1088.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 221.236µs | 10370 | 2000 | 51.00% | 1.22 | 275.88 MB/s |
| Quicksort | 10000 | 667.438µs | 136866 | 0 | 50.97% | 1.22 | 914.47 MB/s |
| Timsort | 10000 | 861.384µs | 141490 | 0 | 50.96% | 1.22 | 708.57 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.633876ms | 0 | 30000 | 50.82% | 1.22 | 108.34 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.026877ms | 0 | 30000 | 50.80% | 1.22 | 101.27 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.080597ms | 193846 | 14351 | 50.96% | 1.22 | 293.35 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 611.26µs | 67438 | 10000 | 50.97% | 1.22 | 998.51 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 220.733µs | 67438 | 0 | 50.97% | 1.22 | 2765.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 229.376µs | 70298 | 0 | 50.97% | 1.22 | 2660.92 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 375.92µs | 63043 | 0 | 50.97% | 1.22 | 1623.62 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 395.946µs | 67007 | 0 | 50.96% | 1.22 | 1541.50 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 233.215µs | 67438 | 0 | 50.97% | 1.22 | 2617.12 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 217.727µs | 67438 | 0 | 50.97% | 1.22 | 2803.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 221.094µs | 67438 | 0 | 50.97% | 1.22 | 2760.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.099196ms | 67438 | 20000 | 50.96% | 1.22 | 555.27 MB/s |
| Quicksort | 100000 | 11.005408ms | 1718762 | 0 | 50.54% | 1.22 | 554.59 MB/s |
| Timsort | 100000 | 12.853616ms | 1759891 | 0 | 50.43% | 1.22 | 474.85 MB/s |
| ARS Gen 1: Foundation | 100000 | 56.457774ms | 0 | 300000 | 47.16% | 1.21 | 108.11 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.146386ms | 0 | 300000 | 46.61% | 1.21 | 99.82 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.58948ms | 1895222 | 108703 | 50.87% | 1.22 | 367.91 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.150651ms | 1029722 | 100000 | 50.86% | 1.22 | 1470.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.759453ms | 1029722 | 0 | 50.88% | 1.22 | 2211.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.124261ms | 1071423 | 0 | 50.86% | 1.22 | 1953.59 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.30952ms | 978520 | 0 | 50.86% | 1.22 | 1844.23 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.474836ms | 1019338 | 0 | 50.84% | 1.22 | 1756.49 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.999964ms | 1029722 | 0 | 50.84% | 1.22 | 2034.53 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.988559ms | 961965 | 0 | 50.88% | 1.22 | 2042.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.832347ms | 1029722 | 0 | 50.84% | 1.22 | 2154.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.084704ms | 1029722 | 200000 | 50.58% | 1.22 | 605.23 MB/s |
| Quicksort | 1000000 | 240.594952ms | 20518628 | 0 | 51.65% | 1.24 | 253.68 MB/s |
| Timsort | 1000000 | 369.636397ms | 20902099 | 0 | 50.64% | 1.22 | 165.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 194.395049ms | 21589743 | 1017407 | 50.73% | 1.22 | 313.97 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.81745ms | 12256776 | 1000000 | 51.44% | 1.17 | 1020.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 47.762566ms | 12256776 | 0 | 51.49% | 1.18 | 1277.89 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 58.635398ms | 12679336 | 0 | 51.11% | 1.16 | 1040.93 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 61.395457ms | 13331493 | 0 | 51.63% | 1.18 | 994.13 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 73.080356ms | 13750405 | 0 | 51.39% | 1.16 | 835.18 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.641782ms | 9434971 | 0 | 51.66% | 1.18 | 1181.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 69.026886ms | 7913738 | 0 | 52.44% | 1.16 | 884.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 56.833669ms | 8719170 | 0 | 52.12% | 1.17 | 1073.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 429.96703ms | 15199655 | 2000000 | 53.83% | 1.19 | 141.95 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 53.125µs | 10370 | 0 | 50.13% | 1.21 | 1148.90 MB/s |
| Timsort | 1000 | 57.339µs | 10522 | 0 | 50.13% | 1.21 | 1064.46 MB/s |
| ARS Gen 1: Foundation | 1000 | 268.17µs | 0 | 2000 | 50.13% | 1.21 | 227.60 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 325.803µs | 0 | 2000 | 50.13% | 1.21 | 187.34 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 58.952µs | 10370 | 0 | 50.13% | 1.21 | 1035.34 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 58.271µs | 10370 | 0 | 50.13% | 1.21 | 1047.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 57.324µs | 10370 | 0 | 50.13% | 1.21 | 1064.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 70.827µs | 10522 | 0 | 50.13% | 1.21 | 861.75 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 57.022µs | 10370 | 0 | 50.13% | 1.21 | 1070.38 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 64.424µs | 10522 | 0 | 50.13% | 1.21 | 947.40 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 57.198µs | 10370 | 0 | 50.13% | 1.21 | 1067.09 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 63.975µs | 10370 | 0 | 50.13% | 1.21 | 954.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 55.289µs | 10370 | 0 | 50.13% | 1.21 | 1103.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 220.318µs | 10370 | 2000 | 50.13% | 1.21 | 277.03 MB/s |
| Quicksort | 10000 | 691.315µs | 136866 | 0 | 50.11% | 1.21 | 882.88 MB/s |
| Timsort | 10000 | 918.487µs | 141490 | 0 | 50.10% | 1.21 | 664.52 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.870588ms | 0 | 30000 | 49.93% | 1.21 | 103.97 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.890679ms | 0 | 30000 | 49.91% | 1.21 | 103.61 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.133924ms | 193846 | 14351 | 50.09% | 1.21 | 286.02 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 664.218µs | 67438 | 10000 | 50.10% | 1.21 | 918.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 220.025µs | 67438 | 0 | 50.10% | 1.21 | 2774.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 232.478µs | 70298 | 0 | 50.10% | 1.21 | 2625.42 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 444.492µs | 63043 | 0 | 50.10% | 1.21 | 1373.14 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 399.771µs | 67007 | 0 | 50.10% | 1.21 | 1526.75 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 254.932µs | 67438 | 0 | 50.10% | 1.21 | 2394.17 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 224.282µs | 67438 | 0 | 50.10% | 1.21 | 2721.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 219.26µs | 67438 | 0 | 50.10% | 1.21 | 2783.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.052192ms | 67438 | 20000 | 50.08% | 1.21 | 580.08 MB/s |
| Quicksort | 100000 | 10.739743ms | 1718762 | 0 | 49.70% | 1.21 | 568.31 MB/s |
| Timsort | 100000 | 13.008739ms | 1759891 | 0 | 49.54% | 1.21 | 469.19 MB/s |
| ARS Gen 1: Foundation | 100000 | 58.982048ms | 0 | 300000 | 46.09% | 1.20 | 103.48 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 60.141737ms | 0 | 300000 | 46.15% | 1.20 | 101.49 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.25332ms | 1895222 | 108703 | 50.01% | 1.21 | 375.52 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.971285ms | 1029722 | 100000 | 50.01% | 1.21 | 1536.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.880457ms | 1029722 | 0 | 50.00% | 1.20 | 2118.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.204271ms | 1071423 | 0 | 49.99% | 1.20 | 1904.81 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.330223ms | 978520 | 0 | 49.99% | 1.20 | 1832.76 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.650575ms | 1019338 | 0 | 49.99% | 1.20 | 1671.93 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.11278ms | 1029722 | 0 | 49.98% | 1.21 | 1960.79 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.20737ms | 961965 | 0 | 50.02% | 1.21 | 1902.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.917202ms | 1029722 | 0 | 49.98% | 1.20 | 2092.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.278394ms | 1029722 | 200000 | 49.74% | 1.21 | 593.82 MB/s |
| Quicksort | 1000000 | 266.532475ms | 20518628 | 0 | 51.21% | 1.23 | 229.00 MB/s |
| Timsort | 1000000 | 357.366089ms | 20902099 | 0 | 50.00% | 1.21 | 170.79 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 193.374239ms | 21589743 | 1017407 | 49.89% | 1.21 | 315.63 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 61.086949ms | 12256776 | 1000000 | 50.63% | 1.16 | 999.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 51.372806ms | 12256776 | 0 | 50.47% | 1.17 | 1188.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 59.155069ms | 12679336 | 0 | 50.34% | 1.16 | 1031.78 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 67.436276ms | 13331493 | 0 | 50.72% | 1.16 | 905.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 74.234883ms | 13750405 | 0 | 50.57% | 1.15 | 822.19 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 53.119786ms | 9434971 | 0 | 50.84% | 1.16 | 1149.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 69.778811ms | 7913738 | 0 | 51.58% | 1.15 | 874.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 58.481607ms | 8719170 | 0 | 51.30% | 1.16 | 1043.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 425.185013ms | 15199655 | 2000000 | 53.10% | 1.18 | 143.55 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 61.567µs | 10370 | 0 | 49.43% | 1.20 | 991.36 MB/s |
| Timsort | 1000 | 66.645µs | 10522 | 0 | 49.43% | 1.20 | 915.82 MB/s |
| ARS Gen 1: Foundation | 1000 | 290.985µs | 0 | 2000 | 49.43% | 1.20 | 209.75 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 290.367µs | 0 | 2000 | 49.43% | 1.20 | 210.20 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 54.328µs | 10370 | 0 | 49.43% | 1.20 | 1123.46 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 56.671µs | 10370 | 0 | 49.43% | 1.20 | 1077.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 58.321µs | 10370 | 0 | 49.43% | 1.20 | 1046.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 68.3µs | 10522 | 0 | 49.43% | 1.20 | 893.63 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 61.16µs | 10370 | 0 | 49.43% | 1.20 | 997.96 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 70.883µs | 10522 | 0 | 49.43% | 1.20 | 861.07 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 62.084µs | 10370 | 0 | 49.43% | 1.20 | 983.11 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 63.859µs | 10370 | 0 | 49.43% | 1.20 | 955.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 64.662µs | 10370 | 0 | 49.43% | 1.20 | 943.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 237.564µs | 10370 | 2000 | 49.43% | 1.20 | 256.92 MB/s |
| Quicksort | 10000 | 773.11µs | 136866 | 0 | 49.41% | 1.20 | 789.48 MB/s |
| Timsort | 10000 | 900.843µs | 141490 | 0 | 49.40% | 1.20 | 677.53 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.028464ms | 0 | 30000 | 49.29% | 1.20 | 101.24 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.956975ms | 0 | 30000 | 49.25% | 1.20 | 102.46 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.910617ms | 193846 | 14351 | 49.40% | 1.20 | 319.45 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 674.43µs | 67438 | 10000 | 49.41% | 1.20 | 904.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 246.603µs | 67438 | 0 | 49.41% | 1.20 | 2475.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 296.215µs | 70298 | 0 | 49.41% | 1.20 | 2060.50 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 379.483µs | 63043 | 0 | 49.40% | 1.20 | 1608.38 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 375.923µs | 67007 | 0 | 49.40% | 1.20 | 1623.61 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 237.781µs | 67438 | 0 | 49.41% | 1.20 | 2566.86 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 225.592µs | 67438 | 0 | 49.41% | 1.20 | 2705.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 238.542µs | 67438 | 0 | 49.41% | 1.20 | 2558.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.083335ms | 67438 | 20000 | 49.38% | 1.20 | 563.40 MB/s |
| Quicksort | 100000 | 10.478293ms | 1718762 | 0 | 49.02% | 1.20 | 582.49 MB/s |
| Timsort | 100000 | 12.605448ms | 1759891 | 0 | 48.89% | 1.20 | 484.20 MB/s |
| ARS Gen 1: Foundation | 100000 | 62.124589ms | 0 | 300000 | 45.89% | 1.19 | 98.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.701625ms | 0 | 300000 | 45.52% | 1.19 | 98.92 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.170675ms | 1895222 | 108703 | 49.33% | 1.20 | 377.44 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.617275ms | 1029722 | 100000 | 49.31% | 1.20 | 1321.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.797475ms | 1029722 | 0 | 49.31% | 1.19 | 2181.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.220551ms | 1071423 | 0 | 49.27% | 1.19 | 1895.18 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.234592ms | 978520 | 0 | 49.29% | 1.19 | 1886.95 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.430946ms | 1019338 | 0 | 49.28% | 1.19 | 1778.96 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.957757ms | 1029722 | 0 | 49.28% | 1.19 | 2063.56 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.057592ms | 961965 | 0 | 49.33% | 1.19 | 1996.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.027491ms | 1029722 | 0 | 49.29% | 1.19 | 2016.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.351766ms | 1029722 | 200000 | 49.09% | 1.20 | 589.61 MB/s |
| Quicksort | 1000000 | 283.107782ms | 20518628 | 0 | 50.21% | 1.22 | 215.59 MB/s |
| Timsort | 1000000 | 382.147484ms | 20902099 | 0 | 49.34% | 1.20 | 159.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 195.593124ms | 21589743 | 1017407 | 49.23% | 1.20 | 312.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.75763ms | 12256776 | 1000000 | 49.93% | 1.15 | 1021.38 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.681017ms | 12256776 | 0 | 49.99% | 1.16 | 1253.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.118678ms | 12679336 | 0 | 49.73% | 1.15 | 1107.34 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 63.787465ms | 13331493 | 0 | 50.09% | 1.16 | 956.85 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 71.598144ms | 13750405 | 0 | 49.94% | 1.14 | 852.47 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.725109ms | 9434971 | 0 | 50.16% | 1.16 | 1179.99 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 70.847775ms | 7913738 | 0 | 50.89% | 1.15 | 861.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 58.692419ms | 8719170 | 0 | 50.51% | 1.15 | 1039.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 424.978142ms | 15151086 | 2000000 | 52.35% | 1.17 | 143.62 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 49.761µs | 10370 | 0 | 48.96% | 1.19 | 1226.57 MB/s |
| Timsort | 1000 | 55.671µs | 10522 | 0 | 48.96% | 1.19 | 1096.35 MB/s |
| ARS Gen 1: Foundation | 1000 | 274.465µs | 0 | 2000 | 48.96% | 1.19 | 222.38 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 309.67µs | 0 | 2000 | 48.96% | 1.19 | 197.10 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 56.296µs | 10370 | 0 | 48.96% | 1.19 | 1084.18 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 55.567µs | 10370 | 0 | 48.96% | 1.19 | 1098.41 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 56.905µs | 10370 | 0 | 48.96% | 1.19 | 1072.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 70.865µs | 10522 | 0 | 48.96% | 1.19 | 861.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 53.037µs | 10370 | 0 | 48.96% | 1.19 | 1150.80 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 63.298µs | 10522 | 0 | 48.96% | 1.19 | 964.25 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 62.915µs | 10370 | 0 | 48.96% | 1.19 | 970.12 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 59.916µs | 10370 | 0 | 48.96% | 1.19 | 1018.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 60.075µs | 10370 | 0 | 48.96% | 1.19 | 1015.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 219.217µs | 10370 | 2000 | 48.96% | 1.19 | 278.42 MB/s |
| Quicksort | 10000 | 760.161µs | 136866 | 0 | 48.94% | 1.19 | 802.92 MB/s |
| Timsort | 10000 | 892.672µs | 141490 | 0 | 48.93% | 1.19 | 683.74 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.716741ms | 0 | 30000 | 48.83% | 1.19 | 106.77 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.093273ms | 0 | 30000 | 48.81% | 1.19 | 100.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.953035ms | 193846 | 14351 | 48.93% | 1.19 | 312.51 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 676.743µs | 67438 | 10000 | 48.94% | 1.19 | 901.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 221.026µs | 67438 | 0 | 48.94% | 1.19 | 2761.45 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 231.551µs | 70298 | 0 | 48.94% | 1.19 | 2635.93 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 354.494µs | 63043 | 0 | 48.93% | 1.19 | 1721.75 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 395.437µs | 67007 | 0 | 48.93% | 1.19 | 1543.49 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 229.202µs | 67438 | 0 | 48.94% | 1.19 | 2662.94 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 221.442µs | 67438 | 0 | 48.94% | 1.19 | 2756.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 235.377µs | 67438 | 0 | 48.94% | 1.19 | 2593.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.05985ms | 67438 | 20000 | 48.92% | 1.19 | 575.88 MB/s |
| Quicksort | 100000 | 9.866848ms | 1718762 | 0 | 48.58% | 1.19 | 618.59 MB/s |
| Timsort | 100000 | 11.339247ms | 1759891 | 0 | 48.47% | 1.19 | 538.26 MB/s |
| ARS Gen 1: Foundation | 100000 | 61.270811ms | 0 | 300000 | 45.51% | 1.18 | 99.62 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 63.009297ms | 0 | 300000 | 44.93% | 1.18 | 96.87 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.025302ms | 1895222 | 108703 | 48.86% | 1.19 | 358.50 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.074904ms | 1029722 | 100000 | 48.85% | 1.19 | 1497.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.853941ms | 1029722 | 0 | 48.82% | 1.19 | 2138.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.122723ms | 1071423 | 0 | 48.83% | 1.19 | 1954.55 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.160804ms | 978520 | 0 | 48.84% | 1.19 | 1931.00 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.415091ms | 1019338 | 0 | 48.84% | 1.19 | 1787.22 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.927985ms | 1029722 | 0 | 48.85% | 1.19 | 2084.54 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.078277ms | 961965 | 0 | 48.87% | 1.19 | 1982.77 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.970536ms | 1029722 | 0 | 48.84% | 1.19 | 2054.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.35911ms | 1029722 | 200000 | 48.61% | 1.19 | 589.19 MB/s |
| Quicksort | 1000000 | 250.51705ms | 20518628 | 0 | 49.66% | 1.21 | 243.64 MB/s |
| Timsort | 1000000 | 355.615796ms | 20902099 | 0 | 48.85% | 1.19 | 171.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 201.566783ms | 21589743 | 1017407 | 48.82% | 1.19 | 302.80 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.957864ms | 12256776 | 1000000 | 49.48% | 1.15 | 1001.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 47.972995ms | 12256776 | 0 | 49.47% | 1.15 | 1272.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 56.64397ms | 12679336 | 0 | 49.26% | 1.15 | 1077.52 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 62.559304ms | 13331493 | 0 | 49.54% | 1.15 | 975.64 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 75.195403ms | 13750405 | 0 | 49.53% | 1.14 | 811.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.887819ms | 9434971 | 0 | 49.64% | 1.15 | 1154.05 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 70.886019ms | 7913738 | 0 | 50.35% | 1.14 | 861.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 58.546104ms | 8719170 | 0 | 50.03% | 1.14 | 1042.51 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 432.644137ms | 15199654 | 2000000 | 51.78% | 1.15 | 141.07 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 33.986µs | 5636 | 0 | 48.24% | 1.18 | 1795.89 MB/s |
| Timsort | 1000 | 36.036µs | 5782 | 0 | 48.24% | 1.18 | 1693.73 MB/s |
| ARS Gen 1: Foundation | 1000 | 114.425µs | 984 | 2000 | 48.24% | 1.18 | 533.41 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 123.987µs | 984 | 2000 | 48.24% | 1.18 | 492.27 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 39.516µs | 5636 | 0 | 48.24% | 1.18 | 1544.57 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 37.474µs | 5636 | 0 | 48.24% | 1.18 | 1628.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 37.801µs | 5636 | 0 | 48.24% | 1.18 | 1614.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 41.01µs | 5782 | 0 | 48.24% | 1.18 | 1488.30 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 35.709µs | 5636 | 0 | 48.24% | 1.18 | 1709.24 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 40.916µs | 5782 | 0 | 48.24% | 1.18 | 1491.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 39.66µs | 5636 | 0 | 48.24% | 1.18 | 1538.96 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 35.433µs | 5636 | 0 | 48.24% | 1.18 | 1722.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 36.668µs | 5636 | 0 | 48.24% | 1.18 | 1664.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 198.451µs | 5636 | 2000 | 48.23% | 1.18 | 307.56 MB/s |
| Quicksort | 10000 | 337.702µs | 53113 | 0 | 48.21% | 1.18 | 1807.37 MB/s |
| Timsort | 10000 | 405.075µs | 54714 | 0 | 48.21% | 1.18 | 1506.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.405738ms | 9984 | 30000 | 48.19% | 1.18 | 434.19 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.42761ms | 9984 | 30000 | 48.19% | 1.18 | 427.53 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.917765ms | 122389 | 14351 | 48.21% | 1.18 | 318.26 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 573.043µs | 14075 | 10000 | 48.21% | 1.18 | 1065.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 166.224µs | 14075 | 0 | 48.21% | 1.18 | 3671.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 151.184µs | 14094 | 0 | 48.21% | 1.18 | 4037.14 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 332.953µs | 12021 | 0 | 48.21% | 1.18 | 1833.15 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 326.143µs | 12028 | 0 | 48.21% | 1.18 | 1871.42 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 137.095µs | 14075 | 0 | 48.21% | 1.18 | 4452.03 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 129.508µs | 14075 | 0 | 48.21% | 1.18 | 4712.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 173.801µs | 14075 | 0 | 48.21% | 1.18 | 3511.78 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.138439ms | 14075 | 20000 | 48.19% | 1.18 | 536.13 MB/s |
| Quicksort | 100000 | 3.792869ms | 516589 | 0 | 48.02% | 1.18 | 1609.21 MB/s |
| Timsort | 100000 | 4.49523ms | 529550 | 0 | 47.94% | 1.18 | 1357.78 MB/s |
| ARS Gen 1: Foundation | 100000 | 15.719047ms | 99984 | 300000 | 48.08% | 1.18 | 388.29 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 16.147252ms | 99984 | 300000 | 48.06% | 1.18 | 377.99 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.07134ms | 1144965 | 108703 | 48.18% | 1.18 | 379.78 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.710182ms | 151083 | 100000 | 48.20% | 1.18 | 2252.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.750622ms | 151083 | 0 | 48.19% | 1.17 | 3486.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.768168ms | 151309 | 0 | 48.19% | 1.17 | 3451.89 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.914826ms | 99990 | 0 | 48.17% | 1.17 | 3187.50 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.943025ms | 99990 | 0 | 48.18% | 1.17 | 3141.24 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.194341ms | 200008 | 0 | 48.12% | 1.17 | 2781.48 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.348771ms | 200008 | 0 | 48.13% | 1.17 | 2598.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.702732ms | 100024 | 0 | 48.19% | 1.17 | 3584.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.092638ms | 151083 | 200000 | 48.00% | 1.18 | 671.26 MB/s |
| Quicksort | 1000000 | 92.692026ms | 5202060 | 0 | 49.65% | 1.18 | 658.47 MB/s |
| Timsort | 1000000 | 151.894456ms | 6111262 | 0 | 49.89% | 1.17 | 401.83 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 164.791536ms | 12085476 | 1017407 | 48.52% | 1.19 | 370.38 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 29.802085ms | 999988 | 1000000 | 48.78% | 1.17 | 2048.02 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 25.038578ms | 999988 | 0 | 48.75% | 1.16 | 2437.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 21.90037ms | 999988 | 0 | 48.74% | 1.16 | 2786.95 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 25.275703ms | 999988 | 0 | 48.76% | 1.16 | 2414.78 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 24.392164ms | 999988 | 0 | 48.79% | 1.16 | 2502.24 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 33.572721ms | 1999972 | 0 | 48.91% | 1.15 | 1818.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 36.51793ms | 1999976 | 0 | 48.97% | 1.15 | 1671.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 35.806837ms | 1999976 | 0 | 49.03% | 1.15 | 1704.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 318.669272ms | 5709060 | 2000000 | 51.47% | 1.16 | 191.53 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 56.582µs | 10308 | 0 | 48.17% | 1.17 | 1078.70 MB/s |
| Timsort | 1000 | 70.831µs | 10658 | 0 | 48.17% | 1.17 | 861.70 MB/s |
| ARS Gen 1: Foundation | 1000 | 133.53µs | 10308 | 2000 | 48.17% | 1.17 | 457.09 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 140.203µs | 10308 | 2000 | 48.17% | 1.17 | 435.33 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 55.201µs | 10308 | 0 | 48.17% | 1.17 | 1105.69 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 58.002µs | 10308 | 0 | 48.17% | 1.17 | 1052.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 59.738µs | 10308 | 0 | 48.17% | 1.17 | 1021.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 73.294µs | 10658 | 0 | 48.17% | 1.17 | 832.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 61.666µs | 10308 | 0 | 48.17% | 1.17 | 989.77 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 71.399µs | 10658 | 0 | 48.17% | 1.17 | 854.85 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 62.907µs | 10308 | 0 | 48.17% | 1.17 | 970.24 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 61.677µs | 10308 | 0 | 48.17% | 1.17 | 989.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 62.15µs | 10308 | 0 | 48.17% | 1.17 | 982.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 240.592µs | 10308 | 2000 | 48.17% | 1.17 | 253.69 MB/s |
| Quicksort | 10000 | 823.814µs | 138349 | 0 | 48.13% | 1.17 | 740.89 MB/s |
| Timsort | 10000 | 1.039389ms | 142268 | 0 | 48.12% | 1.17 | 587.22 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.38792ms | 138349 | 30000 | 48.09% | 1.17 | 255.60 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.202084ms | 138349 | 30000 | 48.09% | 1.17 | 277.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.332498ms | 193925 | 14351 | 48.12% | 1.17 | 261.67 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.690013ms | 138355 | 10000 | 48.12% | 1.17 | 361.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 924.556µs | 138355 | 0 | 48.11% | 1.17 | 660.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.087929ms | 142274 | 0 | 48.11% | 1.17 | 561.02 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.022089ms | 138355 | 0 | 48.11% | 1.17 | 597.16 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.192172ms | 142274 | 0 | 48.10% | 1.17 | 511.97 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 910.936µs | 138355 | 0 | 48.12% | 1.17 | 670.03 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 932.943µs | 138355 | 0 | 48.11% | 1.17 | 654.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 955.041µs | 138355 | 0 | 48.12% | 1.17 | 639.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.867604ms | 138355 | 20000 | 48.09% | 1.17 | 326.81 MB/s |
| Quicksort | 100000 | 14.927819ms | 1715173 | 0 | 47.91% | 1.17 | 408.87 MB/s |
| Timsort | 100000 | 17.708573ms | 1762853 | 0 | 47.85% | 1.17 | 344.66 MB/s |
| ARS Gen 1: Foundation | 100000 | 48.234009ms | 1715173 | 300000 | 47.96% | 1.17 | 126.54 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.760923ms | 1715173 | 300000 | 47.89% | 1.17 | 130.53 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 20.547651ms | 1895407 | 108703 | 48.18% | 1.17 | 297.04 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 19.487833ms | 1715179 | 100000 | 47.88% | 1.17 | 313.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 19.978051ms | 1715179 | 0 | 47.97% | 1.17 | 305.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 23.157516ms | 1762859 | 0 | 47.95% | 1.17 | 263.57 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 20.381725ms | 1715179 | 0 | 47.97% | 1.17 | 299.46 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 24.347297ms | 1762859 | 0 | 47.92% | 1.17 | 250.69 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 17.416126ms | 1715179 | 0 | 47.93% | 1.17 | 350.45 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 18.716498ms | 1715179 | 0 | 47.93% | 1.17 | 326.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 18.495426ms | 1715179 | 0 | 47.96% | 1.17 | 330.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 34.306625ms | 1715179 | 200000 | 47.75% | 1.17 | 177.91 MB/s |
| Quicksort | 1000000 | 470.334936ms | 20523276 | 0 | 51.05% | 1.16 | 129.77 MB/s |
| Timsort | 1000000 | 705.206054ms | 20914644 | 0 | 50.66% | 1.12 | 86.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 244.796855ms | 21586854 | 1017407 | 48.31% | 1.15 | 249.33 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 508.264519ms | 20523280 | 1000000 | 51.34% | 1.14 | 120.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 519.849308ms | 20523280 | 0 | 51.53% | 1.14 | 117.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 584.776176ms | 20914648 | 0 | 50.72% | 1.13 | 104.37 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 542.909735ms | 20523280 | 0 | 51.83% | 1.13 | 112.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 707.030883ms | 20914648 | 0 | 51.14% | 1.11 | 86.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 466.400336ms | 20523280 | 0 | 51.27% | 1.15 | 130.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 455.505241ms | 20523280 | 0 | 51.24% | 1.15 | 133.99 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 482.597746ms | 20523280 | 0 | 51.28% | 1.15 | 126.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 669.143784ms | 21912254 | 2000000 | 55.09% | 1.10 | 91.21 MB/s |

## Category: Custom

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 20.35µs | 10378 | 0 | 63.27% | 1.06 | 2249.45 MB/s |
| Timsort | 1000 | 27.665µs | 10965 | 0 | 63.27% | 1.06 | 1654.67 MB/s |
| ARS Gen 1: Foundation | 1000 | 210.822µs | 0 | 2000 | 63.27% | 1.06 | 217.13 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 239.873µs | 0 | 2000 | 63.27% | 1.06 | 190.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 19.465µs | 10378 | 0 | 63.27% | 1.06 | 2351.73 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 19.211µs | 10378 | 0 | 63.27% | 1.06 | 2382.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.8µs | 10378 | 0 | 63.27% | 1.06 | 2434.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 25.957µs | 10965 | 0 | 63.27% | 1.06 | 1763.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.148µs | 10378 | 0 | 63.27% | 1.06 | 2066.84 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 28.277µs | 10965 | 0 | 63.27% | 1.06 | 1618.86 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.678µs | 10378 | 0 | 63.27% | 1.06 | 2326.27 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 21.647µs | 10378 | 0 | 63.27% | 1.06 | 2114.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 18.76µs | 10378 | 0 | 63.27% | 1.06 | 2440.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 113.913µs | 10378 | 2000 | 63.27% | 1.06 | 401.85 MB/s |
| Quicksort | 10000 | 228.508µs | 138485 | 0 | 63.26% | 1.06 | 2003.27 MB/s |
| Timsort | 10000 | 322.118µs | 142802 | 0 | 63.26% | 1.06 | 1421.11 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.478404ms | 0 | 30000 | 63.21% | 1.06 | 102.22 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.725095ms | 0 | 30000 | 63.21% | 1.06 | 96.88 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 365.017µs | 194235 | 14351 | 63.26% | 1.06 | 1254.09 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 222.676µs | 53078 | 10000 | 63.25% | 1.06 | 2055.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 137.171µs | 53078 | 0 | 63.25% | 1.06 | 3337.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 147.86µs | 57974 | 0 | 63.25% | 1.06 | 3095.93 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 275.903µs | 60130 | 0 | 63.25% | 1.06 | 1659.15 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 202.209µs | 62739 | 0 | 63.25% | 1.06 | 2263.81 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 152.242µs | 53078 | 0 | 63.25% | 1.06 | 3006.82 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 138.634µs | 53078 | 0 | 63.25% | 1.06 | 3301.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 156.431µs | 53078 | 0 | 63.26% | 1.06 | 2926.30 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 289.881µs | 53078 | 20000 | 63.25% | 1.06 | 1579.14 MB/s |
| Quicksort | 100000 | 2.962981ms | 1716233 | 0 | 63.20% | 1.06 | 1544.94 MB/s |
| Timsort | 100000 | 4.409248ms | 1759914 | 0 | 63.14% | 1.06 | 1038.19 MB/s |
| ARS Gen 1: Foundation | 100000 | 38.275889ms | 0 | 300000 | 60.81% | 1.06 | 119.60 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 41.42399ms | 0 | 300000 | 60.70% | 1.06 | 110.51 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 30.0146ms | 1895170 | 108703 | 63.21% | 1.06 | 152.51 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.560827ms | 891495 | 100000 | 63.23% | 1.06 | 1787.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.151681ms | 891495 | 0 | 63.24% | 1.06 | 1452.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.375668ms | 927102 | 0 | 63.23% | 1.06 | 1926.88 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.141793ms | 954799 | 0 | 63.22% | 1.06 | 1105.23 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.375744ms | 993233 | 0 | 63.21% | 1.06 | 1356.04 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.279646ms | 891495 | 0 | 63.23% | 1.06 | 1395.77 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.29116ms | 780845 | 0 | 63.23% | 1.06 | 1390.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.542112ms | 891495 | 0 | 63.22% | 1.06 | 1800.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.908497ms | 891495 | 200000 | 63.24% | 1.06 | 1171.20 MB/s |
| Quicksort | 1000000 | 50.90772ms | 20512439 | 0 | 62.85% | 1.08 | 899.20 MB/s |
| Timsort | 1000000 | 80.108647ms | 20899150 | 0 | 62.32% | 1.07 | 571.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 41.763836ms | 21596717 | 1017407 | 62.94% | 1.07 | 1096.08 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 22.702515ms | 10310056 | 1000000 | 63.26% | 1.06 | 2016.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.564355ms | 10310056 | 0 | 63.27% | 1.06 | 2465.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.367388ms | 10709205 | 0 | 63.23% | 1.05 | 2363.58 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 19.551985ms | 13007245 | 0 | 63.25% | 1.05 | 2341.26 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 22.67795ms | 13425517 | 0 | 63.12% | 1.05 | 2018.54 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.373893ms | 10310056 | 0 | 63.26% | 1.06 | 2362.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.312005ms | 11367051 | 0 | 63.32% | 1.05 | 2253.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 21.262346ms | 12398342 | 0 | 63.31% | 1.06 | 2152.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.464311ms | 12262438 | 2000000 | 63.39% | 1.05 | 1053.19 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.264µs | 10308 | 0 | 63.32% | 1.04 | 1811.92 MB/s |
| Timsort | 1000 | 34.968µs | 10818 | 0 | 63.32% | 1.04 | 1309.09 MB/s |
| ARS Gen 1: Foundation | 1000 | 216.819µs | 458 | 2000 | 63.32% | 1.04 | 211.13 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 243.217µs | 458 | 2000 | 63.32% | 1.04 | 188.21 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 25.501µs | 10308 | 0 | 63.32% | 1.04 | 1795.08 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 25.494µs | 10308 | 0 | 63.32% | 1.04 | 1795.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 25.449µs | 10308 | 0 | 63.32% | 1.04 | 1798.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 35.615µs | 10818 | 0 | 63.32% | 1.04 | 1285.31 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 25.297µs | 10308 | 0 | 63.32% | 1.04 | 1809.56 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 35.988µs | 10818 | 0 | 63.32% | 1.04 | 1271.99 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 25.455µs | 10308 | 0 | 63.32% | 1.04 | 1798.33 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 25.267µs | 10308 | 0 | 63.32% | 1.04 | 1811.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 25.315µs | 10308 | 0 | 63.32% | 1.04 | 1808.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 134.982µs | 10308 | 2000 | 63.32% | 1.04 | 339.13 MB/s |
| Quicksort | 10000 | 293.658µs | 135501 | 0 | 63.31% | 1.04 | 1558.83 MB/s |
| Timsort | 10000 | 396.036µs | 140463 | 0 | 63.31% | 1.04 | 1155.86 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.481006ms | 53061 | 30000 | 63.30% | 1.04 | 309.09 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.533736ms | 53088 | 30000 | 63.30% | 1.04 | 298.46 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 472.994µs | 191553 | 14351 | 63.31% | 1.04 | 967.80 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 270.81µs | 59910 | 10000 | 63.31% | 1.04 | 1690.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 157.276µs | 59910 | 0 | 63.31% | 1.04 | 2910.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 177.951µs | 62899 | 0 | 63.31% | 1.04 | 2572.41 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 331.067µs | 59126 | 0 | 63.31% | 1.04 | 1382.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 356.532µs | 61853 | 0 | 63.31% | 1.04 | 1283.93 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 179.55µs | 59910 | 0 | 63.31% | 1.04 | 2549.51 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 164.915µs | 59910 | 0 | 63.31% | 1.04 | 2775.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 169.372µs | 59910 | 0 | 63.31% | 1.04 | 2702.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 404.582µs | 59910 | 20000 | 63.30% | 1.04 | 1131.45 MB/s |
| Quicksort | 100000 | 2.816066ms | 1420515 | 0 | 63.27% | 1.04 | 1625.54 MB/s |
| Timsort | 100000 | 3.611136ms | 1424196 | 0 | 63.20% | 1.04 | 1267.64 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.297307ms | 1360088 | 300000 | 63.16% | 1.04 | 444.55 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 11.466555ms | 1360044 | 300000 | 63.16% | 1.04 | 399.22 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.196937ms | 1616363 | 108703 | 63.26% | 1.04 | 1431.88 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.510504ms | 713263 | 100000 | 63.29% | 1.04 | 3030.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.356803ms | 713263 | 0 | 63.29% | 1.04 | 3373.84 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.485815ms | 718641 | 0 | 63.28% | 1.04 | 3080.89 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.507431ms | 681503 | 0 | 63.27% | 1.04 | 3036.71 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.727286ms | 688539 | 0 | 63.27% | 1.04 | 2650.19 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.50217ms | 713263 | 0 | 63.28% | 1.04 | 3047.35 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.451209ms | 609629 | 0 | 63.28% | 1.04 | 3154.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.386223ms | 713263 | 0 | 63.28% | 1.04 | 3302.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.115312ms | 713263 | 200000 | 63.28% | 1.04 | 2164.05 MB/s |
| Quicksort | 1000000 | 26.572588ms | 13518116 | 0 | 62.87% | 1.05 | 1722.69 MB/s |
| Timsort | 1000000 | 60.776489ms | 14666956 | 0 | 62.39% | 1.05 | 753.19 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.591454ms | 14952891 | 1017407 | 62.99% | 1.05 | 1449.01 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 15.523162ms | 4752528 | 1000000 | 63.37% | 1.03 | 2948.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.376679ms | 4752528 | 0 | 63.37% | 1.03 | 2977.00 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.660873ms | 4776632 | 0 | 63.29% | 1.03 | 2591.97 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.950207ms | 6246227 | 0 | 63.29% | 1.04 | 2869.95 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.770753ms | 6276231 | 0 | 63.13% | 1.04 | 2438.71 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.798183ms | 4706394 | 0 | 63.36% | 1.04 | 2571.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.347734ms | 2307619 | 0 | 63.28% | 1.03 | 1805.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 23.796487ms | 2550221 | 0 | 63.31% | 1.03 | 1923.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 46.368171ms | 11398111 | 2000000 | 63.41% | 1.03 | 987.24 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 24.619µs | 9427 | 0 | 63.59% | 1.04 | 1859.39 MB/s |
| Timsort | 1000 | 29.49µs | 9314 | 0 | 63.59% | 1.04 | 1552.27 MB/s |
| ARS Gen 1: Foundation | 1000 | 137.484µs | 9547 | 2000 | 63.59% | 1.04 | 332.96 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 145.295µs | 9540 | 2000 | 63.59% | 1.04 | 315.06 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 24.829µs | 9427 | 0 | 63.59% | 1.04 | 1843.67 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 24.973µs | 9427 | 0 | 63.59% | 1.04 | 1833.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 24.888µs | 9427 | 0 | 63.59% | 1.04 | 1839.29 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.729µs | 9314 | 0 | 63.59% | 1.04 | 1539.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 24.491µs | 9427 | 0 | 63.59% | 1.04 | 1869.11 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.908µs | 9314 | 0 | 63.59% | 1.04 | 1530.57 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 24.803µs | 9427 | 0 | 63.59% | 1.04 | 1845.60 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 24.578µs | 9427 | 0 | 63.59% | 1.04 | 1862.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 24.293µs | 9427 | 0 | 63.59% | 1.04 | 1884.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 158.478µs | 9427 | 2000 | 63.58% | 1.04 | 288.85 MB/s |
| Quicksort | 10000 | 323.884µs | 133978 | 0 | 63.58% | 1.04 | 1413.36 MB/s |
| Timsort | 10000 | 335.128µs | 128297 | 0 | 63.58% | 1.04 | 1365.94 MB/s |
| ARS Gen 1: Foundation | 10000 | 844.135µs | 126223 | 30000 | 63.57% | 1.04 | 542.29 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 950.832µs | 126108 | 30000 | 63.57% | 1.04 | 481.43 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 334.068µs | 183316 | 14351 | 63.57% | 1.04 | 1370.27 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 210.828µs | 42006 | 10000 | 63.57% | 1.04 | 2171.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 130.353µs | 42006 | 0 | 63.57% | 1.04 | 3511.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 131.014µs | 34856 | 0 | 63.57% | 1.04 | 3494.01 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 288.861µs | 48982 | 0 | 63.57% | 1.04 | 1584.72 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 285.117µs | 42275 | 0 | 63.57% | 1.04 | 1605.53 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 146.486µs | 42006 | 0 | 63.57% | 1.04 | 3124.97 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 130.9µs | 42006 | 0 | 63.57% | 1.04 | 3497.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 135.532µs | 42006 | 0 | 63.57% | 1.04 | 3377.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 292.273µs | 42006 | 20000 | 63.57% | 1.04 | 1566.22 MB/s |
| Quicksort | 100000 | 2.929499ms | 1688686 | 0 | 63.54% | 1.04 | 1562.60 MB/s |
| Timsort | 100000 | 4.189188ms | 1619959 | 0 | 63.48% | 1.04 | 1092.73 MB/s |
| ARS Gen 1: Foundation | 100000 | 11.596824ms | 1609619 | 300000 | 63.49% | 1.04 | 394.73 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 11.114155ms | 1609452 | 300000 | 63.49% | 1.04 | 411.87 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.648822ms | 1798628 | 108703 | 63.55% | 1.04 | 1728.18 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.502398ms | 801237 | 100000 | 63.55% | 1.04 | 3046.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.412672ms | 801237 | 0 | 63.55% | 1.04 | 3240.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.195788ms | 405369 | 0 | 63.55% | 1.04 | 3828.13 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.607483ms | 871959 | 0 | 63.54% | 1.04 | 2847.70 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.516509ms | 443409 | 0 | 63.54% | 1.04 | 3018.54 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.50851ms | 801237 | 0 | 63.54% | 1.04 | 3034.54 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.500288ms | 689539 | 0 | 63.54% | 1.04 | 3051.17 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.486286ms | 801237 | 0 | 63.54% | 1.04 | 3079.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.059235ms | 801237 | 200000 | 63.54% | 1.04 | 2222.98 MB/s |
| Quicksort | 1000000 | 43.419765ms | 20499945 | 0 | 63.21% | 1.05 | 1054.27 MB/s |
| Timsort | 1000000 | 67.275477ms | 19254168 | 0 | 62.66% | 1.05 | 680.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.465156ms | 20728167 | 1017407 | 63.50% | 1.04 | 1410.02 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 19.344539ms | 9491317 | 1000000 | 63.62% | 1.03 | 2366.37 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.709018ms | 9491317 | 0 | 63.62% | 1.03 | 2584.92 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 16.018121ms | 4131087 | 0 | 63.62% | 1.03 | 2857.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.685443ms | 12332035 | 0 | 63.58% | 1.03 | 2449.84 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 16.269362ms | 5755621 | 0 | 63.60% | 1.03 | 2813.65 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.234077ms | 9491317 | 0 | 63.60% | 1.03 | 2379.96 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 21.416796ms | 10583380 | 0 | 63.63% | 1.03 | 2137.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.709588ms | 11681981 | 0 | 63.61% | 1.04 | 2210.39 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 49.592098ms | 14810770 | 2000000 | 63.60% | 1.03 | 923.06 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 8.267µs | 3761 | 0 | 63.76% | 1.03 | 5537.24 MB/s |
| Timsort | 1000 | 11.387µs | 3799 | 0 | 63.76% | 1.03 | 4020.06 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.546µs | 995 | 2000 | 63.76% | 1.03 | 962.78 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 61.553µs | 995 | 2000 | 63.76% | 1.03 | 743.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 8.194µs | 3761 | 0 | 63.76% | 1.03 | 5586.57 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 8.409µs | 3761 | 0 | 63.76% | 1.03 | 5443.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 8.372µs | 3761 | 0 | 63.76% | 1.03 | 5467.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.353µs | 3799 | 0 | 63.76% | 1.03 | 4032.09 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 8.285µs | 3761 | 0 | 63.76% | 1.03 | 5525.21 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 11.389µs | 3799 | 0 | 63.76% | 1.03 | 4019.35 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 8.165µs | 3761 | 0 | 63.76% | 1.03 | 5606.41 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 8.213µs | 3761 | 0 | 63.76% | 1.03 | 5573.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 8.224µs | 3761 | 0 | 63.76% | 1.03 | 5566.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 153.123µs | 3761 | 2000 | 63.76% | 1.03 | 298.95 MB/s |
| Quicksort | 10000 | 80.438µs | 36513 | 0 | 63.75% | 1.03 | 5690.89 MB/s |
| Timsort | 10000 | 119.934µs | 36606 | 0 | 63.75% | 1.03 | 3816.80 MB/s |
| ARS Gen 1: Foundation | 10000 | 328.933µs | 9995 | 30000 | 63.75% | 1.03 | 1391.66 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 404.198µs | 9995 | 30000 | 63.75% | 1.03 | 1132.52 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 428.511µs | 115165 | 14351 | 63.75% | 1.03 | 1068.27 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 286.399µs | 10001 | 10000 | 63.75% | 1.03 | 1598.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 175.738µs | 10001 | 0 | 63.75% | 1.03 | 2604.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 167.793µs | 10001 | 0 | 63.75% | 1.03 | 2728.15 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 380.848µs | 10001 | 0 | 63.74% | 1.03 | 1201.96 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 405.864µs | 10001 | 0 | 63.74% | 1.03 | 1127.87 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 189.926µs | 10001 | 0 | 63.75% | 1.03 | 2410.22 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 168.1µs | 10001 | 0 | 63.75% | 1.03 | 2723.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 162.803µs | 10001 | 0 | 63.75% | 1.03 | 2811.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 414.202µs | 10001 | 20000 | 63.74% | 1.03 | 1105.17 MB/s |
| Quicksort | 100000 | 770.218µs | 362118 | 0 | 63.71% | 1.03 | 5943.30 MB/s |
| Timsort | 100000 | 1.41499ms | 362412 | 0 | 63.67% | 1.03 | 3235.10 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.682722ms | 99995 | 300000 | 63.72% | 1.03 | 1706.34 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.965425ms | 99995 | 300000 | 63.72% | 1.03 | 1543.67 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.511852ms | 1131774 | 108703 | 63.72% | 1.03 | 1822.41 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.116493ms | 99999 | 100000 | 63.72% | 1.03 | 4100.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 949.638µs | 99999 | 0 | 63.72% | 1.03 | 4820.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 977.691µs | 99999 | 0 | 63.72% | 1.03 | 4682.09 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.024587ms | 99999 | 0 | 63.71% | 1.03 | 4467.79 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 989.806µs | 99999 | 0 | 63.71% | 1.03 | 4624.78 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.389465ms | 199994 | 0 | 63.71% | 1.03 | 3294.53 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.269256ms | 199994 | 0 | 63.71% | 1.03 | 3606.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.32429ms | 199994 | 0 | 63.72% | 1.03 | 3456.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.601016ms | 99999 | 200000 | 63.71% | 1.03 | 2859.21 MB/s |
| Quicksort | 1000000 | 12.279627ms | 3806932 | 0 | 63.65% | 1.04 | 3727.83 MB/s |
| Timsort | 1000000 | 34.768963ms | 4710561 | 0 | 63.53% | 1.03 | 1316.59 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 28.34987ms | 12059635 | 1017407 | 63.75% | 1.04 | 1614.69 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 14.897135ms | 1000001 | 1000000 | 63.81% | 1.03 | 3072.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.236916ms | 1000001 | 0 | 63.81% | 1.03 | 2819.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.275823ms | 1000001 | 0 | 63.81% | 1.03 | 2996.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 14.06073ms | 1000001 | 0 | 63.81% | 1.03 | 3255.62 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.473526ms | 1000001 | 0 | 63.81% | 1.03 | 3669.88 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 20.646496ms | 1999996 | 0 | 63.82% | 1.03 | 2217.15 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.327671ms | 1999996 | 0 | 63.83% | 1.03 | 2497.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.537404ms | 1999996 | 0 | 63.82% | 1.03 | 2343.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.585654ms | 5365482 | 2000000 | 63.79% | 1.03 | 1026.71 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 12.468µs | 5226 | 0 | 63.80% | 1.02 | 3671.51 MB/s |
| Timsort | 1000 | 16.408µs | 5250 | 0 | 63.80% | 1.02 | 2789.88 MB/s |
| ARS Gen 1: Foundation | 1000 | 48.526µs | 4636 | 2000 | 63.80% | 1.02 | 943.34 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.275µs | 4636 | 2000 | 63.80% | 1.02 | 785.52 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.358µs | 5226 | 0 | 63.80% | 1.02 | 4030.32 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.455µs | 5226 | 0 | 63.80% | 1.02 | 4378.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.533µs | 5226 | 0 | 63.80% | 1.02 | 4346.00 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 13.962µs | 5250 | 0 | 63.80% | 1.02 | 3278.64 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 9.851µs | 5226 | 0 | 63.80% | 1.02 | 4646.88 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 13.606µs | 5250 | 0 | 63.80% | 1.02 | 3364.43 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 9.6µs | 5226 | 0 | 63.80% | 1.02 | 4768.37 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 9.532µs | 5226 | 0 | 63.80% | 1.02 | 4802.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 9.202µs | 5226 | 0 | 63.80% | 1.02 | 4974.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 110.212µs | 5226 | 2000 | 63.80% | 1.02 | 415.35 MB/s |
| Quicksort | 10000 | 82.514µs | 53591 | 0 | 63.80% | 1.02 | 5547.71 MB/s |
| Timsort | 10000 | 108.503µs | 53226 | 0 | 63.80% | 1.02 | 4218.90 MB/s |
| ARS Gen 1: Foundation | 10000 | 280.009µs | 55100 | 30000 | 63.79% | 1.02 | 1634.82 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 333.72µs | 55099 | 30000 | 63.79% | 1.02 | 1371.70 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 294.643µs | 125304 | 14351 | 63.79% | 1.02 | 1553.62 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 284.543µs | 52153 | 10000 | 63.79% | 1.02 | 1608.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 221.884µs | 52153 | 0 | 63.79% | 1.02 | 2063.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 252.872µs | 50387 | 0 | 63.79% | 1.02 | 1810.26 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 399.215µs | 42939 | 0 | 63.79% | 1.02 | 1146.66 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 445.66µs | 43078 | 0 | 63.79% | 1.02 | 1027.16 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 213.656µs | 16855 | 0 | 63.79% | 1.02 | 2142.53 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 207.917µs | 52153 | 0 | 63.79% | 1.02 | 2201.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 213.062µs | 52153 | 0 | 63.79% | 1.02 | 2148.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 341.033µs | 52153 | 20000 | 63.79% | 1.02 | 1342.29 MB/s |
| Quicksort | 100000 | 876.06µs | 529990 | 0 | 63.77% | 1.02 | 5225.25 MB/s |
| Timsort | 100000 | 1.486754ms | 531868 | 0 | 63.72% | 1.02 | 3078.95 MB/s |
| ARS Gen 1: Foundation | 100000 | 3.548532ms | 501611 | 300000 | 63.75% | 1.02 | 1290.01 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.698215ms | 501611 | 300000 | 63.75% | 1.02 | 1237.80 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.378672ms | 1172752 | 108703 | 63.77% | 1.02 | 1924.45 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.182976ms | 516727 | 100000 | 63.76% | 1.02 | 2096.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.92187ms | 516727 | 0 | 63.75% | 1.02 | 2381.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.665813ms | 519617 | 0 | 63.71% | 1.02 | 1717.16 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.121488ms | 512024 | 0 | 63.76% | 1.02 | 2157.75 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.039639ms | 502467 | 0 | 63.74% | 1.02 | 1133.18 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.592935ms | 206221 | 0 | 63.72% | 1.02 | 1765.43 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.223475ms | 182412 | 0 | 63.75% | 1.02 | 2058.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.857903ms | 200760 | 0 | 63.72% | 1.02 | 1601.75 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.386399ms | 516727 | 200000 | 63.74% | 1.02 | 1918.22 MB/s |
| Quicksort | 1000000 | 14.81221ms | 5281309 | 0 | 63.63% | 1.03 | 3090.45 MB/s |
| Timsort | 1000000 | 34.625042ms | 6327917 | 0 | 63.35% | 1.03 | 1322.06 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 29.911527ms | 12313781 | 1017407 | 63.77% | 1.03 | 1530.39 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 33.375381ms | 5208498 | 1000000 | 63.72% | 1.02 | 1371.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 34.220451ms | 5208498 | 0 | 63.75% | 1.02 | 1337.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 58.427539ms | 6511840 | 0 | 63.45% | 1.02 | 783.47 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 27.742652ms | 5225265 | 0 | 63.72% | 1.02 | 1650.04 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 50.584514ms | 6529655 | 0 | 63.43% | 1.02 | 904.95 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 34.554755ms | 1939650 | 0 | 63.89% | 1.02 | 1324.75 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 49.180335ms | 2064127 | 0 | 63.91% | 1.02 | 930.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 47.887699ms | 2062304 | 0 | 63.90% | 1.01 | 955.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 49.989605ms | 9851509 | 2000000 | 63.82% | 1.02 | 915.72 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.512µs | 10133 | 0 | 63.77% | 1.02 | 1794.31 MB/s |
| Timsort | 1000 | 34.592µs | 10734 | 0 | 63.77% | 1.02 | 1323.32 MB/s |
| ARS Gen 1: Foundation | 1000 | 186.972µs | 691 | 2000 | 63.76% | 1.02 | 244.83 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 193.359µs | 691 | 2000 | 63.76% | 1.02 | 236.74 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 21.806µs | 10133 | 0 | 63.77% | 1.02 | 2099.26 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 21.126µs | 10133 | 0 | 63.77% | 1.02 | 2166.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.005µs | 10133 | 0 | 63.77% | 1.02 | 2179.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.71µs | 10734 | 0 | 63.77% | 1.02 | 1540.77 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 20.309µs | 10133 | 0 | 63.77% | 1.02 | 2253.99 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.036µs | 10734 | 0 | 63.77% | 1.02 | 1576.54 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.692µs | 10133 | 0 | 63.77% | 1.02 | 2324.62 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 19.476µs | 10133 | 0 | 63.77% | 1.02 | 2350.40 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.969µs | 10133 | 0 | 63.77% | 1.02 | 2292.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.158µs | 10133 | 2000 | 63.76% | 1.02 | 419.36 MB/s |
| Quicksort | 10000 | 223.848µs | 133996 | 0 | 63.76% | 1.02 | 2044.98 MB/s |
| Timsort | 10000 | 336.439µs | 137398 | 0 | 63.76% | 1.02 | 1360.61 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.367904ms | 77629 | 30000 | 63.75% | 1.02 | 334.65 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.053782ms | 77623 | 30000 | 63.75% | 1.02 | 434.40 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 364.673µs | 189660 | 14351 | 63.75% | 1.02 | 1255.27 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 226.041µs | 69470 | 10000 | 63.75% | 1.02 | 2025.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 141.563µs | 69470 | 0 | 63.75% | 1.02 | 3233.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 163.298µs | 72482 | 0 | 63.75% | 1.02 | 2803.24 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 340.969µs | 59470 | 0 | 63.75% | 1.02 | 1342.54 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 347.891µs | 62562 | 0 | 63.75% | 1.02 | 1315.82 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 149.087µs | 69470 | 0 | 63.75% | 1.02 | 3070.45 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 136.219µs | 69470 | 0 | 63.75% | 1.02 | 3360.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 144.457µs | 69470 | 0 | 63.75% | 1.02 | 3168.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 296.073µs | 69470 | 20000 | 63.75% | 1.02 | 1546.12 MB/s |
| Quicksort | 100000 | 2.02889ms | 1339911 | 0 | 63.72% | 1.02 | 2256.23 MB/s |
| Timsort | 100000 | 3.351885ms | 1340773 | 0 | 63.67% | 1.02 | 1365.69 MB/s |
| ARS Gen 1: Foundation | 100000 | 9.847135ms | 1262245 | 300000 | 63.67% | 1.02 | 464.87 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 9.332794ms | 1262822 | 300000 | 63.66% | 1.02 | 490.49 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.597816ms | 1543517 | 108703 | 63.71% | 1.02 | 1762.11 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.38111ms | 727700 | 100000 | 63.73% | 1.02 | 3314.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.336939ms | 727700 | 0 | 63.73% | 1.02 | 3423.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.472985ms | 737053 | 0 | 63.73% | 1.02 | 3107.73 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.543423ms | 628511 | 0 | 63.72% | 1.02 | 2965.90 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.684257ms | 634320 | 0 | 63.72% | 1.02 | 2717.90 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.428101ms | 701327 | 0 | 63.73% | 1.02 | 3205.40 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.422734ms | 628891 | 0 | 63.73% | 1.02 | 3217.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.475263ms | 727700 | 0 | 63.73% | 1.02 | 3102.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.869374ms | 727700 | 200000 | 63.72% | 1.02 | 2448.75 MB/s |
| Quicksort | 1000000 | 31.420674ms | 12880459 | 0 | 63.42% | 1.03 | 1456.89 MB/s |
| Timsort | 1000000 | 56.290006ms | 13984642 | 0 | 62.93% | 1.03 | 813.22 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.508134ms | 14266844 | 1017407 | 63.49% | 1.02 | 1408.15 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.12119ms | 5509338 | 1000000 | 63.77% | 1.01 | 2526.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.134955ms | 5509338 | 0 | 63.78% | 1.02 | 2524.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 20.293192ms | 5538714 | 0 | 63.63% | 1.01 | 2255.75 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.043006ms | 6195850 | 0 | 63.75% | 1.01 | 2685.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 19.719283ms | 6227611 | 0 | 63.54% | 1.01 | 2321.40 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 24.052019ms | 2169826 | 0 | 63.70% | 1.01 | 1903.22 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 28.28899ms | 1707337 | 0 | 63.73% | 1.01 | 1618.17 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.994761ms | 1857655 | 0 | 63.78% | 1.01 | 1760.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 48.839171ms | 11852679 | 2000000 | 63.83% | 1.02 | 937.29 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 27.665µs | 9985 | 0 | 63.93% | 1.01 | 1654.67 MB/s |
| Timsort | 1000 | 37.946µs | 10392 | 0 | 63.93% | 1.01 | 1206.36 MB/s |
| ARS Gen 1: Foundation | 1000 | 139.052µs | 5421 | 2000 | 63.93% | 1.01 | 329.20 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 150.278µs | 5356 | 2000 | 63.93% | 1.01 | 304.61 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 24.168µs | 9985 | 0 | 63.93% | 1.01 | 1894.09 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 23.37µs | 9985 | 0 | 63.93% | 1.01 | 1958.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 22.646µs | 9985 | 0 | 63.93% | 1.01 | 2021.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 30.97µs | 10392 | 0 | 63.93% | 1.01 | 1478.09 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.507µs | 9985 | 0 | 63.93% | 1.01 | 2033.87 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 30.507µs | 10392 | 0 | 63.93% | 1.01 | 1500.52 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 21.45µs | 9985 | 0 | 63.93% | 1.01 | 2134.10 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 21.73µs | 9985 | 0 | 63.93% | 1.01 | 2106.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 21.456µs | 9985 | 0 | 63.93% | 1.01 | 2133.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 119.554µs | 9985 | 2000 | 63.93% | 1.01 | 382.89 MB/s |
| Quicksort | 10000 | 174.933µs | 107604 | 0 | 63.92% | 1.01 | 2616.79 MB/s |
| Timsort | 10000 | 234.383µs | 109657 | 0 | 63.92% | 1.01 | 1953.06 MB/s |
| ARS Gen 1: Foundation | 10000 | 563.839µs | 73762 | 30000 | 63.92% | 1.01 | 811.87 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 652.864µs | 73552 | 30000 | 63.92% | 1.01 | 701.16 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 361.796µs | 160276 | 14351 | 63.92% | 1.01 | 1265.25 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 259.329µs | 70340 | 10000 | 63.92% | 1.01 | 1765.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 168.067µs | 70340 | 0 | 63.92% | 1.01 | 2723.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 181.131µs | 71216 | 0 | 63.92% | 1.01 | 2527.25 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 262.544µs | 59344 | 0 | 63.92% | 1.01 | 1743.57 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 304.517µs | 60054 | 0 | 63.92% | 1.01 | 1503.25 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 170.68µs | 70340 | 0 | 63.92% | 1.01 | 2682.00 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 159.984µs | 70340 | 0 | 63.92% | 1.01 | 2861.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 165.032µs | 70340 | 0 | 63.92% | 1.01 | 2773.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 322.72µs | 70340 | 20000 | 63.91% | 1.01 | 1418.45 MB/s |
| Quicksort | 100000 | 1.643078ms | 1011458 | 0 | 63.89% | 1.02 | 2786.01 MB/s |
| Timsort | 100000 | 2.390099ms | 1014769 | 0 | 63.84% | 1.01 | 1915.25 MB/s |
| ARS Gen 1: Foundation | 100000 | 5.101667ms | 696758 | 300000 | 63.89% | 1.01 | 897.28 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 4.83408ms | 697287 | 300000 | 63.89% | 1.01 | 946.95 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.706307ms | 1231300 | 108703 | 63.89% | 1.01 | 1691.47 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.476203ms | 671477 | 100000 | 63.89% | 1.01 | 3100.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.342721ms | 671477 | 0 | 63.90% | 1.01 | 3409.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.689581ms | 673524 | 0 | 63.88% | 1.01 | 2709.33 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.285142ms | 554286 | 0 | 63.90% | 1.01 | 3561.97 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.423318ms | 555220 | 0 | 63.89% | 1.01 | 3216.17 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.636218ms | 105158 | 0 | 63.88% | 1.01 | 2797.69 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.723238ms | 179970 | 0 | 63.88% | 1.01 | 2656.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.600709ms | 140724 | 0 | 63.89% | 1.01 | 2859.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.13159ms | 671477 | 200000 | 63.89% | 1.01 | 2147.52 MB/s |
| Quicksort | 1000000 | 21.926149ms | 9937773 | 0 | 63.55% | 1.02 | 2087.75 MB/s |
| Timsort | 1000000 | 51.115972ms | 11004404 | 0 | 63.11% | 1.02 | 895.54 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 33.24982ms | 12334215 | 1017407 | 63.72% | 1.01 | 1376.74 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 19.907757ms | 4762552 | 1000000 | 63.86% | 1.01 | 2299.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.206032ms | 4762552 | 0 | 63.90% | 1.00 | 2061.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 29.579097ms | 4748110 | 0 | 63.74% | 1.00 | 1547.59 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.247145ms | 4888204 | 0 | 63.89% | 1.01 | 2508.69 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 26.949237ms | 4904839 | 0 | 63.73% | 1.00 | 1698.61 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 25.929578ms | 1096506 | 0 | 63.99% | 1.00 | 1765.41 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 30.431476ms | 1053340 | 0 | 64.02% | 1.00 | 1504.24 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 30.398357ms | 1036862 | 0 | 64.03% | 1.00 | 1505.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 45.405666ms | 10645199 | 2000000 | 63.88% | 1.01 | 1008.16 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 30.035µs | 10337 | 0 | 63.88% | 1.01 | 1524.10 MB/s |
| Timsort | 1000 | 42.532µs | 10667 | 0 | 63.88% | 1.01 | 1076.28 MB/s |
| ARS Gen 1: Foundation | 1000 | 325.004µs | 0 | 2000 | 63.88% | 1.01 | 140.85 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 375.591µs | 0 | 2000 | 63.88% | 1.01 | 121.88 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 30.358µs | 10337 | 0 | 63.88% | 1.01 | 1507.88 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 30.325µs | 10337 | 0 | 63.88% | 1.01 | 1509.53 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 30.238µs | 10337 | 0 | 63.88% | 1.01 | 1513.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 42.287µs | 10667 | 0 | 63.88% | 1.01 | 1082.52 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 30.466µs | 10337 | 0 | 63.88% | 1.01 | 1502.54 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 43.184µs | 10667 | 0 | 63.88% | 1.01 | 1060.03 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 30.102µs | 10337 | 0 | 63.88% | 1.01 | 1520.71 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 30.311µs | 10337 | 0 | 63.88% | 1.01 | 1510.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 30.437µs | 10337 | 0 | 63.88% | 1.01 | 1503.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 173.676µs | 10337 | 2000 | 63.87% | 1.01 | 263.57 MB/s |
| Quicksort | 10000 | 333.567µs | 137946 | 0 | 63.87% | 1.01 | 1372.33 MB/s |
| Timsort | 10000 | 364.158µs | 142499 | 0 | 63.87% | 1.01 | 1257.05 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.076108ms | 0 | 30000 | 63.84% | 1.01 | 90.18 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.261698ms | 0 | 30000 | 63.83% | 1.01 | 87.00 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 355.307µs | 194806 | 14351 | 63.86% | 1.01 | 1288.36 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 218.545µs | 52643 | 10000 | 63.86% | 1.01 | 2094.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 139.057µs | 52643 | 0 | 63.86% | 1.01 | 3291.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 143.592µs | 58028 | 0 | 63.86% | 1.01 | 3187.95 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 209.204µs | 60571 | 0 | 63.86% | 1.01 | 2188.12 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 219.181µs | 63560 | 0 | 63.86% | 1.01 | 2088.52 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 157.624µs | 52643 | 0 | 63.86% | 1.01 | 2904.15 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 144.05µs | 52643 | 0 | 63.86% | 1.01 | 3177.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 141.851µs | 52643 | 0 | 63.87% | 1.01 | 3227.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 301.477µs | 52643 | 20000 | 63.86% | 1.01 | 1518.40 MB/s |
| Quicksort | 100000 | 3.206806ms | 1718970 | 0 | 63.83% | 1.01 | 1427.48 MB/s |
| Timsort | 100000 | 4.716347ms | 1756228 | 0 | 63.78% | 1.01 | 970.59 MB/s |
| ARS Gen 1: Foundation | 100000 | 38.967904ms | 5 | 300000 | 62.15% | 1.01 | 117.47 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 43.352358ms | 5 | 300000 | 62.30% | 1.01 | 105.59 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.519259ms | 1893310 | 108703 | 63.82% | 1.01 | 1300.74 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.68407ms | 888976 | 100000 | 63.84% | 1.01 | 2718.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.528076ms | 888976 | 0 | 63.84% | 1.01 | 2995.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.774708ms | 929234 | 0 | 63.84% | 1.01 | 2579.37 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.666222ms | 956140 | 0 | 63.83% | 1.01 | 2747.32 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.962326ms | 992831 | 0 | 63.83% | 1.01 | 2332.76 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.678375ms | 888976 | 0 | 63.84% | 1.01 | 2727.42 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.61148ms | 780493 | 0 | 63.83% | 1.01 | 2840.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.594788ms | 888976 | 0 | 63.84% | 1.01 | 2870.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.015228ms | 888976 | 200000 | 63.84% | 1.01 | 2271.52 MB/s |
| Quicksort | 1000000 | 45.877156ms | 20525437 | 0 | 63.50% | 1.02 | 997.80 MB/s |
| Timsort | 1000000 | 75.383522ms | 20897754 | 0 | 63.08% | 1.01 | 607.25 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 36.819186ms | 21586005 | 1017407 | 63.59% | 1.01 | 1243.27 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.820296ms | 10308690 | 1000000 | 63.84% | 1.00 | 2432.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.964844ms | 10308690 | 0 | 63.85% | 1.00 | 2548.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 20.196409ms | 10708698 | 0 | 63.82% | 1.00 | 2266.56 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.931496ms | 13010120 | 0 | 63.82% | 1.00 | 2418.00 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 22.59846ms | 13427133 | 0 | 63.74% | 1.00 | 2025.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.758458ms | 10308690 | 0 | 63.84% | 1.00 | 2316.80 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 21.439034ms | 11360616 | 0 | 63.88% | 1.00 | 2135.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 21.73865ms | 12417054 | 0 | 63.88% | 1.01 | 2105.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 51.115534ms | 13805437 | 2000000 | 63.92% | 1.00 | 895.55 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 9.792µs | 5628 | 0 | 64.06% | 1.00 | 4674.87 MB/s |
| Timsort | 1000 | 12.173µs | 5482 | 0 | 64.06% | 1.00 | 3760.48 MB/s |
| ARS Gen 1: Foundation | 1000 | 52.372µs | 984 | 2000 | 64.06% | 1.00 | 874.06 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 62.276µs | 984 | 2000 | 64.06% | 1.00 | 735.06 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 9.238µs | 5628 | 0 | 64.06% | 1.00 | 4955.22 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 9.184µs | 5628 | 0 | 64.06% | 1.00 | 4984.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 9.206µs | 5628 | 0 | 64.06% | 1.00 | 4972.45 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.507µs | 5482 | 0 | 64.06% | 1.00 | 3978.13 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 9.242µs | 5628 | 0 | 64.06% | 1.00 | 4953.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 11.608µs | 5482 | 0 | 64.06% | 1.00 | 3943.52 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 9.206µs | 5628 | 0 | 64.06% | 1.00 | 4972.45 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 9.279µs | 5628 | 0 | 64.06% | 1.00 | 4933.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 9.303µs | 5628 | 0 | 64.06% | 1.00 | 4920.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 119.25µs | 5628 | 2000 | 64.06% | 1.00 | 383.87 MB/s |
| Quicksort | 10000 | 86.397µs | 54006 | 0 | 64.06% | 1.00 | 5298.37 MB/s |
| Timsort | 10000 | 126.11µs | 53486 | 0 | 64.06% | 1.00 | 3629.88 MB/s |
| ARS Gen 1: Foundation | 10000 | 333.408µs | 9984 | 30000 | 64.05% | 1.00 | 1372.98 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 341.291µs | 9984 | 30000 | 64.05% | 1.00 | 1341.27 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 334.603µs | 122898 | 14351 | 64.05% | 1.00 | 1368.08 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 212.049µs | 9990 | 10000 | 64.05% | 1.00 | 2158.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 127.565µs | 9990 | 0 | 64.05% | 1.00 | 3588.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 135.168µs | 9990 | 0 | 64.05% | 1.00 | 3386.63 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 276.054µs | 9990 | 0 | 64.05% | 1.00 | 1658.24 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 265.512µs | 9990 | 0 | 64.05% | 1.00 | 1724.08 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 137.298µs | 9990 | 0 | 64.05% | 1.00 | 3334.09 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 137.433µs | 9990 | 0 | 64.05% | 1.00 | 3330.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 144.963µs | 9990 | 0 | 64.05% | 1.00 | 3157.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 356.857µs | 9990 | 20000 | 64.05% | 1.00 | 1282.77 MB/s |
| Quicksort | 100000 | 971.287µs | 522721 | 0 | 64.02% | 1.00 | 4712.96 MB/s |
| Timsort | 100000 | 1.685744ms | 535563 | 0 | 63.98% | 1.00 | 2715.50 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.95094ms | 99984 | 300000 | 64.03% | 1.00 | 1551.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.234358ms | 99984 | 300000 | 64.03% | 1.00 | 1415.32 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.59461ms | 1145301 | 108703 | 64.03% | 1.00 | 1764.29 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.192753ms | 119528 | 100000 | 64.03% | 1.00 | 3837.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.061712ms | 119528 | 0 | 64.03% | 1.00 | 4311.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.109652ms | 119779 | 0 | 64.03% | 1.00 | 4125.29 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.171069ms | 99990 | 0 | 64.03% | 1.00 | 3908.94 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.165997ms | 99990 | 0 | 64.03% | 1.00 | 3925.94 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.360371ms | 199986 | 0 | 64.02% | 1.00 | 3364.99 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.280271ms | 199974 | 0 | 64.03% | 1.00 | 3575.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.252895ms | 100002 | 0 | 64.03% | 1.00 | 3653.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.74169ms | 119528 | 200000 | 64.02% | 1.00 | 2628.27 MB/s |
| Quicksort | 1000000 | 14.771392ms | 5200332 | 0 | 63.88% | 1.00 | 3098.99 MB/s |
| Timsort | 1000000 | 38.066462ms | 6204510 | 0 | 63.59% | 1.00 | 1202.54 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.831022ms | 12086670 | 1017407 | 64.05% | 1.00 | 1644.80 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 16.584443ms | 999988 | 1000000 | 64.13% | 1.00 | 2760.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.525211ms | 999988 | 0 | 64.12% | 1.00 | 2948.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 16.493568ms | 999988 | 0 | 64.13% | 1.00 | 2775.41 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 13.178123ms | 999988 | 0 | 64.11% | 1.00 | 3473.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 13.913097ms | 999988 | 0 | 64.11% | 1.00 | 3290.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 18.386979ms | 1999972 | 0 | 64.12% | 1.00 | 2489.61 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.563138ms | 1999972 | 0 | 64.13% | 0.99 | 2465.98 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.540812ms | 1999972 | 0 | 64.12% | 0.99 | 2468.95 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.017487ms | 5484640 | 2000000 | 64.03% | 0.99 | 1064.13 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 31.523µs | 10337 | 0 | 64.05% | 1.00 | 1452.16 MB/s |
| Timsort | 1000 | 43.158µs | 10667 | 0 | 64.05% | 1.00 | 1060.67 MB/s |
| ARS Gen 1: Foundation | 1000 | 302.32µs | 0 | 2000 | 64.05% | 1.00 | 151.42 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 326.712µs | 0 | 2000 | 64.05% | 1.00 | 140.11 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 24.184µs | 10337 | 0 | 64.05% | 1.00 | 1892.84 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 24.294µs | 10337 | 0 | 64.05% | 1.00 | 1884.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 23.41µs | 10337 | 0 | 64.05% | 1.00 | 1955.42 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 32.924µs | 10667 | 0 | 64.05% | 1.00 | 1390.36 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.826µs | 10337 | 0 | 64.05% | 1.00 | 2005.45 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 31.104µs | 10667 | 0 | 64.05% | 1.00 | 1471.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 22.449µs | 10337 | 0 | 64.05% | 1.00 | 2039.13 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 21.98µs | 10337 | 0 | 64.05% | 1.00 | 2082.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 21.977µs | 10337 | 0 | 64.05% | 1.00 | 2082.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 122.959µs | 10337 | 2000 | 64.05% | 1.00 | 372.29 MB/s |
| Quicksort | 10000 | 257.704µs | 137946 | 0 | 64.05% | 1.00 | 1776.32 MB/s |
| Timsort | 10000 | 342.769µs | 142499 | 0 | 64.05% | 1.00 | 1335.49 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.805701ms | 0 | 30000 | 64.02% | 1.00 | 95.25 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.265817ms | 0 | 30000 | 64.01% | 1.00 | 86.93 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 377.512µs | 194806 | 14351 | 64.04% | 1.00 | 1212.58 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 219.395µs | 52643 | 10000 | 64.04% | 1.00 | 2086.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 145.789µs | 52643 | 0 | 64.04% | 1.00 | 3139.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 142.225µs | 58028 | 0 | 64.04% | 1.00 | 3218.59 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 188.302µs | 60571 | 0 | 64.04% | 1.00 | 2431.01 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 184.906µs | 63560 | 0 | 64.04% | 1.00 | 2475.66 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 161.384µs | 52643 | 0 | 64.04% | 1.00 | 2836.49 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 142.151µs | 52643 | 0 | 64.04% | 1.00 | 3220.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 144.81µs | 52643 | 0 | 64.04% | 1.00 | 3161.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 350.588µs | 52643 | 20000 | 64.04% | 1.00 | 1305.70 MB/s |
| Quicksort | 100000 | 3.242481ms | 1718970 | 0 | 64.02% | 1.00 | 1411.77 MB/s |
| Timsort | 100000 | 4.624547ms | 1756228 | 0 | 63.98% | 1.00 | 989.86 MB/s |
| ARS Gen 1: Foundation | 100000 | 38.778304ms | 5 | 300000 | 62.46% | 1.00 | 118.05 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.685964ms | 5 | 300000 | 62.49% | 1.00 | 102.44 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.876789ms | 1893310 | 108703 | 64.01% | 1.00 | 1591.23 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.586029ms | 888976 | 100000 | 64.03% | 1.00 | 2886.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.382353ms | 888976 | 0 | 64.03% | 1.00 | 3311.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.495753ms | 929234 | 0 | 64.03% | 1.00 | 3060.42 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.519139ms | 956140 | 0 | 64.02% | 1.00 | 3013.31 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.699416ms | 992831 | 0 | 64.02% | 1.00 | 2693.65 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.557645ms | 888976 | 0 | 64.02% | 1.00 | 2938.82 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.557155ms | 780493 | 0 | 64.02% | 1.00 | 2939.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.671045ms | 888976 | 0 | 64.02% | 1.00 | 2739.39 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.035886ms | 888976 | 200000 | 64.02% | 1.00 | 2248.47 MB/s |
| Quicksort | 1000000 | 46.974582ms | 20525437 | 0 | 63.76% | 1.01 | 974.49 MB/s |
| Timsort | 1000000 | 76.561322ms | 20897754 | 0 | 63.39% | 1.00 | 597.90 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 36.950982ms | 21586005 | 1017407 | 63.82% | 1.00 | 1238.84 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 19.290024ms | 10308690 | 1000000 | 64.04% | 0.99 | 2373.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.3962ms | 10308690 | 0 | 64.04% | 0.99 | 2488.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.807876ms | 10708698 | 0 | 64.02% | 0.99 | 2311.02 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 20.987222ms | 13010120 | 0 | 64.02% | 0.99 | 2181.15 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.234375ms | 13427133 | 0 | 63.93% | 0.99 | 2262.31 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 20.859565ms | 10308690 | 0 | 64.04% | 0.99 | 2194.50 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 21.821544ms | 11360616 | 0 | 64.09% | 0.99 | 2097.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.86514ms | 12417054 | 0 | 64.08% | 0.99 | 2304.36 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 50.3087ms | 13805432 | 2000000 | 64.09% | 0.99 | 909.91 MB/s |
