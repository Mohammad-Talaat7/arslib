# ARS Evolution Atlas: Final Research Study

## 1. Experimental Setup
- **Cores:** 8 | **RAM:** 15864 MB
- **PMC Instrumentation:** true (Multi-thread Inherit: Enabled)
- **Statistical Setup:** Reps=7, Seed=42

## Category: i64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 16.676µs | 10227 | 0 | 0.00% | 2.57 | 915.01 MB/s |
| Timsort | 1000 | 27.92µs | 10588 | 0 | 0.00% | 2.03 | 546.52 MB/s |
| ARS Gen 1: Foundation | 1000 | 348.351µs | 0 | 2000 | 0.00% | 2.07 | 43.80 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 338.463µs | 0 | 2000 | 0.11% | 2.09 | 45.08 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 14.23µs | 10227 | 0 | 0.00% | 2.55 | 1072.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.935µs | 10227 | 0 | 0.00% | 2.63 | 1278.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 12.177µs | 10227 | 0 | 0.00% | 2.59 | 1253.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 20.291µs | 10588 | 0 | 0.00% | 2.05 | 752.00 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 12.555µs | 10227 | 0 | 0.00% | 2.52 | 1215.36 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 20.389µs | 10588 | 0 | 0.00% | 2.04 | 748.38 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.792µs | 10227 | 0 | 0.00% | 2.57 | 1413.90 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.695µs | 10227 | 0 | 0.00% | 2.61 | 1426.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.718µs | 10227 | 0 | 0.00% | 2.60 | 1423.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 111.432µs | 10227 | 2000 | 0.74% | 0.65 | 136.93 MB/s |
| Quicksort | 10000 | 122.604µs | 136654 | 0 | 0.26% | 0.99 | 1244.56 MB/s |
| Timsort | 10000 | 195.46µs | 140327 | 0 | 0.30% | 1.03 | 780.66 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.385686ms | 0 | 30000 | 0.16% | 1.70 | 23.90 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.211085ms | 0 | 30000 | 0.31% | 1.76 | 24.57 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 332.503µs | 193611 | 14351 | 0.27% | 1.07 | 458.91 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 265.106µs | 51695 | 10000 | 0.16% | 0.84 | 575.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 176.779µs | 51695 | 0 | 0.15% | 0.74 | 863.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 173.195µs | 57359 | 0 | 0.13% | 0.78 | 881.02 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 379.923µs | 59671 | 0 | 1.06% | 0.73 | 401.63 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 316.817µs | 62214 | 0 | 1.52% | 0.73 | 481.63 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 166.1µs | 51695 | 0 | 0.14% | 0.72 | 918.65 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 178.39µs | 51695 | 0 | 0.12% | 0.74 | 855.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 164.223µs | 51695 | 0 | 0.13% | 0.76 | 929.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 372.25µs | 51695 | 20000 | 0.19% | 0.73 | 409.91 MB/s |
| Quicksort | 100000 | 1.395714ms | 1709595 | 0 | 1.27% | 1.69 | 1093.26 MB/s |
| Timsort | 100000 | 2.215315ms | 1743505 | 0 | 1.28% | 1.63 | 688.79 MB/s |
| ARS Gen 1: Foundation | 100000 | 54.354058ms | 0 | 300000 | 1.57% | 0.95 | 28.07 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 59.032231ms | 0 | 300000 | 2.27% | 1.02 | 25.85 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.22313ms | 1885062 | 108703 | 8.28% | 1.31 | 686.37 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.066601ms | 881353 | 100000 | 13.14% | 1.03 | 1430.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.000979ms | 881353 | 0 | 15.64% | 1.02 | 1524.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 947.772µs | 921838 | 0 | 13.90% | 0.98 | 1609.96 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.04276ms | 955554 | 0 | 13.48% | 1.09 | 1463.31 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.226079ms | 991979 | 0 | 12.81% | 1.07 | 1244.52 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 751.244µs | 881353 | 0 | 10.60% | 1.00 | 2031.14 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 847.78µs | 772388 | 0 | 15.15% | 1.04 | 1799.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 783.333µs | 881353 | 0 | 7.66% | 0.97 | 1947.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.196248ms | 881353 | 200000 | 19.58% | 0.95 | 1275.55 MB/s |
| Quicksort | 1000000 | 16.898579ms | 20423287 | 0 | 17.19% | 2.15 | 902.96 MB/s |
| Timsort | 1000000 | 28.210658ms | 20813246 | 0 | 23.24% | 1.83 | 540.89 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.727155ms | 21493355 | 1017407 | 31.10% | 1.42 | 702.29 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.956ms | 10218658 | 1000000 | 49.25% | 0.99 | 1532.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.202183ms | 10218658 | 0 | 52.13% | 0.88 | 1495.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.499451ms | 10628212 | 0 | 51.10% | 0.92 | 1453.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.162893ms | 13023009 | 0 | 40.13% | 1.02 | 2130.26 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.804422ms | 13432511 | 0 | 36.83% | 1.02 | 1733.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.273429ms | 10218658 | 0 | 44.01% | 0.97 | 1844.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.028908ms | 11276404 | 0 | 51.05% | 1.10 | 1689.99 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.119557ms | 12320223 | 0 | 50.17% | 0.89 | 1673.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 21.159651ms | 12171638 | 2000000 | 48.95% | 0.88 | 721.13 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.846µs | 10330 | 0 | 38.90% | 0.94 | 1288.10 MB/s |
| Timsort | 1000 | 18.596µs | 10648 | 0 | 38.89% | 0.94 | 820.54 MB/s |
| ARS Gen 1: Foundation | 1000 | 188.659µs | 503 | 2000 | 38.89% | 0.94 | 80.88 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 210.667µs | 503 | 2000 | 38.88% | 0.94 | 72.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.668µs | 10330 | 0 | 38.90% | 0.94 | 1307.75 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.487µs | 10330 | 0 | 38.90% | 0.94 | 1328.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.566µs | 10330 | 0 | 38.90% | 0.94 | 1319.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 18.426µs | 10648 | 0 | 38.89% | 0.94 | 828.11 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.886µs | 10330 | 0 | 38.90% | 0.94 | 1283.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.497µs | 10648 | 0 | 38.89% | 0.94 | 824.93 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.586µs | 10330 | 0 | 38.90% | 0.94 | 1317.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.672µs | 10330 | 0 | 38.90% | 0.94 | 1307.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.473µs | 10330 | 0 | 38.90% | 0.94 | 1329.97 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 108.846µs | 10330 | 2000 | 38.80% | 0.94 | 140.19 MB/s |
| Quicksort | 10000 | 140.678µs | 134638 | 0 | 38.54% | 0.94 | 1084.66 MB/s |
| Timsort | 10000 | 212.03µs | 140096 | 0 | 38.53% | 0.94 | 719.65 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.270361ms | 57643 | 30000 | 38.07% | 0.95 | 120.11 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.380062ms | 57632 | 30000 | 37.97% | 0.95 | 110.57 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 337.82µs | 191358 | 14351 | 38.32% | 0.94 | 451.68 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 232.009µs | 61389 | 10000 | 38.22% | 0.94 | 657.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 150.45µs | 61389 | 0 | 38.16% | 0.94 | 1014.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 155.988µs | 64672 | 0 | 38.16% | 0.94 | 978.20 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 313.52µs | 58551 | 0 | 38.04% | 0.94 | 486.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 331.514µs | 61376 | 0 | 38.06% | 0.94 | 460.28 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 149.28µs | 61389 | 0 | 38.14% | 0.94 | 1022.16 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 142.793µs | 61389 | 0 | 38.15% | 0.94 | 1068.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 148.834µs | 61389 | 0 | 38.17% | 0.94 | 1025.22 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 319.216µs | 61389 | 20000 | 38.02% | 0.94 | 478.01 MB/s |
| Quicksort | 100000 | 1.300512ms | 1446704 | 0 | 37.07% | 0.96 | 1173.29 MB/s |
| Timsort | 100000 | 1.5912ms | 1445193 | 0 | 36.22% | 0.96 | 958.95 MB/s |
| ARS Gen 1: Foundation | 100000 | 6.976516ms | 1387258 | 300000 | 34.73% | 0.96 | 218.72 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.390824ms | 1386968 | 300000 | 35.20% | 0.97 | 181.85 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.181847ms | 1645061 | 108703 | 36.62% | 0.96 | 699.35 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 868.041µs | 734392 | 100000 | 36.53% | 0.95 | 1757.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 655.883µs | 734392 | 0 | 36.73% | 0.94 | 2326.45 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 595.371µs | 735546 | 0 | 36.39% | 0.95 | 2562.90 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 808.087µs | 701300 | 0 | 36.52% | 0.95 | 1888.26 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 822.403µs | 706496 | 0 | 36.48% | 0.95 | 1855.39 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 620.372µs | 734392 | 0 | 36.32% | 0.95 | 2459.62 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 636.273µs | 629097 | 0 | 36.41% | 0.95 | 2398.15 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 599.554µs | 734392 | 0 | 36.24% | 0.95 | 2545.02 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 936.46µs | 734392 | 200000 | 36.41% | 0.95 | 1629.41 MB/s |
| Quicksort | 1000000 | 9.568002ms | 13567694 | 0 | 32.02% | 1.13 | 1594.77 MB/s |
| Timsort | 1000000 | 14.604037ms | 14681691 | 0 | 29.29% | 1.11 | 1044.83 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 18.591769ms | 14956001 | 1017407 | 35.58% | 1.07 | 820.73 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.981665ms | 4787996 | 1000000 | 42.97% | 0.91 | 2185.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.756246ms | 4787996 | 0 | 43.04% | 0.91 | 2258.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.792724ms | 4821847 | 0 | 43.00% | 0.91 | 2246.34 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.446529ms | 6224416 | 0 | 39.54% | 0.99 | 2801.56 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.661009ms | 6253879 | 0 | 38.54% | 0.99 | 2695.42 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 6.583797ms | 4757456 | 0 | 41.31% | 0.91 | 2317.63 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.863621ms | 2295151 | 0 | 40.64% | 0.93 | 1721.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.508226ms | 2529783 | 0 | 40.24% | 0.90 | 1793.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.192498ms | 11703943 | 2000000 | 37.21% | 1.01 | 838.74 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.194µs | 9762 | 0 | 43.88% | 1.12 | 1363.12 MB/s |
| Timsort | 1000 | 15.476µs | 9882 | 0 | 43.88% | 1.12 | 985.96 MB/s |
| ARS Gen 1: Foundation | 1000 | 89.594µs | 9788 | 2000 | 43.88% | 1.12 | 170.31 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 99.143µs | 9815 | 2000 | 43.88% | 1.12 | 153.91 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.41µs | 9762 | 0 | 43.88% | 1.12 | 1337.32 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.267µs | 9762 | 0 | 43.88% | 1.12 | 1354.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.348µs | 9762 | 0 | 43.88% | 1.12 | 1344.62 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 15.529µs | 9882 | 0 | 43.88% | 1.12 | 982.60 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.359µs | 9762 | 0 | 43.88% | 1.12 | 1343.32 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 15.497µs | 9882 | 0 | 43.88% | 1.12 | 984.63 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 11.249µs | 9762 | 0 | 43.88% | 1.12 | 1356.46 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.298µs | 9762 | 0 | 43.88% | 1.12 | 1350.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.203µs | 9762 | 0 | 43.88% | 1.12 | 1362.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.192µs | 9762 | 2000 | 43.85% | 1.12 | 132.46 MB/s |
| Quicksort | 10000 | 138.211µs | 134689 | 0 | 43.73% | 1.12 | 1104.02 MB/s |
| Timsort | 10000 | 174.657µs | 132195 | 0 | 43.72% | 1.12 | 873.64 MB/s |
| ARS Gen 1: Foundation | 10000 | 759.995µs | 130386 | 30000 | 43.57% | 1.12 | 200.77 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 792.736µs | 130325 | 30000 | 43.57% | 1.12 | 192.48 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 296.766µs | 187157 | 14351 | 43.64% | 1.12 | 514.17 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 206.275µs | 45304 | 10000 | 43.59% | 1.12 | 739.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 130.37µs | 45304 | 0 | 43.59% | 1.12 | 1170.42 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 131.347µs | 36417 | 0 | 43.58% | 1.12 | 1161.72 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 296.592µs | 52081 | 0 | 43.53% | 1.12 | 514.47 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 308.941µs | 47021 | 0 | 43.54% | 1.12 | 493.91 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 138.581µs | 45304 | 0 | 43.58% | 1.12 | 1101.07 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 138.905µs | 45304 | 0 | 43.59% | 1.12 | 1098.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 142.249µs | 45304 | 0 | 43.58% | 1.12 | 1072.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 303.886µs | 45304 | 20000 | 43.50% | 1.12 | 502.12 MB/s |
| Quicksort | 100000 | 1.585669ms | 1716043 | 0 | 43.05% | 1.13 | 962.29 MB/s |
| Timsort | 100000 | 1.893364ms | 1660908 | 0 | 42.65% | 1.13 | 805.91 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.166415ms | 1643878 | 300000 | 42.19% | 1.14 | 212.92 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 7.633198ms | 1643640 | 300000 | 42.55% | 1.14 | 199.90 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.15222ms | 1830188 | 108703 | 42.84% | 1.12 | 708.98 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 876.933µs | 827444 | 100000 | 42.67% | 1.12 | 1740.02 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 667.977µs | 827444 | 0 | 42.77% | 1.12 | 2284.33 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 494.806µs | 410171 | 0 | 42.64% | 1.12 | 3083.79 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 620.479µs | 906132 | 0 | 42.76% | 1.12 | 2459.20 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 633.835µs | 448015 | 0 | 42.68% | 1.12 | 2407.38 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 644.95µs | 827444 | 0 | 42.66% | 1.12 | 2365.89 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 745.742µs | 718138 | 0 | 42.92% | 1.12 | 2046.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 660.879µs | 827444 | 0 | 42.63% | 1.12 | 2308.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.144409ms | 827444 | 200000 | 42.68% | 1.12 | 1333.33 MB/s |
| Quicksort | 1000000 | 17.784363ms | 20672771 | 0 | 40.88% | 1.24 | 857.99 MB/s |
| Timsort | 1000000 | 23.648669ms | 19775927 | 0 | 37.55% | 1.23 | 645.23 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.791484ms | 20984698 | 1017407 | 43.19% | 1.19 | 770.98 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.153183ms | 9742173 | 1000000 | 45.06% | 1.09 | 1667.05 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.693936ms | 9742173 | 0 | 45.16% | 1.06 | 1755.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.305837ms | 4127840 | 0 | 45.04% | 1.04 | 2088.58 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.384588ms | 12610499 | 0 | 44.05% | 1.13 | 2066.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.239168ms | 5755875 | 0 | 43.76% | 1.13 | 2445.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.236517ms | 9742173 | 0 | 44.63% | 1.08 | 1852.58 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.792702ms | 10843448 | 0 | 45.03% | 1.10 | 1735.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.130957ms | 11954018 | 0 | 44.81% | 1.07 | 1671.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.38073ms | 14936473 | 2000000 | 44.57% | 1.11 | 830.15 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 3.082µs | 3735 | 0 | 46.91% | 1.23 | 4950.94 MB/s |
| Timsort | 1000 | 4.278µs | 3747 | 0 | 46.91% | 1.23 | 3566.80 MB/s |
| ARS Gen 1: Foundation | 1000 | 30.489µs | 995 | 2000 | 46.91% | 1.23 | 500.47 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 41.34µs | 995 | 2000 | 46.91% | 1.23 | 369.10 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 3.024µs | 3735 | 0 | 46.91% | 1.23 | 5045.90 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 3µs | 3735 | 0 | 46.91% | 1.23 | 5086.26 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 3.032µs | 3735 | 0 | 46.91% | 1.23 | 5032.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 4.13µs | 3747 | 0 | 46.91% | 1.23 | 3694.62 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 3.121µs | 3735 | 0 | 46.91% | 1.23 | 4889.07 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 4.353µs | 3747 | 0 | 46.91% | 1.23 | 3505.35 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 3.07µs | 3735 | 0 | 46.91% | 1.23 | 4970.29 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 3.07µs | 3735 | 0 | 46.91% | 1.23 | 4970.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 3.031µs | 3735 | 0 | 46.91% | 1.23 | 5034.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 100.752µs | 3735 | 2000 | 46.89% | 1.23 | 151.45 MB/s |
| Quicksort | 10000 | 27.489µs | 36573 | 0 | 46.83% | 1.23 | 5550.87 MB/s |
| Timsort | 10000 | 36.038µs | 36775 | 0 | 46.82% | 1.23 | 4234.08 MB/s |
| ARS Gen 1: Foundation | 10000 | 176.256µs | 9995 | 30000 | 46.80% | 1.23 | 865.72 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 241.685µs | 9995 | 30000 | 46.79% | 1.23 | 631.35 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 233.132µs | 115988 | 14351 | 46.79% | 1.23 | 654.51 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 189.535µs | 9999 | 10000 | 46.76% | 1.23 | 805.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 113.337µs | 9999 | 0 | 46.76% | 1.23 | 1346.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 112.862µs | 9999 | 0 | 46.76% | 1.23 | 1351.99 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 291.482µs | 9999 | 0 | 46.75% | 1.23 | 523.49 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 294.289µs | 9999 | 0 | 46.74% | 1.23 | 518.50 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 122.765µs | 9999 | 0 | 46.76% | 1.23 | 1242.93 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 123.29µs | 9999 | 0 | 46.76% | 1.23 | 1237.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 122.992µs | 9999 | 0 | 46.76% | 1.23 | 1240.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 275.031µs | 9999 | 20000 | 46.73% | 1.23 | 554.80 MB/s |
| Quicksort | 100000 | 282.132µs | 362094 | 0 | 46.53% | 1.23 | 5408.39 MB/s |
| Timsort | 100000 | 381.082µs | 382517 | 0 | 46.32% | 1.23 | 4004.07 MB/s |
| ARS Gen 1: Foundation | 100000 | 975.659µs | 99995 | 300000 | 46.51% | 1.23 | 1563.95 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.374353ms | 99995 | 300000 | 46.52% | 1.23 | 1110.25 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.755498ms | 1129938 | 108703 | 46.48% | 1.24 | 869.20 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 582.778µs | 100001 | 100000 | 46.32% | 1.23 | 2618.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 330.65µs | 100001 | 0 | 46.32% | 1.23 | 4614.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 340.895µs | 100001 | 0 | 46.33% | 1.23 | 4476.10 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 548.02µs | 100001 | 0 | 46.37% | 1.23 | 2784.35 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 611.402µs | 100001 | 0 | 46.36% | 1.23 | 2495.70 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 498.914µs | 199996 | 0 | 46.24% | 1.23 | 3058.40 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 480.683µs | 199996 | 0 | 46.31% | 1.23 | 3174.40 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 490.365µs | 199996 | 0 | 46.22% | 1.23 | 3111.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 733.66µs | 100001 | 200000 | 46.29% | 1.23 | 2079.82 MB/s |
| Quicksort | 1000000 | 2.983769ms | 3809528 | 0 | 45.70% | 1.24 | 5113.93 MB/s |
| Timsort | 1000000 | 6.693963ms | 4510660 | 0 | 45.25% | 1.24 | 2279.49 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.387937ms | 12062959 | 1017407 | 46.25% | 1.26 | 787.02 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.287052ms | 999999 | 1000000 | 47.67% | 1.18 | 2427.02 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.524496ms | 999999 | 0 | 47.62% | 1.18 | 2338.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.554611ms | 999999 | 0 | 47.66% | 1.19 | 2327.95 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.487756ms | 999999 | 0 | 46.97% | 1.21 | 3400.09 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.507393ms | 999999 | 0 | 46.97% | 1.21 | 3385.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.400763ms | 1999994 | 0 | 47.70% | 1.17 | 1816.36 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.008043ms | 1999994 | 0 | 47.49% | 1.19 | 2177.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.110611ms | 1999994 | 0 | 47.78% | 1.17 | 1881.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 14.716566ms | 5364815 | 2000000 | 46.29% | 1.21 | 1036.84 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.519µs | 5508 | 0 | 44.89% | 1.24 | 2764.77 MB/s |
| Timsort | 1000 | 7.202µs | 5460 | 0 | 44.89% | 1.24 | 2118.69 MB/s |
| ARS Gen 1: Foundation | 1000 | 29.499µs | 4914 | 2000 | 44.89% | 1.24 | 517.26 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 39.341µs | 4914 | 2000 | 44.89% | 1.24 | 387.86 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 5.032µs | 5508 | 0 | 44.89% | 1.24 | 3032.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.686µs | 5508 | 0 | 44.89% | 1.24 | 2683.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 4.883µs | 5508 | 0 | 44.89% | 1.24 | 3124.88 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 8.074µs | 5460 | 0 | 44.89% | 1.24 | 1889.87 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.708µs | 5508 | 0 | 44.89% | 1.24 | 2673.23 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.666µs | 5460 | 0 | 44.89% | 1.24 | 1990.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5.561µs | 5508 | 0 | 44.89% | 1.24 | 2743.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 5.535µs | 5508 | 0 | 44.89% | 1.24 | 2756.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 5.425µs | 5508 | 0 | 44.89% | 1.24 | 2812.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 107.315µs | 5508 | 2000 | 44.87% | 1.24 | 142.19 MB/s |
| Quicksort | 10000 | 43.879µs | 53621 | 0 | 44.82% | 1.24 | 3477.47 MB/s |
| Timsort | 10000 | 49.673µs | 53742 | 0 | 44.82% | 1.24 | 3071.85 MB/s |
| ARS Gen 1: Foundation | 10000 | 228.35µs | 50132 | 30000 | 44.79% | 1.24 | 668.22 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 254.232µs | 50259 | 30000 | 44.79% | 1.24 | 600.19 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 223.71µs | 124917 | 14351 | 44.79% | 1.24 | 682.08 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 248.603µs | 52500 | 10000 | 44.77% | 1.24 | 613.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 170.994µs | 52500 | 0 | 44.77% | 1.24 | 892.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 175.181µs | 51829 | 0 | 44.76% | 1.24 | 871.03 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 367.525µs | 42054 | 0 | 44.75% | 1.24 | 415.18 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 380.696µs | 42636 | 0 | 44.75% | 1.24 | 400.81 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 185.44µs | 16860 | 0 | 44.73% | 1.24 | 822.84 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 159.376µs | 52500 | 0 | 44.77% | 1.24 | 957.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 160.091µs | 52500 | 0 | 44.77% | 1.24 | 953.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 295.686µs | 52500 | 20000 | 44.74% | 1.24 | 516.05 MB/s |
| Quicksort | 100000 | 356.542µs | 532062 | 0 | 44.68% | 1.24 | 4279.66 MB/s |
| Timsort | 100000 | 485.619µs | 535405 | 0 | 44.52% | 1.24 | 3142.13 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.588916ms | 506805 | 300000 | 44.61% | 1.24 | 960.33 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.826349ms | 506783 | 300000 | 44.62% | 1.24 | 835.48 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.569319ms | 1174310 | 108703 | 44.61% | 1.24 | 972.32 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.239144ms | 519466 | 100000 | 44.46% | 1.24 | 1231.40 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 779.356µs | 519466 | 0 | 44.47% | 1.24 | 1957.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 907.989µs | 520212 | 0 | 44.38% | 1.24 | 1680.50 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 947.993µs | 499545 | 0 | 44.49% | 1.24 | 1609.59 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.143182ms | 502501 | 0 | 44.36% | 1.24 | 1334.76 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.176952ms | 203055 | 0 | 44.12% | 1.23 | 1296.47 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 861.915µs | 182074 | 0 | 44.30% | 1.24 | 1770.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.415986ms | 197448 | 0 | 44.17% | 1.23 | 1077.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.187349ms | 519466 | 200000 | 44.47% | 1.24 | 1285.11 MB/s |
| Quicksort | 1000000 | 4.043224ms | 5301519 | 0 | 44.08% | 1.25 | 3773.92 MB/s |
| Timsort | 1000000 | 8.000967ms | 6302942 | 0 | 43.31% | 1.24 | 1907.12 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 20.545838ms | 12308876 | 1017407 | 44.57% | 1.26 | 742.67 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 13.59176ms | 5221477 | 1000000 | 45.06% | 1.21 | 1122.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.474855ms | 5221477 | 0 | 44.99% | 1.21 | 1456.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.421269ms | 6004244 | 0 | 44.43% | 1.21 | 1058.08 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.934837ms | 5265586 | 0 | 44.69% | 1.23 | 1923.01 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.526647ms | 6045570 | 0 | 44.16% | 1.23 | 1218.11 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.225988ms | 1938046 | 0 | 45.85% | 1.17 | 1002.15 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 22.015211ms | 2076365 | 0 | 46.62% | 1.20 | 693.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 26.220744ms | 2063926 | 0 | 46.78% | 1.18 | 581.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 15.894729ms | 9694969 | 2000000 | 44.74% | 1.22 | 959.99 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.106µs | 10296 | 0 | 44.65% | 1.31 | 1509.87 MB/s |
| Timsort | 1000 | 16.515µs | 10670 | 0 | 44.65% | 1.31 | 923.94 MB/s |
| ARS Gen 1: Foundation | 1000 | 151.978µs | 808 | 2000 | 44.65% | 1.31 | 100.40 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 188.402µs | 808 | 2000 | 44.65% | 1.31 | 80.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.123µs | 10296 | 0 | 44.65% | 1.31 | 1507.34 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.319µs | 10296 | 0 | 44.65% | 1.31 | 1478.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 9.997µs | 10296 | 0 | 44.65% | 1.31 | 1526.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 16.7µs | 10670 | 0 | 44.65% | 1.31 | 913.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 12.32µs | 10296 | 0 | 44.65% | 1.31 | 1238.54 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.215µs | 10670 | 0 | 44.65% | 1.31 | 837.70 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.099µs | 10296 | 0 | 44.65% | 1.31 | 1510.92 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.079µs | 10296 | 0 | 44.65% | 1.31 | 1513.92 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 9.973µs | 10296 | 0 | 44.65% | 1.31 | 1530.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 102.561µs | 10296 | 2000 | 44.64% | 1.31 | 148.78 MB/s |
| Quicksort | 10000 | 124.701µs | 134101 | 0 | 44.61% | 1.31 | 1223.63 MB/s |
| Timsort | 10000 | 187.915µs | 137729 | 0 | 44.61% | 1.31 | 812.00 MB/s |
| ARS Gen 1: Foundation | 10000 | 905.982µs | 84429 | 30000 | 44.56% | 1.31 | 168.42 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 968.511µs | 84430 | 30000 | 44.56% | 1.31 | 157.55 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 277.865µs | 190005 | 14351 | 44.58% | 1.31 | 549.14 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 208.776µs | 71389 | 10000 | 44.57% | 1.31 | 730.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 133.126µs | 71389 | 0 | 44.57% | 1.31 | 1146.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 147.716µs | 73990 | 0 | 44.57% | 1.31 | 1032.98 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 369.086µs | 60048 | 0 | 44.55% | 1.31 | 413.42 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 402.82µs | 62612 | 0 | 44.56% | 1.31 | 378.80 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 144.352µs | 71389 | 0 | 44.56% | 1.31 | 1057.05 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 148.603µs | 71389 | 0 | 44.56% | 1.31 | 1026.82 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 151.46µs | 71389 | 0 | 44.56% | 1.31 | 1007.45 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 324.055µs | 71389 | 20000 | 44.54% | 1.31 | 470.87 MB/s |
| Quicksort | 100000 | 1.172349ms | 1353942 | 0 | 44.43% | 1.32 | 1301.56 MB/s |
| Timsort | 100000 | 1.455706ms | 1358979 | 0 | 44.31% | 1.32 | 1048.21 MB/s |
| ARS Gen 1: Foundation | 100000 | 5.965019ms | 1260666 | 300000 | 44.16% | 1.30 | 255.80 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 6.203977ms | 1260598 | 300000 | 44.14% | 1.30 | 245.95 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.136457ms | 1555111 | 108703 | 44.36% | 1.31 | 714.21 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 831.28µs | 735888 | 100000 | 44.33% | 1.31 | 1835.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 600.543µs | 735888 | 0 | 44.34% | 1.31 | 2540.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 712.883µs | 741765 | 0 | 44.33% | 1.31 | 2140.43 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 870.359µs | 651349 | 0 | 44.34% | 1.31 | 1753.16 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 945.015µs | 657321 | 0 | 44.40% | 1.31 | 1614.66 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 788.448µs | 710308 | 0 | 44.34% | 1.31 | 1935.29 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 633.601µs | 631417 | 0 | 44.37% | 1.31 | 2408.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 734.659µs | 735888 | 0 | 44.38% | 1.31 | 2076.99 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.054721ms | 735888 | 200000 | 44.35% | 1.31 | 1446.71 MB/s |
| Quicksort | 1000000 | 9.21659ms | 12909957 | 0 | 43.59% | 1.35 | 1655.58 MB/s |
| Timsort | 1000000 | 13.73377ms | 14007926 | 0 | 42.42% | 1.34 | 1111.04 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 18.925147ms | 14286900 | 1017407 | 43.89% | 1.33 | 806.27 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.427954ms | 5157050 | 1000000 | 45.18% | 1.27 | 2054.24 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.111773ms | 5157050 | 0 | 45.19% | 1.28 | 2145.57 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.226014ms | 5175392 | 0 | 45.06% | 1.27 | 2111.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 5.748385ms | 6007232 | 0 | 44.77% | 1.31 | 2654.45 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.892347ms | 6044896 | 0 | 44.48% | 1.30 | 2589.59 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.856117ms | 2361022 | 0 | 44.56% | 1.27 | 1722.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.742608ms | 1866734 | 0 | 44.62% | 1.29 | 1566.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.415997ms | 2025491 | 0 | 44.85% | 1.26 | 1464.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.818637ms | 11791276 | 2000000 | 44.90% | 1.29 | 810.83 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 16.277µs | 10451 | 0 | 47.23% | 1.32 | 937.44 MB/s |
| Timsort | 1000 | 27.258µs | 10742 | 0 | 47.23% | 1.32 | 559.79 MB/s |
| ARS Gen 1: Foundation | 1000 | 143.832µs | 5331 | 2000 | 47.23% | 1.32 | 106.09 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 156.813µs | 5339 | 2000 | 47.23% | 1.32 | 97.31 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 14.895µs | 10451 | 0 | 47.23% | 1.32 | 1024.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 15.24µs | 10451 | 0 | 47.23% | 1.32 | 1001.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 14.482µs | 10451 | 0 | 47.23% | 1.32 | 1053.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 22.745µs | 10742 | 0 | 47.23% | 1.32 | 670.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 14.307µs | 10451 | 0 | 47.23% | 1.32 | 1066.53 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 22.36µs | 10742 | 0 | 47.23% | 1.32 | 682.41 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 13.686µs | 10451 | 0 | 47.23% | 1.32 | 1114.92 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 13.773µs | 10451 | 0 | 47.23% | 1.32 | 1107.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 14.306µs | 10451 | 0 | 47.23% | 1.32 | 1066.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 140.32µs | 10451 | 2000 | 47.22% | 1.32 | 108.74 MB/s |
| Quicksort | 10000 | 133.351µs | 111159 | 0 | 47.19% | 1.32 | 1144.26 MB/s |
| Timsort | 10000 | 290.386µs | 110728 | 0 | 47.19% | 1.32 | 525.47 MB/s |
| ARS Gen 1: Foundation | 10000 | 641.504µs | 75427 | 30000 | 47.17% | 1.32 | 237.86 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 741.712µs | 74701 | 30000 | 47.19% | 1.32 | 205.72 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 416.762µs | 163143 | 14351 | 47.17% | 1.32 | 366.13 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 336.456µs | 72583 | 10000 | 47.17% | 1.32 | 453.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 204.326µs | 72583 | 0 | 47.17% | 1.32 | 746.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 254.058µs | 72287 | 0 | 47.17% | 1.32 | 600.60 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 499.182µs | 63448 | 0 | 47.15% | 1.32 | 305.68 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 441.036µs | 63348 | 0 | 47.15% | 1.32 | 345.98 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 298.516µs | 72583 | 0 | 47.17% | 1.32 | 511.15 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 249.71µs | 72583 | 0 | 47.16% | 1.32 | 611.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 222.097µs | 72583 | 0 | 47.16% | 1.32 | 687.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 464.804µs | 72583 | 20000 | 47.16% | 1.32 | 328.28 MB/s |
| Quicksort | 100000 | 1.01332ms | 1016581 | 0 | 47.09% | 1.32 | 1505.82 MB/s |
| Timsort | 100000 | 1.222238ms | 1021185 | 0 | 46.99% | 1.32 | 1248.43 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.682605ms | 680916 | 300000 | 47.11% | 1.32 | 568.80 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.27077ms | 680031 | 300000 | 47.11% | 1.32 | 466.52 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.333006ms | 1237724 | 108703 | 47.06% | 1.32 | 654.04 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.281117ms | 631252 | 100000 | 47.08% | 1.32 | 1191.05 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 987.98µs | 631252 | 0 | 47.07% | 1.32 | 1544.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 784.176µs | 634097 | 0 | 47.02% | 1.32 | 1945.84 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 818.14µs | 555626 | 0 | 47.06% | 1.32 | 1865.06 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.13347ms | 562372 | 0 | 47.07% | 1.32 | 1346.20 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.209422ms | 134521 | 0 | 46.92% | 1.32 | 1261.66 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.100808ms | 169903 | 0 | 47.00% | 1.32 | 1386.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.103059ms | 264519 | 0 | 47.03% | 1.32 | 1383.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.356627ms | 631252 | 200000 | 47.04% | 1.32 | 1124.76 MB/s |
| Quicksort | 1000000 | 10.876063ms | 9921218 | 0 | 46.56% | 1.34 | 1402.97 MB/s |
| Timsort | 1000000 | 19.216002ms | 11000160 | 0 | 45.88% | 1.33 | 794.07 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 25.72052ms | 12348632 | 1017407 | 46.66% | 1.33 | 593.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 7.724102ms | 5346522 | 1000000 | 47.53% | 1.29 | 1975.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.86741ms | 5346522 | 0 | 47.49% | 1.29 | 1939.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 12.273277ms | 5363683 | 0 | 47.22% | 1.30 | 1243.25 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.988082ms | 5434749 | 0 | 47.26% | 1.32 | 2183.54 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.842079ms | 5451863 | 0 | 46.78% | 1.31 | 1945.76 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 16.00247ms | 1070102 | 0 | 47.70% | 1.28 | 953.53 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 17.366144ms | 1041286 | 0 | 48.17% | 1.29 | 878.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 14.316487ms | 1013215 | 0 | 48.09% | 1.27 | 1065.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.251823ms | 11100986 | 2000000 | 48.19% | 1.31 | 604.26 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 13.694µs | 10179 | 0 | 49.77% | 1.32 | 1114.27 MB/s |
| Timsort | 1000 | 21.608µs | 10913 | 0 | 49.77% | 1.32 | 706.16 MB/s |
| ARS Gen 1: Foundation | 1000 | 274.34µs | 0 | 2000 | 49.77% | 1.32 | 55.62 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 315.338µs | 0 | 2000 | 49.77% | 1.32 | 48.39 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 12.996µs | 10179 | 0 | 49.77% | 1.32 | 1174.11 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 12.921µs | 10179 | 0 | 49.77% | 1.32 | 1180.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 14.06µs | 10179 | 0 | 49.77% | 1.32 | 1085.26 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 21.394µs | 10913 | 0 | 49.77% | 1.32 | 713.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 13.32µs | 10179 | 0 | 49.77% | 1.32 | 1145.55 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 21.301µs | 10913 | 0 | 49.77% | 1.32 | 716.34 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 13.007µs | 10179 | 0 | 49.77% | 1.32 | 1173.12 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 13.678µs | 10179 | 0 | 49.77% | 1.32 | 1115.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 13.161µs | 10179 | 0 | 49.77% | 1.32 | 1159.39 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 132.929µs | 10179 | 2000 | 49.76% | 1.32 | 114.79 MB/s |
| Quicksort | 10000 | 162.83µs | 137738 | 0 | 49.74% | 1.32 | 937.10 MB/s |
| Timsort | 10000 | 244.878µs | 141392 | 0 | 49.74% | 1.32 | 623.12 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.48583ms | 0 | 30000 | 49.42% | 1.33 | 23.53 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 7.073816ms | 0 | 30000 | 49.35% | 1.33 | 21.57 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 409.454µs | 193231 | 14351 | 49.73% | 1.32 | 372.66 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 319.158µs | 51645 | 10000 | 49.71% | 1.32 | 478.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 173.323µs | 51645 | 0 | 49.71% | 1.32 | 880.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 207.338µs | 57426 | 0 | 49.71% | 1.32 | 735.94 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 433.008µs | 59080 | 0 | 49.70% | 1.32 | 352.39 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 433.36µs | 61965 | 0 | 49.70% | 1.32 | 352.10 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 189.134µs | 51645 | 0 | 49.70% | 1.32 | 806.77 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 184.26µs | 51645 | 0 | 49.70% | 1.32 | 828.11 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 172.19µs | 51645 | 0 | 49.71% | 1.32 | 886.16 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 393.536µs | 51645 | 20000 | 49.70% | 1.32 | 387.74 MB/s |
| Quicksort | 100000 | 1.979888ms | 1704558 | 0 | 49.62% | 1.33 | 770.69 MB/s |
| Timsort | 100000 | 3.017585ms | 1748721 | 0 | 49.55% | 1.33 | 505.66 MB/s |
| ARS Gen 1: Foundation | 100000 | 44.474655ms | 6 | 300000 | 38.58% | 1.27 | 34.31 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.232818ms | 6 | 300000 | 38.30% | 1.26 | 34.50 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.445979ms | 1886207 | 108703 | 49.58% | 1.32 | 623.83 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.01003ms | 879882 | 100000 | 49.57% | 1.32 | 1510.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 786.863µs | 879882 | 0 | 49.58% | 1.32 | 1939.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 886.202µs | 922129 | 0 | 49.58% | 1.32 | 1721.82 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 763.81µs | 954423 | 0 | 49.53% | 1.32 | 1997.72 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 950.676µs | 993675 | 0 | 49.56% | 1.32 | 1605.05 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 697.522µs | 879882 | 0 | 49.54% | 1.32 | 2187.57 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 715.609µs | 773088 | 0 | 49.54% | 1.32 | 2132.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 778.221µs | 879882 | 0 | 49.56% | 1.32 | 1960.73 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.14954ms | 879882 | 200000 | 49.55% | 1.32 | 1327.38 MB/s |
| Quicksort | 1000000 | 17.394729ms | 20437271 | 0 | 48.79% | 1.36 | 877.21 MB/s |
| Timsort | 1000000 | 27.682991ms | 20799465 | 0 | 47.87% | 1.35 | 551.20 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 22.659057ms | 21505010 | 1017407 | 49.04% | 1.33 | 673.41 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.407025ms | 10221412 | 1000000 | 49.74% | 1.30 | 1622.06 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.517932ms | 10221412 | 0 | 49.72% | 1.29 | 1791.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.425054ms | 10628930 | 0 | 49.75% | 1.29 | 1618.96 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.380374ms | 12929332 | 0 | 49.51% | 1.31 | 2067.48 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.846934ms | 13335182 | 0 | 49.36% | 1.31 | 1724.75 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.774689ms | 10221412 | 0 | 49.60% | 1.30 | 1738.95 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.882592ms | 11275443 | 0 | 49.83% | 1.31 | 1544.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.592677ms | 12322876 | 0 | 49.81% | 1.30 | 1590.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.776243ms | 13615043 | 2000000 | 49.44% | 1.30 | 591.97 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.411µs | 5504 | 0 | 50.13% | 1.28 | 2819.96 MB/s |
| Timsort | 1000 | 7.704µs | 5497 | 0 | 50.13% | 1.28 | 1980.63 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.786µs | 984 | 2000 | 50.13% | 1.28 | 319.32 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 60.889µs | 984 | 2000 | 50.13% | 1.28 | 250.60 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 5.267µs | 5504 | 0 | 50.13% | 1.28 | 2897.06 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.228µs | 5504 | 0 | 50.13% | 1.28 | 2918.67 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.189µs | 5504 | 0 | 50.13% | 1.28 | 2940.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 7.337µs | 5497 | 0 | 50.13% | 1.28 | 2079.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.448µs | 5504 | 0 | 50.13% | 1.28 | 2800.81 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 7.456µs | 5497 | 0 | 50.13% | 1.28 | 2046.51 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5.18µs | 5504 | 0 | 50.13% | 1.28 | 2945.71 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 5.267µs | 5504 | 0 | 50.13% | 1.28 | 2897.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 5.159µs | 5504 | 0 | 50.13% | 1.28 | 2957.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 113.197µs | 5504 | 2000 | 50.13% | 1.28 | 134.80 MB/s |
| Quicksort | 10000 | 45.476µs | 53753 | 0 | 50.10% | 1.28 | 3355.35 MB/s |
| Timsort | 10000 | 56.204µs | 54514 | 0 | 50.10% | 1.28 | 2714.89 MB/s |
| ARS Gen 1: Foundation | 10000 | 253.067µs | 9984 | 30000 | 50.09% | 1.28 | 602.95 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 332.699µs | 9984 | 30000 | 50.09% | 1.28 | 458.64 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 261.541µs | 121806 | 14351 | 50.09% | 1.28 | 583.42 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 203.608µs | 12063 | 10000 | 50.08% | 1.28 | 749.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 121.17µs | 12063 | 0 | 50.08% | 1.28 | 1259.29 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 122.097µs | 12087 | 0 | 50.08% | 1.28 | 1249.73 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 288.794µs | 12063 | 0 | 50.07% | 1.28 | 528.36 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 292.11µs | 12087 | 0 | 50.07% | 1.28 | 522.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 127.979µs | 12063 | 0 | 50.08% | 1.28 | 1192.29 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 124.41µs | 12063 | 0 | 50.08% | 1.28 | 1226.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 125.683µs | 12063 | 0 | 50.08% | 1.28 | 1214.07 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 296.529µs | 12063 | 20000 | 50.06% | 1.28 | 514.58 MB/s |
| Quicksort | 100000 | 424.973µs | 522910 | 0 | 49.99% | 1.28 | 3590.53 MB/s |
| Timsort | 100000 | 519.698µs | 516617 | 0 | 49.91% | 1.28 | 2936.09 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.306623ms | 99984 | 300000 | 49.99% | 1.28 | 1167.80 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.523407ms | 99984 | 300000 | 49.98% | 1.28 | 1001.62 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 1.753588ms | 1144941 | 108703 | 49.97% | 1.28 | 870.15 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 527.215µs | 144579 | 100000 | 49.92% | 1.28 | 2894.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 320.902µs | 144579 | 0 | 49.92% | 1.28 | 4754.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 325.37µs | 145223 | 0 | 49.92% | 1.28 | 4689.67 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 588.942µs | 99988 | 0 | 49.94% | 1.28 | 2590.88 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 575.587µs | 99988 | 0 | 49.93% | 1.28 | 2651.00 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 450.186µs | 199988 | 0 | 49.90% | 1.28 | 3389.44 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 542.56µs | 199972 | 0 | 49.92% | 1.28 | 2812.37 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 380.421µs | 100004 | 0 | 49.91% | 1.28 | 4011.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 783.919µs | 144579 | 200000 | 49.90% | 1.28 | 1946.48 MB/s |
| Quicksort | 1000000 | 4.454305ms | 5201420 | 0 | 49.44% | 1.29 | 3425.63 MB/s |
| Timsort | 1000000 | 8.412757ms | 6174589 | 0 | 48.90% | 1.29 | 1813.77 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 19.939197ms | 12089713 | 1017407 | 49.80% | 1.29 | 765.27 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.416263ms | 999990 | 1000000 | 50.30% | 1.28 | 2378.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.392396ms | 999990 | 0 | 50.30% | 1.25 | 2387.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.555138ms | 999990 | 0 | 50.31% | 1.25 | 2327.76 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.289596ms | 999990 | 0 | 50.08% | 1.27 | 3557.16 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.359918ms | 999990 | 0 | 50.10% | 1.27 | 3499.79 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 7.799828ms | 1999974 | 0 | 50.23% | 1.25 | 1956.30 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 6.757925ms | 1999974 | 0 | 50.16% | 1.27 | 2257.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 7.705156ms | 1999984 | 0 | 50.20% | 1.25 | 1980.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 15.875652ms | 5706274 | 2000000 | 49.14% | 1.28 | 961.14 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.104µs | 10179 | 0 | 49.65% | 1.28 | 1374.17 MB/s |
| Timsort | 1000 | 18.095µs | 10913 | 0 | 49.65% | 1.28 | 843.26 MB/s |
| ARS Gen 1: Foundation | 1000 | 232.537µs | 0 | 2000 | 49.64% | 1.28 | 65.62 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 268.566µs | 0 | 2000 | 49.64% | 1.28 | 56.82 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 11.18µs | 10179 | 0 | 49.65% | 1.28 | 1364.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 11.077µs | 10179 | 0 | 49.65% | 1.28 | 1377.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 11.118µs | 10179 | 0 | 49.65% | 1.28 | 1372.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 17.905µs | 10913 | 0 | 49.65% | 1.28 | 852.21 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 11.253µs | 10179 | 0 | 49.65% | 1.28 | 1355.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 18.069µs | 10913 | 0 | 49.65% | 1.28 | 844.47 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.961µs | 10179 | 0 | 49.65% | 1.28 | 1392.10 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 11.025µs | 10179 | 0 | 49.65% | 1.28 | 1384.02 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 11.144µs | 10179 | 0 | 49.65% | 1.28 | 1369.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 108.22µs | 10179 | 2000 | 49.64% | 1.28 | 141.00 MB/s |
| Quicksort | 10000 | 136.366µs | 137738 | 0 | 49.62% | 1.28 | 1118.96 MB/s |
| Timsort | 10000 | 207.901µs | 141392 | 0 | 49.62% | 1.28 | 733.94 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.299825ms | 0 | 30000 | 49.32% | 1.29 | 28.79 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.594913ms | 0 | 30000 | 49.33% | 1.29 | 27.27 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 321.943µs | 193231 | 14351 | 49.60% | 1.28 | 473.96 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 228.978µs | 51645 | 10000 | 49.59% | 1.28 | 666.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 142.558µs | 51645 | 0 | 49.59% | 1.28 | 1070.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 158.094µs | 57426 | 0 | 49.59% | 1.28 | 965.17 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 316.481µs | 59080 | 0 | 49.58% | 1.28 | 482.14 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 324.75µs | 61965 | 0 | 49.58% | 1.28 | 469.86 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 151.443µs | 51645 | 0 | 49.59% | 1.28 | 1007.56 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 149.456µs | 51645 | 0 | 49.58% | 1.28 | 1020.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 143.205µs | 51645 | 0 | 49.59% | 1.28 | 1065.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 307.248µs | 51645 | 20000 | 49.57% | 1.28 | 496.63 MB/s |
| Quicksort | 100000 | 1.638031ms | 1704558 | 0 | 49.48% | 1.28 | 931.53 MB/s |
| Timsort | 100000 | 2.226881ms | 1748721 | 0 | 49.41% | 1.28 | 685.21 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.055008ms | 6 | 300000 | 40.11% | 1.23 | 35.44 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.219035ms | 6 | 300000 | 40.20% | 1.24 | 33.01 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.440739ms | 1886207 | 108703 | 49.47% | 1.28 | 625.17 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 975.361µs | 879882 | 100000 | 49.44% | 1.28 | 1564.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 794.944µs | 879882 | 0 | 49.44% | 1.28 | 1919.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 710.141µs | 922129 | 0 | 49.42% | 1.28 | 2148.70 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 849.052µs | 954423 | 0 | 49.43% | 1.28 | 1797.16 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 976.512µs | 993675 | 0 | 49.43% | 1.28 | 1562.58 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 691.494µs | 879882 | 0 | 49.43% | 1.28 | 2206.64 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 719.26µs | 773088 | 0 | 49.43% | 1.28 | 2121.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 667.736µs | 879882 | 0 | 49.41% | 1.28 | 2285.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.006788ms | 879882 | 200000 | 49.43% | 1.28 | 1515.59 MB/s |
| Quicksort | 1000000 | 16.927258ms | 20437271 | 0 | 48.84% | 1.31 | 901.43 MB/s |
| Timsort | 1000000 | 27.224054ms | 20799465 | 0 | 48.20% | 1.30 | 560.49 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 22.906579ms | 21505010 | 1017407 | 49.02% | 1.29 | 666.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 9.263697ms | 10221412 | 1000000 | 49.57% | 1.26 | 1647.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 8.648561ms | 10221412 | 0 | 49.57% | 1.26 | 1764.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 9.49115ms | 10628930 | 0 | 49.58% | 1.26 | 1607.69 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.730952ms | 12929332 | 0 | 49.41% | 1.27 | 1973.73 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.929802ms | 13335182 | 0 | 49.28% | 1.27 | 1924.23 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.511502ms | 10221412 | 0 | 49.44% | 1.27 | 1792.73 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 9.25977ms | 11275443 | 0 | 49.62% | 1.27 | 1647.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.306971ms | 12322876 | 0 | 49.61% | 1.26 | 1639.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.195171ms | 13615045 | 2000000 | 49.75% | 1.25 | 605.62 MB/s |

## Category: f64

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.048µs | 10325 | 0 | 49.72% | 1.25 | 845.46 MB/s |
| Timsort | 1000 | 26.273µs | 10521 | 0 | 49.72% | 1.25 | 580.78 MB/s |
| ARS Gen 1: Foundation | 1000 | 233.162µs | 0 | 2000 | 49.72% | 1.25 | 65.44 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 280.145µs | 0 | 2000 | 49.72% | 1.25 | 54.47 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.467µs | 10325 | 0 | 49.72% | 1.25 | 873.58 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.365µs | 10325 | 0 | 49.72% | 1.25 | 878.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.412µs | 10325 | 0 | 49.72% | 1.25 | 876.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 26.369µs | 10521 | 0 | 49.72% | 1.25 | 578.66 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.216µs | 10325 | 0 | 49.72% | 1.25 | 837.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 26.176µs | 10521 | 0 | 49.72% | 1.25 | 582.93 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.135µs | 10325 | 0 | 49.72% | 1.25 | 841.40 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.381µs | 10325 | 0 | 49.72% | 1.25 | 877.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.197µs | 10325 | 0 | 49.72% | 1.25 | 887.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 125.206µs | 10325 | 2000 | 49.72% | 1.25 | 121.87 MB/s |
| Quicksort | 10000 | 225.695µs | 136464 | 0 | 49.70% | 1.25 | 676.08 MB/s |
| Timsort | 10000 | 320.378µs | 141512 | 0 | 49.69% | 1.25 | 476.27 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.410014ms | 0 | 30000 | 49.44% | 1.26 | 28.20 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.637645ms | 0 | 30000 | 49.45% | 1.26 | 27.07 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 400.234µs | 193135 | 14351 | 49.68% | 1.25 | 381.25 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 241.395µs | 73138 | 10000 | 49.68% | 1.25 | 632.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 159.438µs | 73138 | 0 | 49.67% | 1.25 | 957.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 174.669µs | 76380 | 0 | 49.67% | 1.25 | 873.58 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 309.525µs | 62698 | 0 | 49.67% | 1.25 | 492.97 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 304.127µs | 65867 | 0 | 49.66% | 1.25 | 501.72 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 168.999µs | 73138 | 0 | 49.67% | 1.25 | 902.89 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 163.057µs | 73138 | 0 | 49.67% | 1.25 | 935.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 164.928µs | 73138 | 0 | 49.67% | 1.25 | 925.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 340.076µs | 73138 | 20000 | 49.66% | 1.25 | 448.69 MB/s |
| Quicksort | 100000 | 2.621823ms | 1705718 | 0 | 49.59% | 1.25 | 581.99 MB/s |
| Timsort | 100000 | 3.33172ms | 1751732 | 0 | 49.53% | 1.25 | 457.99 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.454079ms | 0 | 300000 | 41.35% | 1.22 | 36.81 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.531112ms | 0 | 300000 | 42.30% | 1.22 | 32.79 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.179938ms | 1884272 | 108703 | 49.57% | 1.25 | 479.85 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.290841ms | 1101865 | 100000 | 49.58% | 1.25 | 1182.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 901.378µs | 1101865 | 0 | 49.56% | 1.25 | 1692.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.16281ms | 1142841 | 0 | 49.58% | 1.25 | 1312.23 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.018201ms | 1002379 | 0 | 49.57% | 1.25 | 1498.60 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.118684ms | 1045724 | 0 | 49.55% | 1.25 | 1363.99 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 941.679µs | 1101865 | 0 | 49.55% | 1.25 | 1620.38 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 920.833µs | 999614 | 0 | 49.57% | 1.25 | 1657.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 914.999µs | 1101865 | 0 | 49.53% | 1.25 | 1667.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.614665ms | 1101865 | 200000 | 49.58% | 1.25 | 945.01 MB/s |
| Quicksort | 1000000 | 27.376793ms | 20430901 | 0 | 49.13% | 1.28 | 557.36 MB/s |
| Timsort | 1000000 | 39.232035ms | 20822215 | 0 | 48.59% | 1.27 | 388.94 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 29.375956ms | 21498086 | 1017407 | 49.24% | 1.26 | 519.43 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.491437ms | 12665814 | 1000000 | 49.87% | 1.25 | 1327.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.319436ms | 12665814 | 0 | 49.86% | 1.25 | 1478.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.915262ms | 13081361 | 0 | 49.79% | 1.24 | 1280.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.217824ms | 13583765 | 0 | 49.65% | 1.25 | 1655.36 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.317991ms | 14002566 | 0 | 49.51% | 1.25 | 1348.19 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.684621ms | 6406252 | 0 | 49.43% | 1.24 | 1428.11 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.584155ms | 5861815 | 0 | 49.41% | 1.24 | 1317.21 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.21044ms | 7398340 | 0 | 49.48% | 1.24 | 1361.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.726487ms | 14470210 | 2000000 | 49.83% | 1.24 | 617.10 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 20.086µs | 10345 | 0 | 49.53% | 1.23 | 759.67 MB/s |
| Timsort | 1000 | 28.703µs | 10685 | 0 | 49.53% | 1.23 | 531.61 MB/s |
| ARS Gen 1: Foundation | 1000 | 255.294µs | 0 | 2000 | 49.53% | 1.23 | 59.77 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 296.027µs | 0 | 2000 | 49.53% | 1.23 | 51.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 18.34µs | 10345 | 0 | 49.53% | 1.23 | 832.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 18.523µs | 10345 | 0 | 49.53% | 1.23 | 823.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 18.371µs | 10345 | 0 | 49.53% | 1.23 | 830.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.757µs | 10685 | 0 | 49.53% | 1.23 | 549.73 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 19.272µs | 10345 | 0 | 49.53% | 1.23 | 791.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 28.095µs | 10685 | 0 | 49.53% | 1.23 | 543.11 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.64µs | 10345 | 0 | 49.53% | 1.23 | 776.92 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 18.393µs | 10345 | 0 | 49.53% | 1.23 | 829.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 18.524µs | 10345 | 0 | 49.53% | 1.23 | 823.73 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 132.229µs | 10345 | 2000 | 49.53% | 1.23 | 115.40 MB/s |
| Quicksort | 10000 | 239.685µs | 137462 | 0 | 49.51% | 1.23 | 636.62 MB/s |
| Timsort | 10000 | 333.687µs | 141011 | 0 | 49.51% | 1.23 | 457.28 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.578967ms | 0 | 30000 | 49.27% | 1.23 | 27.35 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.739359ms | 0 | 30000 | 49.28% | 1.23 | 26.59 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 396.79µs | 192671 | 14351 | 49.50% | 1.23 | 384.56 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 369.333µs | 125399 | 10000 | 49.49% | 1.23 | 413.14 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 277.171µs | 125399 | 0 | 49.49% | 1.23 | 550.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 314.449µs | 130052 | 0 | 49.49% | 1.23 | 485.25 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 406.155µs | 109718 | 0 | 49.49% | 1.23 | 375.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 447.623µs | 113881 | 0 | 49.48% | 1.23 | 340.88 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 244.377µs | 48812 | 0 | 49.47% | 1.23 | 624.40 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 277.164µs | 125399 | 0 | 49.49% | 1.23 | 550.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 296.803µs | 125399 | 0 | 49.49% | 1.23 | 514.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 451.274µs | 125399 | 20000 | 49.48% | 1.23 | 338.13 MB/s |
| Quicksort | 100000 | 2.600174ms | 1710455 | 0 | 49.43% | 1.23 | 586.84 MB/s |
| Timsort | 100000 | 3.397215ms | 1746462 | 0 | 49.37% | 1.23 | 449.16 MB/s |
| ARS Gen 1: Foundation | 100000 | 43.368342ms | 0 | 300000 | 41.75% | 1.21 | 35.18 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.819678ms | 0 | 300000 | 42.46% | 1.21 | 34.04 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.957571ms | 1884751 | 108703 | 49.42% | 1.23 | 515.92 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.220699ms | 1586392 | 100000 | 49.37% | 1.23 | 687.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.546701ms | 1586392 | 0 | 49.37% | 1.23 | 986.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.133759ms | 1629438 | 0 | 49.36% | 1.23 | 715.11 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.610541ms | 1447738 | 0 | 49.41% | 1.23 | 947.43 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.809784ms | 1487078 | 0 | 49.39% | 1.23 | 843.13 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.054956ms | 834504 | 0 | 49.30% | 1.23 | 1446.39 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.145348ms | 657220 | 0 | 49.28% | 1.23 | 1332.24 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.159006ms | 834504 | 0 | 49.30% | 1.23 | 1316.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.094036ms | 1586392 | 200000 | 49.38% | 1.23 | 728.68 MB/s |
| Quicksort | 1000000 | 26.879289ms | 20420624 | 0 | 49.04% | 1.26 | 567.68 MB/s |
| Timsort | 1000000 | 38.200355ms | 20810565 | 0 | 48.52% | 1.25 | 399.44 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 30.066506ms | 21491076 | 1017407 | 49.17% | 1.24 | 507.50 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 21.506635ms | 17729670 | 1000000 | 49.51% | 1.24 | 709.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.363059ms | 17729670 | 0 | 49.53% | 1.23 | 830.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 23.442713ms | 18126422 | 0 | 48.98% | 1.23 | 650.90 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 13.706655ms | 17798278 | 0 | 49.34% | 1.24 | 1113.24 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.52857ms | 18171061 | 0 | 48.99% | 1.23 | 743.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 14.614231ms | 9157468 | 0 | 49.49% | 1.21 | 1044.10 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.557091ms | 9620349 | 0 | 49.80% | 1.22 | 980.83 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 15.474848ms | 11727786 | 0 | 49.89% | 1.21 | 986.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.269664ms | 19223484 | 2000000 | 49.60% | 1.23 | 521.32 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.126µs | 9762 | 0 | 49.33% | 1.23 | 890.97 MB/s |
| Timsort | 1000 | 21.981µs | 9882 | 0 | 49.33% | 1.23 | 694.18 MB/s |
| ARS Gen 1: Foundation | 1000 | 116.708µs | 0 | 2000 | 49.33% | 1.23 | 130.74 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 152.84µs | 0 | 2000 | 49.33% | 1.23 | 99.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.16µs | 9762 | 0 | 49.33% | 1.23 | 944.23 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 16.36µs | 9762 | 0 | 49.33% | 1.23 | 932.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.364µs | 9762 | 0 | 49.33% | 1.23 | 932.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 21.947µs | 9882 | 0 | 49.33% | 1.23 | 695.26 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.046µs | 9762 | 0 | 49.33% | 1.23 | 895.15 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 22.16µs | 9882 | 0 | 49.33% | 1.23 | 688.57 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.184µs | 9762 | 0 | 49.33% | 1.23 | 887.96 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 16.247µs | 9762 | 0 | 49.33% | 1.23 | 939.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.317µs | 9762 | 0 | 49.33% | 1.23 | 935.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 128.081µs | 9762 | 2000 | 49.33% | 1.23 | 119.13 MB/s |
| Quicksort | 10000 | 215.977µs | 134689 | 0 | 49.31% | 1.23 | 706.50 MB/s |
| Timsort | 10000 | 273.858µs | 132195 | 0 | 49.31% | 1.23 | 557.18 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.19275ms | 0 | 30000 | 49.21% | 1.23 | 69.59 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.39619ms | 0 | 30000 | 49.18% | 1.23 | 63.68 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 386.208µs | 187157 | 14351 | 49.29% | 1.23 | 395.09 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 454.801µs | 129133 | 10000 | 49.29% | 1.23 | 335.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 347.82µs | 129133 | 0 | 49.29% | 1.23 | 438.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 408.454µs | 124389 | 0 | 49.29% | 1.23 | 373.57 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 330.334µs | 112273 | 0 | 49.28% | 1.23 | 461.92 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 366.239µs | 109531 | 0 | 49.28% | 1.23 | 416.63 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 228.079µs | 51743 | 0 | 49.27% | 1.23 | 669.01 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 374.883µs | 129133 | 0 | 49.29% | 1.23 | 407.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 376.655µs | 129133 | 0 | 49.29% | 1.23 | 405.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 548.76µs | 129133 | 20000 | 49.28% | 1.23 | 278.06 MB/s |
| Quicksort | 100000 | 2.527767ms | 1716043 | 0 | 49.23% | 1.23 | 603.65 MB/s |
| Timsort | 100000 | 3.049174ms | 1660908 | 0 | 49.17% | 1.23 | 500.42 MB/s |
| ARS Gen 1: Foundation | 100000 | 18.465816ms | 0 | 300000 | 47.31% | 1.23 | 82.63 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 21.310475ms | 0 | 300000 | 46.96% | 1.23 | 71.60 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.888055ms | 1830188 | 108703 | 49.22% | 1.23 | 528.34 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.502778ms | 1653890 | 100000 | 49.19% | 1.23 | 435.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.69596ms | 1653890 | 0 | 49.16% | 1.23 | 565.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.13282ms | 1589383 | 0 | 49.13% | 1.23 | 487.06 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.614637ms | 1472393 | 0 | 49.19% | 1.23 | 945.03 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.982284ms | 1387582 | 0 | 49.16% | 1.23 | 769.76 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.124407ms | 815713 | 0 | 49.12% | 1.23 | 1357.05 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.069194ms | 631229 | 0 | 49.11% | 1.23 | 1427.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.236149ms | 815713 | 0 | 49.11% | 1.23 | 1234.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.542331ms | 1653890 | 200000 | 49.18% | 1.23 | 430.76 MB/s |
| Quicksort | 1000000 | 29.555074ms | 20672771 | 0 | 48.91% | 1.26 | 516.28 MB/s |
| Timsort | 1000000 | 38.807618ms | 19775927 | 0 | 48.25% | 1.25 | 393.19 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 25.61926ms | 20984698 | 1017407 | 49.19% | 1.24 | 595.60 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 21.286584ms | 18442598 | 1000000 | 49.31% | 1.24 | 716.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.396177ms | 18442598 | 0 | 49.32% | 1.24 | 829.45 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 23.180173ms | 17501336 | 0 | 48.97% | 1.23 | 658.27 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.871643ms | 18449113 | 0 | 49.16% | 1.25 | 904.40 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.998253ms | 17575612 | 0 | 48.71% | 1.24 | 693.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 14.85504ms | 8914015 | 0 | 49.53% | 1.22 | 1027.18 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.311842ms | 9611874 | 0 | 49.65% | 1.22 | 833.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.108734ms | 11855374 | 0 | 49.72% | 1.21 | 842.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.626304ms | 16617919 | 2000000 | 49.47% | 1.23 | 619.61 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 5.862µs | 3735 | 0 | 49.23% | 1.22 | 2603.00 MB/s |
| Timsort | 1000 | 8.25µs | 3747 | 0 | 49.23% | 1.22 | 1849.55 MB/s |
| ARS Gen 1: Foundation | 1000 | 37.574µs | 995 | 2000 | 49.23% | 1.22 | 406.10 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 34.709µs | 995 | 2000 | 49.23% | 1.22 | 439.62 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 5.739µs | 3735 | 0 | 49.23% | 1.22 | 2658.79 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 5.732µs | 3735 | 0 | 49.23% | 1.22 | 2662.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 5.857µs | 3735 | 0 | 49.23% | 1.22 | 2605.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 8.1µs | 3747 | 0 | 49.23% | 1.22 | 1883.80 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 5.856µs | 3735 | 0 | 49.23% | 1.22 | 2605.67 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 8.36µs | 3747 | 0 | 49.23% | 1.22 | 1825.21 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 5.784µs | 3735 | 0 | 49.23% | 1.22 | 2638.10 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 5.778µs | 3735 | 0 | 49.23% | 1.22 | 2640.84 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 5.562µs | 3735 | 0 | 49.23% | 1.22 | 2743.40 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 120.354µs | 3735 | 2000 | 49.22% | 1.22 | 126.78 MB/s |
| Quicksort | 10000 | 49.885µs | 36573 | 0 | 49.21% | 1.22 | 3058.79 MB/s |
| Timsort | 10000 | 70.803µs | 36775 | 0 | 49.20% | 1.22 | 2155.10 MB/s |
| ARS Gen 1: Foundation | 10000 | 231.843µs | 9995 | 30000 | 49.20% | 1.22 | 658.15 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 297.048µs | 9995 | 30000 | 49.20% | 1.22 | 513.68 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 324.073µs | 115988 | 14351 | 49.20% | 1.22 | 470.84 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 231.359µs | 9999 | 10000 | 49.19% | 1.22 | 659.53 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 137.792µs | 9999 | 0 | 49.19% | 1.22 | 1107.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.084µs | 9999 | 0 | 49.19% | 1.22 | 953.17 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 328.175µs | 9999 | 0 | 49.19% | 1.22 | 464.96 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 392.495µs | 9999 | 0 | 49.18% | 1.22 | 388.76 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 137.548µs | 9999 | 0 | 49.19% | 1.22 | 1109.34 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 135.995µs | 9999 | 0 | 49.19% | 1.22 | 1122.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 139.678µs | 9999 | 0 | 49.19% | 1.22 | 1092.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 355.469µs | 9999 | 20000 | 49.18% | 1.22 | 429.26 MB/s |
| Quicksort | 100000 | 492.596µs | 362094 | 0 | 49.13% | 1.22 | 3097.63 MB/s |
| Timsort | 100000 | 728.027µs | 382517 | 0 | 49.08% | 1.22 | 2095.91 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.306717ms | 99995 | 300000 | 49.13% | 1.22 | 1167.72 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.333026ms | 99995 | 300000 | 49.13% | 1.22 | 1144.67 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.401832ms | 1129938 | 108703 | 49.12% | 1.22 | 635.30 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 557.554µs | 100001 | 100000 | 49.07% | 1.22 | 2736.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 396.471µs | 100001 | 0 | 49.07% | 1.22 | 3848.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 393.001µs | 100001 | 0 | 49.07% | 1.22 | 3882.63 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 656.75µs | 100001 | 0 | 49.09% | 1.22 | 2323.38 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 689.946µs | 100001 | 0 | 49.08% | 1.22 | 2211.59 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 572.536µs | 199996 | 0 | 49.05% | 1.22 | 2665.12 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 498.825µs | 199996 | 0 | 49.07% | 1.22 | 3058.95 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 627.983µs | 199996 | 0 | 49.06% | 1.22 | 2429.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 845.767µs | 100001 | 200000 | 49.07% | 1.22 | 1804.14 MB/s |
| Quicksort | 1000000 | 4.204996ms | 3809528 | 0 | 48.90% | 1.23 | 3628.73 MB/s |
| Timsort | 1000000 | 7.811777ms | 4510660 | 0 | 48.77% | 1.22 | 1953.31 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.247244ms | 12062959 | 1017407 | 49.05% | 1.23 | 629.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.408236ms | 999999 | 1000000 | 49.39% | 1.21 | 2381.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.392736ms | 999999 | 0 | 49.38% | 1.21 | 2386.89 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.33793ms | 999999 | 0 | 49.38% | 1.21 | 2407.54 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.746234ms | 999999 | 0 | 49.20% | 1.22 | 3214.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.889136ms | 999999 | 0 | 49.21% | 1.22 | 3120.96 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.511965ms | 1999994 | 0 | 49.42% | 1.20 | 1792.63 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 7.24735ms | 1999994 | 0 | 49.34% | 1.21 | 2105.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 8.417699ms | 1999994 | 0 | 49.41% | 1.21 | 1812.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 16.399667ms | 5364815 | 2000000 | 48.62% | 1.23 | 930.43 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 8.67µs | 5508 | 0 | 48.85% | 1.23 | 1759.95 MB/s |
| Timsort | 1000 | 11.66µs | 5460 | 0 | 48.85% | 1.23 | 1308.64 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.704µs | 921 | 2000 | 48.85% | 1.23 | 319.86 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 54.331µs | 921 | 2000 | 48.85% | 1.23 | 280.85 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 7.908µs | 5508 | 0 | 48.85% | 1.23 | 1929.54 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 8.193µs | 5508 | 0 | 48.85% | 1.23 | 1862.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 7.973µs | 5508 | 0 | 48.85% | 1.23 | 1913.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 11.526µs | 5460 | 0 | 48.85% | 1.23 | 1323.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 8.305µs | 5508 | 0 | 48.85% | 1.23 | 1837.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 11.851µs | 5460 | 0 | 48.85% | 1.23 | 1287.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 8.417µs | 5508 | 0 | 48.85% | 1.23 | 1812.85 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 8.059µs | 5508 | 0 | 48.85% | 1.23 | 1893.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 8.027µs | 5508 | 0 | 48.85% | 1.23 | 1900.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 102.229µs | 5508 | 2000 | 48.85% | 1.23 | 149.26 MB/s |
| Quicksort | 10000 | 69.651µs | 53621 | 0 | 48.83% | 1.23 | 2190.75 MB/s |
| Timsort | 10000 | 96.229µs | 53742 | 0 | 48.83% | 1.23 | 1585.67 MB/s |
| ARS Gen 1: Foundation | 10000 | 303.602µs | 9683 | 30000 | 48.82% | 1.23 | 502.59 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 330.089µs | 9683 | 30000 | 48.82% | 1.23 | 462.26 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 291.754µs | 124917 | 14351 | 48.82% | 1.23 | 523.00 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 193.449µs | 10961 | 10000 | 48.82% | 1.23 | 788.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 125.444µs | 10961 | 0 | 48.81% | 1.23 | 1216.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 113.879µs | 11013 | 0 | 48.81% | 1.23 | 1339.91 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 363.599µs | 13929 | 0 | 48.81% | 1.23 | 419.66 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 363.951µs | 14152 | 0 | 48.81% | 1.23 | 419.25 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 130.691µs | 15115 | 0 | 48.81% | 1.23 | 1167.55 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 125.109µs | 10961 | 0 | 48.81% | 1.23 | 1219.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 126.02µs | 10961 | 0 | 48.81% | 1.23 | 1210.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 288.343µs | 10961 | 20000 | 48.81% | 1.23 | 529.19 MB/s |
| Quicksort | 100000 | 650.166µs | 532062 | 0 | 48.75% | 1.23 | 2346.91 MB/s |
| Timsort | 100000 | 880.603µs | 535405 | 0 | 48.70% | 1.23 | 1732.77 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.782457ms | 98733 | 300000 | 48.73% | 1.23 | 856.05 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.002422ms | 98733 | 300000 | 48.73% | 1.23 | 762.02 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.262178ms | 1174310 | 108703 | 48.74% | 1.23 | 674.52 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 530.741µs | 122228 | 100000 | 48.70% | 1.23 | 2875.00 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 383.569µs | 122228 | 0 | 48.70% | 1.23 | 3978.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 394.552µs | 122352 | 0 | 48.70% | 1.23 | 3867.37 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 699.193µs | 151498 | 0 | 48.71% | 1.23 | 2182.34 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 746.259µs | 152054 | 0 | 48.71% | 1.23 | 2044.70 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 527.353µs | 192482 | 0 | 48.68% | 1.23 | 2893.47 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 524.715µs | 182525 | 0 | 48.70% | 1.23 | 2908.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 546.744µs | 186875 | 0 | 48.69% | 1.23 | 2790.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 741.397µs | 122228 | 200000 | 48.69% | 1.23 | 2058.11 MB/s |
| Quicksort | 1000000 | 6.013803ms | 5301519 | 0 | 48.55% | 1.24 | 2537.29 MB/s |
| Timsort | 1000000 | 10.244824ms | 6302942 | 0 | 48.27% | 1.24 | 1489.41 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.85483ms | 12308876 | 1017407 | 48.66% | 1.23 | 613.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.158421ms | 1094612 | 1000000 | 49.02% | 1.22 | 2477.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 6.204466ms | 1094612 | 0 | 49.03% | 1.22 | 2459.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 6.20415ms | 1095552 | 0 | 49.02% | 1.22 | 2459.45 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 4.747764ms | 1534410 | 0 | 48.89% | 1.23 | 3213.89 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 5.370042ms | 1541577 | 0 | 48.90% | 1.23 | 2841.47 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.03037ms | 1989097 | 0 | 49.08% | 1.21 | 1689.72 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.629857ms | 2062330 | 0 | 49.06% | 1.22 | 1768.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.495756ms | 2113182 | 0 | 49.12% | 1.22 | 1606.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 22.086521ms | 5807618 | 2000000 | 48.71% | 1.23 | 690.86 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 18.763µs | 10241 | 0 | 49.20% | 1.25 | 813.24 MB/s |
| Timsort | 1000 | 27.595µs | 10555 | 0 | 49.20% | 1.25 | 552.95 MB/s |
| ARS Gen 1: Foundation | 1000 | 241.669µs | 0 | 2000 | 49.20% | 1.25 | 63.14 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 286.57µs | 0 | 2000 | 49.20% | 1.25 | 53.25 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.696µs | 10241 | 0 | 49.20% | 1.25 | 862.27 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.771µs | 10241 | 0 | 49.20% | 1.25 | 858.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.63µs | 10241 | 0 | 49.20% | 1.25 | 865.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.115µs | 10555 | 0 | 49.20% | 1.25 | 562.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.593µs | 10241 | 0 | 49.20% | 1.25 | 820.67 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.303µs | 10555 | 0 | 49.20% | 1.25 | 558.87 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.712µs | 10241 | 0 | 49.20% | 1.25 | 815.45 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.784µs | 10241 | 0 | 49.20% | 1.25 | 858.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.535µs | 10241 | 0 | 49.20% | 1.25 | 870.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 127.222µs | 10241 | 2000 | 49.20% | 1.25 | 119.94 MB/s |
| Quicksort | 10000 | 234.879µs | 137603 | 0 | 49.18% | 1.25 | 649.64 MB/s |
| Timsort | 10000 | 326.326µs | 140916 | 0 | 49.18% | 1.25 | 467.59 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.625096ms | 0 | 30000 | 49.01% | 1.25 | 27.13 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.219309ms | 0 | 30000 | 49.00% | 1.25 | 24.53 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 424.631µs | 192365 | 14351 | 49.17% | 1.25 | 359.34 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 251.624µs | 66763 | 10000 | 49.16% | 1.25 | 606.41 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 176.593µs | 66763 | 0 | 49.16% | 1.25 | 864.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 174.088µs | 69738 | 0 | 49.16% | 1.25 | 876.50 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 308.398µs | 61148 | 0 | 49.16% | 1.25 | 494.78 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 317.783µs | 64565 | 0 | 49.16% | 1.25 | 480.16 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 174.142µs | 66763 | 0 | 49.16% | 1.25 | 876.23 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 164.743µs | 66763 | 0 | 49.16% | 1.25 | 926.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 171.13µs | 66763 | 0 | 49.16% | 1.25 | 891.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 334.488µs | 66763 | 20000 | 49.15% | 1.25 | 456.18 MB/s |
| Quicksort | 100000 | 2.780783ms | 1710395 | 0 | 49.11% | 1.25 | 548.72 MB/s |
| Timsort | 100000 | 3.748287ms | 1746952 | 0 | 49.07% | 1.25 | 407.09 MB/s |
| ARS Gen 1: Foundation | 100000 | 46.500103ms | 0 | 300000 | 42.53% | 1.22 | 32.81 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 48.942972ms | 0 | 300000 | 42.91% | 1.23 | 31.18 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.276227ms | 1885598 | 108703 | 49.10% | 1.25 | 465.74 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.265845ms | 1045510 | 100000 | 49.10% | 1.25 | 1205.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.02693ms | 1045510 | 0 | 49.11% | 1.25 | 1485.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.334539ms | 1086813 | 0 | 49.11% | 1.25 | 1143.38 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.183812ms | 981178 | 0 | 49.10% | 1.25 | 1288.95 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.345046ms | 1020346 | 0 | 49.09% | 1.25 | 1134.44 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.014206ms | 1045510 | 0 | 49.08% | 1.25 | 1504.51 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.046381ms | 936769 | 0 | 49.10% | 1.25 | 1458.24 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.043651ms | 1045510 | 0 | 49.09% | 1.25 | 1462.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.401706ms | 1045510 | 200000 | 49.08% | 1.25 | 1088.59 MB/s |
| Quicksort | 1000000 | 28.628895ms | 20431039 | 0 | 48.81% | 1.27 | 532.99 MB/s |
| Timsort | 1000000 | 41.716951ms | 20806652 | 0 | 48.54% | 1.26 | 365.77 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.622507ms | 21500526 | 1017407 | 48.91% | 1.25 | 467.74 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.598966ms | 12082942 | 1000000 | 49.33% | 1.24 | 1315.53 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.857408ms | 12082942 | 0 | 49.34% | 1.24 | 1405.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 13.012704ms | 12502702 | 0 | 49.33% | 1.24 | 1172.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.830648ms | 13210593 | 0 | 49.15% | 1.25 | 1552.17 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.317633ms | 13633723 | 0 | 49.04% | 1.25 | 1238.78 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 10.653107ms | 7768109 | 0 | 49.10% | 1.24 | 1432.33 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.293299ms | 6294342 | 0 | 49.00% | 1.24 | 1351.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.814488ms | 7082878 | 0 | 49.03% | 1.24 | 1410.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.521643ms | 13879903 | 2000000 | 49.20% | 1.24 | 622.26 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 19.88µs | 10551 | 0 | 49.08% | 1.24 | 767.54 MB/s |
| Timsort | 1000 | 28.603µs | 10537 | 0 | 49.08% | 1.24 | 533.47 MB/s |
| ARS Gen 1: Foundation | 1000 | 246.526µs | 0 | 2000 | 49.08% | 1.24 | 61.90 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 296.061µs | 0 | 2000 | 49.08% | 1.24 | 51.54 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 19.026µs | 10551 | 0 | 49.08% | 1.24 | 802.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 19.297µs | 10551 | 0 | 49.08% | 1.24 | 790.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 19.031µs | 10551 | 0 | 49.08% | 1.24 | 801.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 28.395µs | 10537 | 0 | 49.08% | 1.24 | 537.38 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 19.791µs | 10551 | 0 | 49.08% | 1.24 | 771.00 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 28.792µs | 10537 | 0 | 49.08% | 1.24 | 529.97 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.325µs | 10551 | 0 | 49.08% | 1.24 | 750.74 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 19.269µs | 10551 | 0 | 49.08% | 1.24 | 791.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.058µs | 10551 | 0 | 49.08% | 1.24 | 800.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 132.52µs | 10551 | 2000 | 49.08% | 1.24 | 115.14 MB/s |
| Quicksort | 10000 | 234.863µs | 136744 | 0 | 49.06% | 1.24 | 649.69 MB/s |
| Timsort | 10000 | 329.067µs | 140772 | 0 | 49.06% | 1.24 | 463.70 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.941522ms | 0 | 30000 | 48.88% | 1.24 | 25.68 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.094119ms | 0 | 30000 | 48.86% | 1.24 | 25.04 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 420.995µs | 193085 | 14351 | 49.05% | 1.24 | 362.45 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 435.446µs | 126000 | 10000 | 49.05% | 1.24 | 350.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 327.191µs | 126000 | 0 | 49.05% | 1.24 | 466.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 359.06µs | 130426 | 0 | 49.04% | 1.24 | 424.96 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 560.2µs | 118809 | 0 | 49.04% | 1.24 | 272.38 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 489.899µs | 123101 | 0 | 49.04% | 1.24 | 311.47 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 271.243µs | 90031 | 0 | 49.03% | 1.24 | 562.55 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 303.756µs | 126000 | 0 | 49.05% | 1.24 | 502.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 341.01µs | 126000 | 0 | 49.05% | 1.24 | 447.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 464.687µs | 126000 | 20000 | 49.04% | 1.24 | 328.37 MB/s |
| Quicksort | 100000 | 2.640575ms | 1704961 | 0 | 49.01% | 1.24 | 577.86 MB/s |
| Timsort | 100000 | 3.419614ms | 1748322 | 0 | 48.96% | 1.24 | 446.21 MB/s |
| ARS Gen 1: Foundation | 100000 | 42.989758ms | 0 | 300000 | 43.71% | 1.23 | 35.49 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.766857ms | 0 | 300000 | 42.36% | 1.22 | 34.09 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.010314ms | 1885129 | 108703 | 48.99% | 1.24 | 506.88 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.072489ms | 1618379 | 100000 | 48.96% | 1.24 | 736.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.795893ms | 1618379 | 0 | 48.99% | 1.24 | 849.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.090648ms | 1658575 | 0 | 48.94% | 1.24 | 729.86 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.888537ms | 1529988 | 0 | 48.99% | 1.24 | 807.97 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.21295ms | 1573500 | 0 | 48.98% | 1.24 | 689.52 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.192078ms | 673827 | 0 | 48.89% | 1.24 | 1280.02 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.278145ms | 837701 | 0 | 48.92% | 1.24 | 1193.82 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.456593ms | 967480 | 0 | 48.92% | 1.24 | 1047.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.298245ms | 1618379 | 200000 | 48.96% | 1.24 | 663.93 MB/s |
| Quicksort | 1000000 | 29.63377ms | 20435426 | 0 | 48.68% | 1.26 | 514.91 MB/s |
| Timsort | 1000000 | 42.308631ms | 20818465 | 0 | 48.26% | 1.25 | 360.65 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.373792ms | 21488833 | 1017407 | 48.76% | 1.24 | 557.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 21.855711ms | 19275700 | 1000000 | 49.01% | 1.24 | 698.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 19.111946ms | 19275700 | 0 | 49.02% | 1.24 | 798.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 27.328002ms | 19658200 | 0 | 48.67% | 1.24 | 558.36 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 14.673049ms | 19275700 | 0 | 48.91% | 1.25 | 1039.92 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.145547ms | 19658200 | 0 | 48.64% | 1.24 | 757.43 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.133201ms | 6708948 | 0 | 48.93% | 1.22 | 890.60 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 17.934936ms | 6379291 | 0 | 49.35% | 1.23 | 850.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.544469ms | 8360463 | 0 | 49.40% | 1.22 | 822.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.480642ms | 20771275 | 2000000 | 49.04% | 1.24 | 500.61 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 19.05µs | 10288 | 0 | 49.65% | 1.25 | 800.99 MB/s |
| Timsort | 1000 | 27.171µs | 10450 | 0 | 49.65% | 1.25 | 561.58 MB/s |
| ARS Gen 1: Foundation | 1000 | 239.269µs | 0 | 2000 | 49.65% | 1.25 | 63.77 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 283.585µs | 0 | 2000 | 49.65% | 1.25 | 53.81 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 17.675µs | 10288 | 0 | 49.65% | 1.25 | 863.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.778µs | 10288 | 0 | 49.65% | 1.25 | 858.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 17.761µs | 10288 | 0 | 49.65% | 1.25 | 859.12 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.028µs | 10450 | 0 | 49.65% | 1.25 | 564.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 18.77µs | 10288 | 0 | 49.65% | 1.25 | 812.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 27.361µs | 10450 | 0 | 49.65% | 1.25 | 557.68 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 18.891µs | 10288 | 0 | 49.65% | 1.25 | 807.73 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 17.768µs | 10288 | 0 | 49.65% | 1.25 | 858.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 17.886µs | 10288 | 0 | 49.65% | 1.25 | 853.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 128.012µs | 10288 | 2000 | 49.65% | 1.25 | 119.20 MB/s |
| Quicksort | 10000 | 232.293µs | 136714 | 0 | 49.63% | 1.25 | 656.88 MB/s |
| Timsort | 10000 | 327.577µs | 140903 | 0 | 49.63% | 1.25 | 465.81 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.522068ms | 160 | 30000 | 49.46% | 1.25 | 27.63 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.744434ms | 160 | 30000 | 49.43% | 1.25 | 26.56 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 410.697µs | 193162 | 14351 | 49.62% | 1.25 | 371.53 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 242.505µs | 52333 | 10000 | 49.62% | 1.25 | 629.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 174.713µs | 52333 | 0 | 49.62% | 1.25 | 873.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 191.479µs | 57763 | 0 | 49.61% | 1.25 | 796.89 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 346.525µs | 59057 | 0 | 49.61% | 1.25 | 440.34 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 238.438µs | 62100 | 0 | 49.61% | 1.25 | 639.95 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 170.873µs | 52333 | 0 | 49.61% | 1.25 | 892.99 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 161.572µs | 52333 | 0 | 49.61% | 1.25 | 944.40 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 161.433µs | 52333 | 0 | 49.61% | 1.25 | 945.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 334.331µs | 52333 | 20000 | 49.61% | 1.25 | 456.40 MB/s |
| Quicksort | 100000 | 2.601649ms | 1706033 | 0 | 49.55% | 1.25 | 586.50 MB/s |
| Timsort | 100000 | 3.425895ms | 1748408 | 0 | 49.50% | 1.25 | 445.40 MB/s |
| ARS Gen 1: Foundation | 100000 | 40.780484ms | 15822 | 300000 | 43.41% | 1.22 | 37.42 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.830077ms | 15822 | 300000 | 43.94% | 1.23 | 33.29 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.93397ms | 1885784 | 108703 | 49.53% | 1.25 | 520.07 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 952.534µs | 882348 | 100000 | 49.51% | 1.25 | 1601.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 728.557µs | 882348 | 0 | 49.51% | 1.25 | 2094.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.007884ms | 921462 | 0 | 49.52% | 1.24 | 1513.94 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.113086ms | 939598 | 0 | 49.53% | 1.25 | 1370.85 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.221673ms | 975983 | 0 | 49.51% | 1.25 | 1249.01 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 847.917µs | 882348 | 0 | 49.51% | 1.25 | 1799.56 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 831.107µs | 771432 | 0 | 49.52% | 1.25 | 1835.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 731.597µs | 882348 | 0 | 49.50% | 1.25 | 2085.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.227017ms | 882348 | 200000 | 49.52% | 1.24 | 1243.57 MB/s |
| Quicksort | 1000000 | 28.760191ms | 20389196 | 0 | 49.18% | 1.26 | 530.55 MB/s |
| Timsort | 1000000 | 42.628817ms | 20780417 | 0 | 48.75% | 1.26 | 357.95 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.933882ms | 21441825 | 1017407 | 49.26% | 1.25 | 566.53 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.0429ms | 10157321 | 1000000 | 49.63% | 1.24 | 1519.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.95744ms | 10157321 | 0 | 49.64% | 1.24 | 1532.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.311876ms | 10561958 | 0 | 49.62% | 1.24 | 1348.92 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.139624ms | 12859603 | 0 | 49.51% | 1.25 | 1669.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.100272ms | 13271645 | 0 | 49.43% | 1.24 | 1374.63 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 9.810615ms | 10157321 | 0 | 49.56% | 1.24 | 1555.33 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.810802ms | 11214454 | 0 | 49.65% | 1.24 | 1411.44 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 11.132898ms | 12268914 | 0 | 49.67% | 1.24 | 1370.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.917216ms | 13561646 | 2000000 | 49.56% | 1.24 | 510.03 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 7.931µs | 5797 | 0 | 49.84% | 1.25 | 1923.94 MB/s |
| Timsort | 1000 | 10.999µs | 5499 | 0 | 49.84% | 1.25 | 1387.29 MB/s |
| ARS Gen 1: Foundation | 1000 | 47.849µs | 984 | 2000 | 49.84% | 1.25 | 318.89 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 58.107µs | 984 | 2000 | 49.84% | 1.25 | 262.60 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 7.418µs | 5797 | 0 | 49.84% | 1.25 | 2057.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 7.301µs | 5797 | 0 | 49.84% | 1.25 | 2089.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 7.327µs | 5797 | 0 | 49.84% | 1.25 | 2082.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 10.955µs | 5499 | 0 | 49.84% | 1.25 | 1392.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 7.767µs | 5797 | 0 | 49.84% | 1.25 | 1964.57 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 10.894µs | 5499 | 0 | 49.84% | 1.25 | 1400.66 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 7.97µs | 5797 | 0 | 49.84% | 1.25 | 1914.53 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 7.746µs | 5797 | 0 | 49.84% | 1.25 | 1969.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.569µs | 5797 | 0 | 49.84% | 1.25 | 2015.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 113.325µs | 5797 | 2000 | 49.84% | 1.25 | 134.65 MB/s |
| Quicksort | 10000 | 68.224µs | 53838 | 0 | 49.82% | 1.25 | 2236.57 MB/s |
| Timsort | 10000 | 89.298µs | 53843 | 0 | 49.82% | 1.25 | 1708.75 MB/s |
| ARS Gen 1: Foundation | 10000 | 248.641µs | 9984 | 30000 | 49.82% | 1.25 | 613.69 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 303.819µs | 9984 | 30000 | 49.82% | 1.25 | 502.23 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 301.958µs | 122148 | 14351 | 49.82% | 1.25 | 505.33 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 201.228µs | 12061 | 10000 | 49.81% | 1.25 | 758.28 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 127.687µs | 12061 | 0 | 49.81% | 1.25 | 1195.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 123.474µs | 12085 | 0 | 49.81% | 1.25 | 1235.79 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 291.638µs | 12061 | 0 | 49.81% | 1.25 | 523.21 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 271.372µs | 12085 | 0 | 49.81% | 1.25 | 562.28 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 144.63µs | 12061 | 0 | 49.81% | 1.25 | 1055.02 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 139.405µs | 12061 | 0 | 49.81% | 1.25 | 1094.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 133.243µs | 12061 | 0 | 49.81% | 1.25 | 1145.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 301.615µs | 12061 | 20000 | 49.80% | 1.25 | 505.90 MB/s |
| Quicksort | 100000 | 686.313µs | 529379 | 0 | 49.76% | 1.25 | 2223.30 MB/s |
| Timsort | 100000 | 888.889µs | 529674 | 0 | 49.72% | 1.25 | 1716.61 MB/s |
| ARS Gen 1: Foundation | 100000 | 1.408211ms | 99984 | 300000 | 49.76% | 1.25 | 1083.56 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 1.583747ms | 99984 | 300000 | 49.76% | 1.25 | 963.46 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.266137ms | 1143461 | 108703 | 49.76% | 1.25 | 673.34 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 564.447µs | 151116 | 100000 | 49.72% | 1.25 | 2703.32 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 421.475µs | 151116 | 0 | 49.73% | 1.25 | 3620.33 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 387.401µs | 151622 | 0 | 49.72% | 1.25 | 3938.76 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 590.166µs | 99988 | 0 | 49.73% | 1.25 | 2585.51 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 642.242µs | 99988 | 0 | 49.73% | 1.25 | 2375.86 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 623.503µs | 199984 | 0 | 49.70% | 1.25 | 2447.27 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 550.623µs | 199972 | 0 | 49.72% | 1.25 | 2771.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 772.604µs | 100000 | 0 | 49.73% | 1.25 | 1974.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.098683ms | 151116 | 200000 | 49.75% | 1.25 | 1388.83 MB/s |
| Quicksort | 1000000 | 6.131138ms | 5138620 | 0 | 49.53% | 1.25 | 2488.74 MB/s |
| Timsort | 1000000 | 10.261218ms | 6175006 | 0 | 49.27% | 1.25 | 1487.03 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.491967ms | 12087538 | 1017407 | 49.74% | 1.25 | 623.01 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 6.224772ms | 999988 | 1000000 | 49.99% | 1.24 | 2451.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 7.87934ms | 999988 | 0 | 50.00% | 1.24 | 1936.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 7.607919ms | 999988 | 0 | 49.99% | 1.24 | 2005.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.267577ms | 999988 | 0 | 49.88% | 1.25 | 2434.56 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 4.50888ms | 999988 | 0 | 49.87% | 1.25 | 3384.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 8.192078ms | 1999972 | 0 | 49.95% | 1.24 | 1862.63 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 8.892587ms | 1999972 | 0 | 49.95% | 1.25 | 1715.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 9.609937ms | 1999976 | 0 | 49.99% | 1.24 | 1587.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 18.393728ms | 5620187 | 2000000 | 49.57% | 1.25 | 829.56 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 17.638µs | 10288 | 0 | 49.71% | 1.25 | 865.11 MB/s |
| Timsort | 1000 | 25.601µs | 10450 | 0 | 49.71% | 1.25 | 596.02 MB/s |
| ARS Gen 1: Foundation | 1000 | 224.561µs | 0 | 2000 | 49.71% | 1.25 | 67.95 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 268.317µs | 0 | 2000 | 49.71% | 1.25 | 56.87 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 16.831µs | 10288 | 0 | 49.71% | 1.25 | 906.59 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 17.013µs | 10288 | 0 | 49.71% | 1.25 | 896.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 16.809µs | 10288 | 0 | 49.71% | 1.25 | 907.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 25.415µs | 10450 | 0 | 49.71% | 1.25 | 600.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 17.416µs | 10288 | 0 | 49.71% | 1.25 | 876.14 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 25.626µs | 10450 | 0 | 49.71% | 1.25 | 595.44 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 17.764µs | 10288 | 0 | 49.71% | 1.25 | 858.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 16.756µs | 10288 | 0 | 49.71% | 1.25 | 910.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 16.623µs | 10288 | 0 | 49.71% | 1.25 | 917.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 119.08µs | 10288 | 2000 | 49.71% | 1.25 | 128.14 MB/s |
| Quicksort | 10000 | 218.607µs | 136714 | 0 | 49.69% | 1.25 | 698.00 MB/s |
| Timsort | 10000 | 306.192µs | 140903 | 0 | 49.69% | 1.25 | 498.34 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.566354ms | 160 | 30000 | 49.55% | 1.25 | 27.41 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.18964ms | 160 | 30000 | 49.53% | 1.25 | 24.65 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 413.756µs | 193162 | 14351 | 49.69% | 1.25 | 368.79 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 256.101µs | 52333 | 10000 | 49.68% | 1.25 | 595.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 194.203µs | 52333 | 0 | 49.68% | 1.25 | 785.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 209.076µs | 57763 | 0 | 49.68% | 1.25 | 729.82 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 287.347µs | 59057 | 0 | 49.67% | 1.25 | 531.02 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 307.934µs | 62100 | 0 | 49.67% | 1.25 | 495.52 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 196.696µs | 52333 | 0 | 49.68% | 1.25 | 775.75 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 182.815µs | 52333 | 0 | 49.68% | 1.25 | 834.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 188.839µs | 52333 | 0 | 49.68% | 1.25 | 808.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 406.211µs | 52333 | 20000 | 49.67% | 1.25 | 375.64 MB/s |
| Quicksort | 100000 | 2.984296ms | 1706033 | 0 | 49.63% | 1.25 | 511.30 MB/s |
| Timsort | 100000 | 3.884779ms | 1748408 | 0 | 49.60% | 1.25 | 392.78 MB/s |
| ARS Gen 1: Foundation | 100000 | 51.284828ms | 15822 | 300000 | 44.72% | 1.23 | 29.75 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 50.433744ms | 15822 | 300000 | 44.21% | 1.23 | 30.26 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.309601ms | 1885784 | 108703 | 49.61% | 1.25 | 461.05 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.268709ms | 882348 | 100000 | 49.61% | 1.25 | 1202.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 994.842µs | 882348 | 0 | 49.62% | 1.25 | 1533.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.103096ms | 921462 | 0 | 49.61% | 1.25 | 1383.27 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.400668ms | 939598 | 0 | 49.61% | 1.25 | 1089.39 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.666871ms | 975983 | 0 | 49.61% | 1.25 | 915.42 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 938.36µs | 882348 | 0 | 49.60% | 1.25 | 1626.11 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.172606ms | 771432 | 0 | 49.62% | 1.25 | 1301.27 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.020652ms | 882348 | 0 | 49.60% | 1.25 | 1495.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.566006ms | 882348 | 200000 | 49.60% | 1.25 | 974.38 MB/s |
| Quicksort | 1000000 | 29.334952ms | 20389196 | 0 | 49.31% | 1.26 | 520.16 MB/s |
| Timsort | 1000000 | 42.268843ms | 20780417 | 0 | 48.93% | 1.26 | 360.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.182327ms | 21441825 | 1017407 | 49.39% | 1.25 | 561.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.301955ms | 10157321 | 1000000 | 49.67% | 1.25 | 1481.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 9.577113ms | 10157321 | 0 | 49.69% | 1.24 | 1593.26 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 13.472202ms | 10561958 | 0 | 49.70% | 1.24 | 1132.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.26376ms | 12859603 | 0 | 49.56% | 1.25 | 1647.15 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 11.074689ms | 13271645 | 0 | 49.50% | 1.25 | 1377.81 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 11.877861ms | 10157321 | 0 | 49.66% | 1.24 | 1284.64 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.863824ms | 11214454 | 0 | 49.70% | 1.24 | 1404.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 10.559043ms | 12268914 | 0 | 49.70% | 1.24 | 1445.09 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.996694ms | 13561638 | 2000000 | 49.52% | 1.24 | 508.68 MB/s |

## Category: String

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 70.584µs | 10370 | 0 | 49.58% | 1.25 | 864.72 MB/s |
| Timsort | 1000 | 78.859µs | 10522 | 0 | 49.58% | 1.25 | 773.98 MB/s |
| ARS Gen 1: Foundation | 1000 | 324.066µs | 0 | 2000 | 49.58% | 1.25 | 188.34 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 365.623µs | 0 | 2000 | 49.58% | 1.25 | 166.93 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 68.49µs | 10370 | 0 | 49.58% | 1.25 | 891.15 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 70.086µs | 10370 | 0 | 49.58% | 1.25 | 870.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 70.183µs | 10370 | 0 | 49.58% | 1.25 | 869.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 78.227µs | 10522 | 0 | 49.58% | 1.25 | 780.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 69.599µs | 10370 | 0 | 49.58% | 1.25 | 876.95 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 75.526µs | 10522 | 0 | 49.58% | 1.25 | 808.13 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 68.095µs | 10370 | 0 | 49.58% | 1.25 | 896.32 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 69.305µs | 10370 | 0 | 49.58% | 1.25 | 880.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 69.266µs | 10370 | 0 | 49.58% | 1.25 | 881.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 260.27µs | 10370 | 2000 | 49.57% | 1.25 | 234.51 MB/s |
| Quicksort | 10000 | 830.619µs | 136866 | 0 | 49.54% | 1.25 | 734.82 MB/s |
| Timsort | 10000 | 972.861µs | 141490 | 0 | 49.53% | 1.25 | 627.38 MB/s |
| ARS Gen 1: Foundation | 10000 | 6.443117ms | 0 | 30000 | 49.38% | 1.25 | 94.73 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.843476ms | 0 | 30000 | 49.37% | 1.25 | 89.19 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.327656ms | 193846 | 14351 | 49.52% | 1.25 | 262.22 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 713.062µs | 67438 | 10000 | 49.53% | 1.25 | 855.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 244.774µs | 67438 | 0 | 49.53% | 1.25 | 2493.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 283.374µs | 70298 | 0 | 49.53% | 1.25 | 2153.87 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 469.918µs | 63043 | 0 | 49.52% | 1.25 | 1298.85 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 480.317µs | 67007 | 0 | 49.52% | 1.25 | 1270.73 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 271.6µs | 67438 | 0 | 49.53% | 1.25 | 2247.24 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 262.376µs | 67438 | 0 | 49.54% | 1.25 | 2326.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 264.072µs | 67438 | 0 | 49.54% | 1.25 | 2311.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.18864ms | 67438 | 20000 | 49.51% | 1.25 | 513.49 MB/s |
| Quicksort | 100000 | 10.258344ms | 1718762 | 0 | 49.00% | 1.25 | 594.98 MB/s |
| Timsort | 100000 | 13.676775ms | 1759891 | 0 | 48.77% | 1.25 | 446.27 MB/s |
| ARS Gen 1: Foundation | 100000 | 57.553653ms | 0 | 300000 | 44.73% | 1.23 | 106.05 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 60.648502ms | 0 | 300000 | 44.61% | 1.24 | 100.64 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 18.86787ms | 1895222 | 108703 | 49.43% | 1.25 | 323.49 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.186151ms | 1029722 | 100000 | 49.42% | 1.25 | 1458.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.124451ms | 1029722 | 0 | 49.41% | 1.24 | 1953.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.281133ms | 1071423 | 0 | 49.38% | 1.24 | 1860.19 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.559535ms | 978520 | 0 | 49.36% | 1.25 | 1714.69 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.885508ms | 1019338 | 0 | 49.34% | 1.24 | 1570.84 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.576529ms | 1029722 | 0 | 49.37% | 1.25 | 1706.55 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.36139ms | 961965 | 0 | 49.43% | 1.24 | 1815.77 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.405176ms | 1029722 | 0 | 49.37% | 1.24 | 1792.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 12.005476ms | 1029722 | 200000 | 49.09% | 1.25 | 508.39 MB/s |
| Quicksort | 1000000 | 270.220584ms | 20518628 | 0 | 51.21% | 1.27 | 225.87 MB/s |
| Timsort | 1000000 | 357.945649ms | 20902099 | 0 | 49.67% | 1.25 | 170.52 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 193.894795ms | 21589743 | 1017407 | 49.25% | 1.24 | 314.78 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.671142ms | 12256776 | 1000000 | 50.26% | 1.19 | 1022.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 51.012582ms | 12256776 | 0 | 50.33% | 1.20 | 1196.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 56.276579ms | 12679336 | 0 | 49.98% | 1.19 | 1084.56 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 63.61559ms | 13331493 | 0 | 50.37% | 1.18 | 959.44 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 70.572096ms | 13750405 | 0 | 50.20% | 1.18 | 864.86 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.989836ms | 9434971 | 0 | 50.59% | 1.20 | 1151.83 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 68.533898ms | 7913738 | 0 | 51.58% | 1.18 | 890.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 58.343852ms | 8719170 | 0 | 51.25% | 1.19 | 1046.13 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 404.324933ms | 15199655 | 2000000 | 53.32% | 1.20 | 150.96 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 53.461µs | 10370 | 0 | 49.00% | 1.24 | 1141.68 MB/s |
| Timsort | 1000 | 73.219µs | 10522 | 0 | 49.00% | 1.24 | 833.60 MB/s |
| ARS Gen 1: Foundation | 1000 | 289.367µs | 0 | 2000 | 49.00% | 1.24 | 210.93 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 297.928µs | 0 | 2000 | 49.00% | 1.24 | 204.87 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 54.679µs | 10370 | 0 | 49.00% | 1.24 | 1116.24 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 69.909µs | 10370 | 0 | 49.00% | 1.24 | 873.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 55.005µs | 10370 | 0 | 49.00% | 1.24 | 1109.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 64.114µs | 10522 | 0 | 49.00% | 1.24 | 951.98 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 56.766µs | 10370 | 0 | 49.00% | 1.24 | 1075.21 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 67.638µs | 10522 | 0 | 49.00% | 1.24 | 902.38 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 55.285µs | 10370 | 0 | 49.00% | 1.24 | 1104.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 55.184µs | 10370 | 0 | 49.00% | 1.24 | 1106.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 54.973µs | 10370 | 0 | 49.00% | 1.24 | 1110.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 256.905µs | 10370 | 2000 | 49.00% | 1.24 | 237.58 MB/s |
| Quicksort | 10000 | 695.651µs | 136866 | 0 | 48.97% | 1.24 | 877.38 MB/s |
| Timsort | 10000 | 833.563µs | 141490 | 0 | 48.96% | 1.24 | 732.22 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.549599ms | 0 | 30000 | 48.82% | 1.24 | 109.98 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.949885ms | 0 | 30000 | 48.81% | 1.24 | 102.58 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.087719ms | 193846 | 14351 | 48.96% | 1.24 | 292.35 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 678.753µs | 67438 | 10000 | 48.96% | 1.24 | 899.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 254.562µs | 67438 | 0 | 48.96% | 1.24 | 2397.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 237.512µs | 70298 | 0 | 48.96% | 1.24 | 2569.77 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 432.36µs | 63043 | 0 | 48.95% | 1.24 | 1411.67 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 478.573µs | 67007 | 0 | 48.96% | 1.24 | 1275.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 234.335µs | 67438 | 0 | 48.96% | 1.24 | 2604.61 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 229.317µs | 67438 | 0 | 48.96% | 1.24 | 2661.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 219.566µs | 67438 | 0 | 48.96% | 1.24 | 2779.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.097902ms | 67438 | 20000 | 48.94% | 1.24 | 555.93 MB/s |
| Quicksort | 100000 | 9.548476ms | 1718762 | 0 | 48.47% | 1.24 | 639.21 MB/s |
| Timsort | 100000 | 11.780371ms | 1759891 | 0 | 48.26% | 1.24 | 518.11 MB/s |
| ARS Gen 1: Foundation | 100000 | 56.803528ms | 0 | 300000 | 44.55% | 1.23 | 107.45 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 57.646738ms | 0 | 300000 | 44.69% | 1.23 | 105.88 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.326704ms | 1895222 | 108703 | 48.87% | 1.24 | 352.26 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.843963ms | 1029722 | 100000 | 48.85% | 1.24 | 1587.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.769572ms | 1029722 | 0 | 48.86% | 1.23 | 2203.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.293351ms | 1071423 | 0 | 48.83% | 1.23 | 1853.28 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.333013ms | 978520 | 0 | 48.83% | 1.23 | 1831.23 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.587497ms | 1019338 | 0 | 48.83% | 1.23 | 1701.33 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.406892ms | 1029722 | 0 | 48.84% | 1.23 | 1791.52 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.26326ms | 961965 | 0 | 48.87% | 1.23 | 1870.37 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.98869ms | 1029722 | 0 | 48.84% | 1.24 | 2042.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 11.221922ms | 1029722 | 200000 | 48.48% | 1.24 | 543.89 MB/s |
| Quicksort | 1000000 | 283.350989ms | 20518628 | 0 | 50.66% | 1.26 | 215.40 MB/s |
| Timsort | 1000000 | 407.894802ms | 20902099 | 0 | 49.46% | 1.22 | 149.63 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 199.595007ms | 21589743 | 1017407 | 48.65% | 1.24 | 305.80 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.046572ms | 12256776 | 1000000 | 49.71% | 1.18 | 1016.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.41968ms | 12256776 | 0 | 49.74% | 1.19 | 1260.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 55.512103ms | 12679336 | 0 | 49.50% | 1.19 | 1099.49 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 62.425471ms | 13331493 | 0 | 49.75% | 1.18 | 977.73 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 81.026298ms | 13750405 | 0 | 50.10% | 1.16 | 753.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 57.058796ms | 9434971 | 0 | 50.14% | 1.19 | 1069.69 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 79.074531ms | 7913738 | 0 | 51.12% | 1.16 | 771.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 61.894549ms | 8719170 | 0 | 50.67% | 1.18 | 986.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 431.417713ms | 15151081 | 2000000 | 53.54% | 1.19 | 141.48 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 42.884µs | 9540 | 0 | 51.58% | 1.20 | 1423.26 MB/s |
| Timsort | 1000 | 47.122µs | 9492 | 0 | 51.58% | 1.20 | 1295.26 MB/s |
| ARS Gen 1: Foundation | 1000 | 128.838µs | 9394 | 2000 | 51.58% | 1.20 | 473.74 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 135.785µs | 9417 | 2000 | 51.58% | 1.20 | 449.50 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 50.635µs | 9540 | 0 | 51.58% | 1.20 | 1205.39 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 53.572µs | 9540 | 0 | 51.58% | 1.20 | 1139.31 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 53.827µs | 9540 | 0 | 51.58% | 1.20 | 1133.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 53.456µs | 9492 | 0 | 51.58% | 1.20 | 1141.78 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 48.864µs | 9540 | 0 | 51.58% | 1.20 | 1249.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 53.923µs | 9492 | 0 | 51.58% | 1.20 | 1131.89 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 53.028µs | 9540 | 0 | 51.58% | 1.20 | 1151.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 54.808µs | 9540 | 0 | 51.58% | 1.20 | 1113.62 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 50.208µs | 9540 | 0 | 51.58% | 1.20 | 1215.65 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 207.028µs | 9540 | 2000 | 51.58% | 1.20 | 294.82 MB/s |
| Quicksort | 10000 | 719.831µs | 132500 | 0 | 51.56% | 1.20 | 847.91 MB/s |
| Timsort | 10000 | 741.29µs | 127861 | 0 | 51.55% | 1.20 | 823.36 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.932258ms | 94604 | 30000 | 51.53% | 1.21 | 315.87 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.962961ms | 94565 | 30000 | 51.52% | 1.21 | 310.93 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.977028ms | 182797 | 14351 | 51.54% | 1.20 | 308.72 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 858.946µs | 88075 | 10000 | 51.55% | 1.20 | 710.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 272.197µs | 88075 | 0 | 51.55% | 1.20 | 2242.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 267.819µs | 63479 | 0 | 51.55% | 1.20 | 2278.97 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 397.438µs | 73151 | 0 | 51.54% | 1.20 | 1535.72 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 372.09µs | 48448 | 0 | 51.54% | 1.20 | 1640.33 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 297.711µs | 88075 | 0 | 51.55% | 1.20 | 2050.15 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 321.786µs | 88075 | 0 | 51.55% | 1.20 | 1896.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 298.865µs | 88075 | 0 | 51.55% | 1.20 | 2042.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.165638ms | 88075 | 20000 | 51.53% | 1.20 | 523.62 MB/s |
| Quicksort | 100000 | 9.473442ms | 1695729 | 0 | 51.37% | 1.21 | 644.28 MB/s |
| Timsort | 100000 | 10.097611ms | 1618264 | 0 | 51.24% | 1.21 | 604.45 MB/s |
| ARS Gen 1: Foundation | 100000 | 23.162915ms | 958264 | 300000 | 51.58% | 1.21 | 263.50 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 23.443993ms | 958287 | 300000 | 51.54% | 1.21 | 260.34 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.777432ms | 1799629 | 108703 | 51.54% | 1.21 | 343.33 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.875968ms | 1250176 | 100000 | 51.44% | 1.20 | 1251.75 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.329095ms | 1250176 | 0 | 51.47% | 1.20 | 1833.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.756306ms | 957484 | 0 | 51.43% | 1.20 | 1624.87 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.191334ms | 1082137 | 0 | 51.45% | 1.20 | 1912.53 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.804374ms | 561919 | 0 | 51.43% | 1.20 | 2176.43 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.199224ms | 877121 | 0 | 51.45% | 1.20 | 1907.81 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.383946ms | 943356 | 0 | 51.43% | 1.20 | 1803.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.246613ms | 1084625 | 0 | 51.45% | 1.20 | 1879.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 12.157896ms | 1250176 | 200000 | 51.42% | 1.20 | 502.02 MB/s |
| Quicksort | 1000000 | 140.687566ms | 20467458 | 0 | 50.25% | 1.25 | 433.83 MB/s |
| Timsort | 1000000 | 176.47102ms | 19247236 | 0 | 49.84% | 1.24 | 345.87 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 221.91874ms | 20726079 | 1017407 | 52.13% | 1.23 | 275.03 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.218323ms | 14427992 | 1000000 | 51.93% | 1.18 | 1013.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 40.063155ms | 14427992 | 0 | 51.28% | 1.19 | 1523.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 40.993466ms | 9562892 | 0 | 50.84% | 1.17 | 1488.90 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 56.00703ms | 14500857 | 0 | 51.58% | 1.18 | 1089.78 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 51.938586ms | 9781181 | 0 | 51.06% | 1.17 | 1175.14 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 43.892091ms | 10121426 | 0 | 52.00% | 1.19 | 1390.57 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 54.815006ms | 10340217 | 0 | 52.27% | 1.19 | 1113.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 49.914796ms | 11441302 | 0 | 52.03% | 1.18 | 1222.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 246.949278ms | 19220981 | 2000000 | 52.56% | 1.22 | 247.16 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 41.026µs | 5636 | 0 | 55.07% | 1.23 | 1487.72 MB/s |
| Timsort | 1000 | 47.312µs | 5782 | 0 | 55.07% | 1.23 | 1290.06 MB/s |
| ARS Gen 1: Foundation | 1000 | 124.582µs | 984 | 2000 | 55.07% | 1.23 | 489.92 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 132.491µs | 984 | 2000 | 55.07% | 1.23 | 460.67 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 36.08µs | 5636 | 0 | 55.07% | 1.23 | 1691.66 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 38.059µs | 5636 | 0 | 55.07% | 1.23 | 1603.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 36.99µs | 5636 | 0 | 55.07% | 1.23 | 1650.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 37.912µs | 5782 | 0 | 55.07% | 1.23 | 1609.92 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 32.994µs | 5636 | 0 | 55.07% | 1.23 | 1849.89 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 36.762µs | 5782 | 0 | 55.07% | 1.23 | 1660.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 31.891µs | 5636 | 0 | 55.07% | 1.23 | 1913.87 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 35.19µs | 5636 | 0 | 55.07% | 1.23 | 1734.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 36.29µs | 5636 | 0 | 55.07% | 1.23 | 1681.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 194.727µs | 5636 | 2000 | 55.06% | 1.23 | 313.44 MB/s |
| Quicksort | 10000 | 340.561µs | 53113 | 0 | 55.04% | 1.23 | 1792.19 MB/s |
| Timsort | 10000 | 380.637µs | 54714 | 0 | 55.04% | 1.23 | 1603.50 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.395684ms | 9984 | 30000 | 55.01% | 1.23 | 437.31 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.524153ms | 9984 | 30000 | 55.02% | 1.23 | 400.45 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.856943ms | 122389 | 14351 | 55.04% | 1.23 | 328.69 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 511.677µs | 14075 | 10000 | 55.04% | 1.23 | 1192.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 140.432µs | 14075 | 0 | 55.04% | 1.23 | 4346.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 139.417µs | 14094 | 0 | 55.04% | 1.23 | 4377.88 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 281.731µs | 12021 | 0 | 55.03% | 1.23 | 2166.43 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 325.719µs | 12028 | 0 | 55.03% | 1.23 | 1873.86 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 149.008µs | 14075 | 0 | 55.04% | 1.23 | 4096.10 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 150.927µs | 14075 | 0 | 55.04% | 1.23 | 4044.02 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 191.646µs | 14075 | 0 | 55.04% | 1.23 | 3184.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 971.795µs | 14075 | 20000 | 55.02% | 1.23 | 628.07 MB/s |
| Quicksort | 100000 | 3.934471ms | 516589 | 0 | 54.83% | 1.23 | 1551.29 MB/s |
| Timsort | 100000 | 4.668446ms | 529550 | 0 | 54.74% | 1.23 | 1307.40 MB/s |
| ARS Gen 1: Foundation | 100000 | 14.887543ms | 99984 | 300000 | 54.82% | 1.23 | 409.97 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 15.370141ms | 99984 | 300000 | 54.83% | 1.23 | 397.10 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 15.218559ms | 1144965 | 108703 | 54.99% | 1.23 | 401.06 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.036191ms | 151083 | 100000 | 55.02% | 1.23 | 2010.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.671365ms | 151083 | 0 | 55.00% | 1.23 | 3651.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.969538ms | 151309 | 0 | 55.02% | 1.23 | 3098.96 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.075983ms | 99990 | 0 | 55.00% | 1.23 | 2940.06 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.01295ms | 99990 | 0 | 54.99% | 1.23 | 3032.12 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.013749ms | 200008 | 0 | 54.92% | 1.23 | 3030.92 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.033742ms | 200008 | 0 | 54.93% | 1.23 | 3001.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.63353ms | 100024 | 0 | 55.00% | 1.23 | 3736.40 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 8.758755ms | 151083 | 200000 | 54.76% | 1.23 | 696.85 MB/s |
| Quicksort | 1000000 | 89.04475ms | 5202060 | 0 | 56.08% | 1.23 | 685.44 MB/s |
| Timsort | 1000000 | 145.750595ms | 6111262 | 0 | 56.08% | 1.22 | 418.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 164.614946ms | 12085476 | 1017407 | 55.12% | 1.24 | 370.78 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 33.380611ms | 999988 | 1000000 | 55.47% | 1.22 | 1828.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 23.095926ms | 999988 | 0 | 55.41% | 1.21 | 2642.68 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 23.290933ms | 999988 | 0 | 55.46% | 1.20 | 2620.55 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 24.676605ms | 999988 | 0 | 55.42% | 1.21 | 2473.40 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 25.088304ms | 999988 | 0 | 55.47% | 1.21 | 2432.81 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 34.215166ms | 1999972 | 0 | 55.56% | 1.20 | 1783.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 39.639675ms | 1999976 | 0 | 55.69% | 1.20 | 1539.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 37.039963ms | 1999976 | 0 | 55.68% | 1.20 | 1647.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 310.028245ms | 5709060 | 2000000 | 57.45% | 1.21 | 196.87 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 57.836µs | 10370 | 0 | 54.90% | 1.22 | 1055.31 MB/s |
| Timsort | 1000 | 65.811µs | 10522 | 0 | 54.90% | 1.22 | 927.43 MB/s |
| ARS Gen 1: Foundation | 1000 | 274.445µs | 0 | 2000 | 54.89% | 1.22 | 222.39 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 310.305µs | 0 | 2000 | 54.89% | 1.22 | 196.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 54.354µs | 10370 | 0 | 54.90% | 1.22 | 1122.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 52.311µs | 10370 | 0 | 54.90% | 1.22 | 1166.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 52.566µs | 10370 | 0 | 54.90% | 1.22 | 1161.11 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 63.104µs | 10522 | 0 | 54.90% | 1.22 | 967.22 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 62.517µs | 10370 | 0 | 54.90% | 1.22 | 976.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 61.57µs | 10522 | 0 | 54.90% | 1.22 | 991.31 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 55.851µs | 10370 | 0 | 54.90% | 1.22 | 1092.82 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 57.507µs | 10370 | 0 | 54.90% | 1.22 | 1061.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 61.312µs | 10370 | 0 | 54.90% | 1.22 | 995.48 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 217.155µs | 10370 | 2000 | 54.89% | 1.22 | 281.07 MB/s |
| Quicksort | 10000 | 711.725µs | 136866 | 0 | 54.87% | 1.22 | 857.57 MB/s |
| Timsort | 10000 | 855.746µs | 141490 | 0 | 54.86% | 1.22 | 713.24 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.547421ms | 0 | 30000 | 54.77% | 1.22 | 110.02 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.051024ms | 0 | 30000 | 54.76% | 1.22 | 100.87 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.966859ms | 193846 | 14351 | 54.86% | 1.22 | 310.32 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 620.037µs | 67438 | 10000 | 54.87% | 1.22 | 984.38 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 245.851µs | 67438 | 0 | 54.87% | 1.22 | 2482.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 235.943µs | 70298 | 0 | 54.87% | 1.22 | 2586.86 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 393.237µs | 63043 | 0 | 54.86% | 1.22 | 1552.12 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 412.186µs | 67007 | 0 | 54.86% | 1.22 | 1480.77 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 244.142µs | 67438 | 0 | 54.87% | 1.22 | 2499.99 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 240.685µs | 67438 | 0 | 54.87% | 1.22 | 2535.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 224.713µs | 67438 | 0 | 54.87% | 1.22 | 2716.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.047758ms | 67438 | 20000 | 54.85% | 1.22 | 582.53 MB/s |
| Quicksort | 100000 | 10.138926ms | 1718762 | 0 | 54.49% | 1.22 | 601.99 MB/s |
| Timsort | 100000 | 11.906236ms | 1759891 | 0 | 54.36% | 1.22 | 512.63 MB/s |
| ARS Gen 1: Foundation | 100000 | 56.405692ms | 0 | 300000 | 51.37% | 1.21 | 108.21 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 58.351246ms | 0 | 300000 | 51.49% | 1.21 | 104.60 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.88756ms | 1895222 | 108703 | 54.77% | 1.22 | 361.42 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.64633ms | 1029722 | 100000 | 54.77% | 1.22 | 1673.88 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.946782ms | 1029722 | 0 | 54.73% | 1.21 | 2071.25 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.171912ms | 1071423 | 0 | 54.71% | 1.21 | 1924.24 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.244466ms | 978520 | 0 | 54.75% | 1.21 | 1881.21 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.577859ms | 1019338 | 0 | 54.72% | 1.21 | 1705.91 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.038439ms | 1029722 | 0 | 54.75% | 1.21 | 2008.77 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.131857ms | 961965 | 0 | 54.78% | 1.21 | 1948.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.044475ms | 1029722 | 0 | 54.74% | 1.21 | 2004.78 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.918477ms | 1029722 | 200000 | 54.52% | 1.22 | 615.37 MB/s |
| Quicksort | 1000000 | 260.32545ms | 20518628 | 0 | 55.00% | 1.23 | 234.46 MB/s |
| Timsort | 1000000 | 340.666845ms | 20902099 | 0 | 53.97% | 1.22 | 179.16 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 194.796093ms | 21589743 | 1017407 | 54.44% | 1.22 | 313.33 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 59.299293ms | 12256776 | 1000000 | 54.99% | 1.17 | 1029.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.243242ms | 12256776 | 0 | 55.10% | 1.18 | 1265.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 60.034932ms | 12679336 | 0 | 54.72% | 1.17 | 1016.66 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 61.066159ms | 13331493 | 0 | 55.11% | 1.18 | 999.49 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 75.294945ms | 13750405 | 0 | 54.85% | 1.17 | 810.61 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 51.132109ms | 9434971 | 0 | 55.30% | 1.18 | 1193.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 71.496517ms | 7913738 | 0 | 55.82% | 1.16 | 853.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 56.987297ms | 8719170 | 0 | 55.54% | 1.17 | 1071.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 431.018725ms | 15199653 | 2000000 | 56.87% | 1.18 | 141.61 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 56.323µs | 10370 | 0 | 54.16% | 1.19 | 1083.66 MB/s |
| Timsort | 1000 | 58.968µs | 10522 | 0 | 54.16% | 1.19 | 1035.06 MB/s |
| ARS Gen 1: Foundation | 1000 | 253.53µs | 0 | 2000 | 54.16% | 1.19 | 240.74 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 292.695µs | 0 | 2000 | 54.16% | 1.19 | 208.53 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 56.152µs | 10370 | 0 | 54.16% | 1.19 | 1086.96 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 58.472µs | 10370 | 0 | 54.16% | 1.19 | 1043.84 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 56.723µs | 10370 | 0 | 54.16% | 1.19 | 1076.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 70.748µs | 10522 | 0 | 54.16% | 1.19 | 862.71 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 58.045µs | 10370 | 0 | 54.16% | 1.19 | 1051.51 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 62.763µs | 10522 | 0 | 54.16% | 1.19 | 972.47 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 57.812µs | 10370 | 0 | 54.16% | 1.19 | 1055.75 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 55.324µs | 10370 | 0 | 54.16% | 1.19 | 1103.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 56.2µs | 10370 | 0 | 54.16% | 1.19 | 1086.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 219.193µs | 10370 | 2000 | 54.16% | 1.19 | 278.45 MB/s |
| Quicksort | 10000 | 680.005µs | 136866 | 0 | 54.14% | 1.19 | 897.57 MB/s |
| Timsort | 10000 | 881.897µs | 141490 | 0 | 54.13% | 1.19 | 692.09 MB/s |
| ARS Gen 1: Foundation | 10000 | 9.831886ms | 0 | 30000 | 53.74% | 1.19 | 62.08 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.346465ms | 0 | 30000 | 53.97% | 1.19 | 96.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.073239ms | 193846 | 14351 | 54.13% | 1.19 | 294.40 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 662.543µs | 67438 | 10000 | 54.13% | 1.19 | 921.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 242.552µs | 67438 | 0 | 54.14% | 1.19 | 2516.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 254.647µs | 70298 | 0 | 54.13% | 1.19 | 2396.85 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 401.459µs | 63043 | 0 | 54.13% | 1.19 | 1520.33 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 420.262µs | 67007 | 0 | 54.13% | 1.19 | 1452.31 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 238.538µs | 67438 | 0 | 54.14% | 1.19 | 2558.72 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 239.158µs | 67438 | 0 | 54.14% | 1.19 | 2552.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 237.897µs | 67438 | 0 | 54.14% | 1.19 | 2565.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.033705ms | 67438 | 20000 | 54.12% | 1.19 | 590.45 MB/s |
| Quicksort | 100000 | 9.260884ms | 1718762 | 0 | 53.79% | 1.20 | 659.06 MB/s |
| Timsort | 100000 | 12.509526ms | 1759891 | 0 | 53.68% | 1.20 | 487.91 MB/s |
| ARS Gen 1: Foundation | 100000 | 67.028651ms | 0 | 300000 | 50.81% | 1.18 | 91.06 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 72.523272ms | 0 | 300000 | 50.82% | 1.18 | 84.16 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.778154ms | 1895222 | 108703 | 54.06% | 1.20 | 343.32 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.762552ms | 1029722 | 100000 | 54.05% | 1.19 | 1281.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.949238ms | 1029722 | 0 | 54.00% | 1.19 | 2069.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.122011ms | 1071423 | 0 | 54.01% | 1.19 | 1954.99 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.191557ms | 978520 | 0 | 54.03% | 1.19 | 1912.39 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.515448ms | 1019338 | 0 | 54.03% | 1.19 | 1736.20 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.159218ms | 1029722 | 0 | 54.04% | 1.19 | 1931.97 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.11637ms | 961965 | 0 | 54.06% | 1.19 | 1958.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.083077ms | 1029722 | 0 | 54.03% | 1.19 | 1979.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.778871ms | 1029722 | 200000 | 53.81% | 1.19 | 566.25 MB/s |
| Quicksort | 1000000 | 230.941159ms | 20518628 | 0 | 54.46% | 1.22 | 264.29 MB/s |
| Timsort | 1000000 | 347.142073ms | 20902099 | 0 | 53.49% | 1.20 | 175.82 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 194.116675ms | 21589743 | 1017407 | 53.79% | 1.19 | 314.43 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.102372ms | 12256776 | 1000000 | 54.33% | 1.16 | 1015.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 52.022308ms | 12256776 | 0 | 54.22% | 1.16 | 1173.25 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 58.218699ms | 12679336 | 0 | 54.06% | 1.15 | 1048.38 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 68.102449ms | 13331493 | 0 | 54.33% | 1.15 | 896.23 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 77.915038ms | 13750405 | 0 | 54.21% | 1.14 | 783.36 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 53.431733ms | 9434971 | 0 | 54.56% | 1.16 | 1142.30 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 71.704755ms | 7913738 | 0 | 55.11% | 1.15 | 851.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 58.598558ms | 8719170 | 0 | 54.88% | 1.15 | 1041.58 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 427.75933ms | 15151080 | 2000000 | 56.19% | 1.17 | 142.69 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 51.86µs | 10370 | 0 | 53.20% | 1.18 | 1176.92 MB/s |
| Timsort | 1000 | 65.813µs | 10522 | 0 | 53.20% | 1.18 | 927.40 MB/s |
| ARS Gen 1: Foundation | 1000 | 253.599µs | 0 | 2000 | 53.20% | 1.18 | 240.68 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 287.065µs | 0 | 2000 | 53.20% | 1.18 | 212.62 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 59.795µs | 10370 | 0 | 53.20% | 1.18 | 1020.74 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 58.179µs | 10370 | 0 | 53.20% | 1.18 | 1049.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 58.587µs | 10370 | 0 | 53.20% | 1.18 | 1041.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 62.786µs | 10522 | 0 | 53.20% | 1.18 | 972.11 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 58.218µs | 10370 | 0 | 53.20% | 1.18 | 1048.39 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 66.456µs | 10522 | 0 | 53.20% | 1.18 | 918.43 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 58.003µs | 10370 | 0 | 53.20% | 1.18 | 1052.28 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 59.558µs | 10370 | 0 | 53.20% | 1.18 | 1024.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 55.156µs | 10370 | 0 | 53.20% | 1.18 | 1106.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 220.187µs | 10370 | 2000 | 53.20% | 1.18 | 277.20 MB/s |
| Quicksort | 10000 | 716.698µs | 136866 | 0 | 53.18% | 1.18 | 851.62 MB/s |
| Timsort | 10000 | 863.851µs | 141490 | 0 | 53.17% | 1.18 | 706.55 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.750365ms | 0 | 30000 | 53.08% | 1.18 | 106.14 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 6.081647ms | 0 | 30000 | 53.07% | 1.18 | 100.36 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.068743ms | 193846 | 14351 | 53.17% | 1.18 | 295.03 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 614.517µs | 67438 | 10000 | 53.17% | 1.18 | 993.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 223.669µs | 67438 | 0 | 53.17% | 1.18 | 2728.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 231.765µs | 70298 | 0 | 53.17% | 1.18 | 2633.49 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 390.489µs | 63043 | 0 | 53.17% | 1.18 | 1563.04 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 423.668µs | 67007 | 0 | 53.17% | 1.18 | 1440.64 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 235.366µs | 67438 | 0 | 53.17% | 1.18 | 2593.20 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 235.629µs | 67438 | 0 | 53.17% | 1.18 | 2590.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 234.78µs | 67438 | 0 | 53.17% | 1.18 | 2599.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.029558ms | 67438 | 20000 | 53.16% | 1.18 | 592.83 MB/s |
| Quicksort | 100000 | 10.389432ms | 1718762 | 0 | 52.82% | 1.18 | 587.47 MB/s |
| Timsort | 100000 | 12.062012ms | 1759891 | 0 | 52.72% | 1.18 | 506.01 MB/s |
| ARS Gen 1: Foundation | 100000 | 59.677939ms | 0 | 300000 | 50.08% | 1.17 | 102.27 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 61.749572ms | 0 | 300000 | 50.40% | 1.18 | 98.84 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.16878ms | 1895222 | 108703 | 53.09% | 1.18 | 377.49 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.119863ms | 1029722 | 100000 | 53.09% | 1.18 | 1481.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.875017ms | 1029722 | 0 | 53.06% | 1.18 | 2122.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.129688ms | 1071423 | 0 | 53.07% | 1.18 | 1950.20 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.275199ms | 978520 | 0 | 53.06% | 1.18 | 1863.56 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.651027ms | 1019338 | 0 | 53.06% | 1.18 | 1671.73 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.018949ms | 1029722 | 0 | 53.06% | 1.18 | 2021.74 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.18621ms | 961965 | 0 | 53.10% | 1.18 | 1915.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.116806ms | 1029722 | 0 | 53.07% | 1.18 | 1958.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 10.109551ms | 1029722 | 200000 | 52.87% | 1.18 | 603.74 MB/s |
| Quicksort | 1000000 | 260.615062ms | 20518628 | 0 | 53.70% | 1.20 | 234.20 MB/s |
| Timsort | 1000000 | 385.179664ms | 20902099 | 0 | 52.80% | 1.18 | 158.46 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 200.495581ms | 21589743 | 1017407 | 52.83% | 1.19 | 304.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 62.509413ms | 12256776 | 1000000 | 53.45% | 1.15 | 976.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 48.91037ms | 12256776 | 0 | 53.54% | 1.15 | 1247.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 56.956404ms | 12679336 | 0 | 53.37% | 1.14 | 1071.61 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 64.508886ms | 13331493 | 0 | 53.52% | 1.15 | 946.15 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 74.353074ms | 13750405 | 0 | 53.33% | 1.14 | 820.88 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.602583ms | 9434971 | 0 | 53.65% | 1.15 | 1160.31 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 71.696258ms | 7913738 | 0 | 54.17% | 1.14 | 851.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.24506ms | 8719170 | 0 | 53.95% | 1.15 | 1066.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 426.219818ms | 15151080 | 2000000 | 55.08% | 1.15 | 143.20 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 52.761µs | 10370 | 0 | 52.60% | 1.17 | 1156.82 MB/s |
| Timsort | 1000 | 67.976µs | 10522 | 0 | 52.60% | 1.17 | 897.89 MB/s |
| ARS Gen 1: Foundation | 1000 | 282.781µs | 0 | 2000 | 52.60% | 1.17 | 215.84 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 314.912µs | 0 | 2000 | 52.60% | 1.17 | 193.82 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 58.03µs | 10370 | 0 | 52.60% | 1.17 | 1051.79 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 54.504µs | 10370 | 0 | 52.60% | 1.17 | 1119.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 56.284µs | 10370 | 0 | 52.60% | 1.17 | 1084.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 65.261µs | 10522 | 0 | 52.60% | 1.17 | 935.25 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 56.104µs | 10370 | 0 | 52.60% | 1.17 | 1087.89 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 64.793µs | 10522 | 0 | 52.60% | 1.17 | 942.00 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 59.516µs | 10370 | 0 | 52.60% | 1.17 | 1025.53 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 57.931µs | 10370 | 0 | 52.60% | 1.17 | 1053.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 55.646µs | 10370 | 0 | 52.60% | 1.17 | 1096.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 206.108µs | 10370 | 2000 | 52.60% | 1.17 | 296.13 MB/s |
| Quicksort | 10000 | 801.927µs | 136866 | 0 | 52.58% | 1.17 | 761.11 MB/s |
| Timsort | 10000 | 921.82µs | 141490 | 0 | 52.58% | 1.17 | 662.12 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.669858ms | 0 | 30000 | 52.50% | 1.17 | 107.65 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.660062ms | 0 | 30000 | 52.49% | 1.17 | 107.83 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.961532ms | 193846 | 14351 | 52.58% | 1.17 | 311.16 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 639.481µs | 67438 | 10000 | 52.58% | 1.17 | 954.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 245.236µs | 67438 | 0 | 52.58% | 1.17 | 2488.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 241.081µs | 70298 | 0 | 52.58% | 1.17 | 2531.73 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 422.098µs | 63043 | 0 | 52.58% | 1.17 | 1445.99 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 518.904µs | 67007 | 0 | 52.57% | 1.17 | 1176.23 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 229.878µs | 67438 | 0 | 52.58% | 1.17 | 2655.11 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 227.84µs | 67438 | 0 | 52.58% | 1.17 | 2678.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 215.939µs | 67438 | 0 | 52.58% | 1.17 | 2826.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.096389ms | 67438 | 20000 | 52.57% | 1.17 | 556.69 MB/s |
| Quicksort | 100000 | 9.490493ms | 1718762 | 0 | 52.27% | 1.18 | 643.12 MB/s |
| Timsort | 100000 | 12.449099ms | 1759891 | 0 | 52.15% | 1.18 | 490.28 MB/s |
| ARS Gen 1: Foundation | 100000 | 61.1379ms | 0 | 300000 | 49.72% | 1.17 | 99.83 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 60.690343ms | 0 | 300000 | 49.14% | 1.16 | 100.57 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.499868ms | 1895222 | 108703 | 52.51% | 1.18 | 369.91 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.383018ms | 1029722 | 100000 | 52.51% | 1.17 | 1392.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.91319ms | 1029722 | 0 | 52.50% | 1.17 | 2095.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.185871ms | 1071423 | 0 | 52.48% | 1.17 | 1915.81 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.638299ms | 978520 | 0 | 52.48% | 1.17 | 1677.57 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.605821ms | 1019338 | 0 | 52.49% | 1.17 | 1692.68 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.041021ms | 1029722 | 0 | 52.49% | 1.17 | 2007.06 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.14275ms | 961965 | 0 | 52.52% | 1.17 | 1942.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.055699ms | 1029722 | 0 | 52.49% | 1.17 | 1997.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.747045ms | 1029722 | 200000 | 52.30% | 1.17 | 626.19 MB/s |
| Quicksort | 1000000 | 259.326142ms | 20518628 | 0 | 53.08% | 1.19 | 235.36 MB/s |
| Timsort | 1000000 | 385.041852ms | 20902099 | 0 | 52.32% | 1.17 | 158.52 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 199.281894ms | 21589743 | 1017407 | 52.37% | 1.18 | 306.28 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 60.082753ms | 12256776 | 1000000 | 52.85% | 1.15 | 1015.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 53.965215ms | 12256776 | 0 | 52.90% | 1.14 | 1131.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 60.528469ms | 12679336 | 0 | 52.61% | 1.13 | 1008.37 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 65.720857ms | 13331493 | 0 | 52.94% | 1.14 | 928.70 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 84.334452ms | 13750405 | 0 | 52.96% | 1.13 | 723.73 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 62.353756ms | 9434971 | 0 | 53.20% | 1.14 | 978.85 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 73.121659ms | 7913738 | 0 | 53.64% | 1.13 | 834.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 65.338107ms | 8719170 | 0 | 53.43% | 1.13 | 934.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 455.648917ms | 15199655 | 2000000 | 55.09% | 1.14 | 133.95 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 32.22µs | 5636 | 0 | 52.17% | 1.16 | 1894.33 MB/s |
| Timsort | 1000 | 37.763µs | 5782 | 0 | 52.17% | 1.16 | 1616.27 MB/s |
| ARS Gen 1: Foundation | 1000 | 130.9µs | 984 | 2000 | 52.16% | 1.16 | 466.27 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 119.761µs | 984 | 2000 | 52.16% | 1.16 | 509.64 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 35.769µs | 5636 | 0 | 52.17% | 1.16 | 1706.37 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 40.024µs | 5636 | 0 | 52.17% | 1.16 | 1524.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 41.594µs | 5636 | 0 | 52.17% | 1.16 | 1467.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 40.684µs | 5782 | 0 | 52.17% | 1.16 | 1500.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 34.94µs | 5636 | 0 | 52.17% | 1.16 | 1746.86 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 37.393µs | 5782 | 0 | 52.17% | 1.16 | 1632.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 45.898µs | 5636 | 0 | 52.17% | 1.16 | 1329.80 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 42.835µs | 5636 | 0 | 52.17% | 1.16 | 1424.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 36.607µs | 5636 | 0 | 52.17% | 1.16 | 1667.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 183.885µs | 5636 | 2000 | 52.16% | 1.16 | 331.92 MB/s |
| Quicksort | 10000 | 328.124µs | 53113 | 0 | 52.15% | 1.16 | 1860.12 MB/s |
| Timsort | 10000 | 358.109µs | 54714 | 0 | 52.14% | 1.16 | 1704.37 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.491792ms | 9984 | 30000 | 52.13% | 1.16 | 409.14 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.547464ms | 9984 | 30000 | 52.13% | 1.16 | 394.42 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.744895ms | 122389 | 14351 | 52.14% | 1.16 | 349.79 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 616.231µs | 14075 | 10000 | 52.14% | 1.16 | 990.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 207.741µs | 14075 | 0 | 52.15% | 1.16 | 2938.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 165.213µs | 14094 | 0 | 52.14% | 1.16 | 3694.33 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 380.832µs | 12021 | 0 | 52.14% | 1.16 | 1602.68 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 367.345µs | 12028 | 0 | 52.14% | 1.16 | 1661.52 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 145.252µs | 14075 | 0 | 52.14% | 1.16 | 4202.02 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 163.186µs | 14075 | 0 | 52.14% | 1.16 | 3740.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 154.99µs | 14075 | 0 | 52.14% | 1.16 | 3938.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 986.552µs | 14075 | 20000 | 52.14% | 1.16 | 618.67 MB/s |
| Quicksort | 100000 | 4.211817ms | 516589 | 0 | 51.97% | 1.16 | 1449.14 MB/s |
| Timsort | 100000 | 4.750944ms | 529550 | 0 | 51.90% | 1.16 | 1284.70 MB/s |
| ARS Gen 1: Foundation | 100000 | 15.879942ms | 99984 | 300000 | 52.00% | 1.16 | 384.35 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 20.029764ms | 99984 | 300000 | 52.10% | 1.16 | 304.72 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 15.868509ms | 1144965 | 108703 | 52.12% | 1.16 | 384.63 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.078158ms | 151083 | 100000 | 52.14% | 1.16 | 1496.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.039104ms | 151083 | 0 | 52.14% | 1.16 | 2993.23 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.349076ms | 151309 | 0 | 52.14% | 1.16 | 2598.26 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.039332ms | 99990 | 0 | 52.13% | 1.16 | 2992.90 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.144702ms | 99990 | 0 | 52.13% | 1.16 | 2845.86 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.378425ms | 200008 | 0 | 52.05% | 1.16 | 2566.20 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.440742ms | 200008 | 0 | 52.08% | 1.16 | 2500.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.146449ms | 100024 | 0 | 52.12% | 1.16 | 2843.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.113351ms | 151083 | 200000 | 51.95% | 1.16 | 669.73 MB/s |
| Quicksort | 1000000 | 96.38632ms | 5202060 | 0 | 53.29% | 1.16 | 633.23 MB/s |
| Timsort | 1000000 | 150.920329ms | 6111262 | 0 | 53.39% | 1.16 | 404.42 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 163.924358ms | 12085476 | 1017407 | 52.31% | 1.18 | 372.34 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 37.091355ms | 999988 | 1000000 | 52.58% | 1.15 | 1645.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 25.955042ms | 999988 | 0 | 52.58% | 1.15 | 2351.57 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 22.750056ms | 999988 | 0 | 52.52% | 1.15 | 2682.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 29.968296ms | 999988 | 0 | 52.59% | 1.15 | 2036.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 26.478171ms | 999988 | 0 | 52.58% | 1.15 | 2305.11 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 33.926238ms | 1999972 | 0 | 52.64% | 1.14 | 1799.05 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 43.118565ms | 1999976 | 0 | 52.85% | 1.14 | 1415.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 37.408657ms | 1999976 | 0 | 52.67% | 1.14 | 1631.58 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 329.208183ms | 5709060 | 2000000 | 54.84% | 1.14 | 185.40 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 64.345µs | 10308 | 0 | 52.04% | 1.15 | 948.56 MB/s |
| Timsort | 1000 | 65.451µs | 10658 | 0 | 52.04% | 1.15 | 932.53 MB/s |
| ARS Gen 1: Foundation | 1000 | 149.276µs | 10308 | 2000 | 52.04% | 1.15 | 408.87 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 144.482µs | 10308 | 2000 | 52.04% | 1.15 | 422.44 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 70.986µs | 10308 | 0 | 52.04% | 1.15 | 859.82 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 74.567µs | 10308 | 0 | 52.04% | 1.15 | 818.53 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 54.273µs | 10308 | 0 | 52.04% | 1.15 | 1124.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 73.387µs | 10658 | 0 | 52.04% | 1.15 | 831.69 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 53.962µs | 10308 | 0 | 52.04% | 1.15 | 1131.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 63.741µs | 10658 | 0 | 52.04% | 1.15 | 957.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 70.347µs | 10308 | 0 | 52.04% | 1.15 | 867.63 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 70.07µs | 10308 | 0 | 52.04% | 1.15 | 871.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 73.076µs | 10308 | 0 | 52.04% | 1.15 | 835.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 276.836µs | 10308 | 2000 | 52.04% | 1.15 | 220.47 MB/s |
| Quicksort | 10000 | 768.517µs | 138349 | 0 | 52.00% | 1.15 | 794.19 MB/s |
| Timsort | 10000 | 939.291µs | 142268 | 0 | 51.99% | 1.15 | 649.80 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.308003ms | 138349 | 30000 | 51.97% | 1.15 | 264.45 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.433134ms | 138349 | 30000 | 51.97% | 1.15 | 250.85 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.445592ms | 193925 | 14351 | 52.00% | 1.15 | 249.57 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.589646ms | 138355 | 10000 | 52.00% | 1.15 | 383.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 943.378µs | 138355 | 0 | 51.99% | 1.15 | 646.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.186461ms | 142274 | 0 | 51.98% | 1.15 | 514.43 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 996.749µs | 138355 | 0 | 51.98% | 1.15 | 612.34 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.45619ms | 142274 | 0 | 51.98% | 1.15 | 419.14 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 984.444µs | 138355 | 0 | 51.99% | 1.15 | 620.00 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.089664ms | 138355 | 0 | 51.99% | 1.15 | 560.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.026321ms | 138355 | 0 | 51.99% | 1.15 | 594.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.959447ms | 138355 | 20000 | 51.97% | 1.15 | 311.49 MB/s |
| Quicksort | 100000 | 18.037386ms | 1715173 | 0 | 51.86% | 1.15 | 338.38 MB/s |
| Timsort | 100000 | 18.813149ms | 1762853 | 0 | 51.74% | 1.15 | 324.43 MB/s |
| ARS Gen 1: Foundation | 100000 | 51.815648ms | 1715173 | 300000 | 51.85% | 1.15 | 117.79 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 51.490299ms | 1715173 | 300000 | 51.81% | 1.15 | 118.54 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 20.666934ms | 1895407 | 108703 | 52.02% | 1.15 | 295.33 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 24.594528ms | 1715179 | 100000 | 51.89% | 1.15 | 248.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 22.908567ms | 1715179 | 0 | 51.83% | 1.15 | 266.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 31.555977ms | 1762859 | 0 | 51.80% | 1.15 | 193.42 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 22.462981ms | 1715179 | 0 | 51.82% | 1.15 | 271.71 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 33.637329ms | 1762859 | 0 | 51.84% | 1.15 | 181.45 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 22.463207ms | 1715179 | 0 | 51.82% | 1.15 | 271.71 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 23.076067ms | 1715179 | 0 | 51.87% | 1.15 | 264.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 22.517597ms | 1715179 | 0 | 51.81% | 1.15 | 271.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 37.16043ms | 1715179 | 200000 | 51.63% | 1.15 | 164.25 MB/s |
| Quicksort | 1000000 | 579.948804ms | 20523276 | 0 | 54.51% | 1.12 | 105.24 MB/s |
| Timsort | 1000000 | 768.190844ms | 20914644 | 0 | 54.20% | 1.10 | 79.45 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 261.677614ms | 21586854 | 1017407 | 51.73% | 1.13 | 233.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 505.74989ms | 20523280 | 1000000 | 54.12% | 1.13 | 120.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 547.571831ms | 20523280 | 0 | 54.70% | 1.12 | 111.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 640.34219ms | 20914648 | 0 | 53.58% | 1.11 | 95.32 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 526.521135ms | 20523280 | 0 | 54.58% | 1.12 | 115.92 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 711.730447ms | 20914648 | 0 | 54.27% | 1.10 | 85.76 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 456.106561ms | 20523280 | 0 | 54.15% | 1.13 | 133.82 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 441.044172ms | 20523280 | 0 | 54.07% | 1.14 | 138.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 461.779888ms | 20523280 | 0 | 54.13% | 1.13 | 132.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 681.266013ms | 21815112 | 2000000 | 56.85% | 1.08 | 89.59 MB/s |

## Category: Custom

### Distribution: Random

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 20.882µs | 10378 | 0 | 63.42% | 1.04 | 2192.14 MB/s |
| Timsort | 1000 | 28.437µs | 10965 | 0 | 63.42% | 1.04 | 1609.75 MB/s |
| ARS Gen 1: Foundation | 1000 | 215.892µs | 0 | 2000 | 63.42% | 1.04 | 212.03 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 296.427µs | 0 | 2000 | 63.42% | 1.04 | 154.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 20.244µs | 10378 | 0 | 63.42% | 1.04 | 2261.23 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 20.3µs | 10378 | 0 | 63.42% | 1.04 | 2254.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 19.7µs | 10378 | 0 | 63.42% | 1.04 | 2323.67 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 27.38µs | 10965 | 0 | 63.42% | 1.04 | 1671.89 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 19.282µs | 10378 | 0 | 63.42% | 1.04 | 2374.05 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 26.425µs | 10965 | 0 | 63.42% | 1.04 | 1732.31 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 19.3µs | 10378 | 0 | 63.42% | 1.04 | 2371.83 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 19.031µs | 10378 | 0 | 63.42% | 1.04 | 2405.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.061µs | 10378 | 0 | 63.42% | 1.04 | 2401.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 105.105µs | 10378 | 2000 | 63.42% | 1.04 | 435.53 MB/s |
| Quicksort | 10000 | 230.609µs | 138485 | 0 | 63.41% | 1.04 | 1985.02 MB/s |
| Timsort | 10000 | 321.28µs | 142802 | 0 | 63.41% | 1.04 | 1424.81 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.798336ms | 0 | 30000 | 63.35% | 1.04 | 95.40 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.068792ms | 0 | 30000 | 63.33% | 1.04 | 90.31 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 426.793µs | 194235 | 14351 | 63.41% | 1.04 | 1072.57 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 222.455µs | 53078 | 10000 | 63.41% | 1.04 | 2057.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 139.293µs | 53078 | 0 | 63.41% | 1.04 | 3286.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 147.639µs | 57974 | 0 | 63.41% | 1.04 | 3100.56 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 226.608µs | 60130 | 0 | 63.40% | 1.04 | 2020.07 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 237.859µs | 62739 | 0 | 63.40% | 1.04 | 1924.52 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 158.268µs | 53078 | 0 | 63.41% | 1.04 | 2892.33 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 136.532µs | 53078 | 0 | 63.41% | 1.04 | 3352.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 134.595µs | 53078 | 0 | 63.41% | 1.04 | 3401.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 294.809µs | 53078 | 20000 | 63.40% | 1.04 | 1552.75 MB/s |
| Quicksort | 100000 | 2.844443ms | 1716233 | 0 | 63.36% | 1.04 | 1609.33 MB/s |
| Timsort | 100000 | 4.398314ms | 1759914 | 0 | 63.31% | 1.04 | 1040.77 MB/s |
| ARS Gen 1: Foundation | 100000 | 36.54586ms | 0 | 300000 | 61.22% | 1.04 | 125.26 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 37.956313ms | 0 | 300000 | 61.12% | 1.04 | 120.60 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.887327ms | 1895170 | 108703 | 63.35% | 1.04 | 1585.42 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.58115ms | 891495 | 100000 | 63.38% | 1.04 | 2895.13 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.37204ms | 891495 | 0 | 63.38% | 1.04 | 3336.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.517076ms | 927102 | 0 | 63.38% | 1.04 | 3017.41 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.704921ms | 954799 | 0 | 63.36% | 1.04 | 2684.96 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.777394ms | 993233 | 0 | 63.36% | 1.04 | 2575.48 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.401139ms | 891495 | 0 | 63.37% | 1.04 | 3267.08 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.53045ms | 780845 | 0 | 63.37% | 1.04 | 2991.04 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.546327ms | 891495 | 0 | 63.37% | 1.04 | 2960.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.765425ms | 891495 | 200000 | 63.37% | 1.04 | 2592.94 MB/s |
| Quicksort | 1000000 | 42.123751ms | 20512439 | 0 | 62.97% | 1.05 | 1086.71 MB/s |
| Timsort | 1000000 | 72.069739ms | 20899150 | 0 | 62.49% | 1.05 | 635.17 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 35.669928ms | 21596717 | 1017407 | 63.08% | 1.04 | 1283.33 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.829341ms | 10310056 | 1000000 | 63.39% | 1.03 | 2431.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.277109ms | 10310056 | 0 | 63.39% | 1.03 | 2649.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.244821ms | 10709205 | 0 | 63.36% | 1.03 | 2378.63 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.067303ms | 13007245 | 0 | 63.33% | 1.03 | 2533.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.449922ms | 13425517 | 0 | 63.24% | 1.03 | 2238.46 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.106904ms | 10310056 | 0 | 63.36% | 1.03 | 2395.80 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.725935ms | 11367051 | 0 | 63.42% | 1.04 | 2208.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 19.89007ms | 12398342 | 0 | 63.41% | 1.04 | 2301.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.36156ms | 12262439 | 2000000 | 63.46% | 1.03 | 1055.69 MB/s |

### Distribution: Gaussian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.141µs | 10308 | 0 | 63.42% | 1.03 | 1820.79 MB/s |
| Timsort | 1000 | 34.82µs | 10818 | 0 | 63.42% | 1.03 | 1314.66 MB/s |
| ARS Gen 1: Foundation | 1000 | 215.939µs | 458 | 2000 | 63.42% | 1.03 | 211.99 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 240.233µs | 458 | 2000 | 63.42% | 1.03 | 190.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 25.214µs | 10308 | 0 | 63.42% | 1.03 | 1815.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 25.823µs | 10308 | 0 | 63.42% | 1.03 | 1772.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 26.14µs | 10308 | 0 | 63.42% | 1.03 | 1751.20 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 35.744µs | 10818 | 0 | 63.42% | 1.03 | 1280.67 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 25.205µs | 10308 | 0 | 63.42% | 1.03 | 1816.16 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 35.846µs | 10818 | 0 | 63.42% | 1.03 | 1277.03 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 25.089µs | 10308 | 0 | 63.42% | 1.03 | 1824.56 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 25.323µs | 10308 | 0 | 63.42% | 1.03 | 1807.70 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 25.102µs | 10308 | 0 | 63.42% | 1.03 | 1823.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 142.905µs | 10308 | 2000 | 63.42% | 1.03 | 320.33 MB/s |
| Quicksort | 10000 | 310.857µs | 135501 | 0 | 63.41% | 1.03 | 1472.59 MB/s |
| Timsort | 10000 | 412.263µs | 140463 | 0 | 63.41% | 1.03 | 1110.37 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.48935ms | 53061 | 30000 | 63.40% | 1.03 | 307.36 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.556017ms | 53088 | 30000 | 63.40% | 1.03 | 294.19 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 476.284µs | 191553 | 14351 | 63.41% | 1.03 | 961.11 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 263.588µs | 59910 | 10000 | 63.41% | 1.03 | 1736.66 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 168.552µs | 59910 | 0 | 63.41% | 1.03 | 2715.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 178.214µs | 62899 | 0 | 63.41% | 1.03 | 2568.62 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 378.481µs | 59126 | 0 | 63.40% | 1.03 | 1209.48 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 424.353µs | 61853 | 0 | 63.40% | 1.03 | 1078.73 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 193.727µs | 59910 | 0 | 63.41% | 1.03 | 2362.93 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 179.493µs | 59910 | 0 | 63.41% | 1.03 | 2550.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 221.08µs | 59910 | 0 | 63.41% | 1.03 | 2070.58 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 378.738µs | 59910 | 20000 | 63.40% | 1.03 | 1208.66 MB/s |
| Quicksort | 100000 | 2.604864ms | 1420515 | 0 | 63.37% | 1.03 | 1757.34 MB/s |
| Timsort | 100000 | 3.524642ms | 1424196 | 0 | 63.31% | 1.03 | 1298.75 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.322091ms | 1360088 | 300000 | 63.29% | 1.03 | 443.48 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 11.677635ms | 1360044 | 300000 | 63.29% | 1.03 | 392.00 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.219902ms | 1616363 | 108703 | 63.36% | 1.03 | 1421.67 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.427683ms | 713263 | 100000 | 63.38% | 1.03 | 3206.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.330207ms | 713263 | 0 | 63.38% | 1.03 | 3441.30 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.480081ms | 718641 | 0 | 63.38% | 1.03 | 3092.83 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.64367ms | 681503 | 0 | 63.37% | 1.03 | 2785.01 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.63354ms | 688539 | 0 | 63.37% | 1.03 | 2802.28 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.419561ms | 713263 | 0 | 63.38% | 1.03 | 3224.68 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.495924ms | 609629 | 0 | 63.38% | 1.03 | 3060.07 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.509898ms | 713263 | 0 | 63.38% | 1.03 | 3031.75 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.797349ms | 713263 | 200000 | 63.38% | 1.03 | 2546.88 MB/s |
| Quicksort | 1000000 | 27.000043ms | 13518116 | 0 | 62.99% | 1.03 | 1695.42 MB/s |
| Timsort | 1000000 | 51.910181ms | 14666956 | 0 | 62.45% | 1.03 | 881.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.575919ms | 14952891 | 1017407 | 63.09% | 1.03 | 1405.22 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 16.004598ms | 4752528 | 1000000 | 63.46% | 1.02 | 2860.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.864755ms | 4752528 | 0 | 63.46% | 1.02 | 2885.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.658231ms | 4776632 | 0 | 63.39% | 1.02 | 2592.35 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.474748ms | 6246227 | 0 | 63.34% | 1.02 | 2958.13 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.154388ms | 6276231 | 0 | 63.18% | 1.02 | 2521.50 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 16.064264ms | 4706394 | 0 | 63.42% | 1.02 | 2849.58 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 22.838529ms | 2307619 | 0 | 63.35% | 1.02 | 2004.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 22.741716ms | 2550221 | 0 | 63.39% | 1.02 | 2012.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.785264ms | 11446688 | 2000000 | 63.42% | 1.02 | 1022.13 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 23.894µs | 9427 | 0 | 63.48% | 1.02 | 1915.81 MB/s |
| Timsort | 1000 | 28.604µs | 9314 | 0 | 63.48% | 1.02 | 1600.35 MB/s |
| ARS Gen 1: Foundation | 1000 | 128.356µs | 9547 | 2000 | 63.48% | 1.02 | 356.64 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 132.985µs | 9540 | 2000 | 63.48% | 1.02 | 344.22 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 21.868µs | 9427 | 0 | 63.48% | 1.02 | 2093.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 21.578µs | 9427 | 0 | 63.48% | 1.02 | 2121.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.1µs | 9427 | 0 | 63.48% | 1.02 | 2169.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 24.873µs | 9314 | 0 | 63.48% | 1.02 | 1840.40 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.141µs | 9427 | 0 | 63.48% | 1.02 | 2165.29 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 24.847µs | 9314 | 0 | 63.48% | 1.02 | 1842.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.914µs | 9427 | 0 | 63.48% | 1.02 | 2188.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.905µs | 9427 | 0 | 63.48% | 1.02 | 2189.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 21.003µs | 9427 | 0 | 63.48% | 1.02 | 2179.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 130.074µs | 9427 | 2000 | 63.48% | 1.02 | 351.93 MB/s |
| Quicksort | 10000 | 272.175µs | 133978 | 0 | 63.47% | 1.02 | 1681.87 MB/s |
| Timsort | 10000 | 315.709µs | 128297 | 0 | 63.47% | 1.02 | 1449.95 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.124258ms | 126223 | 30000 | 63.46% | 1.02 | 407.17 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.331296ms | 126108 | 30000 | 63.47% | 1.02 | 343.85 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 429.188µs | 183316 | 14351 | 63.47% | 1.02 | 1066.58 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 245.291µs | 42006 | 10000 | 63.47% | 1.02 | 1866.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 187.442µs | 42006 | 0 | 63.47% | 1.02 | 2442.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 167.246µs | 34856 | 0 | 63.47% | 1.02 | 2737.07 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 361.17µs | 48982 | 0 | 63.46% | 1.02 | 1267.45 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 405.076µs | 42275 | 0 | 63.46% | 1.02 | 1130.07 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 216.683µs | 42006 | 0 | 63.47% | 1.02 | 2112.60 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 196.904µs | 42006 | 0 | 63.47% | 1.02 | 2324.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 195.838µs | 42006 | 0 | 63.47% | 1.02 | 2337.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 402.816µs | 42006 | 20000 | 63.46% | 1.02 | 1136.41 MB/s |
| Quicksort | 100000 | 3.458861ms | 1688686 | 0 | 63.43% | 1.02 | 1323.45 MB/s |
| Timsort | 100000 | 4.04526ms | 1619959 | 0 | 63.37% | 1.02 | 1131.61 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.846554ms | 1609619 | 300000 | 63.38% | 1.02 | 422.04 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 10.659356ms | 1609452 | 300000 | 63.38% | 1.02 | 429.45 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.115198ms | 1798628 | 108703 | 63.44% | 1.02 | 1469.45 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.538324ms | 801237 | 100000 | 63.44% | 1.02 | 2975.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.363117ms | 801237 | 0 | 63.44% | 1.02 | 3358.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.200148ms | 405369 | 0 | 63.44% | 1.02 | 3814.23 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.81959ms | 871959 | 0 | 63.44% | 1.02 | 2515.75 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.579753ms | 443409 | 0 | 63.43% | 1.02 | 2897.69 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.494337ms | 801237 | 0 | 63.43% | 1.02 | 3063.32 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.489564ms | 689539 | 0 | 63.43% | 1.02 | 3073.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.525574ms | 801237 | 0 | 63.44% | 1.02 | 3000.60 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.890327ms | 801237 | 200000 | 63.44% | 1.02 | 2421.61 MB/s |
| Quicksort | 1000000 | 40.455154ms | 20499945 | 0 | 63.09% | 1.03 | 1131.53 MB/s |
| Timsort | 1000000 | 64.258551ms | 19254168 | 0 | 62.58% | 1.02 | 712.38 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.849946ms | 20728167 | 1017407 | 63.36% | 1.02 | 1437.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.142801ms | 9491317 | 1000000 | 63.49% | 1.01 | 2523.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.425841ms | 9491317 | 0 | 63.49% | 1.01 | 2786.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.365931ms | 4131087 | 0 | 63.49% | 1.01 | 2979.08 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.847714ms | 12332035 | 0 | 63.42% | 1.02 | 2564.83 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 17.356638ms | 5755621 | 0 | 63.44% | 1.01 | 2637.40 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 18.890253ms | 9491317 | 0 | 63.46% | 1.02 | 2423.28 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.578125ms | 10583380 | 0 | 63.48% | 1.01 | 2338.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.852215ms | 11681981 | 0 | 63.47% | 1.02 | 2428.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.264766ms | 14616270 | 2000000 | 63.45% | 1.01 | 1034.15 MB/s |

### Distribution: Duplicates

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 7.096µs | 3761 | 0 | 63.47% | 1.01 | 6451.01 MB/s |
| Timsort | 1000 | 9.306µs | 3799 | 0 | 63.47% | 1.01 | 4919.02 MB/s |
| ARS Gen 1: Foundation | 1000 | 40.947µs | 995 | 2000 | 63.47% | 1.01 | 1117.94 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 53.26µs | 995 | 2000 | 63.47% | 1.01 | 859.49 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 7.066µs | 3761 | 0 | 63.47% | 1.01 | 6478.40 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 7.113µs | 3761 | 0 | 63.47% | 1.01 | 6435.59 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 7.134µs | 3761 | 0 | 63.47% | 1.01 | 6416.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 9.559µs | 3799 | 0 | 63.47% | 1.01 | 4788.82 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 7.078µs | 3761 | 0 | 63.47% | 1.01 | 6467.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 9.42µs | 3799 | 0 | 63.47% | 1.01 | 4859.49 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 7.096µs | 3761 | 0 | 63.47% | 1.01 | 6451.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 7.098µs | 3761 | 0 | 63.47% | 1.01 | 6449.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 7.191µs | 3761 | 0 | 63.47% | 1.01 | 6365.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 121.452µs | 3761 | 2000 | 63.46% | 1.01 | 376.91 MB/s |
| Quicksort | 10000 | 68.389µs | 36513 | 0 | 63.46% | 1.01 | 6693.53 MB/s |
| Timsort | 10000 | 103.769µs | 36606 | 0 | 63.46% | 1.01 | 4411.37 MB/s |
| ARS Gen 1: Foundation | 10000 | 268.03µs | 9995 | 30000 | 63.46% | 1.01 | 1707.88 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 375.565µs | 9995 | 30000 | 63.46% | 1.01 | 1218.87 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 291.812µs | 115165 | 14351 | 63.46% | 1.01 | 1568.69 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 200.435µs | 10001 | 10000 | 63.46% | 1.01 | 2283.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 119.897µs | 10001 | 0 | 63.46% | 1.01 | 3817.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 120.552µs | 10001 | 0 | 63.46% | 1.01 | 3797.23 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 314.685µs | 10001 | 0 | 63.45% | 1.01 | 1454.67 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 306.117µs | 10001 | 0 | 63.45% | 1.01 | 1495.39 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 165.38µs | 10001 | 0 | 63.46% | 1.01 | 2767.95 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 142.773µs | 10001 | 0 | 63.46% | 1.01 | 3206.23 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 147.922µs | 10001 | 0 | 63.46% | 1.01 | 3094.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 314.196µs | 10001 | 20000 | 63.45% | 1.01 | 1456.94 MB/s |
| Quicksort | 100000 | 665.332µs | 362118 | 0 | 63.42% | 1.01 | 6880.23 MB/s |
| Timsort | 100000 | 1.172119ms | 362412 | 0 | 63.39% | 1.01 | 3905.44 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.515747ms | 99995 | 300000 | 63.42% | 1.01 | 1819.59 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.757589ms | 99995 | 300000 | 63.42% | 1.01 | 1660.01 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.561062ms | 1131774 | 108703 | 63.42% | 1.01 | 1787.40 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 923.45µs | 99999 | 100000 | 63.42% | 1.01 | 4957.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 924.862µs | 99999 | 0 | 63.42% | 1.01 | 4949.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 846.091µs | 99999 | 0 | 63.42% | 1.01 | 5410.34 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.002368ms | 99999 | 0 | 63.42% | 1.01 | 4566.82 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 954.147µs | 99999 | 0 | 63.42% | 1.01 | 4797.62 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.268808ms | 199994 | 0 | 63.42% | 1.01 | 3607.82 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.244953ms | 199994 | 0 | 63.42% | 1.01 | 3676.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.252525ms | 199994 | 0 | 63.42% | 1.01 | 3654.73 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.682219ms | 99999 | 200000 | 63.41% | 1.01 | 2721.19 MB/s |
| Quicksort | 1000000 | 10.515765ms | 3806932 | 0 | 63.34% | 1.01 | 4353.12 MB/s |
| Timsort | 1000000 | 31.357195ms | 4710561 | 0 | 63.18% | 1.01 | 1459.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.706965ms | 12059635 | 1017407 | 63.43% | 1.01 | 1652.16 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 15.599019ms | 1000001 | 1000000 | 63.50% | 1.01 | 2934.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 14.86037ms | 1000001 | 0 | 63.49% | 1.01 | 3080.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 14.320322ms | 1000001 | 0 | 63.49% | 1.01 | 3196.60 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.505056ms | 1000001 | 0 | 63.49% | 1.01 | 3660.63 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.585941ms | 1000001 | 0 | 63.49% | 1.01 | 3637.10 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.344527ms | 1999996 | 0 | 63.50% | 1.01 | 2639.24 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 16.53211ms | 1999996 | 0 | 63.51% | 1.01 | 2768.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.37935ms | 1999996 | 0 | 63.51% | 1.01 | 2633.95 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.62533ms | 5365482 | 2000000 | 63.50% | 1.00 | 1049.31 MB/s |

### Distribution: Zipfian

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 11.812µs | 5226 | 0 | 63.45% | 1.00 | 3875.41 MB/s |
| Timsort | 1000 | 16.014µs | 5250 | 0 | 63.45% | 1.00 | 2858.52 MB/s |
| ARS Gen 1: Foundation | 1000 | 46.853µs | 4636 | 2000 | 63.44% | 1.00 | 977.02 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 56.396µs | 4636 | 2000 | 63.44% | 1.00 | 811.70 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.537µs | 5226 | 0 | 63.45% | 1.00 | 4344.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.485µs | 5226 | 0 | 63.45% | 1.00 | 4365.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.215µs | 5226 | 0 | 63.45% | 1.00 | 4481.29 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 13.383µs | 5250 | 0 | 63.45% | 1.00 | 3420.49 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 9.655µs | 5226 | 0 | 63.45% | 1.00 | 4741.21 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 13.227µs | 5250 | 0 | 63.45% | 1.00 | 3460.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 9.596µs | 5226 | 0 | 63.45% | 1.00 | 4770.36 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 9.329µs | 5226 | 0 | 63.45% | 1.00 | 4906.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 9.025µs | 5226 | 0 | 63.45% | 1.00 | 5072.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 101.335µs | 5226 | 2000 | 63.44% | 1.00 | 451.73 MB/s |
| Quicksort | 10000 | 76.887µs | 53591 | 0 | 63.44% | 1.00 | 5953.72 MB/s |
| Timsort | 10000 | 106.917µs | 53226 | 0 | 63.44% | 1.00 | 4281.49 MB/s |
| ARS Gen 1: Foundation | 10000 | 334.977µs | 55100 | 30000 | 63.43% | 1.00 | 1366.55 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 347.622µs | 55099 | 30000 | 63.43% | 1.00 | 1316.84 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 279.37µs | 125304 | 14351 | 63.44% | 1.00 | 1638.56 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 298.331µs | 52153 | 10000 | 63.43% | 1.00 | 1534.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 208.416µs | 52153 | 0 | 63.43% | 1.00 | 2196.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 239.572µs | 50387 | 0 | 63.43% | 1.00 | 1910.76 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 480.724µs | 42939 | 0 | 63.43% | 1.00 | 952.24 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 517.925µs | 43078 | 0 | 63.43% | 1.00 | 883.84 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 247.552µs | 16855 | 0 | 63.43% | 1.00 | 1849.16 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 214.96µs | 52153 | 0 | 63.43% | 1.00 | 2129.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 214.42µs | 52153 | 0 | 63.43% | 1.00 | 2134.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 332.271µs | 52153 | 20000 | 63.43% | 1.00 | 1377.68 MB/s |
| Quicksort | 100000 | 865.217µs | 529990 | 0 | 63.41% | 1.00 | 5290.74 MB/s |
| Timsort | 100000 | 1.44688ms | 531868 | 0 | 63.37% | 1.00 | 3163.80 MB/s |
| ARS Gen 1: Foundation | 100000 | 3.385985ms | 501611 | 300000 | 63.40% | 1.00 | 1351.94 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.547942ms | 501611 | 300000 | 63.40% | 1.00 | 1290.22 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.381379ms | 1172752 | 108703 | 63.41% | 1.00 | 1922.26 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.219303ms | 516727 | 100000 | 63.40% | 1.00 | 2062.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.80117ms | 516727 | 0 | 63.40% | 1.00 | 2541.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.435795ms | 519617 | 0 | 63.36% | 1.00 | 1879.32 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.921613ms | 512024 | 0 | 63.40% | 1.00 | 2382.18 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.58474ms | 502467 | 0 | 63.37% | 1.00 | 1771.02 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.219897ms | 206221 | 0 | 63.37% | 1.00 | 2062.09 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.935815ms | 182412 | 0 | 63.39% | 1.00 | 2364.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.658245ms | 200760 | 0 | 63.37% | 1.00 | 1722.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.10865ms | 516727 | 200000 | 63.39% | 1.00 | 2170.89 MB/s |
| Quicksort | 1000000 | 13.117924ms | 5281309 | 0 | 63.28% | 1.01 | 3489.60 MB/s |
| Timsort | 1000000 | 35.687862ms | 6327917 | 0 | 63.06% | 1.00 | 1282.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 29.616653ms | 12313781 | 1017407 | 63.41% | 1.00 | 1545.63 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 30.663642ms | 5208498 | 1000000 | 63.37% | 1.00 | 1492.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 28.35075ms | 5208498 | 0 | 63.36% | 1.00 | 1614.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 52.204775ms | 6511840 | 0 | 63.07% | 1.00 | 876.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 25.981832ms | 5225265 | 0 | 63.36% | 1.00 | 1761.86 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 48.527964ms | 6529655 | 0 | 63.08% | 1.00 | 943.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 31.901799ms | 1939650 | 0 | 63.53% | 1.00 | 1434.91 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 46.399827ms | 2064127 | 0 | 63.53% | 1.00 | 986.56 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 47.310856ms | 2062304 | 0 | 63.53% | 1.00 | 967.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 52.146084ms | 9657197 | 2000000 | 63.37% | 1.00 | 877.85 MB/s |

### Distribution: Skewed

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 25.98µs | 10133 | 0 | 63.44% | 1.00 | 1761.98 MB/s |
| Timsort | 1000 | 36.865µs | 10734 | 0 | 63.44% | 1.00 | 1241.73 MB/s |
| ARS Gen 1: Foundation | 1000 | 194.088µs | 691 | 2000 | 63.43% | 1.00 | 235.85 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 222.66µs | 691 | 2000 | 63.43% | 1.00 | 205.59 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 21.912µs | 10133 | 0 | 63.44% | 1.00 | 2089.10 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 22.087µs | 10133 | 0 | 63.44% | 1.00 | 2072.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.941µs | 10133 | 0 | 63.44% | 1.00 | 2086.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.853µs | 10734 | 0 | 63.44% | 1.00 | 1533.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 20.861µs | 10133 | 0 | 63.44% | 1.00 | 2194.35 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 28.912µs | 10734 | 0 | 63.44% | 1.00 | 1583.30 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.177µs | 10133 | 0 | 63.44% | 1.00 | 2268.74 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 19.683µs | 10133 | 0 | 63.44% | 1.00 | 2325.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 19.616µs | 10133 | 0 | 63.44% | 1.00 | 2333.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.562µs | 10133 | 2000 | 63.43% | 1.00 | 417.81 MB/s |
| Quicksort | 10000 | 226.593µs | 133996 | 0 | 63.43% | 1.00 | 2020.20 MB/s |
| Timsort | 10000 | 317.458µs | 137398 | 0 | 63.43% | 1.00 | 1441.97 MB/s |
| ARS Gen 1: Foundation | 10000 | 961.313µs | 77629 | 30000 | 63.42% | 1.00 | 476.19 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.037833ms | 77623 | 30000 | 63.42% | 1.00 | 441.08 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 368.697µs | 189660 | 14351 | 63.42% | 1.00 | 1241.57 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 235.356µs | 69470 | 10000 | 63.42% | 1.00 | 1944.98 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 145.522µs | 69470 | 0 | 63.42% | 1.00 | 3145.67 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 160.094µs | 72482 | 0 | 63.42% | 1.00 | 2859.34 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 353.108µs | 59470 | 0 | 63.42% | 1.00 | 1296.38 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 364.242µs | 62562 | 0 | 63.42% | 1.00 | 1256.76 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 177.037µs | 69470 | 0 | 63.42% | 1.00 | 2585.69 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 138.535µs | 69470 | 0 | 63.42% | 1.00 | 3304.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 139.329µs | 69470 | 0 | 63.42% | 1.00 | 3285.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 318.493µs | 69470 | 20000 | 63.42% | 1.00 | 1437.28 MB/s |
| Quicksort | 100000 | 2.194676ms | 1339911 | 0 | 63.39% | 1.00 | 2085.79 MB/s |
| Timsort | 100000 | 3.22085ms | 1340773 | 0 | 63.34% | 1.00 | 1421.25 MB/s |
| ARS Gen 1: Foundation | 100000 | 8.433404ms | 1262245 | 300000 | 63.34% | 1.00 | 542.80 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 9.240185ms | 1262822 | 300000 | 63.34% | 1.00 | 495.41 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.573248ms | 1543517 | 108703 | 63.38% | 1.00 | 1778.93 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.598482ms | 727700 | 100000 | 63.40% | 1.00 | 2863.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.197818ms | 727700 | 0 | 63.40% | 1.00 | 3821.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.439612ms | 737053 | 0 | 63.40% | 1.00 | 3179.77 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.618082ms | 628511 | 0 | 63.39% | 1.00 | 2829.05 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.533064ms | 634320 | 0 | 63.39% | 1.00 | 2985.94 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.443035ms | 701327 | 0 | 63.40% | 1.00 | 3172.23 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.443037ms | 628891 | 0 | 63.40% | 1.00 | 3172.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.319853ms | 727700 | 0 | 63.40% | 1.00 | 3468.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.615234ms | 727700 | 200000 | 63.41% | 1.00 | 2834.04 MB/s |
| Quicksort | 1000000 | 27.171153ms | 12880459 | 0 | 63.09% | 1.01 | 1684.74 MB/s |
| Timsort | 1000000 | 51.907833ms | 13984642 | 0 | 62.68% | 1.00 | 881.88 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 33.523797ms | 14266844 | 1017407 | 63.16% | 1.00 | 1365.49 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.291544ms | 5509338 | 1000000 | 63.44% | 1.00 | 2647.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 16.820151ms | 5509338 | 0 | 63.45% | 0.99 | 2721.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.299269ms | 5538714 | 0 | 63.32% | 0.99 | 2501.54 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.097684ms | 6195850 | 0 | 63.41% | 1.00 | 2843.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.288033ms | 6227611 | 0 | 63.18% | 0.99 | 2503.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.807619ms | 2169826 | 0 | 63.37% | 0.99 | 2007.06 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.409135ms | 1707337 | 0 | 63.39% | 0.99 | 1801.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 24.710147ms | 1857655 | 0 | 63.46% | 0.99 | 1852.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.150921ms | 11901262 | 2000000 | 63.43% | 0.99 | 1036.82 MB/s |

### Distribution: Clustered

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 26.587µs | 9985 | 0 | 63.42% | 0.99 | 1721.76 MB/s |
| Timsort | 1000 | 35.038µs | 10392 | 0 | 63.42% | 0.99 | 1306.48 MB/s |
| ARS Gen 1: Foundation | 1000 | 125.698µs | 5421 | 2000 | 63.42% | 0.99 | 364.18 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 139.318µs | 5356 | 2000 | 63.42% | 0.99 | 328.57 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 22.195µs | 9985 | 0 | 63.42% | 0.99 | 2062.46 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 21.598µs | 9985 | 0 | 63.42% | 0.99 | 2119.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.562µs | 9985 | 0 | 63.42% | 0.99 | 2123.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 30.087µs | 10392 | 0 | 63.42% | 0.99 | 1521.47 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.129µs | 9985 | 0 | 63.42% | 0.99 | 2166.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.349µs | 10392 | 0 | 63.42% | 0.99 | 1559.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 21.115µs | 9985 | 0 | 63.42% | 0.99 | 2167.95 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.367µs | 9985 | 0 | 63.42% | 0.99 | 2247.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.393µs | 9985 | 0 | 63.42% | 0.99 | 2244.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 109.004µs | 9985 | 2000 | 63.42% | 0.99 | 419.95 MB/s |
| Quicksort | 10000 | 163.728µs | 107604 | 0 | 63.42% | 0.99 | 2795.88 MB/s |
| Timsort | 10000 | 238.635µs | 109657 | 0 | 63.42% | 0.99 | 1918.26 MB/s |
| ARS Gen 1: Foundation | 10000 | 469.637µs | 73762 | 30000 | 63.41% | 0.99 | 974.72 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 580.663µs | 73552 | 30000 | 63.41% | 0.99 | 788.35 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 340.503µs | 160276 | 14351 | 63.41% | 0.99 | 1344.37 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 243.806µs | 70340 | 10000 | 63.41% | 0.99 | 1877.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 146.449µs | 70340 | 0 | 63.41% | 0.99 | 3125.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 164.366µs | 71216 | 0 | 63.41% | 0.99 | 2785.03 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 283.869µs | 59344 | 0 | 63.41% | 0.99 | 1612.59 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 295.369µs | 60054 | 0 | 63.41% | 0.99 | 1549.80 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 158.314µs | 70340 | 0 | 63.41% | 0.99 | 2891.49 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 144.744µs | 70340 | 0 | 63.41% | 0.99 | 3162.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 150.186µs | 70340 | 0 | 63.41% | 0.99 | 3047.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 302.219µs | 70340 | 20000 | 63.41% | 0.99 | 1514.68 MB/s |
| Quicksort | 100000 | 1.661048ms | 1011458 | 0 | 63.39% | 1.00 | 2755.87 MB/s |
| Timsort | 100000 | 2.367853ms | 1014769 | 0 | 63.34% | 1.00 | 1933.24 MB/s |
| ARS Gen 1: Foundation | 100000 | 4.245115ms | 696758 | 300000 | 63.39% | 0.99 | 1078.33 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 4.355104ms | 697287 | 300000 | 63.39% | 1.00 | 1051.10 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.649874ms | 1231300 | 108703 | 63.38% | 1.00 | 1727.49 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.512121ms | 671477 | 100000 | 63.39% | 0.99 | 3027.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.316512ms | 671477 | 0 | 63.39% | 0.99 | 3477.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.561601ms | 673524 | 0 | 63.38% | 0.99 | 2931.37 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.261587ms | 554286 | 0 | 63.40% | 0.99 | 3628.47 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.646491ms | 555220 | 0 | 63.39% | 0.99 | 2780.24 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.60172ms | 105158 | 0 | 63.38% | 0.99 | 2857.95 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.615543ms | 179970 | 0 | 63.38% | 0.99 | 2833.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.463837ms | 140724 | 0 | 63.39% | 0.99 | 3127.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.769636ms | 671477 | 200000 | 63.38% | 0.99 | 2586.77 MB/s |
| Quicksort | 1000000 | 21.28888ms | 9937773 | 0 | 63.09% | 1.00 | 2150.25 MB/s |
| Timsort | 1000000 | 47.370775ms | 11004404 | 0 | 62.67% | 1.00 | 966.34 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.960953ms | 12334215 | 1017407 | 63.19% | 1.00 | 1388.81 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 19.009334ms | 4762552 | 1000000 | 63.37% | 0.99 | 2408.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.981195ms | 4762552 | 0 | 63.37% | 0.99 | 2411.67 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 30.319754ms | 4748110 | 0 | 63.24% | 0.98 | 1509.79 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 16.408204ms | 4888204 | 0 | 63.35% | 0.99 | 2789.85 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 27.722016ms | 4904839 | 0 | 63.22% | 0.98 | 1651.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 27.300203ms | 1096506 | 0 | 63.49% | 0.99 | 1676.78 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 29.70106ms | 1053340 | 0 | 63.51% | 0.99 | 1541.24 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 27.210327ms | 1036862 | 0 | 63.51% | 0.99 | 1682.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 46.920811ms | 10645205 | 2000000 | 63.35% | 0.99 | 975.61 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 26.778µs | 10337 | 0 | 63.37% | 0.99 | 1709.48 MB/s |
| Timsort | 1000 | 36.462µs | 10667 | 0 | 63.37% | 0.99 | 1255.45 MB/s |
| ARS Gen 1: Foundation | 1000 | 265.721µs | 0 | 2000 | 63.37% | 0.99 | 172.27 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 290.577µs | 0 | 2000 | 63.37% | 0.99 | 157.54 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 22.112µs | 10337 | 0 | 63.37% | 0.99 | 2070.20 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 21.759µs | 10337 | 0 | 63.37% | 0.99 | 2103.79 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 21.432µs | 10337 | 0 | 63.37% | 0.99 | 2135.89 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 29.892µs | 10667 | 0 | 63.37% | 0.99 | 1531.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 21.482µs | 10337 | 0 | 63.37% | 0.99 | 2130.92 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 29.296µs | 10667 | 0 | 63.37% | 0.99 | 1562.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 20.719µs | 10337 | 0 | 63.37% | 0.99 | 2209.39 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 20.497µs | 10337 | 0 | 63.37% | 0.99 | 2233.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 20.326µs | 10337 | 0 | 63.37% | 0.99 | 2252.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 106.995µs | 10337 | 2000 | 63.37% | 0.99 | 427.84 MB/s |
| Quicksort | 10000 | 233.596µs | 137946 | 0 | 63.37% | 0.99 | 1959.64 MB/s |
| Timsort | 10000 | 322.614µs | 142499 | 0 | 63.37% | 0.99 | 1418.92 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.875439ms | 0 | 30000 | 63.32% | 0.99 | 93.89 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.208408ms | 0 | 30000 | 63.31% | 0.99 | 87.89 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 427.355µs | 194806 | 14351 | 63.36% | 0.99 | 1071.16 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 236.162µs | 52643 | 10000 | 63.36% | 0.99 | 1938.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 139.379µs | 52643 | 0 | 63.36% | 0.99 | 3284.31 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 151.08µs | 58028 | 0 | 63.36% | 0.99 | 3029.94 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 228.209µs | 60571 | 0 | 63.36% | 0.99 | 2005.90 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 383.903µs | 63560 | 0 | 63.36% | 0.99 | 1192.39 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 218.944µs | 52643 | 0 | 63.36% | 0.99 | 2090.78 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 156.032µs | 52643 | 0 | 63.36% | 0.99 | 2933.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 149.193µs | 52643 | 0 | 63.36% | 0.99 | 3068.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 336.24µs | 52643 | 20000 | 63.36% | 0.99 | 1361.42 MB/s |
| Quicksort | 100000 | 3.262042ms | 1718970 | 0 | 63.33% | 0.99 | 1403.30 MB/s |
| Timsort | 100000 | 4.943415ms | 1756228 | 0 | 63.29% | 0.99 | 926.01 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.204803ms | 5 | 300000 | 61.72% | 0.99 | 111.09 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 46.494014ms | 5 | 300000 | 61.81% | 0.99 | 98.46 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.659767ms | 1893310 | 108703 | 63.33% | 0.99 | 1250.80 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.669495ms | 888976 | 100000 | 63.34% | 0.99 | 2741.93 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.761361ms | 888976 | 0 | 63.35% | 0.99 | 2598.92 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.254431ms | 929234 | 0 | 63.35% | 0.99 | 2030.51 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.094269ms | 956140 | 0 | 63.34% | 0.99 | 2185.79 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.478383ms | 992831 | 0 | 63.34% | 0.99 | 1847.03 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.863868ms | 888976 | 0 | 63.34% | 0.99 | 2455.99 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.840376ms | 780493 | 0 | 63.34% | 0.99 | 2487.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.814858ms | 888976 | 0 | 63.34% | 0.99 | 2522.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.167057ms | 888976 | 200000 | 63.34% | 0.99 | 2112.37 MB/s |
| Quicksort | 1000000 | 45.490041ms | 20525437 | 0 | 63.05% | 0.99 | 1006.29 MB/s |
| Timsort | 1000000 | 76.543325ms | 20897754 | 0 | 62.65% | 0.99 | 598.05 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 38.534951ms | 21586005 | 1017407 | 63.09% | 0.99 | 1187.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.780287ms | 10308690 | 1000000 | 63.37% | 0.98 | 2437.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.45173ms | 10308690 | 0 | 63.37% | 0.98 | 2623.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.485062ms | 10708698 | 0 | 63.35% | 0.98 | 2476.40 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.83346ms | 13010120 | 0 | 63.30% | 0.98 | 2430.59 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 20.079372ms | 13427133 | 0 | 63.24% | 0.98 | 2279.77 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 20.547467ms | 10308690 | 0 | 63.34% | 0.98 | 2227.84 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.232964ms | 11360616 | 0 | 63.39% | 0.98 | 2262.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.221195ms | 12417054 | 0 | 63.38% | 0.98 | 2263.78 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 49.520941ms | 13708291 | 2000000 | 63.38% | 0.98 | 924.38 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 10.329µs | 5628 | 0 | 63.49% | 0.98 | 4431.83 MB/s |
| Timsort | 1000 | 12.662µs | 5482 | 0 | 63.49% | 0.98 | 3615.26 MB/s |
| ARS Gen 1: Foundation | 1000 | 54.483µs | 984 | 2000 | 63.49% | 0.98 | 840.20 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 65.864µs | 984 | 2000 | 63.49% | 0.98 | 695.01 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 10.364µs | 5628 | 0 | 63.49% | 0.98 | 4416.86 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 10.448µs | 5628 | 0 | 63.49% | 0.98 | 4381.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 10.384µs | 5628 | 0 | 63.49% | 0.98 | 4408.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 12.916µs | 5482 | 0 | 63.49% | 0.98 | 3544.16 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 10.366µs | 5628 | 0 | 63.49% | 0.98 | 4416.01 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 12.643µs | 5482 | 0 | 63.49% | 0.98 | 3620.69 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 10.336µs | 5628 | 0 | 63.49% | 0.98 | 4428.83 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 10.381µs | 5628 | 0 | 63.49% | 0.98 | 4409.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 10.462µs | 5628 | 0 | 63.49% | 0.98 | 4375.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 121.802µs | 5628 | 2000 | 63.49% | 0.98 | 375.83 MB/s |
| Quicksort | 10000 | 95.907µs | 54006 | 0 | 63.49% | 0.98 | 4773.00 MB/s |
| Timsort | 10000 | 139.109µs | 53486 | 0 | 63.49% | 0.98 | 3290.68 MB/s |
| ARS Gen 1: Foundation | 10000 | 310.39µs | 9984 | 30000 | 63.48% | 0.98 | 1474.80 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 373.491µs | 9984 | 30000 | 63.48% | 0.98 | 1225.64 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 326.095µs | 122898 | 14351 | 63.49% | 0.98 | 1403.77 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 207.971µs | 9990 | 10000 | 63.48% | 0.98 | 2201.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 121.492µs | 9990 | 0 | 63.48% | 0.98 | 3767.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 127.906µs | 9990 | 0 | 63.48% | 0.98 | 3578.91 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 286.726µs | 9990 | 0 | 63.48% | 0.98 | 1596.52 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 286.609µs | 9990 | 0 | 63.48% | 0.98 | 1597.17 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 139.408µs | 9990 | 0 | 63.48% | 0.98 | 3283.63 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 121.057µs | 9990 | 0 | 63.48% | 0.98 | 3781.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 125.645µs | 9990 | 0 | 63.48% | 0.98 | 3643.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 304.034µs | 9990 | 20000 | 63.48% | 0.98 | 1505.63 MB/s |
| Quicksort | 100000 | 958.395µs | 522721 | 0 | 63.45% | 0.98 | 4776.36 MB/s |
| Timsort | 100000 | 1.545042ms | 535563 | 0 | 63.41% | 0.98 | 2962.79 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.883439ms | 99984 | 300000 | 63.46% | 0.98 | 1587.56 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 3.005863ms | 99984 | 300000 | 63.46% | 0.98 | 1522.90 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 2.555829ms | 1145301 | 108703 | 63.46% | 0.98 | 1791.06 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.11875ms | 119528 | 100000 | 63.46% | 0.98 | 4091.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 948.799µs | 119528 | 0 | 63.46% | 0.98 | 4824.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 929.393µs | 119779 | 0 | 63.46% | 0.98 | 4925.40 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.036879ms | 99990 | 0 | 63.46% | 0.98 | 4414.82 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.177593ms | 99990 | 0 | 63.46% | 0.98 | 3887.28 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.596007ms | 199986 | 0 | 63.46% | 0.98 | 2868.18 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.371903ms | 199974 | 0 | 63.46% | 0.98 | 3336.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.190071ms | 100002 | 0 | 63.46% | 0.98 | 3846.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.78627ms | 119528 | 200000 | 63.46% | 0.98 | 2562.68 MB/s |
| Quicksort | 1000000 | 17.447821ms | 5200332 | 0 | 63.35% | 0.98 | 2623.62 MB/s |
| Timsort | 1000000 | 36.385906ms | 6204510 | 0 | 63.06% | 0.98 | 1258.08 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.279078ms | 12086670 | 1017407 | 63.47% | 0.98 | 1678.08 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 14.704102ms | 999988 | 1000000 | 63.55% | 0.97 | 3113.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 14.897432ms | 999988 | 0 | 63.55% | 0.97 | 3072.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 15.076412ms | 999988 | 0 | 63.55% | 0.98 | 3036.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.708799ms | 999988 | 0 | 63.53% | 0.98 | 3601.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 14.906898ms | 999988 | 0 | 63.54% | 0.98 | 3070.82 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.163927ms | 1999972 | 0 | 63.54% | 0.98 | 2667.01 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 17.222115ms | 1999972 | 0 | 63.55% | 0.97 | 2658.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.446254ms | 1999972 | 0 | 63.54% | 0.98 | 2623.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 43.102359ms | 5484640 | 2000000 | 63.49% | 0.97 | 1062.04 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Time | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 30.519µs | 10337 | 0 | 63.52% | 0.97 | 1499.93 MB/s |
| Timsort | 1000 | 40.935µs | 10667 | 0 | 63.52% | 0.97 | 1118.27 MB/s |
| ARS Gen 1: Foundation | 1000 | 312.98µs | 0 | 2000 | 63.52% | 0.97 | 146.26 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 313.699µs | 0 | 2000 | 63.52% | 0.97 | 145.92 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 25.566µs | 10337 | 0 | 63.52% | 0.97 | 1790.52 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 24.018µs | 10337 | 0 | 63.52% | 0.97 | 1905.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 23.056µs | 10337 | 0 | 63.52% | 0.97 | 1985.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 31.753µs | 10667 | 0 | 63.52% | 0.97 | 1441.64 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 22.608µs | 10337 | 0 | 63.52% | 0.97 | 2024.79 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 31.448µs | 10667 | 0 | 63.52% | 0.97 | 1455.62 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 22.489µs | 10337 | 0 | 63.52% | 0.97 | 2035.50 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 22.509µs | 10337 | 0 | 63.52% | 0.97 | 2033.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 21.701µs | 10337 | 0 | 63.52% | 0.97 | 2109.41 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 115.247µs | 10337 | 2000 | 63.52% | 0.97 | 397.20 MB/s |
| Quicksort | 10000 | 276.659µs | 137946 | 0 | 63.52% | 0.97 | 1654.61 MB/s |
| Timsort | 10000 | 566.999µs | 142499 | 0 | 63.52% | 0.97 | 807.34 MB/s |
| ARS Gen 1: Foundation | 10000 | 5.547176ms | 0 | 30000 | 63.45% | 0.98 | 82.52 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 5.65856ms | 0 | 30000 | 63.46% | 0.98 | 80.90 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 393.996µs | 194806 | 14351 | 63.51% | 0.97 | 1161.85 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 223.479µs | 52643 | 10000 | 63.51% | 0.97 | 2048.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 177.921µs | 52643 | 0 | 63.51% | 0.97 | 2572.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 161.753µs | 58028 | 0 | 63.51% | 0.97 | 2830.02 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 305.918µs | 60571 | 0 | 63.51% | 0.97 | 1496.36 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 250.628µs | 63560 | 0 | 63.51% | 0.97 | 1826.47 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 142.828µs | 52643 | 0 | 63.51% | 0.97 | 3205.00 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 137.835µs | 52643 | 0 | 63.51% | 0.97 | 3321.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 143.004µs | 52643 | 0 | 63.51% | 0.97 | 3201.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 302.391µs | 52643 | 20000 | 63.51% | 0.97 | 1513.81 MB/s |
| Quicksort | 100000 | 3.33255ms | 1718970 | 0 | 63.48% | 0.98 | 1373.61 MB/s |
| Timsort | 100000 | 4.975063ms | 1756228 | 0 | 63.44% | 0.98 | 920.12 MB/s |
| ARS Gen 1: Foundation | 100000 | 42.050371ms | 5 | 300000 | 61.90% | 0.98 | 108.86 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 340.675145ms | 5 | 300000 | 61.86% | 0.98 | 13.44 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.327655ms | 1893310 | 108703 | 63.48% | 0.97 | 1057.76 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.315541ms | 888976 | 100000 | 63.49% | 0.97 | 1976.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.032208ms | 888976 | 0 | 63.49% | 0.97 | 2252.54 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.370522ms | 929234 | 0 | 63.49% | 0.97 | 1931.07 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.817918ms | 956140 | 0 | 63.49% | 0.97 | 1624.47 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.25336ms | 992831 | 0 | 63.48% | 0.97 | 2031.47 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.99066ms | 888976 | 0 | 63.49% | 0.97 | 2299.56 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.877947ms | 780493 | 0 | 63.49% | 0.97 | 2437.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.005588ms | 888976 | 0 | 63.49% | 0.97 | 2282.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.028975ms | 888976 | 200000 | 63.50% | 0.97 | 1511.28 MB/s |
| Quicksort | 1000000 | 48.429612ms | 20525437 | 0 | 63.24% | 0.98 | 945.21 MB/s |
| Timsort | 1000000 | 80.782829ms | 20897754 | 0 | 62.90% | 0.98 | 566.66 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 43.082676ms | 21586005 | 1017407 | 63.27% | 0.98 | 1062.52 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 21.963811ms | 10308690 | 1000000 | 63.53% | 0.97 | 2084.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.548235ms | 10308690 | 0 | 63.53% | 0.97 | 2608.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.756503ms | 10708698 | 0 | 63.51% | 0.97 | 2317.03 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.832317ms | 13010120 | 0 | 63.48% | 0.97 | 2567.05 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.991005ms | 13427133 | 0 | 63.41% | 0.97 | 2081.60 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.382295ms | 10308690 | 0 | 63.51% | 0.97 | 2361.76 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 22.682877ms | 11360616 | 0 | 63.56% | 0.97 | 2018.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.44566ms | 12417054 | 0 | 63.54% | 0.97 | 2238.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 48.534282ms | 13659724 | 2000000 | 63.55% | 0.97 | 943.18 MB/s |
