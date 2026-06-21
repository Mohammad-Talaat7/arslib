# ARS Evolution Atlas: Final Research Study

## 1. Experimental Setup
- **Cores:** 8 | **RAM:** 15864 MB
- **PMC Instrumentation:** true (Multi-thread Inherit: Enabled)
- **Statistical Setup:** Reps=10, Seed=42

## Category: i64

### Distribution: Random

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0190ms | 0.0544ms | 28.40% | 10106 | 0 | 57.91% | 2.16 | 804.15 MB/s |
| Timsort | 1000 | 0.0448ms | 0.0985ms | 24.42% | 10817 | 0 | 48.48% | 1.71 | 340.76 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.4017ms | 1.1618ms | 25.21% | 0 | 2000 | 29.64% | 1.99 | 37.98 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 1.2107ms | 1.3924ms | 6.42% | 0 | 2000 | 27.26% | 1.97 | 12.60 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0245ms | 0.0595ms | 21.79% | 10106 | 0 | 74.79% | 1.98 | 624.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0178ms | 0.0584ms | 25.26% | 10106 | 0 | 70.81% | 1.98 | 856.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0578ms | 0.0602ms | 10.18% | 10106 | 0 | 75.08% | 1.99 | 263.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0318ms | 0.0939ms | 26.90% | 10817 | 0 | 62.36% | 1.69 | 479.50 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0165ms | 0.0558ms | 22.76% | 10106 | 0 | 77.41% | 2.12 | 926.07 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0256ms | 0.1029ms | 31.36% | 10817 | 0 | 58.75% | 1.73 | 595.79 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0175ms | 0.0588ms | 22.72% | 10106 | 0 | 73.85% | 2.01 | 872.98 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0194ms | 0.0603ms | 24.10% | 10106 | 0 | 72.83% | 1.99 | 784.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0581ms | 0.0604ms | 17.41% | 10106 | 0 | 75.55% | 1.98 | 262.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4183ms | 0.8927ms | 20.69% | 10106 | 2000 | 25.97% | 0.45 | 36.48 MB/s |
| Quicksort | 10000 | 0.2180ms | 0.6515ms | 21.59% | 137237 | 0 | 19.59% | 0.62 | 700.05 MB/s |
| Timsort | 10000 | 0.3905ms | 1.0574ms | 23.14% | 140791 | 0 | 19.28% | 0.67 | 390.76 MB/s |
| ARS Gen 1: Foundation | 10000 | 10.2263ms | 25.9069ms | 26.71% | 0 | 30000 | 7.25% | 1.60 | 14.92 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 23.3804ms | 26.6666ms | 8.42% | 0 | 30000 | 12.16% | 1.61 | 6.53 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.7865ms | 1.8104ms | 30.80% | 193309 | 14351 | 18.20% | 0.78 | 194.02 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.4728ms | 1.7116ms | 33.01% | 51605 | 10000 | 16.80% | 0.65 | 322.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.5028ms | 1.1349ms | 45.31% | 51605 | 0 | 15.87% | 0.59 | 303.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.8983ms | 1.3861ms | 30.34% | 57146 | 0 | 15.80% | 0.63 | 169.87 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.0580ms | 1.7127ms | 21.72% | 60498 | 0 | 17.70% | 0.57 | 144.22 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.5411ms | 1.5577ms | 32.83% | 63395 | 0 | 18.48% | 0.60 | 282.00 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.4248ms | 1.2373ms | 28.83% | 51605 | 0 | 15.36% | 0.58 | 359.22 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.4187ms | 1.1902ms | 58.90% | 51605 | 0 | 15.08% | 0.58 | 364.44 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3092ms | 1.2370ms | 34.22% | 51605 | 0 | 15.80% | 0.57 | 493.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.8483ms | 2.7014ms | 34.55% | 51605 | 20000 | 17.67% | 0.58 | 82.56 MB/s |
| Quicksort | 100000 | 3.1853ms | 7.3817ms | 18.77% | 1705992 | 0 | 17.11% | 1.49 | 479.04 MB/s |
| Timsort | 100000 | 4.4439ms | 11.8281ms | 20.25% | 1744939 | 0 | 11.94% | 1.45 | 343.36 MB/s |
| ARS Gen 1: Foundation | 100000 | 66.2728ms | 73.4501ms | 4.89% | 0 | 300000 | 3.40% | 1.00 | 23.02 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 47.4797ms | 72.0142ms | 14.50% | 0 | 300000 | 4.02% | 1.05 | 32.14 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.7867ms | 11.7186ms | 38.02% | 1884009 | 108703 | 23.20% | 1.19 | 402.95 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.2428ms | 6.0406ms | 29.48% | 880743 | 100000 | 17.18% | 1.05 | 1227.80 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.4457ms | 4.7562ms | 27.21% | 880743 | 0 | 13.75% | 1.01 | 442.84 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.9011ms | 4.4049ms | 15.33% | 918240 | 0 | 14.14% | 1.05 | 391.14 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.1045ms | 4.3773ms | 37.31% | 948744 | 0 | 13.40% | 0.99 | 1381.46 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.5996ms | 5.2614ms | 12.88% | 987470 | 0 | 20.05% | 1.01 | 331.74 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.4353ms | 4.1547ms | 11.80% | 880743 | 0 | 13.69% | 1.01 | 444.17 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.9996ms | 5.4837ms | 10.18% | 771335 | 0 | 14.34% | 0.96 | 305.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.8288ms | 4.4785ms | 34.55% | 880743 | 0 | 12.39% | 1.03 | 834.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 6.4180ms | 7.9607ms | 11.19% | 880743 | 200000 | 22.64% | 1.02 | 237.75 MB/s |
| Quicksort | 1000000 | 33.4045ms | 37.2752ms | 6.66% | 20438873 | 0 | 28.73% | 2.35 | 456.79 MB/s |
| Timsort | 1000000 | 36.8442ms | 48.7180ms | 12.98% | 20798910 | 0 | 26.60% | 1.97 | 414.14 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 38.6870ms | 47.7470ms | 7.26% | 21496306 | 1017407 | 32.71% | 1.40 | 394.42 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.5703ms | 24.0500ms | 17.44% | 10216154 | 1000000 | 51.50% | 1.25 | 868.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.3963ms | 21.0671ms | 11.70% | 10216154 | 0 | 50.03% | 1.32 | 991.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.1103ms | 24.6993ms | 15.53% | 10629290 | 0 | 50.19% | 1.22 | 798.46 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 14.1855ms | 21.2497ms | 19.37% | 12861536 | 0 | 47.66% | 1.39 | 1075.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 23.2025ms | 25.0733ms | 15.94% | 13270401 | 0 | 46.63% | 1.33 | 657.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.6651ms | 24.8854ms | 8.71% | 10216154 | 0 | 46.64% | 1.29 | 673.23 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 23.6632ms | 28.0078ms | 15.28% | 11278099 | 0 | 51.17% | 1.32 | 644.83 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.8350ms | 21.8725ms | 21.71% | 12316944 | 0 | 55.23% | 1.39 | 855.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 28.4388ms | 38.8644ms | 10.76% | 12168395 | 2000000 | 53.74% | 1.09 | 536.55 MB/s |
| Quicksort | 10000000 | 259.0647ms | 262.4669ms | 1.73% | 237553537 | 0 | 40.49% | 2.24 | 589.00 MB/s |
| Timsort | 10000000 | 457.3840ms | 476.8788ms | 1.95% | 241568435 | 0 | 40.67% | 1.77 | 333.61 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 276.6839ms | 281.1822ms | 1.52% | 247423856 | 10017407 | 34.71% | 1.46 | 551.49 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 144.2392ms | 148.0839ms | 2.22% | 136733625 | 10000000 | 63.23% | 1.09 | 1057.88 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 129.4175ms | 131.7763ms | 1.45% | 136733625 | 0 | 63.23% | 1.09 | 1179.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 143.3017ms | 146.1502ms | 2.07% | 140883095 | 0 | 54.43% | 1.07 | 1064.80 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 103.3404ms | 109.9019ms | 2.92% | 163019324 | 0 | 58.99% | 1.27 | 1476.56 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 118.8479ms | 135.9006ms | 5.31% | 167029804 | 0 | 39.88% | 1.16 | 1283.89 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 155.4376ms | 159.9004ms | 1.84% | 46347538 | 0 | 44.78% | 0.93 | 981.67 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 159.1674ms | 164.1126ms | 1.62% | 51646902 | 0 | 48.00% | 0.91 | 958.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 148.6814ms | 152.6172ms | 4.69% | 51644969 | 0 | 46.47% | 1.01 | 1026.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 341.6089ms | 348.9425ms | 3.05% | 164995932 | 20000000 | 64.64% | 0.77 | 446.67 MB/s |

### Distribution: Gaussian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0284ms | 0.0557ms | 30.02% | 10125 | 0 | 59.53% | 0.91 | 537.43 MB/s |
| Timsort | 1000 | 0.0502ms | 0.1010ms | 15.96% | 10641 | 0 | 59.53% | 0.91 | 304.25 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.9186ms | 0.9742ms | 2.85% | 451 | 2000 | 59.53% | 0.91 | 16.61 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2441ms | 1.0491ms | 32.31% | 451 | 2000 | 59.53% | 0.91 | 62.50 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0590ms | 0.0618ms | 38.22% | 10125 | 0 | 59.53% | 0.91 | 258.72 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0218ms | 0.0618ms | 23.76% | 10125 | 0 | 59.53% | 0.91 | 698.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0180ms | 0.0627ms | 30.44% | 10125 | 0 | 59.53% | 0.91 | 846.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0271ms | 0.1000ms | 40.43% | 10641 | 0 | 59.53% | 0.91 | 562.23 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0163ms | 0.0732ms | 35.03% | 10125 | 0 | 59.53% | 0.91 | 937.33 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0326ms | 0.1029ms | 30.87% | 10641 | 0 | 59.53% | 0.91 | 467.87 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0572ms | 0.0597ms | 20.59% | 10125 | 0 | 59.53% | 0.91 | 266.85 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0369ms | 0.0626ms | 23.74% | 10125 | 0 | 59.53% | 0.91 | 413.46 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0377ms | 0.0618ms | 63.22% | 10125 | 0 | 59.53% | 0.91 | 404.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.7111ms | 0.8182ms | 17.05% | 10125 | 2000 | 59.52% | 0.91 | 21.46 MB/s |
| Quicksort | 10000 | 0.2626ms | 0.6036ms | 22.12% | 136238 | 0 | 59.47% | 0.91 | 581.05 MB/s |
| Timsort | 10000 | 0.7881ms | 1.0934ms | 10.18% | 139989 | 0 | 59.47% | 0.91 | 193.62 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.6051ms | 6.3245ms | 20.31% | 53356 | 30000 | 59.44% | 0.91 | 58.57 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 3.3833ms | 6.6915ms | 26.27% | 53350 | 30000 | 59.44% | 0.91 | 45.10 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.2918ms | 1.8804ms | 14.04% | 191469 | 14351 | 59.45% | 0.91 | 118.12 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.4580ms | 1.7230ms | 29.05% | 60383 | 10000 | 59.44% | 0.91 | 104.66 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.6929ms | 1.0635ms | 26.93% | 60383 | 0 | 59.43% | 0.91 | 220.20 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.3119ms | 1.2455ms | 46.49% | 63258 | 0 | 59.43% | 0.91 | 489.27 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.7953ms | 1.4509ms | 20.19% | 57986 | 0 | 59.44% | 0.91 | 191.86 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.4538ms | 1.6290ms | 36.63% | 61011 | 0 | 59.44% | 0.91 | 336.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.7401ms | 1.1292ms | 14.14% | 60383 | 0 | 59.43% | 0.91 | 206.17 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9028ms | 1.1902ms | 15.26% | 60383 | 0 | 59.43% | 0.91 | 169.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.0021ms | 1.1414ms | 50.52% | 60383 | 0 | 59.43% | 0.91 | 152.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.0047ms | 2.5030ms | 19.01% | 60383 | 20000 | 59.42% | 0.91 | 76.11 MB/s |
| Quicksort | 100000 | 4.1650ms | 5.7604ms | 10.04% | 1447885 | 0 | 59.30% | 0.91 | 366.35 MB/s |
| Timsort | 100000 | 8.4304ms | 8.6702ms | 3.73% | 1450620 | 0 | 59.22% | 0.91 | 181.00 MB/s |
| ARS Gen 1: Foundation | 100000 | 17.0169ms | 32.4038ms | 17.98% | 1390284 | 300000 | 58.95% | 0.91 | 89.67 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 16.4352ms | 34.2583ms | 27.60% | 1390350 | 300000 | 58.92% | 0.91 | 92.84 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.6233ms | 11.3845ms | 21.01% | 1641821 | 108703 | 59.26% | 0.91 | 330.04 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 5.0793ms | 5.8757ms | 22.94% | 733241 | 100000 | 59.22% | 0.91 | 300.41 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 0.8196ms | 3.9951ms | 34.62% | 733241 | 0 | 59.20% | 0.91 | 1861.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.6764ms | 4.6963ms | 19.21% | 740691 | 0 | 59.21% | 0.91 | 415.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.2938ms | 4.4815ms | 33.45% | 705870 | 0 | 59.22% | 0.91 | 1179.41 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.6904ms | 4.9602ms | 23.95% | 710829 | 0 | 59.20% | 0.91 | 902.66 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.0376ms | 3.9911ms | 16.19% | 733241 | 0 | 59.21% | 0.91 | 502.33 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.8115ms | 5.4716ms | 24.55% | 627560 | 0 | 59.23% | 0.91 | 542.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.4359ms | 3.9479ms | 11.79% | 733241 | 0 | 59.21% | 0.91 | 444.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 5.7096ms | 7.0157ms | 14.05% | 733241 | 200000 | 59.23% | 0.91 | 267.25 MB/s |
| Quicksort | 1000000 | 16.3244ms | 17.5955ms | 8.49% | 13538941 | 0 | 58.57% | 0.93 | 934.72 MB/s |
| Timsort | 1000000 | 23.2078ms | 24.2964ms | 5.90% | 14691957 | 0 | 57.47% | 0.93 | 657.49 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 30.2267ms | 31.9375ms | 10.26% | 14953783 | 1017407 | 58.46% | 0.92 | 504.81 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 12.2402ms | 13.8752ms | 8.49% | 4792494 | 1000000 | 59.22% | 0.92 | 1246.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 12.5127ms | 14.0568ms | 8.45% | 4792494 | 0 | 59.32% | 0.91 | 1219.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 12.4551ms | 14.3238ms | 10.00% | 4828822 | 0 | 59.21% | 0.91 | 1225.11 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 9.9212ms | 10.8160ms | 13.63% | 6193223 | 0 | 59.19% | 0.92 | 1538.00 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 10.5215ms | 11.0450ms | 8.75% | 6232034 | 0 | 59.06% | 0.92 | 1450.24 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 11.9540ms | 14.3564ms | 10.97% | 4776517 | 0 | 59.21% | 0.91 | 1276.46 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.4047ms | 16.6183ms | 3.51% | 2279017 | 0 | 58.76% | 0.91 | 990.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 14.3993ms | 15.1904ms | 9.63% | 2539194 | 0 | 58.91% | 0.92 | 1059.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 24.0394ms | 26.0638ms | 6.29% | 11501862 | 2000000 | 58.71% | 0.92 | 634.74 MB/s |
| Quicksort | 10000000 | 151.6126ms | 157.3649ms | 1.98% | 132841729 | 0 | 55.14% | 1.05 | 1006.43 MB/s |
| Timsort | 10000000 | 314.1014ms | 319.7074ms | 1.87% | 148723832 | 0 | 51.15% | 1.05 | 485.79 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 236.0779ms | 241.7113ms | 4.04% | 145946763 | 10017407 | 54.21% | 1.02 | 646.35 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 116.7510ms | 121.0775ms | 3.09% | 47450965 | 10000000 | 60.98% | 0.94 | 1306.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 114.4029ms | 117.1628ms | 2.78% | 47450965 | 0 | 60.99% | 0.92 | 1333.78 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 118.9212ms | 123.5569ms | 2.15% | 47604105 | 0 | 57.21% | 0.92 | 1283.10 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 84.8791ms | 88.3036ms | 3.35% | 60469657 | 0 | 59.22% | 0.96 | 1797.71 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 107.3491ms | 110.4565ms | 1.86% | 60571782 | 0 | 55.53% | 0.93 | 1421.42 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 139.0404ms | 143.9697ms | 4.32% | 12153004 | 0 | 57.55% | 0.92 | 1097.44 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 172.1553ms | 176.0113ms | 2.06% | 10784786 | 0 | 62.75% | 0.88 | 886.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 147.6234ms | 151.7912ms | 2.02% | 12715094 | 0 | 59.07% | 0.90 | 1033.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 367.9634ms | 375.3885ms | 1.87% | 178299183 | 20000000 | 58.04% | 1.04 | 414.68 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0288ms | 0.0555ms | 33.86% | 9803 | 0 | 57.55% | 1.11 | 529.75 MB/s |
| Timsort | 1000 | 0.0327ms | 0.0913ms | 27.01% | 9687 | 0 | 57.55% | 1.11 | 465.93 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2553ms | 0.4812ms | 17.50% | 9628 | 2000 | 57.55% | 1.11 | 59.76 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2484ms | 0.5258ms | 19.80% | 9676 | 2000 | 57.55% | 1.11 | 61.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0184ms | 0.0620ms | 26.12% | 9803 | 0 | 57.55% | 1.11 | 830.23 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0180ms | 0.0608ms | 29.81% | 9803 | 0 | 57.55% | 1.11 | 846.82 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0190ms | 0.0602ms | 26.40% | 9803 | 0 | 57.55% | 1.11 | 801.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0741ms | 0.0974ms | 17.69% | 9687 | 0 | 57.55% | 1.11 | 205.99 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0194ms | 0.0560ms | 37.99% | 9803 | 0 | 57.55% | 1.11 | 787.79 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0916ms | 0.1098ms | 10.37% | 9687 | 0 | 57.55% | 1.11 | 166.67 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0306ms | 0.0607ms | 27.94% | 9803 | 0 | 57.55% | 1.11 | 497.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0307ms | 0.0621ms | 25.89% | 9803 | 0 | 57.55% | 1.11 | 497.50 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0189ms | 0.0618ms | 42.00% | 9803 | 0 | 57.55% | 1.11 | 805.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.7243ms | 0.8418ms | 12.46% | 9803 | 2000 | 57.55% | 1.11 | 21.07 MB/s |
| Quicksort | 10000 | 0.1861ms | 0.6147ms | 23.43% | 135107 | 0 | 57.54% | 1.11 | 819.86 MB/s |
| Timsort | 10000 | 0.8587ms | 0.9129ms | 12.20% | 132213 | 0 | 57.54% | 1.11 | 177.69 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.5982ms | 4.5093ms | 27.12% | 130086 | 30000 | 57.53% | 1.11 | 95.48 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.0843ms | 4.4799ms | 18.65% | 129981 | 30000 | 57.53% | 1.11 | 73.21 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.8073ms | 1.9156ms | 26.81% | 186385 | 14351 | 57.53% | 1.11 | 84.43 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.4746ms | 1.6416ms | 11.34% | 45393 | 10000 | 57.53% | 1.11 | 103.47 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3155ms | 1.0751ms | 45.09% | 45393 | 0 | 57.53% | 1.11 | 483.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.6080ms | 1.2937ms | 30.72% | 37030 | 0 | 57.53% | 1.11 | 250.95 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.3489ms | 1.5446ms | 9.72% | 51182 | 0 | 57.53% | 1.11 | 113.12 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.4047ms | 1.7018ms | 23.97% | 45096 | 0 | 57.53% | 1.11 | 108.62 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3315ms | 1.0716ms | 49.58% | 45393 | 0 | 57.53% | 1.11 | 460.36 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9288ms | 1.2809ms | 24.92% | 45393 | 0 | 57.53% | 1.11 | 164.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.8190ms | 1.2638ms | 26.06% | 45393 | 0 | 57.53% | 1.11 | 186.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.9372ms | 2.2527ms | 21.62% | 45393 | 20000 | 57.52% | 1.11 | 78.77 MB/s |
| Quicksort | 100000 | 1.8479ms | 7.4756ms | 35.12% | 1704789 | 0 | 57.50% | 1.11 | 825.73 MB/s |
| Timsort | 100000 | 10.2821ms | 10.6084ms | 4.98% | 1670195 | 0 | 57.47% | 1.11 | 148.40 MB/s |
| ARS Gen 1: Foundation | 100000 | 10.5612ms | 30.7759ms | 32.47% | 1642908 | 300000 | 57.42% | 1.11 | 144.48 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 11.9841ms | 25.6201ms | 36.55% | 1642836 | 300000 | 57.41% | 1.11 | 127.32 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.5874ms | 11.1891ms | 25.73% | 1828718 | 108703 | 57.50% | 1.11 | 425.35 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 5.0338ms | 5.3936ms | 13.26% | 829558 | 100000 | 57.47% | 1.11 | 303.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.5946ms | 4.4987ms | 29.51% | 829558 | 0 | 57.47% | 1.11 | 956.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 0.8471ms | 3.8423ms | 32.81% | 407054 | 0 | 57.46% | 1.11 | 1801.35 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.0140ms | 4.3681ms | 29.13% | 829558 | 0 | 57.47% | 1.11 | 1504.88 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.9842ms | 4.0915ms | 18.77% | 407054 | 0 | 57.47% | 1.11 | 511.31 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.0348ms | 4.3384ms | 30.48% | 829558 | 0 | 57.46% | 1.11 | 1474.52 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.7945ms | 5.3048ms | 9.45% | 718749 | 0 | 57.47% | 1.11 | 318.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.4382ms | 4.3176ms | 30.08% | 829558 | 0 | 57.46% | 1.11 | 1060.99 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 6.0202ms | 7.2610ms | 18.82% | 829558 | 200000 | 57.49% | 1.11 | 253.46 MB/s |
| Quicksort | 1000000 | 42.6577ms | 45.2509ms | 3.18% | 20743535 | 0 | 57.36% | 1.12 | 357.70 MB/s |
| Timsort | 1000000 | 40.0358ms | 53.5821ms | 8.83% | 19762519 | 0 | 57.02% | 1.12 | 381.13 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 45.2722ms | 50.6819ms | 6.33% | 20996656 | 1017407 | 57.40% | 1.11 | 337.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 16.8836ms | 30.0696ms | 21.79% | 9747020 | 1000000 | 57.48% | 1.11 | 903.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.6617ms | 26.8267ms | 13.50% | 9747020 | 0 | 57.48% | 1.11 | 817.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.1902ms | 22.0289ms | 8.75% | 4120566 | 0 | 57.51% | 1.11 | 887.64 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 12.3156ms | 28.1366ms | 21.73% | 12586996 | 0 | 57.43% | 1.11 | 1238.98 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 12.4941ms | 26.0354ms | 17.87% | 5680423 | 0 | 57.41% | 1.11 | 1221.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.1741ms | 25.6740ms | 21.73% | 9747020 | 0 | 57.50% | 1.11 | 1005.58 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 28.8209ms | 30.2007ms | 14.29% | 10842040 | 0 | 57.49% | 1.11 | 529.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.8991ms | 31.5267ms | 19.95% | 11927172 | 0 | 57.50% | 1.11 | 1097.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.1074ms | 40.5922ms | 13.99% | 14924313 | 2000000 | 57.44% | 1.11 | 524.22 MB/s |
| Quicksort | 10000000 | 268.2934ms | 271.9217ms | 0.94% | 244096088 | 0 | 56.27% | 1.21 | 568.74 MB/s |
| Timsort | 10000000 | 448.1121ms | 453.9939ms | 0.98% | 230737667 | 0 | 54.44% | 1.20 | 340.51 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 272.8091ms | 282.3060ms | 2.11% | 244212329 | 10017407 | 56.98% | 1.17 | 559.32 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 141.5572ms | 146.4819ms | 1.77% | 133749822 | 10000000 | 58.39% | 1.13 | 1077.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 127.9338ms | 133.0972ms | 2.75% | 133749822 | 0 | 58.39% | 1.13 | 1192.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 115.8240ms | 118.0262ms | 2.81% | 48510427 | 0 | 58.29% | 1.11 | 1317.41 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 104.9347ms | 110.8475ms | 2.97% | 162283191 | 0 | 57.67% | 1.15 | 1454.12 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 95.8256ms | 113.5296ms | 7.66% | 93541156 | 0 | 57.06% | 1.13 | 1592.35 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 142.5349ms | 147.8387ms | 1.93% | 23884325 | 0 | 57.11% | 1.11 | 1070.53 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 159.9941ms | 165.9065ms | 4.13% | 45018781 | 0 | 56.90% | 1.11 | 953.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 146.0925ms | 149.8015ms | 1.61% | 45028020 | 0 | 56.81% | 1.11 | 1044.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 410.4198ms | 418.0952ms | 1.07% | 240103886 | 20000000 | 56.76% | 1.17 | 371.78 MB/s |

### Distribution: Duplicates

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0164ms | 0.0174ms | 39.01% | 3696 | 0 | 56.92% | 1.22 | 928.09 MB/s |
| Timsort | 1000 | 0.0121ms | 0.0505ms | 47.19% | 3708 | 0 | 56.92% | 1.22 | 1264.72 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1516ms | 0.1672ms | 16.84% | 995 | 2000 | 56.92% | 1.22 | 100.65 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2194ms | 0.2385ms | 10.55% | 995 | 2000 | 56.92% | 1.22 | 69.53 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0071ms | 0.0194ms | 29.77% | 3696 | 0 | 56.92% | 1.22 | 2148.82 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0067ms | 0.0192ms | 59.40% | 3696 | 0 | 56.92% | 1.22 | 2277.77 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0140ms | 0.0192ms | 39.03% | 3696 | 0 | 56.92% | 1.22 | 1089.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0122ms | 0.0410ms | 38.60% | 3708 | 0 | 56.92% | 1.22 | 1253.70 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0119ms | 0.0180ms | 50.51% | 3696 | 0 | 56.92% | 1.22 | 1285.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0178ms | 0.0409ms | 27.32% | 3708 | 0 | 56.92% | 1.22 | 859.60 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0181ms | 0.0191ms | 45.74% | 3696 | 0 | 56.92% | 1.22 | 843.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0193ms | 0.0208ms | 15.29% | 3696 | 0 | 56.92% | 1.22 | 791.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0076ms | 0.0199ms | 37.87% | 3696 | 0 | 56.92% | 1.22 | 1999.84 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5972ms | 0.7308ms | 11.26% | 3696 | 2000 | 56.92% | 1.22 | 25.55 MB/s |
| Quicksort | 10000 | 0.0389ms | 0.1297ms | 27.64% | 36514 | 0 | 56.91% | 1.22 | 3926.71 MB/s |
| Timsort | 10000 | 0.2495ms | 0.2787ms | 6.70% | 36706 | 0 | 56.91% | 1.22 | 611.56 MB/s |
| ARS Gen 1: Foundation | 10000 | 0.4799ms | 1.5260ms | 23.69% | 9995 | 30000 | 56.91% | 1.22 | 317.96 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 0.4654ms | 1.7290ms | 40.26% | 9995 | 30000 | 56.91% | 1.22 | 327.88 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.5467ms | 1.6417ms | 19.62% | 115253 | 14351 | 56.91% | 1.22 | 98.66 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.7260ms | 1.6164ms | 38.01% | 9999 | 10000 | 56.91% | 1.22 | 210.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.2709ms | 0.8902ms | 25.53% | 9999 | 0 | 56.91% | 1.22 | 563.26 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.9808ms | 1.0840ms | 23.00% | 9999 | 0 | 56.91% | 1.22 | 155.57 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.8706ms | 1.3420ms | 14.76% | 9999 | 0 | 56.91% | 1.22 | 175.28 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.4105ms | 1.6352ms | 26.14% | 9999 | 0 | 56.91% | 1.22 | 371.73 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.8467ms | 1.1712ms | 19.25% | 9999 | 0 | 56.91% | 1.22 | 180.21 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.3656ms | 0.9192ms | 26.66% | 9999 | 0 | 56.91% | 1.22 | 417.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.2551ms | 1.0389ms | 31.13% | 9999 | 0 | 56.91% | 1.22 | 598.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.0562ms | 2.0644ms | 33.90% | 9999 | 20000 | 56.90% | 1.22 | 144.47 MB/s |
| Quicksort | 100000 | 0.3335ms | 1.3492ms | 29.65% | 362149 | 0 | 56.88% | 1.22 | 4575.84 MB/s |
| Timsort | 100000 | 2.5215ms | 2.7292ms | 4.65% | 362807 | 0 | 56.87% | 1.22 | 605.14 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.5746ms | 8.0382ms | 5.60% | 99995 | 300000 | 56.88% | 1.22 | 201.45 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.6626ms | 8.8958ms | 22.98% | 99995 | 300000 | 56.88% | 1.22 | 573.08 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 10.3687ms | 11.0205ms | 5.78% | 1130042 | 108703 | 56.88% | 1.22 | 147.16 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.2506ms | 3.8013ms | 18.38% | 99999 | 100000 | 56.87% | 1.22 | 469.41 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.1750ms | 2.5506ms | 29.07% | 99999 | 0 | 56.87% | 1.22 | 1298.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.2648ms | 3.6478ms | 22.01% | 99999 | 0 | 56.87% | 1.22 | 467.37 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.0540ms | 4.0384ms | 16.32% | 99999 | 0 | 56.88% | 1.22 | 499.63 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 0.8639ms | 3.5027ms | 43.28% | 99999 | 0 | 56.87% | 1.22 | 1766.17 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.7758ms | 3.7689ms | 39.71% | 199994 | 0 | 56.86% | 1.22 | 859.26 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.9531ms | 4.6988ms | 17.26% | 199994 | 0 | 56.87% | 1.22 | 516.70 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.6854ms | 3.7073ms | 13.71% | 199994 | 0 | 56.87% | 1.22 | 568.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.5152ms | 6.0667ms | 27.18% | 99999 | 200000 | 56.87% | 1.22 | 1007.02 MB/s |
| Quicksort | 1000000 | 4.8415ms | 6.2973ms | 17.25% | 3806445 | 0 | 56.82% | 1.22 | 3151.65 MB/s |
| Timsort | 1000000 | 9.5345ms | 11.0192ms | 7.23% | 4510537 | 0 | 56.74% | 1.22 | 1600.38 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.4873ms | 28.3937ms | 3.34% | 12059615 | 1017407 | 56.87% | 1.22 | 555.12 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.0095ms | 12.4970ms | 8.56% | 1000001 | 1000000 | 56.91% | 1.22 | 1385.97 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.3159ms | 12.1440ms | 8.08% | 1000001 | 0 | 56.90% | 1.22 | 1479.15 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.2973ms | 13.2914ms | 8.11% | 1000001 | 0 | 56.93% | 1.22 | 1350.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.4897ms | 10.1809ms | 13.62% | 1000001 | 0 | 56.89% | 1.22 | 2037.29 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.1126ms | 10.0322ms | 11.21% | 1000001 | 0 | 56.89% | 1.22 | 1880.89 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 14.6206ms | 15.4099ms | 5.73% | 1999996 | 0 | 56.93% | 1.22 | 1043.65 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.3572ms | 12.7156ms | 8.81% | 1999996 | 0 | 56.90% | 1.22 | 1343.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.5549ms | 14.9567ms | 6.12% | 1999996 | 0 | 56.93% | 1.22 | 1125.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 21.6551ms | 23.3164ms | 5.09% | 5365058 | 2000000 | 56.73% | 1.22 | 704.63 MB/s |
| Quicksort | 10000000 | 53.6999ms | 56.5719ms | 2.73% | 36019091 | 0 | 56.90% | 1.22 | 2841.49 MB/s |
| Timsort | 10000000 | 183.8064ms | 189.7458ms | 1.17% | 50543152 | 0 | 57.14% | 1.22 | 830.16 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 187.1202ms | 196.3804ms | 3.19% | 120059628 | 10017407 | 57.03% | 1.23 | 815.45 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 97.5637ms | 99.2508ms | 1.74% | 10000003 | 10000000 | 57.50% | 1.19 | 1563.98 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 96.8534ms | 99.9386ms | 1.93% | 10000003 | 0 | 57.50% | 1.19 | 1575.45 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 97.2572ms | 99.2916ms | 2.92% | 10000003 | 0 | 57.52% | 1.19 | 1568.91 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 59.2477ms | 60.8879ms | 2.36% | 10000003 | 0 | 57.24% | 1.20 | 2575.42 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 59.7805ms | 61.8601ms | 2.40% | 10000003 | 0 | 57.24% | 1.20 | 2552.47 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 122.4167ms | 125.9737ms | 1.56% | 19999998 | 0 | 57.74% | 1.19 | 1246.46 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 103.2631ms | 105.5967ms | 1.70% | 19999998 | 0 | 57.64% | 1.20 | 1477.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 124.2124ms | 127.7601ms | 1.90% | 19999998 | 0 | 57.73% | 1.19 | 1228.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 339.7035ms | 348.3952ms | 1.27% | 109768705 | 20000000 | 56.80% | 1.21 | 449.18 MB/s |

### Distribution: Zipfian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0266ms | 0.0294ms | 9.90% | 5850 | 0 | 56.57% | 1.22 | 573.85 MB/s |
| Timsort | 1000 | 0.0178ms | 0.0591ms | 36.84% | 5919 | 0 | 56.57% | 1.22 | 856.56 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1034ms | 0.2226ms | 27.88% | 4500 | 2000 | 56.57% | 1.22 | 147.61 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2254ms | 0.2598ms | 13.25% | 4500 | 2000 | 56.57% | 1.22 | 67.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0128ms | 0.0327ms | 29.99% | 5850 | 0 | 56.57% | 1.22 | 1188.93 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0094ms | 0.0316ms | 28.00% | 5850 | 0 | 56.57% | 1.22 | 1629.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0116ms | 0.0311ms | 22.75% | 5850 | 0 | 56.57% | 1.22 | 1319.05 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0149ms | 0.0575ms | 37.67% | 5919 | 0 | 56.57% | 1.22 | 1021.95 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0083ms | 0.0302ms | 40.52% | 5850 | 0 | 56.57% | 1.22 | 1847.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0166ms | 0.0575ms | 37.26% | 5919 | 0 | 56.57% | 1.22 | 916.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0305ms | 0.0320ms | 31.75% | 5850 | 0 | 56.57% | 1.22 | 500.70 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0127ms | 0.0315ms | 19.91% | 5850 | 0 | 56.57% | 1.22 | 1201.95 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0106ms | 0.0344ms | 37.83% | 5850 | 0 | 56.57% | 1.22 | 1445.51 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5681ms | 0.8148ms | 15.71% | 5850 | 2000 | 56.57% | 1.22 | 26.86 MB/s |
| Quicksort | 10000 | 0.0557ms | 0.2158ms | 26.61% | 61801 | 0 | 56.57% | 1.22 | 2737.84 MB/s |
| Timsort | 10000 | 0.2635ms | 0.3837ms | 15.12% | 58783 | 0 | 56.57% | 1.22 | 579.09 MB/s |
| ARS Gen 1: Foundation | 10000 | 0.6522ms | 1.8042ms | 22.12% | 49663 | 30000 | 56.57% | 1.22 | 233.94 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.0118ms | 1.9977ms | 21.38% | 49663 | 30000 | 56.57% | 1.22 | 150.81 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.5586ms | 1.6983ms | 8.27% | 126177 | 14351 | 56.57% | 1.22 | 97.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.3679ms | 2.0859ms | 16.92% | 48406 | 10000 | 56.56% | 1.22 | 111.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.1701ms | 1.3838ms | 17.89% | 48406 | 0 | 56.56% | 1.22 | 130.41 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.9682ms | 1.3992ms | 17.14% | 49001 | 0 | 56.56% | 1.22 | 157.60 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.8529ms | 2.0811ms | 6.71% | 47554 | 0 | 56.57% | 1.22 | 82.35 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.9936ms | 2.3164ms | 10.91% | 44955 | 0 | 56.56% | 1.22 | 76.54 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.1349ms | 1.6184ms | 24.17% | 12994 | 0 | 56.56% | 1.22 | 134.45 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.3953ms | 1.4529ms | 25.70% | 48406 | 0 | 56.56% | 1.22 | 386.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3764ms | 1.3945ms | 36.39% | 48406 | 0 | 56.56% | 1.22 | 405.42 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.7120ms | 2.3340ms | 25.06% | 48406 | 20000 | 56.56% | 1.22 | 214.30 MB/s |
| Quicksort | 100000 | 1.2499ms | 1.9466ms | 12.06% | 534636 | 0 | 56.56% | 1.22 | 1220.77 MB/s |
| Timsort | 100000 | 0.9991ms | 3.5883ms | 24.80% | 537945 | 0 | 56.55% | 1.22 | 1527.21 MB/s |
| ARS Gen 1: Foundation | 100000 | 8.8644ms | 9.8533ms | 6.96% | 510754 | 300000 | 56.55% | 1.22 | 172.14 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 4.7311ms | 10.6152ms | 20.11% | 510754 | 300000 | 56.55% | 1.22 | 322.52 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.5771ms | 8.9623ms | 32.11% | 1173602 | 108703 | 56.55% | 1.22 | 426.56 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.4185ms | 7.8055ms | 30.27% | 519160 | 100000 | 56.55% | 1.22 | 446.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.8542ms | 5.5647ms | 28.39% | 519160 | 0 | 56.55% | 1.22 | 822.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.4334ms | 5.3563ms | 26.02% | 523903 | 0 | 56.54% | 1.22 | 627.04 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.8119ms | 5.4773ms | 9.16% | 517480 | 0 | 56.55% | 1.22 | 317.10 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 5.5353ms | 6.0377ms | 8.95% | 509167 | 0 | 56.54% | 1.22 | 275.66 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.8688ms | 7.5617ms | 28.02% | 206193 | 0 | 56.52% | 1.22 | 531.88 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 6.7158ms | 7.9125ms | 15.30% | 182098 | 0 | 56.54% | 1.22 | 227.21 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.1976ms | 8.6680ms | 35.70% | 200782 | 0 | 56.53% | 1.22 | 694.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.8680ms | 7.5676ms | 20.92% | 519160 | 200000 | 56.55% | 1.22 | 394.49 MB/s |
| Quicksort | 1000000 | 4.2107ms | 5.1398ms | 13.25% | 5272329 | 0 | 56.51% | 1.22 | 3623.81 MB/s |
| Timsort | 1000000 | 8.9367ms | 10.8375ms | 10.57% | 6330674 | 0 | 56.42% | 1.22 | 1707.42 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 21.5003ms | 22.4593ms | 4.17% | 12320111 | 1017407 | 56.55% | 1.22 | 709.70 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.1400ms | 18.4092ms | 5.78% | 5208968 | 1000000 | 56.54% | 1.22 | 890.24 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 14.5380ms | 16.3568ms | 5.56% | 5208968 | 0 | 56.54% | 1.22 | 1049.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.8013ms | 21.1633ms | 4.86% | 6016924 | 0 | 56.49% | 1.22 | 770.59 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 10.0054ms | 11.5220ms | 7.22% | 5219974 | 0 | 56.53% | 1.22 | 1525.06 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 17.1363ms | 17.4816ms | 5.82% | 6049952 | 0 | 56.48% | 1.22 | 890.44 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 19.7432ms | 22.0031ms | 4.48% | 1939509 | 0 | 56.58% | 1.22 | 772.86 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 26.9411ms | 27.7781ms | 7.25% | 2076206 | 0 | 56.62% | 1.22 | 566.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 31.9022ms | 33.0678ms | 4.61% | 2063636 | 0 | 56.62% | 1.22 | 478.30 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.4755ms | 20.8846ms | 5.25% | 9886880 | 2000000 | 56.50% | 1.22 | 783.49 MB/s |
| Quicksort | 10000000 | 74.5188ms | 76.6158ms | 1.77% | 52910303 | 0 | 56.53% | 1.24 | 2047.64 MB/s |
| Timsort | 10000000 | 205.2979ms | 207.1332ms | 1.38% | 65720450 | 0 | 56.33% | 1.24 | 743.25 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 204.3626ms | 209.5689ms | 1.27% | 122355136 | 10017407 | 56.74% | 1.24 | 746.65 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 194.1991ms | 196.8929ms | 1.73% | 52454409 | 10000000 | 57.02% | 1.22 | 785.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 170.4130ms | 172.4765ms | 2.51% | 52454409 | 0 | 56.95% | 1.22 | 895.40 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 296.3805ms | 304.6473ms | 1.65% | 65723702 | 0 | 56.85% | 1.22 | 514.84 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 130.1768ms | 131.5993ms | 1.76% | 52850002 | 0 | 56.82% | 1.23 | 1172.16 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 257.1309ms | 262.5700ms | 1.23% | 65610667 | 0 | 56.68% | 1.22 | 593.43 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 293.4969ms | 301.3360ms | 1.17% | 20237960 | 0 | 57.71% | 1.18 | 519.90 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 282.6183ms | 286.4323ms | 1.02% | 20109720 | 0 | 57.62% | 1.20 | 539.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 330.9282ms | 336.8186ms | 1.42% | 20324898 | 0 | 57.72% | 1.19 | 461.09 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 373.7314ms | 379.7669ms | 1.38% | 160313749 | 20000000 | 56.34% | 1.23 | 408.28 MB/s |

### Distribution: Skewed

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0166ms | 0.0551ms | 39.45% | 10107 | 0 | 55.67% | 1.26 | 921.81 MB/s |
| Timsort | 1000 | 0.0534ms | 0.1183ms | 30.64% | 10872 | 0 | 55.67% | 1.26 | 285.64 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2913ms | 0.8540ms | 22.69% | 746 | 2000 | 55.67% | 1.26 | 52.38 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.4451ms | 0.9352ms | 16.90% | 743 | 2000 | 55.67% | 1.26 | 34.28 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0260ms | 0.0601ms | 35.58% | 10107 | 0 | 55.67% | 1.26 | 587.03 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0190ms | 0.0601ms | 28.26% | 10107 | 0 | 55.67% | 1.26 | 802.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0173ms | 0.0595ms | 35.03% | 10107 | 0 | 55.67% | 1.26 | 882.88 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0984ms | 0.1205ms | 14.81% | 10872 | 0 | 55.67% | 1.26 | 155.11 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0233ms | 0.0550ms | 27.97% | 10107 | 0 | 55.67% | 1.26 | 654.86 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0999ms | 0.1088ms | 19.85% | 10872 | 0 | 55.67% | 1.26 | 152.76 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0236ms | 0.0611ms | 31.06% | 10107 | 0 | 55.67% | 1.26 | 647.16 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0248ms | 0.0597ms | 23.69% | 10107 | 0 | 55.67% | 1.26 | 616.12 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0585ms | 0.0726ms | 34.51% | 10107 | 0 | 55.67% | 1.26 | 260.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3953ms | 0.7607ms | 22.34% | 10107 | 2000 | 55.67% | 1.26 | 38.60 MB/s |
| Quicksort | 10000 | 0.5821ms | 0.6075ms | 3.92% | 133817 | 0 | 55.66% | 1.26 | 262.12 MB/s |
| Timsort | 10000 | 0.2926ms | 1.0577ms | 32.13% | 138090 | 0 | 55.66% | 1.26 | 521.46 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.4066ms | 5.1943ms | 29.33% | 83701 | 30000 | 55.66% | 1.26 | 108.48 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.6983ms | 5.5027ms | 28.62% | 83702 | 30000 | 55.66% | 1.26 | 89.85 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.7318ms | 1.8917ms | 21.52% | 188751 | 14351 | 55.66% | 1.26 | 88.11 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.3628ms | 1.8040ms | 33.32% | 69084 | 10000 | 55.66% | 1.26 | 111.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.9078ms | 1.1354ms | 13.53% | 69084 | 0 | 55.66% | 1.26 | 168.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.2700ms | 1.2255ms | 45.36% | 72073 | 0 | 55.66% | 1.26 | 565.17 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6982ms | 2.0095ms | 24.85% | 60156 | 0 | 55.66% | 1.26 | 218.55 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.7143ms | 2.0245ms | 13.85% | 62784 | 0 | 55.66% | 1.26 | 89.01 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.2585ms | 1.0768ms | 32.30% | 69084 | 0 | 55.66% | 1.26 | 590.25 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9921ms | 1.2962ms | 17.08% | 69084 | 0 | 55.66% | 1.26 | 153.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.9013ms | 1.2464ms | 40.71% | 69084 | 0 | 55.66% | 1.26 | 169.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.1437ms | 2.4645ms | 27.67% | 69084 | 20000 | 55.66% | 1.26 | 71.18 MB/s |
| Quicksort | 100000 | 3.4531ms | 5.3074ms | 11.28% | 1353832 | 0 | 55.65% | 1.26 | 441.89 MB/s |
| Timsort | 100000 | 2.6914ms | 7.7970ms | 34.21% | 1355073 | 0 | 55.64% | 1.26 | 566.94 MB/s |
| ARS Gen 1: Foundation | 100000 | 11.0716ms | 29.5582ms | 30.53% | 1261637 | 300000 | 55.63% | 1.26 | 137.82 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 10.7296ms | 20.4150ms | 36.62% | 1261447 | 300000 | 55.63% | 1.26 | 142.21 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 8.7152ms | 11.1918ms | 10.66% | 1556552 | 108703 | 55.65% | 1.26 | 175.08 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.8926ms | 5.1968ms | 30.80% | 767652 | 100000 | 55.64% | 1.26 | 806.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.0568ms | 3.9076ms | 21.95% | 767652 | 0 | 55.64% | 1.26 | 499.17 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.5363ms | 4.9605ms | 20.39% | 775288 | 0 | 55.64% | 1.26 | 431.50 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.5842ms | 4.1095ms | 21.35% | 644570 | 0 | 55.64% | 1.26 | 590.46 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.9373ms | 4.7561ms | 28.13% | 649259 | 0 | 55.64% | 1.26 | 787.63 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.1347ms | 4.1823ms | 33.49% | 619173 | 0 | 55.64% | 1.26 | 1344.78 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.6603ms | 5.7847ms | 15.33% | 665028 | 0 | 55.64% | 1.26 | 327.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.2993ms | 4.3172ms | 21.95% | 767652 | 0 | 55.64% | 1.26 | 462.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 6.1664ms | 6.7369ms | 15.15% | 767652 | 200000 | 55.64% | 1.26 | 247.45 MB/s |
| Quicksort | 1000000 | 13.6105ms | 16.9225ms | 7.76% | 12911438 | 0 | 55.60% | 1.26 | 1121.10 MB/s |
| Timsort | 1000000 | 21.6943ms | 24.1953ms | 5.98% | 14001456 | 0 | 55.49% | 1.26 | 703.35 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.7506ms | 32.5352ms | 8.60% | 14290655 | 1017407 | 55.59% | 1.26 | 570.41 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 12.8516ms | 14.6321ms | 6.33% | 5276411 | 1000000 | 55.66% | 1.26 | 1187.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.3225ms | 14.0077ms | 9.48% | 5276411 | 0 | 55.66% | 1.26 | 1347.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 13.4742ms | 14.9936ms | 6.73% | 5291556 | 0 | 55.65% | 1.26 | 1132.45 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 10.3815ms | 11.3070ms | 8.20% | 6090737 | 0 | 55.66% | 1.26 | 1469.81 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 10.4507ms | 11.4235ms | 6.01% | 6091424 | 0 | 55.64% | 1.26 | 1460.07 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 15.0306ms | 17.3577ms | 4.92% | 2296097 | 0 | 55.62% | 1.26 | 1015.18 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.9460ms | 17.8450ms | 8.50% | 1816993 | 0 | 55.61% | 1.26 | 956.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 15.7766ms | 17.5917ms | 10.65% | 1991681 | 0 | 55.64% | 1.26 | 967.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 22.7097ms | 26.6298ms | 9.57% | 11847147 | 2000000 | 55.64% | 1.26 | 671.91 MB/s |
| Quicksort | 10000000 | 148.2494ms | 152.5296ms | 1.70% | 126692640 | 0 | 55.23% | 1.28 | 1029.26 MB/s |
| Timsort | 10000000 | 308.2360ms | 314.8914ms | 1.40% | 142375057 | 0 | 54.63% | 1.28 | 495.04 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 229.0906ms | 234.9462ms | 1.94% | 139581174 | 10017407 | 55.15% | 1.27 | 666.06 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 128.0795ms | 134.8097ms | 2.45% | 53501188 | 10000000 | 55.93% | 1.24 | 1191.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 126.3607ms | 128.8426ms | 1.59% | 53501188 | 0 | 55.94% | 1.24 | 1207.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 145.6714ms | 152.9721ms | 2.50% | 53541482 | 0 | 55.46% | 1.24 | 1047.48 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 85.6482ms | 88.8912ms | 2.55% | 60417961 | 0 | 55.74% | 1.25 | 1781.57 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 119.1096ms | 125.7962ms | 3.61% | 60460278 | 0 | 55.38% | 1.24 | 1281.07 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 157.7483ms | 159.9305ms | 2.14% | 17678377 | 0 | 55.69% | 1.24 | 967.29 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 176.6807ms | 178.6149ms | 1.70% | 16654121 | 0 | 56.35% | 1.23 | 863.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 163.3294ms | 165.6601ms | 1.76% | 14022118 | 0 | 56.01% | 1.23 | 934.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 360.5062ms | 369.4996ms | 1.25% | 181442950 | 20000000 | 55.78% | 1.26 | 423.26 MB/s |

### Distribution: Clustered

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0231ms | 0.0552ms | 41.79% | 9997 | 0 | 56.18% | 1.26 | 660.73 MB/s |
| Timsort | 1000 | 0.0975ms | 0.1086ms | 13.37% | 10453 | 0 | 56.18% | 1.26 | 156.48 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1494ms | 0.5590ms | 35.70% | 5409 | 2000 | 56.18% | 1.26 | 102.16 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2612ms | 0.6046ms | 22.78% | 5409 | 2000 | 56.18% | 1.26 | 58.41 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0299ms | 0.0602ms | 35.87% | 9997 | 0 | 56.18% | 1.26 | 510.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0170ms | 0.0597ms | 40.00% | 9997 | 0 | 56.18% | 1.26 | 897.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0162ms | 0.0599ms | 37.22% | 9997 | 0 | 56.18% | 1.26 | 941.09 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0970ms | 0.1023ms | 23.72% | 10453 | 0 | 56.18% | 1.26 | 157.24 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0165ms | 0.0558ms | 30.45% | 9997 | 0 | 56.18% | 1.26 | 924.72 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0289ms | 0.1077ms | 27.02% | 10453 | 0 | 56.18% | 1.26 | 528.61 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0249ms | 0.0597ms | 21.20% | 9997 | 0 | 56.18% | 1.26 | 613.07 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0585ms | 0.0609ms | 15.82% | 9997 | 0 | 56.18% | 1.26 | 260.62 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0386ms | 0.0645ms | 22.51% | 9997 | 0 | 56.18% | 1.26 | 395.40 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3354ms | 0.8074ms | 26.93% | 9997 | 2000 | 56.18% | 1.26 | 45.49 MB/s |
| Quicksort | 10000 | 0.1894ms | 0.4797ms | 25.29% | 110175 | 0 | 56.18% | 1.26 | 805.74 MB/s |
| Timsort | 10000 | 0.2540ms | 0.8144ms | 26.80% | 111041 | 0 | 56.18% | 1.26 | 600.67 MB/s |
| ARS Gen 1: Foundation | 10000 | 0.9318ms | 2.6805ms | 22.81% | 75791 | 30000 | 56.18% | 1.26 | 163.75 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.5196ms | 2.8769ms | 12.83% | 76177 | 30000 | 56.18% | 1.26 | 60.56 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.5373ms | 1.7907ms | 29.28% | 164574 | 14351 | 56.18% | 1.26 | 283.99 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.4390ms | 1.6939ms | 41.62% | 71469 | 10000 | 56.18% | 1.26 | 106.04 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.2875ms | 1.2508ms | 36.72% | 71469 | 0 | 56.18% | 1.26 | 530.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.1273ms | 1.4290ms | 18.28% | 71238 | 0 | 56.18% | 1.26 | 135.36 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.9976ms | 1.7396ms | 19.88% | 59153 | 0 | 56.18% | 1.26 | 152.96 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.5671ms | 1.5971ms | 38.60% | 59015 | 0 | 56.18% | 1.26 | 269.05 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3229ms | 1.2923ms | 28.69% | 71469 | 0 | 56.18% | 1.26 | 472.61 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.2753ms | 1.1521ms | 39.87% | 71469 | 0 | 56.18% | 1.26 | 554.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3769ms | 1.2290ms | 34.98% | 71469 | 0 | 56.18% | 1.26 | 404.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.9967ms | 2.1832ms | 23.26% | 71469 | 20000 | 56.17% | 1.26 | 153.09 MB/s |
| Quicksort | 100000 | 0.9889ms | 3.5916ms | 23.78% | 1019990 | 0 | 56.17% | 1.26 | 1543.02 MB/s |
| Timsort | 100000 | 5.3960ms | 5.5270ms | 4.89% | 1017755 | 0 | 56.16% | 1.26 | 282.78 MB/s |
| ARS Gen 1: Foundation | 100000 | 11.1548ms | 12.3204ms | 9.18% | 707587 | 300000 | 56.17% | 1.26 | 136.79 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 13.0690ms | 13.7545ms | 3.97% | 709195 | 300000 | 56.17% | 1.26 | 116.76 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 9.9388ms | 10.9892ms | 3.58% | 1240476 | 108703 | 56.17% | 1.26 | 153.53 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.5391ms | 4.7722ms | 21.91% | 663812 | 100000 | 56.16% | 1.26 | 600.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.2236ms | 3.9248ms | 32.46% | 663812 | 0 | 56.16% | 1.26 | 1247.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.5619ms | 4.5244ms | 31.38% | 665362 | 0 | 56.16% | 1.26 | 976.95 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.9269ms | 4.6384ms | 12.89% | 547659 | 0 | 56.17% | 1.26 | 388.57 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.0681ms | 4.6843ms | 16.11% | 555469 | 0 | 56.16% | 1.26 | 375.08 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.0200ms | 4.7831ms | 28.04% | 125749 | 0 | 56.16% | 1.26 | 1495.99 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.5717ms | 5.6343ms | 27.37% | 166857 | 0 | 56.16% | 1.26 | 593.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.8768ms | 4.1909ms | 25.83% | 189331 | 0 | 56.16% | 1.26 | 813.01 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 5.5357ms | 6.4481ms | 13.46% | 663812 | 200000 | 56.16% | 1.26 | 275.64 MB/s |
| Quicksort | 1000000 | 7.7445ms | 8.7253ms | 6.92% | 9971917 | 0 | 56.12% | 1.26 | 1970.27 MB/s |
| Timsort | 1000000 | 15.1009ms | 16.0803ms | 5.22% | 11039488 | 0 | 56.04% | 1.26 | 1010.46 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 24.0667ms | 24.9849ms | 2.16% | 12357126 | 1017407 | 56.13% | 1.26 | 634.02 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 12.3960ms | 12.9232ms | 9.56% | 4934772 | 1000000 | 56.17% | 1.26 | 1230.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.5457ms | 13.6904ms | 8.08% | 4934772 | 0 | 56.18% | 1.26 | 1321.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.8399ms | 15.0827ms | 13.24% | 4944973 | 0 | 56.14% | 1.26 | 1288.76 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.1762ms | 9.8128ms | 11.65% | 5063234 | 0 | 56.17% | 1.26 | 2126.29 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.6750ms | 9.8317ms | 13.75% | 5065153 | 0 | 56.12% | 1.26 | 1758.93 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 16.4483ms | 17.4342ms | 7.97% | 1132903 | 0 | 56.17% | 1.26 | 927.68 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 16.6476ms | 18.2006ms | 5.50% | 1043081 | 0 | 56.19% | 1.26 | 916.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 16.2331ms | 18.2271ms | 6.41% | 1038794 | 0 | 56.19% | 1.26 | 939.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.3077ms | 20.5835ms | 6.20% | 10737611 | 2000000 | 56.14% | 1.26 | 790.29 MB/s |
| Quicksort | 10000000 | 125.8356ms | 129.4809ms | 3.04% | 98371077 | 0 | 55.78% | 1.27 | 1212.60 MB/s |
| Timsort | 10000000 | 275.5224ms | 286.8255ms | 1.74% | 113777861 | 0 | 55.15% | 1.27 | 553.81 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 231.9599ms | 237.5887ms | 1.55% | 121770051 | 10017407 | 55.89% | 1.27 | 657.82 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 150.9735ms | 157.3922ms | 3.31% | 52183839 | 10000000 | 56.46% | 1.23 | 1010.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 149.4308ms | 154.3850ms | 2.98% | 52183839 | 0 | 56.47% | 1.24 | 1021.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 221.5887ms | 226.3684ms | 1.82% | 57144706 | 0 | 56.55% | 1.22 | 688.61 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 106.8208ms | 113.7962ms | 3.75% | 52596238 | 0 | 56.32% | 1.25 | 1428.45 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 181.1283ms | 190.3668ms | 2.70% | 58371336 | 0 | 56.48% | 1.22 | 842.43 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 182.1117ms | 186.6443ms | 1.73% | 19784607 | 0 | 56.64% | 1.23 | 837.88 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 181.8145ms | 184.2671ms | 1.10% | 19725909 | 0 | 56.84% | 1.23 | 839.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 178.2311ms | 185.8234ms | 2.22% | 19445166 | 0 | 56.71% | 1.23 | 856.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 353.7430ms | 357.5292ms | 1.10% | 171902148 | 20000000 | 56.16% | 1.26 | 431.35 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0170ms | 0.0550ms | 42.16% | 10305 | 0 | 56.39% | 1.27 | 899.27 MB/s |
| Timsort | 1000 | 0.0974ms | 0.1209ms | 14.97% | 10933 | 0 | 56.39% | 1.27 | 156.60 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.4809ms | 1.1207ms | 20.79% | 0 | 2000 | 56.39% | 1.27 | 31.73 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.3909ms | 1.4134ms | 24.50% | 0 | 2000 | 56.39% | 1.27 | 39.04 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0584ms | 0.0699ms | 18.40% | 10305 | 0 | 56.39% | 1.27 | 261.32 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0170ms | 0.0597ms | 29.34% | 10305 | 0 | 56.39% | 1.27 | 895.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0585ms | 0.0595ms | 28.29% | 10305 | 0 | 56.39% | 1.27 | 260.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0299ms | 0.1189ms | 32.71% | 10933 | 0 | 56.39% | 1.27 | 509.83 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0169ms | 0.0552ms | 28.85% | 10305 | 0 | 56.39% | 1.27 | 902.19 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0418ms | 0.1156ms | 34.81% | 10933 | 0 | 56.39% | 1.27 | 365.24 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0176ms | 0.0610ms | 32.87% | 10305 | 0 | 56.39% | 1.27 | 867.77 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0240ms | 0.0601ms | 42.84% | 10305 | 0 | 56.39% | 1.27 | 635.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0179ms | 0.0592ms | 35.00% | 10305 | 0 | 56.39% | 1.27 | 850.74 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3901ms | 0.7871ms | 18.93% | 10305 | 2000 | 56.39% | 1.27 | 39.12 MB/s |
| Quicksort | 10000 | 0.5996ms | 0.6457ms | 7.75% | 136672 | 0 | 56.39% | 1.27 | 254.50 MB/s |
| Timsort | 10000 | 0.3021ms | 1.0368ms | 23.63% | 142159 | 0 | 56.39% | 1.27 | 505.17 MB/s |
| ARS Gen 1: Foundation | 10000 | 25.2917ms | 26.8022ms | 3.83% | 0 | 30000 | 56.37% | 1.27 | 6.03 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 7.7671ms | 28.0090ms | 23.99% | 0 | 30000 | 56.35% | 1.27 | 19.65 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.3738ms | 1.7804ms | 8.39% | 193327 | 14351 | 56.38% | 1.27 | 111.07 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.6134ms | 1.7127ms | 4.33% | 52129 | 10000 | 56.38% | 1.27 | 94.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.9350ms | 1.4082ms | 17.80% | 52129 | 0 | 56.38% | 1.27 | 163.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.2853ms | 1.1255ms | 39.08% | 57416 | 0 | 56.38% | 1.27 | 534.89 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6166ms | 1.4769ms | 28.31% | 52129 | 0 | 56.38% | 1.27 | 247.48 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.4496ms | 1.4987ms | 36.25% | 57416 | 0 | 56.38% | 1.27 | 339.39 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3230ms | 1.1201ms | 29.97% | 52129 | 0 | 56.38% | 1.27 | 472.38 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9645ms | 1.2274ms | 29.08% | 52129 | 0 | 56.38% | 1.27 | 158.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.9424ms | 1.2275ms | 43.17% | 52129 | 0 | 56.38% | 1.27 | 161.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.1696ms | 2.0844ms | 20.94% | 52129 | 20000 | 56.38% | 1.27 | 130.46 MB/s |
| Quicksort | 100000 | 2.4420ms | 7.5547ms | 21.94% | 1707097 | 0 | 56.37% | 1.27 | 624.84 MB/s |
| Timsort | 100000 | 11.4176ms | 12.0078ms | 3.24% | 1751146 | 0 | 56.37% | 1.27 | 133.64 MB/s |
| ARS Gen 1: Foundation | 100000 | 44.0220ms | 72.8672ms | 16.82% | 3 | 300000 | 55.54% | 1.27 | 34.66 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 65.1499ms | 72.2453ms | 4.66% | 3 | 300000 | 55.48% | 1.27 | 23.42 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 5.0745ms | 11.6038ms | 18.58% | 1885325 | 108703 | 56.37% | 1.27 | 300.70 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.2251ms | 5.4068ms | 30.29% | 879606 | 100000 | 56.37% | 1.27 | 1245.54 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.5696ms | 4.5505ms | 16.35% | 879606 | 0 | 56.37% | 1.27 | 427.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.8142ms | 4.8198ms | 10.81% | 917984 | 0 | 56.37% | 1.27 | 400.05 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.5002ms | 4.0874ms | 25.82% | 957954 | 0 | 56.37% | 1.27 | 1017.13 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.1852ms | 5.2946ms | 12.53% | 997595 | 0 | 56.37% | 1.27 | 364.59 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.7662ms | 4.5114ms | 12.56% | 879606 | 0 | 56.37% | 1.27 | 405.15 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.5079ms | 5.6252ms | 18.69% | 771914 | 0 | 56.37% | 1.27 | 608.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.2277ms | 4.6347ms | 29.22% | 879606 | 0 | 56.37% | 1.27 | 1242.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.3581ms | 7.4302ms | 21.42% | 879606 | 200000 | 56.37% | 1.27 | 454.39 MB/s |
| Quicksort | 1000000 | 40.1681ms | 42.0959ms | 3.38% | 20415976 | 0 | 56.34% | 1.27 | 379.87 MB/s |
| Timsort | 1000000 | 37.4535ms | 53.9563ms | 14.58% | 20814811 | 0 | 56.28% | 1.27 | 407.41 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 39.4862ms | 48.7886ms | 8.66% | 21495627 | 1017407 | 56.33% | 1.27 | 386.43 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 26.2534ms | 30.6112ms | 16.16% | 10213516 | 1000000 | 56.37% | 1.27 | 581.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 21.8985ms | 24.4386ms | 13.71% | 10213516 | 0 | 56.37% | 1.27 | 696.80 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.6820ms | 28.1564ms | 19.15% | 10626903 | 0 | 56.37% | 1.27 | 816.76 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 21.8369ms | 26.3719ms | 11.07% | 12313388 | 0 | 56.37% | 1.27 | 698.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 14.2892ms | 30.9255ms | 29.33% | 12746461 | 0 | 56.36% | 1.27 | 1067.86 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.9780ms | 26.7941ms | 12.49% | 10213516 | 0 | 56.37% | 1.27 | 664.06 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 14.8298ms | 29.3828ms | 25.54% | 11275265 | 0 | 56.37% | 1.27 | 1028.93 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.6333ms | 23.7690ms | 21.66% | 12313388 | 0 | 56.38% | 1.27 | 818.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 31.5983ms | 44.0662ms | 11.78% | 13568177 | 2000000 | 56.35% | 1.27 | 482.90 MB/s |
| Quicksort | 10000000 | 253.6977ms | 260.8226ms | 3.82% | 237608061 | 0 | 56.16% | 1.29 | 601.46 MB/s |
| Timsort | 10000000 | 474.4542ms | 479.2364ms | 1.29% | 241464906 | 0 | 55.78% | 1.29 | 321.61 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 277.4851ms | 285.8930ms | 2.28% | 247456721 | 10017407 | 56.09% | 1.28 | 549.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 142.3792ms | 146.1232ms | 1.18% | 136761715 | 10000000 | 56.64% | 1.26 | 1071.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 128.7195ms | 132.8084ms | 2.91% | 136761715 | 0 | 56.64% | 1.26 | 1185.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 141.9653ms | 145.5580ms | 2.41% | 140910527 | 0 | 56.48% | 1.26 | 1074.83 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 103.0614ms | 108.5355ms | 3.04% | 157194307 | 0 | 56.55% | 1.27 | 1480.55 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 121.5892ms | 136.7725ms | 5.65% | 161230740 | 0 | 56.13% | 1.26 | 1254.95 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 149.7541ms | 154.6006ms | 1.93% | 46364715 | 0 | 56.14% | 1.26 | 1018.92 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 159.9506ms | 165.5895ms | 2.50% | 51640382 | 0 | 56.31% | 1.25 | 953.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 146.5125ms | 153.3610ms | 3.84% | 51645714 | 0 | 56.24% | 1.26 | 1041.47 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 468.1230ms | 478.2054ms | 1.62% | 199083881 | 20000000 | 56.47% | 1.26 | 325.96 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0219ms | 0.0254ms | 35.87% | 5551 | 0 | 56.84% | 1.23 | 697.74 MB/s |
| Timsort | 1000 | 0.0150ms | 0.0551ms | 31.84% | 5969 | 0 | 56.84% | 1.23 | 1018.07 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1077ms | 0.2653ms | 27.34% | 984 | 2000 | 56.84% | 1.23 | 141.70 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2029ms | 0.3190ms | 15.68% | 984 | 2000 | 56.84% | 1.23 | 75.22 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0092ms | 0.0285ms | 33.38% | 5551 | 0 | 56.84% | 1.23 | 1652.99 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0097ms | 0.0285ms | 38.00% | 5551 | 0 | 56.84% | 1.23 | 1580.89 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0150ms | 0.0282ms | 21.16% | 5551 | 0 | 56.84% | 1.23 | 1017.93 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0251ms | 0.0541ms | 34.34% | 5969 | 0 | 56.84% | 1.23 | 609.06 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0245ms | 0.0256ms | 26.44% | 5551 | 0 | 56.84% | 1.23 | 622.96 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0151ms | 0.0539ms | 31.50% | 5969 | 0 | 56.84% | 1.23 | 1009.05 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0093ms | 0.0283ms | 35.09% | 5551 | 0 | 56.84% | 1.23 | 1649.24 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0103ms | 0.0289ms | 34.16% | 5551 | 0 | 56.84% | 1.23 | 1478.14 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0089ms | 0.0294ms | 36.65% | 5551 | 0 | 56.84% | 1.23 | 1711.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.6270ms | 0.7666ms | 12.83% | 5551 | 2000 | 56.83% | 1.23 | 24.34 MB/s |
| Quicksort | 10000 | 0.0667ms | 0.1801ms | 35.23% | 54394 | 0 | 56.83% | 1.23 | 2288.36 MB/s |
| Timsort | 10000 | 0.1161ms | 0.3393ms | 26.40% | 53253 | 0 | 56.83% | 1.23 | 1314.03 MB/s |
| ARS Gen 1: Foundation | 10000 | 0.6645ms | 1.7232ms | 22.35% | 9984 | 30000 | 56.83% | 1.23 | 229.62 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.6977ms | 2.0065ms | 12.53% | 9984 | 30000 | 56.83% | 1.23 | 89.88 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.4756ms | 1.7477ms | 24.70% | 122450 | 14351 | 56.83% | 1.23 | 320.86 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.4493ms | 1.6004ms | 48.84% | 9988 | 10000 | 56.83% | 1.23 | 339.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.8327ms | 1.0364ms | 18.36% | 9988 | 0 | 56.83% | 1.23 | 183.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.8748ms | 1.2407ms | 17.54% | 9988 | 0 | 56.83% | 1.23 | 174.42 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.3703ms | 1.4460ms | 54.10% | 9988 | 0 | 56.83% | 1.23 | 412.03 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.2270ms | 1.6806ms | 15.67% | 9988 | 0 | 56.83% | 1.23 | 124.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.8158ms | 1.0094ms | 61.69% | 9988 | 0 | 56.83% | 1.23 | 187.05 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.4252ms | 1.1521ms | 26.61% | 9988 | 0 | 56.83% | 1.23 | 358.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3741ms | 1.5541ms | 50.90% | 9988 | 0 | 56.83% | 1.23 | 407.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.8504ms | 2.2512ms | 14.21% | 9988 | 20000 | 56.83% | 1.23 | 82.46 MB/s |
| Quicksort | 100000 | 1.8520ms | 1.9871ms | 7.51% | 529352 | 0 | 56.82% | 1.23 | 823.90 MB/s |
| Timsort | 100000 | 2.0603ms | 3.3061ms | 12.27% | 535711 | 0 | 56.82% | 1.23 | 740.60 MB/s |
| ARS Gen 1: Foundation | 100000 | 2.4130ms | 8.2939ms | 25.25% | 99984 | 300000 | 56.82% | 1.23 | 632.35 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 9.2685ms | 10.4840ms | 6.49% | 99984 | 300000 | 56.82% | 1.23 | 164.63 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.4018ms | 10.8387ms | 22.68% | 1144467 | 108703 | 56.82% | 1.23 | 448.56 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.5049ms | 4.3844ms | 27.63% | 99990 | 100000 | 56.82% | 1.23 | 1013.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 0.7919ms | 2.9803ms | 33.96% | 99990 | 0 | 56.82% | 1.23 | 1926.96 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.9076ms | 3.0170ms | 31.86% | 99990 | 0 | 56.82% | 1.23 | 799.88 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.8039ms | 3.4651ms | 7.79% | 99990 | 0 | 56.82% | 1.23 | 544.20 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.4868ms | 3.4706ms | 25.38% | 99990 | 0 | 56.82% | 1.23 | 1026.29 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 0.8214ms | 3.9333ms | 45.53% | 199974 | 0 | 56.82% | 1.23 | 1857.75 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.3307ms | 4.8032ms | 24.47% | 199974 | 0 | 56.82% | 1.23 | 1146.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 0.9722ms | 3.1769ms | 37.11% | 99990 | 0 | 56.82% | 1.23 | 1569.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 5.1990ms | 5.9732ms | 15.83% | 99990 | 200000 | 56.82% | 1.23 | 293.49 MB/s |
| Quicksort | 1000000 | 5.4831ms | 6.1406ms | 8.39% | 5262225 | 0 | 56.79% | 1.23 | 2782.86 MB/s |
| Timsort | 1000000 | 10.3074ms | 11.7646ms | 10.78% | 6171924 | 0 | 56.75% | 1.23 | 1480.38 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 22.8401ms | 24.1106ms | 5.34% | 12090683 | 1017407 | 56.82% | 1.23 | 668.07 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.3093ms | 13.3184ms | 7.35% | 999988 | 1000000 | 56.84% | 1.23 | 1349.22 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.7134ms | 13.1502ms | 9.10% | 999988 | 0 | 56.84% | 1.23 | 1424.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.5878ms | 12.5118ms | 9.84% | 999988 | 0 | 56.84% | 1.23 | 1441.17 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 6.4624ms | 8.3162ms | 13.98% | 999988 | 0 | 56.83% | 1.23 | 2361.18 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 6.6023ms | 8.6578ms | 8.98% | 999988 | 0 | 56.83% | 1.23 | 2311.14 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 12.7670ms | 13.9123ms | 8.16% | 1999972 | 0 | 56.84% | 1.23 | 1195.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 10.5557ms | 12.3196ms | 7.61% | 1999972 | 0 | 56.83% | 1.23 | 1445.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.8119ms | 14.4797ms | 5.96% | 1999972 | 0 | 56.84% | 1.23 | 1104.75 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 20.0648ms | 20.7810ms | 5.21% | 5484601 | 2000000 | 56.81% | 1.23 | 760.48 MB/s |
| Quicksort | 10000000 | 79.4670ms | 80.3964ms | 2.13% | 51920218 | 0 | 56.82% | 1.23 | 1920.14 MB/s |
| Timsort | 10000000 | 222.6340ms | 226.4450ms | 4.80% | 66331581 | 0 | 56.71% | 1.23 | 685.38 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 195.1523ms | 199.3365ms | 2.74% | 120088471 | 10017407 | 56.89% | 1.23 | 781.89 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 99.1962ms | 103.3069ms | 2.55% | 9999988 | 10000000 | 57.09% | 1.22 | 1538.24 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 99.7780ms | 105.5366ms | 3.38% | 9999988 | 0 | 57.09% | 1.22 | 1529.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 99.5928ms | 103.1843ms | 3.09% | 9999988 | 0 | 57.10% | 1.22 | 1532.12 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 62.1699ms | 64.0421ms | 2.46% | 9999988 | 0 | 57.00% | 1.22 | 2454.37 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 61.4937ms | 64.8061ms | 3.78% | 9999988 | 0 | 57.00% | 1.22 | 2481.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 124.4161ms | 129.4466ms | 2.21% | 19999972 | 0 | 57.18% | 1.22 | 1226.43 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 105.7547ms | 108.1079ms | 3.31% | 19999972 | 0 | 57.14% | 1.22 | 1442.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 123.4825ms | 128.3150ms | 1.59% | 19999972 | 0 | 57.18% | 1.22 | 1235.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 342.2893ms | 348.0052ms | 1.15% | 114690054 | 20000000 | 56.80% | 1.23 | 445.79 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0164ms | 0.0551ms | 32.84% | 10305 | 0 | 56.96% | 1.22 | 928.09 MB/s |
| Timsort | 1000 | 0.0233ms | 0.0995ms | 43.68% | 10933 | 0 | 56.96% | 1.22 | 655.70 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.8335ms | 1.1849ms | 11.27% | 0 | 2000 | 56.96% | 1.22 | 18.31 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.3971ms | 1.4035ms | 23.82% | 0 | 2000 | 56.96% | 1.22 | 38.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0308ms | 0.0667ms | 29.96% | 10305 | 0 | 56.96% | 1.22 | 495.91 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0143ms | 0.0614ms | 66.14% | 10305 | 0 | 56.96% | 1.22 | 1066.15 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0160ms | 0.0598ms | 40.71% | 10305 | 0 | 56.96% | 1.22 | 953.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0283ms | 0.0990ms | 24.50% | 10933 | 0 | 56.96% | 1.22 | 539.58 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0211ms | 0.0710ms | 35.25% | 10305 | 0 | 56.96% | 1.22 | 722.48 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0234ms | 0.1023ms | 28.25% | 10933 | 0 | 56.96% | 1.22 | 650.94 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0192ms | 0.0594ms | 26.63% | 10305 | 0 | 56.96% | 1.22 | 794.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0242ms | 0.0594ms | 32.62% | 10305 | 0 | 56.96% | 1.22 | 629.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0591ms | 0.0615ms | 15.34% | 10305 | 0 | 56.96% | 1.22 | 258.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.2463ms | 0.7819ms | 31.96% | 10305 | 2000 | 56.96% | 1.22 | 61.95 MB/s |
| Quicksort | 10000 | 0.2098ms | 0.6426ms | 23.23% | 136672 | 0 | 56.96% | 1.22 | 727.15 MB/s |
| Timsort | 10000 | 0.2893ms | 1.0712ms | 32.23% | 142159 | 0 | 56.96% | 1.22 | 527.47 MB/s |
| ARS Gen 1: Foundation | 10000 | 24.6156ms | 27.3297ms | 5.54% | 0 | 30000 | 56.94% | 1.22 | 6.20 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 25.0874ms | 28.0454ms | 11.79% | 0 | 30000 | 56.93% | 1.22 | 6.08 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.5089ms | 1.9145ms | 43.24% | 193327 | 14351 | 56.95% | 1.22 | 299.82 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.4327ms | 1.6971ms | 24.77% | 52129 | 10000 | 56.95% | 1.22 | 352.62 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.2925ms | 1.3951ms | 35.22% | 52129 | 0 | 56.95% | 1.22 | 521.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.0569ms | 1.3545ms | 35.82% | 57416 | 0 | 56.95% | 1.22 | 144.37 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.0093ms | 1.5852ms | 19.47% | 52129 | 0 | 56.95% | 1.22 | 151.18 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.7498ms | 1.4768ms | 26.04% | 57416 | 0 | 56.95% | 1.22 | 203.50 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3132ms | 1.1930ms | 38.96% | 52129 | 0 | 56.95% | 1.22 | 487.23 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9540ms | 1.1790ms | 18.00% | 52129 | 0 | 56.95% | 1.22 | 159.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.4821ms | 1.3107ms | 31.66% | 52129 | 0 | 56.95% | 1.22 | 316.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.1942ms | 2.2773ms | 28.52% | 52129 | 20000 | 56.95% | 1.22 | 127.77 MB/s |
| Quicksort | 100000 | 2.6046ms | 7.8460ms | 24.72% | 1707097 | 0 | 56.95% | 1.22 | 585.83 MB/s |
| Timsort | 100000 | 3.7869ms | 11.5850ms | 24.77% | 1751146 | 0 | 56.94% | 1.22 | 402.93 MB/s |
| ARS Gen 1: Foundation | 100000 | 49.2882ms | 71.3547ms | 10.30% | 3 | 300000 | 56.27% | 1.22 | 30.96 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.9008ms | 72.5153ms | 14.05% | 3 | 300000 | 56.32% | 1.22 | 33.24 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 3.1464ms | 11.6140ms | 34.45% | 1885325 | 108703 | 56.94% | 1.22 | 484.96 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.6144ms | 5.7399ms | 16.39% | 879606 | 100000 | 56.94% | 1.22 | 422.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.7632ms | 4.5076ms | 17.40% | 879606 | 0 | 56.94% | 1.22 | 405.47 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.8715ms | 5.2192ms | 16.22% | 917984 | 0 | 56.94% | 1.22 | 394.13 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.1677ms | 4.9534ms | 9.87% | 957954 | 0 | 56.94% | 1.22 | 366.12 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.2974ms | 5.3159ms | 27.75% | 997595 | 0 | 56.94% | 1.22 | 1176.10 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.3972ms | 4.3355ms | 14.51% | 879606 | 0 | 56.94% | 1.22 | 449.16 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.8612ms | 5.5841ms | 7.64% | 771914 | 0 | 56.94% | 1.22 | 313.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.3795ms | 4.3555ms | 17.35% | 879606 | 0 | 56.94% | 1.22 | 641.25 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 6.5621ms | 7.3522ms | 14.36% | 879606 | 200000 | 56.94% | 1.22 | 232.53 MB/s |
| Quicksort | 1000000 | 33.2406ms | 41.4783ms | 6.78% | 20415976 | 0 | 56.92% | 1.22 | 459.04 MB/s |
| Timsort | 1000000 | 36.2207ms | 53.5098ms | 12.58% | 20814811 | 0 | 56.86% | 1.22 | 421.27 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.7875ms | 49.8630ms | 12.11% | 21495627 | 1017407 | 56.91% | 1.22 | 480.02 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.6654ms | 27.5753ms | 13.89% | 10213516 | 1000000 | 56.95% | 1.22 | 817.49 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.2262ms | 26.0078ms | 11.44% | 10213516 | 0 | 56.94% | 1.22 | 686.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.1882ms | 30.2838ms | 23.77% | 10626903 | 0 | 56.95% | 1.22 | 795.22 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 14.6583ms | 28.6158ms | 23.89% | 12313388 | 0 | 56.94% | 1.22 | 1040.96 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 15.2624ms | 29.9045ms | 24.42% | 12746461 | 0 | 56.95% | 1.22 | 999.77 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.7328ms | 25.6898ms | 12.97% | 10213516 | 0 | 56.95% | 1.22 | 671.22 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 26.4513ms | 28.8587ms | 13.35% | 11275265 | 0 | 56.95% | 1.22 | 576.86 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 23.6682ms | 33.9493ms | 15.73% | 12313388 | 0 | 56.95% | 1.22 | 644.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.8433ms | 46.3268ms | 12.28% | 13713898 | 2000000 | 56.94% | 1.22 | 511.30 MB/s |
| Quicksort | 10000000 | 258.1012ms | 264.3274ms | 1.35% | 237608061 | 0 | 56.76% | 1.24 | 591.19 MB/s |
| Timsort | 10000000 | 464.4107ms | 474.8237ms | 1.27% | 241464906 | 0 | 56.43% | 1.24 | 328.56 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 278.7550ms | 285.6695ms | 1.54% | 247456721 | 10017407 | 56.72% | 1.23 | 547.39 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 142.1835ms | 147.7756ms | 2.01% | 136761715 | 10000000 | 57.13% | 1.22 | 1073.18 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 130.4984ms | 133.3855ms | 1.36% | 136761715 | 0 | 57.14% | 1.22 | 1169.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 140.6205ms | 143.7954ms | 2.55% | 140910527 | 0 | 57.02% | 1.22 | 1085.10 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 103.4041ms | 108.5139ms | 3.65% | 157194307 | 0 | 57.06% | 1.22 | 1475.65 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 122.9515ms | 136.8102ms | 5.24% | 161230740 | 0 | 56.73% | 1.22 | 1241.04 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 152.2499ms | 156.2280ms | 1.92% | 46364715 | 0 | 56.74% | 1.21 | 1002.22 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 162.5337ms | 165.2080ms | 1.51% | 51640382 | 0 | 56.87% | 1.21 | 938.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 149.4378ms | 152.4892ms | 1.37% | 51645714 | 0 | 56.82% | 1.22 | 1021.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 468.7277ms | 477.6118ms | 1.28% | 199083830 | 20000000 | 57.03% | 1.22 | 325.54 MB/s |

## Category: f64

### Distribution: Random

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0222ms | 0.0782ms | 30.33% | 10310 | 0 | 57.13% | 1.20 | 688.26 MB/s |
| Timsort | 1000 | 0.0621ms | 0.1605ms | 24.38% | 10766 | 0 | 57.13% | 1.20 | 245.73 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.5753ms | 1.1919ms | 22.74% | 0 | 2000 | 57.13% | 1.20 | 26.52 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 1.1815ms | 1.4323ms | 22.21% | 0 | 2000 | 57.13% | 1.20 | 12.92 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0686ms | 0.0828ms | 12.42% | 10310 | 0 | 57.13% | 1.20 | 222.48 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0404ms | 0.0832ms | 44.24% | 10310 | 0 | 57.13% | 1.20 | 377.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0273ms | 0.0883ms | 31.69% | 10310 | 0 | 57.13% | 1.20 | 558.19 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1284ms | 0.1328ms | 11.34% | 10766 | 0 | 57.13% | 1.20 | 118.88 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0243ms | 0.0808ms | 32.01% | 10310 | 0 | 57.13% | 1.20 | 627.39 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0387ms | 0.1313ms | 30.17% | 10766 | 0 | 57.13% | 1.20 | 394.41 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0282ms | 0.0797ms | 30.01% | 10310 | 0 | 57.13% | 1.20 | 541.80 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0358ms | 0.0844ms | 28.20% | 10310 | 0 | 57.13% | 1.20 | 425.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0828ms | 0.0945ms | 20.35% | 10310 | 0 | 57.13% | 1.20 | 184.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3567ms | 0.8364ms | 21.50% | 10310 | 2000 | 57.13% | 1.20 | 42.78 MB/s |
| Quicksort | 10000 | 0.9052ms | 0.9326ms | 5.35% | 135714 | 0 | 57.13% | 1.20 | 168.57 MB/s |
| Timsort | 10000 | 0.4499ms | 1.5220ms | 23.24% | 141521 | 0 | 57.13% | 1.20 | 339.19 MB/s |
| ARS Gen 1: Foundation | 10000 | 13.8597ms | 27.1015ms | 16.15% | 0 | 30000 | 57.12% | 1.20 | 11.01 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 27.2585ms | 28.9474ms | 2.60% | 0 | 30000 | 57.10% | 1.20 | 5.60 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.7325ms | 2.2250ms | 22.28% | 193952 | 14351 | 57.13% | 1.20 | 208.30 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.5713ms | 1.7426ms | 18.46% | 69701 | 10000 | 57.13% | 1.20 | 97.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.0079ms | 1.2393ms | 24.70% | 69701 | 0 | 57.13% | 1.20 | 151.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.3631ms | 1.3723ms | 36.60% | 72569 | 0 | 57.13% | 1.20 | 420.22 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.3989ms | 1.3097ms | 24.59% | 62514 | 0 | 57.13% | 1.20 | 382.50 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.8695ms | 1.6216ms | 35.15% | 65598 | 0 | 57.13% | 1.20 | 175.48 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.1312ms | 1.5109ms | 33.41% | 69701 | 0 | 57.13% | 1.20 | 134.89 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.3087ms | 1.2085ms | 37.61% | 69701 | 0 | 57.13% | 1.20 | 494.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3502ms | 1.3850ms | 54.07% | 69701 | 0 | 57.13% | 1.20 | 435.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.2306ms | 2.1889ms | 25.12% | 69701 | 20000 | 57.13% | 1.20 | 123.99 MB/s |
| Quicksort | 100000 | 4.9261ms | 11.0644ms | 17.94% | 1706698 | 0 | 57.13% | 1.20 | 309.75 MB/s |
| Timsort | 100000 | 17.0322ms | 17.5232ms | 5.63% | 1745198 | 0 | 57.12% | 1.20 | 89.59 MB/s |
| ARS Gen 1: Foundation | 100000 | 46.7396ms | 68.5092ms | 12.65% | 0 | 300000 | 56.56% | 1.20 | 32.65 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 65.8568ms | 70.0268ms | 3.98% | 0 | 300000 | 56.48% | 1.20 | 23.17 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 13.7663ms | 14.4130ms | 5.52% | 1886258 | 108703 | 57.13% | 1.20 | 110.84 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.9373ms | 6.5388ms | 29.45% | 1093083 | 100000 | 57.12% | 1.20 | 787.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.6691ms | 5.6507ms | 29.52% | 1093083 | 0 | 57.12% | 1.20 | 914.20 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 5.6292ms | 7.0328ms | 18.96% | 1131207 | 0 | 57.12% | 1.20 | 271.06 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.9255ms | 4.5701ms | 24.25% | 984738 | 0 | 57.12% | 1.20 | 521.58 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.8781ms | 5.6523ms | 12.96% | 1026694 | 0 | 57.12% | 1.20 | 312.80 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.6181ms | 5.4141ms | 19.29% | 1093083 | 0 | 57.12% | 1.20 | 330.41 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.6300ms | 6.7164ms | 15.77% | 985491 | 0 | 57.12% | 1.20 | 271.03 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.6814ms | 5.3978ms | 13.31% | 1093083 | 0 | 57.12% | 1.20 | 325.95 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.6604ms | 8.5742ms | 24.72% | 1093083 | 200000 | 57.12% | 1.20 | 573.54 MB/s |
| Quicksort | 1000000 | 33.9045ms | 46.4005ms | 8.49% | 20434861 | 0 | 57.10% | 1.21 | 450.05 MB/s |
| Timsort | 1000000 | 50.6290ms | 63.8008ms | 11.53% | 20809630 | 0 | 57.06% | 1.20 | 301.38 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 47.4412ms | 52.6837ms | 4.39% | 21497315 | 1017407 | 57.09% | 1.20 | 321.64 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.6391ms | 29.4397ms | 12.69% | 12621941 | 1000000 | 57.13% | 1.20 | 818.64 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.8139ms | 26.1150ms | 9.36% | 12621941 | 0 | 57.13% | 1.20 | 668.84 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.9011ms | 27.3519ms | 26.17% | 13031943 | 0 | 57.13% | 1.20 | 852.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 13.8692ms | 23.5567ms | 15.63% | 13370552 | 0 | 57.12% | 1.20 | 1100.19 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.9182ms | 27.8748ms | 24.24% | 13799291 | 0 | 57.12% | 1.20 | 696.17 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 17.2459ms | 24.5632ms | 17.23% | 6373946 | 0 | 57.11% | 1.20 | 884.78 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 18.1924ms | 29.7343ms | 21.31% | 5848191 | 0 | 57.10% | 1.20 | 838.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.4626ms | 26.7742ms | 20.19% | 7359383 | 0 | 57.11% | 1.20 | 745.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 32.2907ms | 40.6233ms | 9.23% | 14404301 | 2000000 | 57.13% | 1.20 | 472.54 MB/s |
| Quicksort | 10000000 | 329.7222ms | 333.7391ms | 1.64% | 237626959 | 0 | 56.91% | 1.22 | 462.78 MB/s |
| Timsort | 10000000 | 605.5097ms | 616.5352ms | 0.74% | 241455097 | 0 | 56.64% | 1.22 | 252.00 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 318.9100ms | 329.3742ms | 3.54% | 247420456 | 10017407 | 56.85% | 1.21 | 478.47 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 176.4693ms | 179.8900ms | 1.73% | 165371492 | 10000000 | 57.22% | 1.20 | 864.67 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 152.9454ms | 156.0699ms | 1.83% | 165371492 | 0 | 57.22% | 1.20 | 997.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 185.6154ms | 192.3588ms | 2.02% | 169379491 | 0 | 56.98% | 1.19 | 822.06 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 131.6173ms | 135.1743ms | 2.33% | 171381149 | 0 | 57.12% | 1.21 | 1159.33 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 173.2179ms | 175.6614ms | 2.49% | 175305245 | 0 | 56.84% | 1.20 | 880.90 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 169.5285ms | 176.3176ms | 2.98% | 74378611 | 0 | 57.18% | 1.19 | 900.07 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 192.4176ms | 201.2048ms | 2.15% | 83436719 | 0 | 57.40% | 1.19 | 793.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 177.1874ms | 184.1076ms | 2.14% | 85006721 | 0 | 57.31% | 1.19 | 861.17 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 370.0287ms | 379.6958ms | 1.91% | 181177264 | 20000000 | 57.27% | 1.19 | 412.37 MB/s |

### Distribution: Gaussian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0414ms | 0.0880ms | 24.24% | 10127 | 0 | 57.12% | 1.19 | 368.50 MB/s |
| Timsort | 1000 | 0.0382ms | 0.1490ms | 27.51% | 10659 | 0 | 57.12% | 1.19 | 399.56 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.7623ms | 1.1695ms | 19.36% | 0 | 2000 | 57.12% | 1.19 | 20.02 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.5382ms | 1.4095ms | 20.39% | 0 | 2000 | 57.12% | 1.19 | 28.35 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0330ms | 0.0829ms | 25.77% | 10127 | 0 | 57.12% | 1.19 | 462.29 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0789ms | 0.0835ms | 7.56% | 10127 | 0 | 57.12% | 1.19 | 193.35 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0355ms | 0.0853ms | 30.53% | 10127 | 0 | 57.12% | 1.19 | 430.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1281ms | 0.1361ms | 5.93% | 10659 | 0 | 57.12% | 1.19 | 119.12 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0224ms | 0.0799ms | 32.28% | 10127 | 0 | 57.12% | 1.19 | 681.90 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0833ms | 0.1502ms | 19.24% | 10659 | 0 | 57.12% | 1.19 | 183.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0781ms | 0.0802ms | 25.35% | 10127 | 0 | 57.12% | 1.19 | 195.37 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0284ms | 0.0839ms | 32.73% | 10127 | 0 | 57.12% | 1.19 | 536.43 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0250ms | 0.0839ms | 26.09% | 10127 | 0 | 57.12% | 1.19 | 610.40 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.2649ms | 0.8172ms | 24.99% | 10127 | 2000 | 57.12% | 1.19 | 57.59 MB/s |
| Quicksort | 10000 | 0.9030ms | 0.9485ms | 9.54% | 136381 | 0 | 57.12% | 1.19 | 168.98 MB/s |
| Timsort | 10000 | 1.4230ms | 1.5191ms | 6.89% | 141768 | 0 | 57.12% | 1.19 | 107.23 MB/s |
| ARS Gen 1: Foundation | 10000 | 7.1284ms | 27.7447ms | 28.69% | 0 | 30000 | 57.10% | 1.19 | 21.41 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 26.7211ms | 27.9571ms | 7.89% | 0 | 30000 | 57.10% | 1.19 | 5.71 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.0801ms | 2.3895ms | 17.77% | 192917 | 14351 | 57.12% | 1.19 | 73.36 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.6523ms | 2.4547ms | 25.01% | 124728 | 10000 | 57.12% | 1.19 | 233.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.6209ms | 1.9283ms | 27.06% | 124728 | 0 | 57.12% | 1.19 | 94.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.7365ms | 2.1001ms | 9.23% | 129515 | 0 | 57.12% | 1.19 | 87.87 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6223ms | 1.8300ms | 44.13% | 109062 | 0 | 57.12% | 1.19 | 245.19 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.8011ms | 2.2360ms | 22.39% | 113473 | 0 | 57.12% | 1.19 | 190.48 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.4568ms | 1.7801ms | 61.75% | 48646 | 0 | 57.11% | 1.19 | 334.03 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.5049ms | 2.0291ms | 27.07% | 124728 | 0 | 57.12% | 1.19 | 302.21 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.5428ms | 1.6955ms | 36.31% | 124728 | 0 | 57.12% | 1.19 | 281.12 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.4492ms | 2.8764ms | 24.46% | 124728 | 20000 | 57.12% | 1.19 | 62.30 MB/s |
| Quicksort | 100000 | 10.8781ms | 11.2108ms | 5.56% | 1712018 | 0 | 57.11% | 1.19 | 140.27 MB/s |
| Timsort | 100000 | 17.3475ms | 17.5452ms | 3.48% | 1750974 | 0 | 57.11% | 1.19 | 87.96 MB/s |
| ARS Gen 1: Foundation | 100000 | 68.4674ms | 71.7240ms | 2.84% | 0 | 300000 | 56.55% | 1.19 | 22.29 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 68.7542ms | 75.0442ms | 3.78% | 0 | 300000 | 56.46% | 1.19 | 22.19 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 13.9616ms | 14.7337ms | 4.19% | 1886096 | 108703 | 57.11% | 1.19 | 109.29 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 10.5546ms | 11.9168ms | 18.74% | 1585452 | 100000 | 57.11% | 1.19 | 144.57 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 8.2910ms | 9.0921ms | 19.35% | 1585452 | 0 | 57.11% | 1.19 | 184.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 10.6735ms | 11.5087ms | 22.32% | 1628590 | 0 | 57.11% | 1.19 | 142.96 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.0353ms | 7.4015ms | 28.03% | 1442845 | 0 | 57.11% | 1.19 | 749.71 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.1201ms | 10.1830ms | 24.92% | 1482185 | 0 | 57.11% | 1.19 | 370.35 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 6.0346ms | 6.9101ms | 17.12% | 834252 | 0 | 57.10% | 1.19 | 252.86 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.1345ms | 8.6324ms | 37.51% | 639489 | 0 | 57.10% | 1.19 | 714.85 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 5.7872ms | 7.2481ms | 10.66% | 834252 | 0 | 57.10% | 1.19 | 263.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.3763ms | 12.2321ms | 34.75% | 1585452 | 200000 | 57.11% | 1.19 | 451.94 MB/s |
| Quicksort | 1000000 | 34.3476ms | 36.5255ms | 6.35% | 20415776 | 0 | 57.09% | 1.19 | 444.25 MB/s |
| Timsort | 1000000 | 51.1096ms | 54.2193ms | 2.95% | 20815819 | 0 | 57.05% | 1.19 | 298.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 40.1999ms | 42.7167ms | 3.79% | 21501814 | 1017407 | 57.09% | 1.19 | 379.57 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 31.4852ms | 34.7839ms | 12.00% | 17736085 | 1000000 | 57.11% | 1.19 | 484.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 26.5344ms | 30.9055ms | 11.72% | 17736085 | 0 | 57.11% | 1.19 | 575.06 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 34.1174ms | 38.2647ms | 8.13% | 18129686 | 0 | 57.08% | 1.19 | 447.24 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 21.1358ms | 23.8478ms | 11.18% | 17761552 | 0 | 57.10% | 1.19 | 721.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 23.3733ms | 29.3932ms | 11.55% | 18146609 | 0 | 57.08% | 1.19 | 652.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.1236ms | 24.6545ms | 8.44% | 9154383 | 0 | 57.11% | 1.19 | 689.71 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 24.0676ms | 26.3092ms | 4.45% | 9616261 | 0 | 57.12% | 1.19 | 634.00 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 22.2007ms | 23.0232ms | 5.44% | 11739315 | 0 | 57.13% | 1.19 | 687.31 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 28.2946ms | 35.4762ms | 15.38% | 19214890 | 2000000 | 57.11% | 1.19 | 539.28 MB/s |
| Quicksort | 10000000 | 328.1087ms | 340.2124ms | 1.51% | 237632035 | 0 | 56.88% | 1.21 | 465.05 MB/s |
| Timsort | 10000000 | 605.7382ms | 611.5855ms | 1.25% | 241445188 | 0 | 56.60% | 1.20 | 251.90 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 333.9558ms | 352.0117ms | 3.34% | 247397600 | 10017407 | 56.82% | 1.20 | 456.91 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 321.4272ms | 332.7877ms | 2.60% | 210671163 | 10000000 | 57.09% | 1.20 | 474.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 270.1356ms | 283.7026ms | 3.15% | 210671163 | 0 | 57.07% | 1.20 | 564.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 385.1270ms | 393.7933ms | 1.98% | 214571359 | 0 | 56.82% | 1.19 | 396.20 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 211.7261ms | 224.0325ms | 2.85% | 210894295 | 0 | 57.07% | 1.20 | 720.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 310.7239ms | 329.0433ms | 2.51% | 214718921 | 0 | 56.94% | 1.19 | 491.07 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 199.1724ms | 219.0517ms | 3.24% | 97142082 | 0 | 57.28% | 1.18 | 766.11 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 222.6121ms | 231.4239ms | 1.96% | 68882615 | 0 | 57.29% | 1.18 | 685.44 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 219.6911ms | 223.1028ms | 1.50% | 88331953 | 0 | 57.30% | 1.18 | 694.56 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 427.2883ms | 435.0219ms | 3.23% | 229232377 | 20000000 | 57.16% | 1.18 | 357.11 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0261ms | 0.0765ms | 39.93% | 9803 | 0 | 56.77% | 1.19 | 584.96 MB/s |
| Timsort | 1000 | 0.1170ms | 0.1370ms | 16.24% | 9687 | 0 | 56.77% | 1.19 | 130.47 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.3545ms | 0.6440ms | 15.23% | 0 | 2000 | 56.77% | 1.19 | 43.04 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.5495ms | 0.9438ms | 13.47% | 0 | 2000 | 56.77% | 1.19 | 27.77 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0804ms | 0.0976ms | 20.16% | 9803 | 0 | 56.77% | 1.19 | 189.90 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0787ms | 0.0819ms | 11.95% | 9803 | 0 | 56.77% | 1.19 | 193.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0280ms | 0.0920ms | 48.25% | 9803 | 0 | 56.77% | 1.19 | 544.00 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0348ms | 0.1288ms | 30.16% | 9687 | 0 | 56.77% | 1.19 | 438.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0505ms | 0.0784ms | 25.61% | 9803 | 0 | 56.77% | 1.19 | 302.14 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0574ms | 0.1227ms | 26.55% | 9687 | 0 | 56.77% | 1.19 | 265.74 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0760ms | 0.0780ms | 16.47% | 9803 | 0 | 56.77% | 1.19 | 200.89 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0550ms | 0.0808ms | 15.53% | 9803 | 0 | 56.77% | 1.19 | 277.36 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0804ms | 0.0963ms | 18.11% | 9803 | 0 | 56.77% | 1.19 | 189.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3801ms | 0.8156ms | 21.10% | 9803 | 2000 | 56.77% | 1.19 | 40.14 MB/s |
| Quicksort | 10000 | 0.5881ms | 0.9375ms | 15.92% | 135107 | 0 | 56.77% | 1.19 | 259.46 MB/s |
| Timsort | 10000 | 0.4330ms | 1.3556ms | 23.16% | 132213 | 0 | 56.77% | 1.19 | 352.44 MB/s |
| ARS Gen 1: Foundation | 10000 | 3.3554ms | 11.9931ms | 28.31% | 0 | 30000 | 56.77% | 1.19 | 45.48 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.1779ms | 13.6731ms | 29.32% | 0 | 30000 | 56.76% | 1.19 | 36.52 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.5674ms | 2.1681ms | 31.65% | 186385 | 14351 | 56.77% | 1.19 | 268.92 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.9153ms | 2.6874ms | 35.08% | 128713 | 10000 | 56.77% | 1.19 | 166.71 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.6443ms | 2.1923ms | 34.02% | 128713 | 0 | 56.77% | 1.19 | 236.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.6288ms | 2.7356ms | 35.16% | 123613 | 0 | 56.77% | 1.19 | 242.67 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.4039ms | 1.7600ms | 11.75% | 110871 | 0 | 56.77% | 1.19 | 108.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.6494ms | 1.9959ms | 7.85% | 102906 | 0 | 56.77% | 1.19 | 92.51 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.4834ms | 1.6674ms | 14.39% | 52132 | 0 | 56.77% | 1.19 | 102.86 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.6586ms | 2.3909ms | 27.55% | 128713 | 0 | 56.77% | 1.19 | 231.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 2.1182ms | 2.4297ms | 37.11% | 128713 | 0 | 56.77% | 1.19 | 72.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.9623ms | 3.1631ms | 7.21% | 128713 | 20000 | 56.77% | 1.19 | 51.51 MB/s |
| Quicksort | 100000 | 7.4068ms | 10.9574ms | 10.83% | 1704789 | 0 | 56.77% | 1.19 | 206.01 MB/s |
| Timsort | 100000 | 15.3992ms | 15.7295ms | 1.65% | 1670195 | 0 | 56.76% | 1.19 | 99.09 MB/s |
| ARS Gen 1: Foundation | 100000 | 24.8743ms | 50.8801ms | 22.37% | 0 | 300000 | 56.63% | 1.19 | 61.34 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 42.3967ms | 52.5396ms | 8.41% | 0 | 300000 | 56.61% | 1.19 | 35.99 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 13.6956ms | 14.1091ms | 2.27% | 1828718 | 108703 | 56.77% | 1.19 | 111.41 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 5.1886ms | 17.9113ms | 29.99% | 1666654 | 100000 | 56.76% | 1.19 | 294.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.8070ms | 13.3348ms | 21.51% | 1666654 | 0 | 56.76% | 1.19 | 317.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 9.2540ms | 17.2169ms | 16.36% | 1599062 | 0 | 56.76% | 1.19 | 164.89 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.5538ms | 8.1996ms | 32.42% | 1472980 | 0 | 56.76% | 1.19 | 597.49 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 8.6915ms | 10.4487ms | 11.97% | 1374476 | 0 | 56.76% | 1.19 | 175.56 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 6.0588ms | 7.1334ms | 10.80% | 817349 | 0 | 56.76% | 1.19 | 251.84 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 7.9358ms | 9.1190ms | 8.29% | 629587 | 0 | 56.76% | 1.19 | 192.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.4098ms | 7.2883ms | 28.64% | 817349 | 0 | 56.76% | 1.19 | 633.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 15.4988ms | 16.4009ms | 8.22% | 1666654 | 200000 | 56.76% | 1.19 | 98.45 MB/s |
| Quicksort | 1000000 | 51.0725ms | 54.3106ms | 3.26% | 20743535 | 0 | 56.75% | 1.20 | 298.77 MB/s |
| Timsort | 1000000 | 54.4750ms | 66.5302ms | 7.20% | 19762519 | 0 | 56.71% | 1.20 | 280.11 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 38.3120ms | 56.2172ms | 13.01% | 20996656 | 1017407 | 56.76% | 1.19 | 398.28 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 40.9033ms | 47.3032ms | 9.76% | 18441046 | 1000000 | 56.76% | 1.20 | 373.05 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 38.5956ms | 42.9814ms | 16.62% | 18441046 | 0 | 56.76% | 1.20 | 395.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 43.8953ms | 50.5062ms | 13.89% | 17472779 | 0 | 56.72% | 1.20 | 347.62 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 40.3686ms | 47.7888ms | 7.53% | 18465647 | 0 | 56.76% | 1.20 | 377.99 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 32.1723ms | 46.9017ms | 12.08% | 17496592 | 0 | 56.73% | 1.20 | 474.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 31.1535ms | 33.9401ms | 8.70% | 8907286 | 0 | 56.77% | 1.19 | 489.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 35.1048ms | 43.2016ms | 17.11% | 9609774 | 0 | 56.77% | 1.19 | 434.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 33.6198ms | 35.8165ms | 23.47% | 11846806 | 0 | 56.78% | 1.19 | 453.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.7260ms | 41.7974ms | 11.03% | 16754686 | 2000000 | 56.76% | 1.19 | 496.61 MB/s |
| Quicksort | 10000000 | 343.0692ms | 349.4863ms | 0.89% | 244096088 | 0 | 56.57% | 1.21 | 444.77 MB/s |
| Timsort | 10000000 | 580.3501ms | 589.9445ms | 0.71% | 230737667 | 0 | 56.30% | 1.21 | 262.92 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 329.6278ms | 342.2547ms | 1.73% | 244212329 | 10017407 | 56.66% | 1.20 | 462.91 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 352.7538ms | 356.7487ms | 3.45% | 221657406 | 10000000 | 56.81% | 1.20 | 432.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 293.4880ms | 306.5495ms | 2.92% | 221657406 | 0 | 56.79% | 1.20 | 519.91 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 409.7617ms | 421.5832ms | 2.30% | 208473942 | 0 | 56.61% | 1.20 | 372.38 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 273.6750ms | 284.4757ms | 2.51% | 221324522 | 0 | 56.71% | 1.21 | 557.55 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 397.2245ms | 403.8958ms | 2.97% | 210348987 | 0 | 56.52% | 1.20 | 384.14 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 197.4292ms | 201.6272ms | 2.98% | 95609819 | 0 | 57.06% | 1.19 | 772.87 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 221.6712ms | 230.9810ms | 2.45% | 53909853 | 0 | 56.92% | 1.19 | 688.35 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 209.0349ms | 214.4894ms | 1.39% | 88073203 | 0 | 56.99% | 1.19 | 729.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 381.2929ms | 387.2960ms | 3.47% | 215171310 | 20000000 | 56.85% | 1.19 | 400.19 MB/s |

### Distribution: Duplicates

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0068ms | 0.0234ms | 53.74% | 3696 | 0 | 56.43% | 1.20 | 2241.63 MB/s |
| Timsort | 1000 | 0.0475ms | 0.0502ms | 26.44% | 3708 | 0 | 56.43% | 1.20 | 320.92 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.0445ms | 0.1796ms | 26.44% | 995 | 2000 | 56.43% | 1.20 | 342.74 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.1201ms | 0.2197ms | 28.50% | 995 | 2000 | 56.43% | 1.20 | 127.01 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0090ms | 0.0252ms | 27.48% | 3696 | 0 | 56.43% | 1.20 | 1690.72 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0112ms | 0.0259ms | 37.83% | 3696 | 0 | 56.43% | 1.20 | 1366.17 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0077ms | 0.0251ms | 27.44% | 3696 | 0 | 56.43% | 1.20 | 1980.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0185ms | 0.0560ms | 66.75% | 3708 | 0 | 56.43% | 1.20 | 825.33 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0241ms | 0.0248ms | 37.69% | 3696 | 0 | 56.43% | 1.20 | 632.41 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0404ms | 0.0519ms | 35.23% | 3708 | 0 | 56.43% | 1.20 | 377.63 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0229ms | 0.0248ms | 30.79% | 3696 | 0 | 56.43% | 1.20 | 667.43 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0136ms | 0.0253ms | 18.03% | 3696 | 0 | 56.43% | 1.20 | 1123.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0137ms | 0.0268ms | 39.08% | 3696 | 0 | 56.43% | 1.20 | 1111.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3232ms | 0.6928ms | 20.07% | 3696 | 2000 | 56.43% | 1.20 | 47.21 MB/s |
| Quicksort | 10000 | 0.1333ms | 0.1984ms | 18.68% | 36514 | 0 | 56.43% | 1.20 | 1144.84 MB/s |
| Timsort | 10000 | 0.1247ms | 0.3831ms | 22.19% | 36706 | 0 | 56.43% | 1.20 | 1223.46 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.3944ms | 1.5453ms | 8.33% | 9995 | 30000 | 56.43% | 1.20 | 109.43 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 0.4378ms | 1.7318ms | 37.79% | 9995 | 30000 | 56.43% | 1.20 | 348.53 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.5354ms | 1.8868ms | 24.59% | 115253 | 14351 | 56.43% | 1.20 | 285.01 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.4337ms | 1.4743ms | 33.68% | 9999 | 10000 | 56.43% | 1.20 | 351.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.8969ms | 1.2878ms | 24.72% | 9999 | 0 | 56.43% | 1.20 | 170.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.2597ms | 1.0281ms | 40.03% | 9999 | 0 | 56.43% | 1.20 | 587.44 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.3920ms | 1.5296ms | 7.72% | 9999 | 0 | 56.43% | 1.20 | 109.62 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.2879ms | 1.4150ms | 15.23% | 9999 | 0 | 56.43% | 1.20 | 118.48 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.4467ms | 1.1547ms | 42.26% | 9999 | 0 | 56.43% | 1.20 | 341.57 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.5710ms | 1.1772ms | 40.92% | 9999 | 0 | 56.43% | 1.20 | 267.21 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.7859ms | 1.3883ms | 23.33% | 9999 | 0 | 56.43% | 1.20 | 194.16 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.4654ms | 2.1486ms | 35.71% | 9999 | 20000 | 56.43% | 1.20 | 104.12 MB/s |
| Quicksort | 100000 | 0.6310ms | 1.9349ms | 26.41% | 362149 | 0 | 56.43% | 1.20 | 2418.06 MB/s |
| Timsort | 100000 | 3.6558ms | 3.9416ms | 7.18% | 362807 | 0 | 56.42% | 1.20 | 417.39 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.1691ms | 8.2194ms | 9.20% | 99995 | 300000 | 56.43% | 1.20 | 212.84 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 7.8778ms | 9.2933ms | 7.34% | 99995 | 300000 | 56.43% | 1.20 | 193.69 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.2805ms | 13.0119ms | 22.00% | 1130042 | 108703 | 56.43% | 1.20 | 356.47 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.1393ms | 4.2047ms | 25.31% | 99999 | 100000 | 56.42% | 1.20 | 1339.27 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.8076ms | 4.4817ms | 19.80% | 99999 | 0 | 56.43% | 1.20 | 543.49 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.3035ms | 3.2599ms | 35.94% | 99999 | 0 | 56.42% | 1.20 | 1170.59 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.5218ms | 3.1664ms | 24.75% | 99999 | 0 | 56.42% | 1.20 | 1002.65 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 0.9659ms | 4.1092ms | 42.76% | 99999 | 0 | 56.42% | 1.20 | 1579.72 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.8892ms | 4.2153ms | 18.84% | 199994 | 0 | 56.42% | 1.20 | 528.13 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.8981ms | 5.2758ms | 25.05% | 199994 | 0 | 56.42% | 1.20 | 803.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.1301ms | 3.8953ms | 35.67% | 199994 | 0 | 56.42% | 1.20 | 1350.21 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.5855ms | 5.6963ms | 25.05% | 99999 | 200000 | 56.42% | 1.20 | 962.42 MB/s |
| Quicksort | 1000000 | 6.0225ms | 7.6129ms | 11.30% | 3806445 | 0 | 56.41% | 1.20 | 2533.62 MB/s |
| Timsort | 1000000 | 12.7248ms | 14.7641ms | 6.54% | 4510537 | 0 | 56.40% | 1.20 | 1199.14 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.5485ms | 34.0290ms | 2.16% | 12059615 | 1017407 | 56.42% | 1.20 | 468.80 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 11.4591ms | 13.4705ms | 10.45% | 1000001 | 1000000 | 56.44% | 1.20 | 1331.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.3492ms | 12.6849ms | 7.34% | 1000001 | 0 | 56.43% | 1.20 | 1344.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 11.0620ms | 12.8469ms | 9.24% | 1000001 | 0 | 56.43% | 1.20 | 1379.38 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.7952ms | 9.3340ms | 10.96% | 1000001 | 0 | 56.43% | 1.20 | 1957.46 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 8.2254ms | 10.3604ms | 11.59% | 1000001 | 0 | 56.43% | 1.20 | 1855.08 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 13.8375ms | 15.1220ms | 6.80% | 1999996 | 0 | 56.44% | 1.20 | 1102.71 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.8682ms | 13.7702ms | 7.22% | 1999996 | 0 | 56.43% | 1.20 | 1285.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.2606ms | 15.6736ms | 12.45% | 1999996 | 0 | 56.43% | 1.20 | 1150.69 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 25.5630ms | 25.8534ms | 5.44% | 5365058 | 2000000 | 56.41% | 1.20 | 596.91 MB/s |
| Quicksort | 10000000 | 59.0386ms | 59.6289ms | 3.03% | 36019091 | 0 | 56.43% | 1.20 | 2584.54 MB/s |
| Timsort | 10000000 | 217.7674ms | 224.2849ms | 1.45% | 50543152 | 0 | 56.53% | 1.20 | 700.69 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 230.1191ms | 240.9689ms | 3.49% | 120059628 | 10017407 | 56.46% | 1.20 | 663.08 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 94.4175ms | 96.6664ms | 3.00% | 10000003 | 10000000 | 56.58% | 1.19 | 1616.10 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 95.0101ms | 98.7980ms | 3.57% | 10000003 | 0 | 56.58% | 1.19 | 1606.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 95.0903ms | 97.9646ms | 2.20% | 10000003 | 0 | 56.58% | 1.19 | 1604.66 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 60.2632ms | 63.0828ms | 2.22% | 10000003 | 0 | 56.50% | 1.20 | 2532.02 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 60.0329ms | 62.1267ms | 3.41% | 10000003 | 0 | 56.50% | 1.20 | 2541.74 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 121.9813ms | 125.8678ms | 2.79% | 19999998 | 0 | 56.64% | 1.19 | 1250.91 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 105.0930ms | 107.6049ms | 2.09% | 19999998 | 0 | 56.59% | 1.19 | 1451.93 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 120.8464ms | 125.7276ms | 2.49% | 19999998 | 0 | 56.64% | 1.19 | 1262.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 362.9194ms | 368.8129ms | 2.28% | 109767496 | 20000000 | 56.46% | 1.20 | 420.45 MB/s |

### Distribution: Zipfian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0380ms | 0.0405ms | 17.00% | 5850 | 0 | 56.41% | 1.22 | 401.56 MB/s |
| Timsort | 1000 | 0.0188ms | 0.0820ms | 44.19% | 5919 | 0 | 56.41% | 1.22 | 812.59 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1147ms | 0.2685ms | 25.81% | 905 | 2000 | 56.41% | 1.22 | 133.07 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2932ms | 0.3294ms | 21.40% | 905 | 2000 | 56.41% | 1.22 | 52.04 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0398ms | 0.0456ms | 26.69% | 5850 | 0 | 56.41% | 1.22 | 382.98 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0395ms | 0.0413ms | 30.72% | 5850 | 0 | 56.41% | 1.22 | 386.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0157ms | 0.0414ms | 28.35% | 5850 | 0 | 56.41% | 1.22 | 971.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0203ms | 0.0744ms | 31.80% | 5919 | 0 | 56.41% | 1.22 | 751.59 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0127ms | 0.0408ms | 27.39% | 5850 | 0 | 56.41% | 1.22 | 1203.18 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0336ms | 0.0734ms | 22.93% | 5919 | 0 | 56.41% | 1.22 | 454.78 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0134ms | 0.0397ms | 47.86% | 5850 | 0 | 56.41% | 1.22 | 1137.53 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0415ms | 0.0436ms | 21.09% | 5850 | 0 | 56.41% | 1.22 | 367.62 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0136ms | 0.0427ms | 24.36% | 5850 | 0 | 56.41% | 1.22 | 1126.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3608ms | 0.7707ms | 18.67% | 5850 | 2000 | 56.41% | 1.22 | 42.29 MB/s |
| Quicksort | 10000 | 0.2881ms | 0.3086ms | 12.55% | 61801 | 0 | 56.41% | 1.22 | 529.71 MB/s |
| Timsort | 10000 | 0.2406ms | 0.5386ms | 30.55% | 58783 | 0 | 56.41% | 1.22 | 634.32 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.8486ms | 2.0442ms | 9.25% | 9660 | 30000 | 56.41% | 1.22 | 82.54 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 0.5977ms | 2.1076ms | 26.23% | 9660 | 30000 | 56.41% | 1.22 | 255.27 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.7814ms | 2.1128ms | 23.26% | 126177 | 14351 | 56.41% | 1.22 | 195.26 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.1720ms | 1.6168ms | 16.18% | 10892 | 10000 | 56.41% | 1.22 | 130.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.9898ms | 1.1262ms | 18.22% | 10892 | 0 | 56.41% | 1.22 | 154.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.3111ms | 1.1512ms | 36.88% | 10906 | 0 | 56.41% | 1.22 | 490.50 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.4943ms | 1.8851ms | 32.88% | 14583 | 0 | 56.41% | 1.22 | 308.68 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.2390ms | 1.8927ms | 13.84% | 14792 | 0 | 56.41% | 1.22 | 123.16 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.0382ms | 1.3050ms | 27.34% | 10892 | 0 | 56.41% | 1.22 | 146.98 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.5510ms | 1.1318ms | 50.83% | 10892 | 0 | 56.41% | 1.22 | 276.95 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.2548ms | 1.0174ms | 32.35% | 10892 | 0 | 56.41% | 1.22 | 598.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.6300ms | 2.2673ms | 28.47% | 10892 | 20000 | 56.41% | 1.22 | 242.20 MB/s |
| Quicksort | 100000 | 0.7867ms | 2.7639ms | 24.25% | 534636 | 0 | 56.40% | 1.22 | 1939.68 MB/s |
| Timsort | 100000 | 4.9248ms | 5.2024ms | 3.65% | 537945 | 0 | 56.40% | 1.22 | 309.84 MB/s |
| ARS Gen 1: Foundation | 100000 | 3.1067ms | 11.3551ms | 24.95% | 98709 | 300000 | 56.40% | 1.22 | 491.16 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 8.7179ms | 12.2170ms | 12.49% | 98709 | 300000 | 56.40% | 1.22 | 175.03 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.2662ms | 9.4701ms | 36.03% | 1173602 | 108703 | 56.40% | 1.22 | 357.66 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.7300ms | 4.3831ms | 11.26% | 122042 | 100000 | 56.40% | 1.22 | 409.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.8708ms | 2.9654ms | 38.43% | 122042 | 0 | 56.40% | 1.22 | 815.62 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.6925ms | 3.9004ms | 24.88% | 121972 | 0 | 56.40% | 1.22 | 566.71 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.4461ms | 4.2369ms | 24.64% | 155142 | 0 | 56.40% | 1.22 | 1055.18 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.6458ms | 4.4359ms | 19.25% | 155964 | 0 | 56.40% | 1.22 | 418.53 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.1310ms | 4.3563ms | 30.26% | 191811 | 0 | 56.40% | 1.22 | 1349.09 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 1.6945ms | 5.1485ms | 30.06% | 182245 | 0 | 56.40% | 1.22 | 900.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.1068ms | 3.9740ms | 34.59% | 186400 | 0 | 56.40% | 1.22 | 724.27 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 5.1758ms | 6.5607ms | 30.52% | 122042 | 200000 | 56.40% | 1.22 | 294.81 MB/s |
| Quicksort | 1000000 | 5.3837ms | 6.2444ms | 10.16% | 5272329 | 0 | 56.39% | 1.22 | 2834.28 MB/s |
| Timsort | 1000000 | 12.8092ms | 14.1229ms | 9.65% | 6330674 | 0 | 56.37% | 1.22 | 1191.24 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 26.0912ms | 26.9517ms | 1.54% | 12320111 | 1017407 | 56.40% | 1.22 | 584.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.6875ms | 12.3316ms | 6.77% | 1097231 | 1000000 | 56.41% | 1.21 | 1427.73 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 10.3404ms | 11.8478ms | 10.18% | 1097231 | 0 | 56.41% | 1.21 | 1475.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.8086ms | 11.8479ms | 9.37% | 1099032 | 0 | 56.41% | 1.21 | 1411.72 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.3761ms | 8.3668ms | 7.17% | 1528416 | 0 | 56.41% | 1.22 | 2068.67 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.8547ms | 9.0955ms | 8.31% | 1533326 | 0 | 56.41% | 1.22 | 1942.63 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 12.6971ms | 14.8179ms | 5.94% | 1991394 | 0 | 56.41% | 1.21 | 1201.75 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.0293ms | 12.4581ms | 9.36% | 2058356 | 0 | 56.41% | 1.21 | 1383.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 13.1924ms | 14.0920ms | 4.67% | 2122811 | 0 | 56.41% | 1.21 | 1156.64 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 20.0202ms | 22.4280ms | 7.06% | 5811564 | 2000000 | 56.37% | 1.22 | 762.17 MB/s |
| Quicksort | 10000000 | 80.0891ms | 81.7289ms | 2.35% | 52910303 | 0 | 56.39% | 1.22 | 1905.23 MB/s |
| Timsort | 10000000 | 246.2049ms | 252.6412ms | 2.82% | 65720450 | 0 | 56.36% | 1.22 | 619.76 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 242.2194ms | 249.6910ms | 1.49% | 122355136 | 10017407 | 56.45% | 1.22 | 629.96 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 100.1635ms | 103.8181ms | 3.53% | 11104102 | 10000000 | 56.56% | 1.21 | 1523.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 99.6321ms | 102.4605ms | 2.83% | 11104102 | 0 | 56.56% | 1.21 | 1531.51 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 99.9259ms | 102.5663ms | 3.45% | 11122191 | 0 | 56.56% | 1.21 | 1527.01 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 66.6340ms | 67.1979ms | 2.76% | 15044247 | 0 | 56.49% | 1.21 | 2289.94 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 69.5487ms | 73.1580ms | 3.32% | 15062892 | 0 | 56.47% | 1.21 | 2193.97 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 124.9741ms | 131.7483ms | 3.11% | 20253803 | 0 | 56.61% | 1.21 | 1220.96 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 112.0692ms | 114.4756ms | 2.16% | 20501519 | 0 | 56.57% | 1.21 | 1361.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 124.6300ms | 131.7308ms | 2.59% | 20518118 | 0 | 56.61% | 1.21 | 1224.33 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 361.7854ms | 368.6484ms | 1.41% | 119927431 | 20000000 | 56.43% | 1.22 | 421.76 MB/s |

### Distribution: Skewed

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0766ms | 0.0787ms | 7.51% | 10053 | 0 | 56.36% | 1.23 | 199.08 MB/s |
| Timsort | 1000 | 0.0410ms | 0.1339ms | 30.84% | 10849 | 0 | 56.36% | 1.23 | 371.78 MB/s |
| ARS Gen 1: Foundation | 1000 | 1.0990ms | 1.1794ms | 12.25% | 0 | 2000 | 56.36% | 1.23 | 13.88 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.4630ms | 1.4121ms | 23.40% | 0 | 2000 | 56.36% | 1.23 | 32.96 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0819ms | 0.0833ms | 11.62% | 10053 | 0 | 56.36% | 1.23 | 186.29 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0244ms | 0.0850ms | 27.10% | 10053 | 0 | 56.36% | 1.23 | 624.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0250ms | 0.0839ms | 22.74% | 10053 | 0 | 56.36% | 1.23 | 610.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0786ms | 0.1338ms | 19.85% | 10849 | 0 | 56.36% | 1.23 | 194.05 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0526ms | 0.0790ms | 20.27% | 10053 | 0 | 56.36% | 1.23 | 289.85 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0439ms | 0.1619ms | 27.05% | 10849 | 0 | 56.36% | 1.23 | 347.42 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0602ms | 0.0942ms | 23.12% | 10053 | 0 | 56.36% | 1.23 | 253.67 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0816ms | 0.0949ms | 21.71% | 10053 | 0 | 56.36% | 1.23 | 187.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0301ms | 0.0841ms | 27.89% | 10053 | 0 | 56.36% | 1.23 | 506.48 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3176ms | 0.8494ms | 24.14% | 10053 | 2000 | 56.36% | 1.23 | 48.05 MB/s |
| Quicksort | 10000 | 0.3374ms | 0.9199ms | 20.56% | 137368 | 0 | 56.36% | 1.23 | 452.24 MB/s |
| Timsort | 10000 | 1.4485ms | 1.5475ms | 12.15% | 142702 | 0 | 56.36% | 1.23 | 105.34 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.2345ms | 26.6715ms | 22.75% | 0 | 30000 | 56.35% | 1.23 | 18.53 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 10.5839ms | 28.5242ms | 22.40% | 0 | 30000 | 56.35% | 1.23 | 14.42 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.7360ms | 2.4106ms | 27.29% | 192264 | 14351 | 56.36% | 1.23 | 207.31 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.5569ms | 1.8249ms | 27.40% | 65398 | 10000 | 56.36% | 1.23 | 273.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3417ms | 1.1741ms | 29.39% | 65398 | 0 | 56.36% | 1.23 | 446.59 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.4751ms | 1.3395ms | 31.83% | 68511 | 0 | 56.36% | 1.23 | 321.16 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6201ms | 1.5092ms | 22.11% | 59344 | 0 | 56.36% | 1.23 | 246.06 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.4648ms | 1.6187ms | 29.64% | 63109 | 0 | 56.36% | 1.23 | 328.27 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.2973ms | 1.2651ms | 29.43% | 65398 | 0 | 56.36% | 1.23 | 513.25 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9995ms | 1.3958ms | 32.06% | 65398 | 0 | 56.36% | 1.23 | 152.67 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.0010ms | 1.2750ms | 36.73% | 65398 | 0 | 56.36% | 1.23 | 152.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.5401ms | 2.2698ms | 35.06% | 65398 | 20000 | 56.36% | 1.23 | 282.53 MB/s |
| Quicksort | 100000 | 8.7097ms | 11.0023ms | 8.27% | 1709967 | 0 | 56.36% | 1.23 | 175.19 MB/s |
| Timsort | 100000 | 16.8462ms | 17.1718ms | 1.27% | 1750501 | 0 | 56.35% | 1.23 | 90.58 MB/s |
| ARS Gen 1: Foundation | 100000 | 67.8107ms | 71.1465ms | 4.38% | 0 | 300000 | 55.89% | 1.23 | 22.50 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.3779ms | 68.6548ms | 16.62% | 0 | 300000 | 55.87% | 1.23 | 33.63 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 12.2520ms | 13.3284ms | 8.67% | 1886197 | 108703 | 56.36% | 1.23 | 124.54 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.5042ms | 5.5197ms | 34.00% | 1067004 | 100000 | 56.35% | 1.23 | 1014.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.4966ms | 5.4653ms | 24.30% | 1067004 | 0 | 56.35% | 1.23 | 611.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 5.3203ms | 6.0302ms | 10.13% | 1110977 | 0 | 56.35% | 1.23 | 286.80 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.6259ms | 5.5753ms | 21.62% | 969682 | 0 | 56.35% | 1.23 | 420.82 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 5.1773ms | 6.7294ms | 12.81% | 1008646 | 0 | 56.35% | 1.23 | 294.73 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.4188ms | 5.4786ms | 18.72% | 1067004 | 0 | 56.35% | 1.23 | 345.31 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.7433ms | 6.6291ms | 17.10% | 964586 | 0 | 56.35% | 1.23 | 265.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.4315ms | 5.5327ms | 11.62% | 1067004 | 0 | 56.35% | 1.23 | 344.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 6.9346ms | 8.5975ms | 15.18% | 1067004 | 200000 | 56.35% | 1.23 | 220.04 MB/s |
| Quicksort | 1000000 | 30.0712ms | 35.8820ms | 5.93% | 20447241 | 0 | 56.34% | 1.23 | 507.42 MB/s |
| Timsort | 1000000 | 51.2387ms | 54.0071ms | 3.08% | 20847648 | 0 | 56.30% | 1.23 | 297.80 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 37.1799ms | 41.9953ms | 5.15% | 21507713 | 1017407 | 56.33% | 1.23 | 410.40 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 18.7087ms | 19.9145ms | 3.75% | 12175428 | 1000000 | 56.36% | 1.23 | 815.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 15.5986ms | 17.9735ms | 5.04% | 12175428 | 0 | 56.36% | 1.23 | 978.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 18.5134ms | 19.7939ms | 5.14% | 12600161 | 0 | 56.35% | 1.23 | 824.20 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.6460ms | 17.0325ms | 6.03% | 13233794 | 0 | 56.35% | 1.23 | 975.25 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.3475ms | 19.5224ms | 7.11% | 13668875 | 0 | 56.35% | 1.23 | 831.66 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 16.8247ms | 19.4016ms | 6.18% | 7210611 | 0 | 56.34% | 1.23 | 906.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.4983ms | 21.7752ms | 7.97% | 6297511 | 0 | 56.34% | 1.23 | 782.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 17.9797ms | 21.0281ms | 6.03% | 7116313 | 0 | 56.34% | 1.23 | 848.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 29.3542ms | 31.0440ms | 9.12% | 14003368 | 2000000 | 56.36% | 1.23 | 519.82 MB/s |
| Quicksort | 10000000 | 325.7747ms | 334.9676ms | 2.50% | 237595346 | 0 | 56.19% | 1.24 | 468.38 MB/s |
| Timsort | 10000000 | 601.8663ms | 609.6966ms | 0.97% | 241521896 | 0 | 55.98% | 1.24 | 253.52 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 328.5192ms | 350.6575ms | 2.88% | 247426529 | 10017407 | 56.15% | 1.23 | 464.47 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 172.6697ms | 176.6143ms | 1.62% | 157303578 | 10000000 | 56.43% | 1.22 | 883.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 149.8884ms | 152.4226ms | 1.54% | 157303578 | 0 | 56.44% | 1.23 | 1018.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 180.2066ms | 185.4609ms | 1.77% | 161361446 | 0 | 56.26% | 1.22 | 846.74 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 131.4760ms | 133.3219ms | 2.90% | 167739468 | 0 | 56.36% | 1.23 | 1160.58 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 170.4827ms | 175.2346ms | 3.98% | 171743389 | 0 | 56.16% | 1.23 | 895.03 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 164.4066ms | 168.0317ms | 1.71% | 68133992 | 0 | 56.34% | 1.22 | 928.11 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 192.0996ms | 196.0825ms | 2.01% | 75446404 | 0 | 56.53% | 1.22 | 794.32 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 172.4445ms | 177.4255ms | 1.98% | 78835451 | 0 | 56.45% | 1.22 | 884.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 371.2247ms | 378.7320ms | 0.86% | 176121228 | 20000000 | 56.49% | 1.22 | 411.04 MB/s |

### Distribution: Clustered

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0695ms | 0.0793ms | 14.08% | 10085 | 0 | 56.39% | 1.22 | 219.60 MB/s |
| Timsort | 1000 | 0.0800ms | 0.1568ms | 17.62% | 10643 | 0 | 56.39% | 1.22 | 190.73 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.7958ms | 1.1534ms | 16.16% | 0 | 2000 | 56.39% | 1.22 | 19.17 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.4893ms | 1.4774ms | 23.69% | 0 | 2000 | 56.39% | 1.22 | 31.18 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0239ms | 0.0836ms | 29.24% | 10085 | 0 | 56.39% | 1.22 | 638.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0797ms | 0.1006ms | 15.25% | 10085 | 0 | 56.39% | 1.22 | 191.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0291ms | 0.0837ms | 46.52% | 10085 | 0 | 56.39% | 1.22 | 523.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0369ms | 0.1402ms | 31.03% | 10643 | 0 | 56.39% | 1.22 | 413.16 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0367ms | 0.0795ms | 27.34% | 10085 | 0 | 56.39% | 1.22 | 415.47 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0828ms | 0.1309ms | 19.52% | 10643 | 0 | 56.39% | 1.22 | 184.20 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0232ms | 0.0886ms | 30.27% | 10085 | 0 | 56.39% | 1.22 | 656.43 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0287ms | 0.0838ms | 31.72% | 10085 | 0 | 56.39% | 1.22 | 531.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0512ms | 0.0841ms | 15.17% | 10085 | 0 | 56.39% | 1.22 | 298.02 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.6540ms | 0.8440ms | 57.76% | 10085 | 2000 | 56.39% | 1.22 | 23.33 MB/s |
| Quicksort | 10000 | 0.6750ms | 0.9149ms | 13.94% | 136330 | 0 | 56.39% | 1.22 | 226.06 MB/s |
| Timsort | 10000 | 0.4456ms | 1.5063ms | 31.62% | 141437 | 0 | 56.39% | 1.22 | 342.43 MB/s |
| ARS Gen 1: Foundation | 10000 | 7.9699ms | 26.9686ms | 30.66% | 0 | 30000 | 56.37% | 1.22 | 19.15 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 15.6717ms | 28.1089ms | 13.96% | 0 | 30000 | 56.38% | 1.22 | 9.74 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.4093ms | 2.5692ms | 20.66% | 192985 | 14351 | 56.39% | 1.22 | 108.27 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.7351ms | 2.4018ms | 29.65% | 127386 | 10000 | 56.39% | 1.22 | 207.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.7522ms | 1.9016ms | 20.25% | 127386 | 0 | 56.39% | 1.22 | 202.86 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.9511ms | 2.3731ms | 16.37% | 131312 | 0 | 56.39% | 1.22 | 78.21 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.9710ms | 2.3943ms | 11.74% | 114293 | 0 | 56.39% | 1.22 | 77.42 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.7591ms | 2.8478ms | 26.32% | 119337 | 0 | 56.39% | 1.22 | 201.02 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.4533ms | 1.8082ms | 36.21% | 103740 | 0 | 56.39% | 1.22 | 336.62 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.4844ms | 1.9486ms | 21.18% | 127386 | 0 | 56.39% | 1.22 | 102.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.7176ms | 1.9787ms | 32.18% | 127386 | 0 | 56.39% | 1.22 | 88.84 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.3880ms | 2.8901ms | 44.81% | 127386 | 20000 | 56.39% | 1.22 | 109.93 MB/s |
| Quicksort | 100000 | 2.6830ms | 10.9820ms | 24.67% | 1704812 | 0 | 56.38% | 1.22 | 568.73 MB/s |
| Timsort | 100000 | 5.3103ms | 17.0360ms | 23.34% | 1751115 | 0 | 56.38% | 1.22 | 287.34 MB/s |
| ARS Gen 1: Foundation | 100000 | 63.6017ms | 67.1502ms | 3.27% | 0 | 300000 | 55.93% | 1.22 | 23.99 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 49.1987ms | 68.3865ms | 9.56% | 0 | 300000 | 56.00% | 1.22 | 31.01 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.4230ms | 10.6313ms | 44.09% | 1885890 | 108703 | 56.38% | 1.22 | 344.99 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.1584ms | 9.2007ms | 45.45% | 1607356 | 100000 | 56.38% | 1.22 | 366.94 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.2779ms | 9.2490ms | 46.71% | 1607356 | 0 | 56.38% | 1.22 | 669.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.0082ms | 11.8565ms | 37.61% | 1648722 | 0 | 56.38% | 1.22 | 507.24 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.7286ms | 6.8365ms | 54.01% | 1567959 | 0 | 56.38% | 1.22 | 559.21 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.1207ms | 8.3424ms | 55.54% | 1611246 | 0 | 56.38% | 1.22 | 488.95 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.2239ms | 8.5025ms | 25.99% | 745151 | 0 | 56.38% | 1.22 | 686.13 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.8343ms | 10.0798ms | 20.09% | 993675 | 0 | 56.38% | 1.22 | 261.54 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.1529ms | 9.1031ms | 20.83% | 993048 | 0 | 56.38% | 1.22 | 367.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.7263ms | 8.8602ms | 56.44% | 1607356 | 200000 | 56.38% | 1.22 | 409.48 MB/s |
| Quicksort | 1000000 | 27.3053ms | 28.6140ms | 3.27% | 20426723 | 0 | 56.37% | 1.22 | 558.82 MB/s |
| Timsort | 1000000 | 43.7741ms | 46.2059ms | 3.57% | 20820518 | 0 | 56.33% | 1.22 | 348.58 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 31.8404ms | 32.8316ms | 1.83% | 21500495 | 1017407 | 56.36% | 1.22 | 479.23 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 29.3754ms | 34.4263ms | 15.13% | 18907750 | 1000000 | 56.38% | 1.22 | 519.44 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.5964ms | 29.4223ms | 12.30% | 18907750 | 0 | 56.38% | 1.22 | 675.27 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 34.4756ms | 37.5749ms | 9.17% | 19283314 | 0 | 56.36% | 1.22 | 442.60 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.3189ms | 25.8716ms | 16.22% | 19459121 | 0 | 56.38% | 1.22 | 832.95 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 28.8246ms | 34.9091ms | 9.87% | 19837631 | 0 | 56.36% | 1.22 | 529.37 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 22.9529ms | 25.3667ms | 4.96% | 6385600 | 0 | 56.37% | 1.22 | 664.79 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 23.2109ms | 26.7398ms | 8.69% | 6103657 | 0 | 56.39% | 1.22 | 657.40 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.1829ms | 26.2848ms | 10.14% | 7939094 | 0 | 56.40% | 1.22 | 605.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 26.8507ms | 34.3254ms | 14.53% | 20689690 | 2000000 | 56.39% | 1.22 | 568.28 MB/s |
| Quicksort | 10000000 | 335.8206ms | 341.9479ms | 1.54% | 237620849 | 0 | 56.27% | 1.23 | 454.37 MB/s |
| Timsort | 10000000 | 601.6465ms | 608.4270ms | 0.65% | 241538603 | 0 | 56.09% | 1.23 | 253.62 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 332.5533ms | 353.8360ms | 2.51% | 247441240 | 10017407 | 56.22% | 1.22 | 458.84 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 344.7926ms | 380.6948ms | 9.29% | 220392561 | 10000000 | 56.44% | 1.22 | 442.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 283.2462ms | 312.1794ms | 10.10% | 220392561 | 0 | 56.41% | 1.22 | 538.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 420.6225ms | 472.2527ms | 10.12% | 224177041 | 0 | 56.24% | 1.22 | 362.77 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 256.1908ms | 287.2147ms | 7.15% | 220392561 | 0 | 56.39% | 1.23 | 595.60 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 380.0423ms | 444.0484ms | 8.21% | 224177041 | 0 | 56.30% | 1.22 | 401.50 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 230.0493ms | 239.0625ms | 2.21% | 82410094 | 0 | 56.47% | 1.21 | 663.28 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 267.2074ms | 278.2813ms | 2.68% | 96497764 | 0 | 56.72% | 1.21 | 571.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 259.4886ms | 275.1894ms | 6.28% | 102496240 | 0 | 56.74% | 1.21 | 588.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 567.4797ms | 660.7611ms | 6.86% | 271656515 | 20000000 | 56.49% | 1.22 | 268.89 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0779ms | 0.0794ms | 11.36% | 10160 | 0 | 56.55% | 1.23 | 196.00 MB/s |
| Timsort | 1000 | 0.0776ms | 0.1340ms | 19.31% | 10742 | 0 | 56.55% | 1.23 | 196.53 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.3194ms | 1.1295ms | 24.77% | 2 | 2000 | 56.55% | 1.23 | 47.78 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.4749ms | 1.4127ms | 24.56% | 2 | 2000 | 56.55% | 1.23 | 32.13 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0285ms | 0.0862ms | 29.59% | 10160 | 0 | 56.55% | 1.23 | 534.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0246ms | 0.0826ms | 22.76% | 10160 | 0 | 56.55% | 1.23 | 620.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0231ms | 0.0831ms | 39.43% | 10160 | 0 | 56.55% | 1.23 | 659.81 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0402ms | 0.1362ms | 28.41% | 10742 | 0 | 56.55% | 1.23 | 379.88 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0793ms | 0.0803ms | 15.90% | 10160 | 0 | 56.55% | 1.23 | 192.51 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0390ms | 0.1369ms | 30.02% | 10742 | 0 | 56.55% | 1.23 | 390.98 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0237ms | 0.0809ms | 39.81% | 10160 | 0 | 56.55% | 1.23 | 643.64 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0822ms | 0.0845ms | 15.20% | 10160 | 0 | 56.55% | 1.23 | 185.62 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0284ms | 0.0830ms | 33.74% | 10160 | 0 | 56.55% | 1.23 | 537.15 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.2370ms | 0.8421ms | 32.96% | 10160 | 2000 | 56.55% | 1.23 | 64.40 MB/s |
| Quicksort | 10000 | 0.2337ms | 0.8877ms | 33.43% | 136996 | 0 | 56.55% | 1.23 | 652.89 MB/s |
| Timsort | 10000 | 1.1443ms | 1.5241ms | 16.71% | 141829 | 0 | 56.55% | 1.23 | 133.34 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.6651ms | 26.3081ms | 21.88% | 44 | 30000 | 56.54% | 1.23 | 17.61 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 26.4158ms | 27.7076ms | 3.13% | 44 | 30000 | 56.54% | 1.23 | 5.78 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.5733ms | 2.2527ms | 14.66% | 193234 | 14351 | 56.55% | 1.23 | 96.99 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.5105ms | 1.7232ms | 11.70% | 52358 | 10000 | 56.55% | 1.23 | 101.02 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3142ms | 1.1915ms | 34.48% | 52358 | 0 | 56.55% | 1.23 | 485.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.3313ms | 1.3538ms | 32.17% | 57703 | 0 | 56.55% | 1.23 | 460.53 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.4262ms | 1.6184ms | 41.29% | 57911 | 0 | 56.55% | 1.23 | 357.99 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.5221ms | 1.7745ms | 29.89% | 60501 | 0 | 56.55% | 1.23 | 292.24 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.0143ms | 1.1893ms | 23.16% | 52358 | 0 | 56.55% | 1.23 | 150.43 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.0526ms | 1.3621ms | 34.69% | 52358 | 0 | 56.55% | 1.23 | 144.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3000ms | 1.2068ms | 38.53% | 52358 | 0 | 56.55% | 1.23 | 508.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.7211ms | 2.4664ms | 39.19% | 52358 | 20000 | 56.55% | 1.23 | 211.62 MB/s |
| Quicksort | 100000 | 10.7743ms | 11.1917ms | 1.86% | 1708661 | 0 | 56.55% | 1.23 | 141.62 MB/s |
| Timsort | 100000 | 7.2156ms | 17.4258ms | 19.16% | 1748978 | 0 | 56.54% | 1.23 | 211.47 MB/s |
| ARS Gen 1: Foundation | 100000 | 63.7070ms | 75.2076ms | 5.94% | 2102 | 300000 | 56.16% | 1.23 | 23.95 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 49.0838ms | 74.7840ms | 10.97% | 2102 | 300000 | 56.21% | 1.23 | 31.09 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 13.6723ms | 14.5825ms | 5.04% | 1886943 | 108703 | 56.55% | 1.23 | 111.60 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.8531ms | 6.0299ms | 21.20% | 881461 | 100000 | 56.54% | 1.23 | 534.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.4354ms | 5.8896ms | 13.87% | 881461 | 0 | 56.54% | 1.23 | 344.02 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 4.5484ms | 5.9460ms | 18.24% | 922226 | 0 | 56.54% | 1.23 | 335.48 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.8121ms | 5.6395ms | 17.84% | 933682 | 0 | 56.54% | 1.23 | 317.09 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 5.1861ms | 6.1540ms | 19.32% | 973015 | 0 | 56.54% | 1.23 | 294.22 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.3319ms | 4.9193ms | 9.38% | 881461 | 0 | 56.54% | 1.23 | 352.24 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.9231ms | 6.7256ms | 10.52% | 769868 | 0 | 56.54% | 1.23 | 257.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.4333ms | 5.4503ms | 8.33% | 881461 | 0 | 56.54% | 1.23 | 344.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.6400ms | 7.0336ms | 23.21% | 881461 | 200000 | 56.54% | 1.23 | 577.99 MB/s |
| Quicksort | 1000000 | 40.9171ms | 47.2722ms | 5.65% | 20371770 | 0 | 56.53% | 1.23 | 372.92 MB/s |
| Timsort | 1000000 | 51.8659ms | 63.5297ms | 6.20% | 20790377 | 0 | 56.50% | 1.23 | 294.20 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 44.4669ms | 52.5662ms | 5.61% | 21441624 | 1017407 | 56.52% | 1.23 | 343.15 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 21.3427ms | 27.1735ms | 12.34% | 10165513 | 1000000 | 56.54% | 1.23 | 714.94 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 20.4878ms | 25.2455ms | 9.34% | 10165513 | 0 | 56.55% | 1.23 | 744.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 20.8630ms | 24.7452ms | 19.96% | 10580504 | 0 | 56.54% | 1.23 | 731.38 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 20.2126ms | 23.8195ms | 30.74% | 12890329 | 0 | 56.55% | 1.23 | 754.91 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 24.4675ms | 27.3719ms | 5.86% | 13317079 | 0 | 56.55% | 1.23 | 623.64 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 16.3999ms | 23.4545ms | 15.58% | 10165513 | 0 | 56.55% | 1.23 | 930.42 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 15.6281ms | 24.5014ms | 24.86% | 11219057 | 0 | 56.55% | 1.23 | 976.37 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 18.6062ms | 24.0902ms | 11.72% | 12282561 | 0 | 56.55% | 1.23 | 820.09 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 30.6115ms | 45.0939ms | 12.78% | 13707893 | 2000000 | 56.55% | 1.23 | 498.47 MB/s |
| Quicksort | 10000000 | 317.2665ms | 329.1061ms | 2.52% | 229710139 | 0 | 56.47% | 1.24 | 480.95 MB/s |
| Timsort | 10000000 | 601.1201ms | 615.3761ms | 1.48% | 238765827 | 0 | 56.29% | 1.24 | 253.84 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 327.1278ms | 342.6363ms | 2.21% | 239580345 | 10017407 | 56.41% | 1.23 | 466.45 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 157.3268ms | 161.6968ms | 1.36% | 128920493 | 10000000 | 56.67% | 1.23 | 969.88 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 139.8192ms | 142.9328ms | 1.28% | 128920493 | 0 | 56.67% | 1.23 | 1091.32 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 156.6728ms | 163.3254ms | 3.77% | 131078234 | 0 | 56.59% | 1.22 | 973.93 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 121.5326ms | 126.1442ms | 4.44% | 154376529 | 0 | 56.63% | 1.23 | 1255.53 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 157.3923ms | 163.4270ms | 3.05% | 156481034 | 0 | 56.44% | 1.23 | 969.47 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 153.2056ms | 158.7622ms | 2.44% | 31143816 | 0 | 56.44% | 1.22 | 995.97 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 169.6722ms | 173.6273ms | 1.31% | 45654340 | 0 | 56.52% | 1.22 | 899.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 151.4752ms | 153.9378ms | 1.28% | 45663918 | 0 | 56.48% | 1.22 | 1007.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 580.1563ms | 600.2824ms | 2.04% | 199050033 | 20000000 | 56.61% | 1.23 | 263.01 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0218ms | 0.0362ms | 40.36% | 5523 | 0 | 56.77% | 1.22 | 700.17 MB/s |
| Timsort | 1000 | 0.0202ms | 0.0692ms | 41.11% | 5690 | 0 | 56.77% | 1.22 | 756.77 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.0760ms | 0.2498ms | 23.23% | 984 | 2000 | 56.77% | 1.22 | 200.68 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.1354ms | 0.3201ms | 20.58% | 984 | 2000 | 56.77% | 1.22 | 112.66 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0143ms | 0.0383ms | 38.93% | 5523 | 0 | 56.77% | 1.22 | 1068.47 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0361ms | 0.0372ms | 22.61% | 5523 | 0 | 56.77% | 1.22 | 423.03 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0171ms | 0.0376ms | 30.51% | 5523 | 0 | 56.77% | 1.22 | 892.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0646ms | 0.0698ms | 18.85% | 5690 | 0 | 56.77% | 1.22 | 236.28 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0307ms | 0.0360ms | 27.97% | 5523 | 0 | 56.77% | 1.22 | 496.72 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0513ms | 0.0676ms | 22.05% | 5690 | 0 | 56.77% | 1.22 | 297.40 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0345ms | 0.0446ms | 27.62% | 5523 | 0 | 56.77% | 1.22 | 442.54 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0352ms | 0.0375ms | 21.85% | 5523 | 0 | 56.77% | 1.22 | 434.09 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0356ms | 0.0380ms | 19.79% | 5523 | 0 | 56.77% | 1.22 | 428.22 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.2631ms | 0.7751ms | 23.22% | 5523 | 2000 | 56.77% | 1.22 | 58.01 MB/s |
| Quicksort | 10000 | 0.1556ms | 0.2858ms | 22.54% | 53926 | 0 | 56.77% | 1.22 | 980.65 MB/s |
| Timsort | 10000 | 0.1412ms | 0.5194ms | 24.00% | 54499 | 0 | 56.77% | 1.22 | 1080.92 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.6699ms | 1.8707ms | 9.52% | 9984 | 30000 | 56.77% | 1.22 | 91.37 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.8986ms | 2.3812ms | 7.12% | 9984 | 30000 | 56.77% | 1.22 | 80.37 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.5101ms | 1.9412ms | 24.57% | 122534 | 14351 | 56.77% | 1.22 | 299.12 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.5332ms | 1.8699ms | 15.94% | 9988 | 10000 | 56.77% | 1.22 | 99.52 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3115ms | 1.0452ms | 27.40% | 9988 | 0 | 56.77% | 1.22 | 489.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.2661ms | 1.1961ms | 31.60% | 9988 | 0 | 56.77% | 1.22 | 573.44 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.2979ms | 1.4721ms | 28.77% | 9988 | 0 | 56.77% | 1.22 | 117.57 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.1558ms | 1.5796ms | 15.77% | 9988 | 0 | 56.77% | 1.22 | 132.02 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.2891ms | 1.0477ms | 32.05% | 9988 | 0 | 56.77% | 1.22 | 527.84 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.8616ms | 1.1047ms | 30.01% | 9988 | 0 | 56.77% | 1.22 | 177.10 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.7439ms | 1.0143ms | 17.98% | 9988 | 0 | 56.77% | 1.22 | 205.11 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.5385ms | 2.0782ms | 25.05% | 9988 | 20000 | 56.77% | 1.22 | 283.37 MB/s |
| Quicksort | 100000 | 0.7552ms | 2.7451ms | 23.82% | 529535 | 0 | 56.77% | 1.22 | 2020.43 MB/s |
| Timsort | 100000 | 1.4908ms | 4.9710ms | 23.69% | 529496 | 0 | 56.77% | 1.22 | 1023.51 MB/s |
| ARS Gen 1: Foundation | 100000 | 8.3279ms | 8.9126ms | 5.07% | 99984 | 300000 | 56.77% | 1.22 | 183.22 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 2.8576ms | 10.0564ms | 24.45% | 99984 | 300000 | 56.77% | 1.22 | 533.97 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 12.4142ms | 12.8524ms | 3.34% | 1144577 | 108703 | 56.77% | 1.22 | 122.91 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.4592ms | 4.3896ms | 11.50% | 99988 | 100000 | 56.77% | 1.22 | 441.11 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 0.7963ms | 3.2568ms | 35.28% | 99988 | 0 | 56.77% | 1.22 | 1916.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.6723ms | 3.6178ms | 25.17% | 99988 | 0 | 56.77% | 1.22 | 912.45 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.3729ms | 3.8363ms | 7.68% | 99988 | 0 | 56.77% | 1.22 | 452.40 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.7056ms | 3.1537ms | 26.71% | 99988 | 0 | 56.77% | 1.22 | 894.63 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.7154ms | 5.5511ms | 16.61% | 199972 | 0 | 56.77% | 1.22 | 410.69 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.2270ms | 4.8303ms | 14.30% | 199972 | 0 | 56.77% | 1.22 | 360.98 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.4834ms | 3.8835ms | 26.00% | 99988 | 0 | 56.77% | 1.22 | 614.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 1.6432ms | 6.0544ms | 27.67% | 99988 | 200000 | 56.77% | 1.22 | 928.59 MB/s |
| Quicksort | 1000000 | 6.9012ms | 7.6863ms | 7.76% | 5137070 | 0 | 56.75% | 1.22 | 2211.02 MB/s |
| Timsort | 1000000 | 13.9862ms | 14.2915ms | 5.69% | 6203899 | 0 | 56.73% | 1.22 | 1090.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 27.5468ms | 29.0511ms | 6.73% | 12087813 | 1017407 | 56.77% | 1.22 | 553.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 10.7737ms | 12.8085ms | 8.86% | 999988 | 1000000 | 56.77% | 1.22 | 1416.30 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 11.1587ms | 12.1222ms | 7.85% | 999988 | 0 | 56.77% | 1.22 | 1367.44 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 10.1276ms | 12.7754ms | 11.07% | 999988 | 0 | 56.77% | 1.22 | 1506.66 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 7.0782ms | 8.1467ms | 12.85% | 999988 | 0 | 56.77% | 1.22 | 2155.75 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 7.0155ms | 9.0073ms | 13.04% | 999988 | 0 | 56.77% | 1.22 | 2175.01 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 13.0199ms | 14.9360ms | 5.99% | 1999972 | 0 | 56.77% | 1.22 | 1171.96 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 11.8648ms | 13.0961ms | 5.34% | 1999972 | 0 | 56.77% | 1.22 | 1286.06 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 12.0062ms | 15.3232ms | 11.86% | 1999976 | 0 | 56.77% | 1.22 | 1270.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 19.7276ms | 23.0637ms | 6.09% | 5660775 | 2000000 | 56.75% | 1.22 | 773.47 MB/s |
| Quicksort | 10000000 | 85.1387ms | 87.6011ms | 4.06% | 51295534 | 0 | 56.78% | 1.23 | 1792.23 MB/s |
| Timsort | 10000000 | 265.6138ms | 271.7331ms | 1.10% | 66963952 | 0 | 56.77% | 1.22 | 574.47 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 234.3076ms | 241.4579ms | 4.76% | 120086692 | 10017407 | 56.80% | 1.22 | 651.23 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 97.2541ms | 99.6301ms | 3.19% | 9999990 | 10000000 | 56.90% | 1.22 | 1568.96 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 97.9834ms | 99.9938ms | 3.07% | 9999990 | 0 | 56.90% | 1.22 | 1557.28 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 96.6182ms | 103.3089ms | 2.69% | 9999990 | 0 | 56.90% | 1.22 | 1579.29 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 61.5440ms | 63.2423ms | 3.82% | 9999990 | 0 | 56.85% | 1.22 | 2479.33 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 60.8867ms | 64.5896ms | 8.93% | 9999990 | 0 | 56.85% | 1.22 | 2506.10 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 125.7982ms | 128.7787ms | 1.27% | 19999974 | 0 | 56.95% | 1.22 | 1212.96 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 107.9112ms | 110.3826ms | 7.29% | 19999974 | 0 | 56.91% | 1.22 | 1414.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 124.2845ms | 126.8283ms | 2.32% | 19999974 | 0 | 56.94% | 1.22 | 1227.73 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 363.8968ms | 370.4872ms | 2.49% | 117027284 | 20000000 | 56.80% | 1.22 | 419.32 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0418ms | 0.0781ms | 19.70% | 10160 | 0 | 56.89% | 1.23 | 365.21 MB/s |
| Timsort | 1000 | 0.0445ms | 0.1348ms | 24.35% | 10742 | 0 | 56.89% | 1.23 | 342.55 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.3290ms | 1.1212ms | 23.25% | 2 | 2000 | 56.89% | 1.23 | 46.38 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.3808ms | 1.3893ms | 24.67% | 2 | 2000 | 56.89% | 1.23 | 40.07 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0243ms | 0.0830ms | 34.22% | 10160 | 0 | 56.89% | 1.23 | 628.30 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0202ms | 0.0829ms | 28.68% | 10160 | 0 | 56.89% | 1.23 | 755.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0285ms | 0.0834ms | 30.52% | 10160 | 0 | 56.89% | 1.23 | 535.73 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0428ms | 0.1363ms | 24.68% | 10742 | 0 | 56.89% | 1.23 | 356.86 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0640ms | 0.0785ms | 24.03% | 10160 | 0 | 56.89% | 1.23 | 238.38 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0452ms | 0.1611ms | 27.63% | 10742 | 0 | 56.89% | 1.23 | 337.87 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0234ms | 0.0792ms | 25.88% | 10160 | 0 | 56.89% | 1.23 | 653.17 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0226ms | 0.1007ms | 28.18% | 10160 | 0 | 56.89% | 1.23 | 675.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0820ms | 0.0833ms | 7.28% | 10160 | 0 | 56.89% | 1.23 | 186.14 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.2102ms | 0.8278ms | 36.94% | 10160 | 2000 | 56.89% | 1.23 | 72.60 MB/s |
| Quicksort | 10000 | 0.2276ms | 0.9107ms | 35.95% | 136996 | 0 | 56.89% | 1.23 | 670.44 MB/s |
| Timsort | 10000 | 0.6292ms | 1.5627ms | 20.50% | 141829 | 0 | 56.89% | 1.23 | 242.51 MB/s |
| ARS Gen 1: Foundation | 10000 | 21.9193ms | 27.2525ms | 17.91% | 44 | 30000 | 56.88% | 1.23 | 6.96 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 15.8218ms | 27.0547ms | 19.80% | 44 | 30000 | 56.88% | 1.23 | 9.64 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.6256ms | 2.1580ms | 24.42% | 193234 | 14351 | 56.89% | 1.23 | 243.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.4858ms | 1.7443ms | 13.41% | 52358 | 10000 | 56.89% | 1.23 | 102.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.0124ms | 1.4779ms | 24.45% | 52358 | 0 | 56.89% | 1.23 | 150.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.0340ms | 1.2272ms | 44.96% | 57703 | 0 | 56.89% | 1.23 | 147.56 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.4976ms | 1.9481ms | 24.04% | 57911 | 0 | 56.89% | 1.23 | 101.89 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.4530ms | 1.7816ms | 42.23% | 60501 | 0 | 56.89% | 1.23 | 336.84 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.0633ms | 1.3984ms | 16.51% | 52358 | 0 | 56.89% | 1.23 | 143.51 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.0825ms | 1.2803ms | 13.68% | 52358 | 0 | 56.89% | 1.23 | 140.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3120ms | 1.2900ms | 46.63% | 52358 | 0 | 56.89% | 1.23 | 489.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.6113ms | 2.2719ms | 32.06% | 52358 | 20000 | 56.89% | 1.23 | 249.62 MB/s |
| Quicksort | 100000 | 8.0911ms | 11.0948ms | 9.15% | 1708661 | 0 | 56.89% | 1.23 | 188.59 MB/s |
| Timsort | 100000 | 5.6942ms | 17.7486ms | 34.21% | 1748978 | 0 | 56.89% | 1.23 | 267.97 MB/s |
| ARS Gen 1: Foundation | 100000 | 46.6371ms | 74.3350ms | 12.10% | 2102 | 300000 | 56.52% | 1.23 | 32.72 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 45.2375ms | 74.8698ms | 14.68% | 2102 | 300000 | 56.54% | 1.23 | 33.73 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.1683ms | 14.5954ms | 25.04% | 1886943 | 108703 | 56.89% | 1.23 | 366.07 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 1.4147ms | 6.2192ms | 26.81% | 881461 | 100000 | 56.89% | 1.23 | 1078.61 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.0437ms | 5.0679ms | 15.36% | 881461 | 0 | 56.89% | 1.23 | 377.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.2776ms | 5.0137ms | 19.47% | 922226 | 0 | 56.89% | 1.23 | 669.95 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.5063ms | 5.5692ms | 18.23% | 933682 | 0 | 56.89% | 1.23 | 338.61 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.4971ms | 5.9115ms | 22.56% | 973015 | 0 | 56.89% | 1.23 | 611.07 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.6872ms | 5.4141ms | 12.98% | 881461 | 0 | 56.89% | 1.23 | 325.54 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.9203ms | 6.0455ms | 28.66% | 769868 | 0 | 56.89% | 1.23 | 522.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.2563ms | 5.2172ms | 25.23% | 881461 | 0 | 56.89% | 1.23 | 358.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 6.5208ms | 8.4212ms | 10.61% | 881461 | 200000 | 56.89% | 1.23 | 234.00 MB/s |
| Quicksort | 1000000 | 36.2265ms | 44.9765ms | 7.93% | 20371770 | 0 | 56.88% | 1.23 | 421.20 MB/s |
| Timsort | 1000000 | 49.3836ms | 63.0262ms | 8.63% | 20790377 | 0 | 56.85% | 1.23 | 308.99 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 47.1600ms | 51.6486ms | 4.85% | 21441624 | 1017407 | 56.87% | 1.23 | 323.55 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 20.4576ms | 25.0587ms | 10.06% | 10165513 | 1000000 | 56.89% | 1.23 | 745.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 19.9559ms | 22.1831ms | 8.18% | 10165513 | 0 | 56.89% | 1.23 | 764.63 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.5213ms | 26.0587ms | 10.64% | 10580504 | 0 | 56.89% | 1.23 | 781.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 17.4306ms | 24.5961ms | 25.36% | 12890329 | 0 | 56.89% | 1.23 | 875.40 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 14.5047ms | 29.6393ms | 17.67% | 13317079 | 0 | 56.89% | 1.23 | 1051.99 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 21.6932ms | 24.2007ms | 15.99% | 10165513 | 0 | 56.89% | 1.23 | 703.39 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 24.8767ms | 26.9950ms | 25.13% | 11219057 | 0 | 56.89% | 1.23 | 613.38 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 21.4360ms | 25.3357ms | 19.81% | 12282561 | 0 | 56.89% | 1.23 | 711.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 27.2424ms | 44.6250ms | 14.73% | 13659325 | 2000000 | 56.89% | 1.23 | 560.11 MB/s |
| Quicksort | 10000000 | 317.9111ms | 323.2718ms | 1.32% | 229710139 | 0 | 56.81% | 1.24 | 479.97 MB/s |
| Timsort | 10000000 | 602.2729ms | 609.9215ms | 0.89% | 238765827 | 0 | 56.67% | 1.23 | 253.35 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 326.1421ms | 338.3100ms | 3.75% | 239580345 | 10017407 | 56.78% | 1.23 | 467.86 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 156.1002ms | 163.2369ms | 2.95% | 128920493 | 10000000 | 56.99% | 1.22 | 977.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 137.9759ms | 142.8255ms | 2.65% | 128920493 | 0 | 56.99% | 1.23 | 1105.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 157.6024ms | 162.1593ms | 2.16% | 131078234 | 0 | 56.93% | 1.22 | 968.18 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 121.8464ms | 127.1009ms | 2.12% | 154376529 | 0 | 56.96% | 1.23 | 1252.30 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 149.9840ms | 164.4852ms | 3.99% | 156481034 | 0 | 56.79% | 1.23 | 1017.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 150.1277ms | 155.9169ms | 2.09% | 31143816 | 0 | 56.80% | 1.22 | 1016.39 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 168.4097ms | 171.4312ms | 2.31% | 45654340 | 0 | 56.84% | 1.22 | 906.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 151.7069ms | 154.3220ms | 2.34% | 45663918 | 0 | 56.82% | 1.22 | 1005.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 569.2396ms | 586.5272ms | 2.01% | 199050009 | 20000000 | 56.94% | 1.23 | 268.06 MB/s |

## Category: String

### Distribution: Random

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0752ms | 0.1887ms | 30.20% | 5530 | 0 | 57.07% | 1.22 | 811.93 MB/s |
| Timsort | 1000 | 0.2193ms | 0.2608ms | 15.15% | 6109 | 0 | 57.07% | 1.22 | 278.33 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.3031ms | 0.7378ms | 20.50% | 984 | 2000 | 57.07% | 1.22 | 201.38 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.3259ms | 0.7517ms | 23.65% | 984 | 2000 | 57.07% | 1.22 | 187.31 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.1836ms | 0.2004ms | 8.86% | 5530 | 0 | 57.07% | 1.22 | 332.40 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1768ms | 0.1979ms | 27.41% | 5530 | 0 | 57.07% | 1.22 | 345.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.1748ms | 0.1914ms | 27.29% | 5530 | 0 | 57.07% | 1.22 | 349.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0596ms | 0.2376ms | 30.08% | 6109 | 0 | 57.07% | 1.22 | 1023.67 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.1180ms | 0.2075ms | 28.03% | 5530 | 0 | 57.07% | 1.22 | 517.44 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0688ms | 0.2296ms | 29.56% | 6109 | 0 | 57.07% | 1.22 | 887.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.1796ms | 0.2308ms | 17.73% | 5530 | 0 | 57.07% | 1.22 | 339.93 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.1433ms | 0.1831ms | 8.90% | 5530 | 0 | 57.07% | 1.22 | 426.05 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0617ms | 0.1829ms | 34.16% | 5530 | 0 | 57.07% | 1.22 | 989.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4325ms | 1.2713ms | 33.56% | 5530 | 2000 | 57.07% | 1.22 | 141.14 MB/s |
| Quicksort | 10000 | 0.4272ms | 1.8418ms | 35.91% | 53207 | 0 | 57.06% | 1.22 | 1428.69 MB/s |
| Timsort | 10000 | 2.1087ms | 2.2494ms | 5.67% | 53257 | 0 | 57.06% | 1.22 | 289.45 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.8189ms | 9.9035ms | 23.19% | 9984 | 30000 | 57.06% | 1.22 | 216.52 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 9.0280ms | 9.8198ms | 5.80% | 9984 | 30000 | 57.06% | 1.22 | 67.61 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 3.3759ms | 11.5195ms | 29.92% | 122576 | 14351 | 57.06% | 1.22 | 180.80 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.0724ms | 3.8187ms | 34.18% | 12651 | 10000 | 57.06% | 1.22 | 569.12 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.0265ms | 1.5262ms | 23.17% | 12651 | 0 | 57.06% | 1.22 | 594.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.3469ms | 1.5929ms | 37.99% | 12634 | 0 | 57.06% | 1.22 | 1759.62 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.9699ms | 2.3475ms | 17.11% | 9990 | 0 | 57.06% | 1.22 | 309.83 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.5032ms | 2.1989ms | 25.05% | 9990 | 0 | 57.06% | 1.22 | 406.05 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3490ms | 1.4829ms | 31.38% | 12651 | 0 | 57.06% | 1.22 | 1748.89 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.4152ms | 1.6485ms | 27.96% | 12651 | 0 | 57.06% | 1.22 | 1470.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.0458ms | 1.3599ms | 14.39% | 12651 | 0 | 57.06% | 1.22 | 583.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 4.0406ms | 6.9580ms | 17.50% | 12651 | 20000 | 57.06% | 1.22 | 151.06 MB/s |
| Quicksort | 100000 | 6.0852ms | 10.6793ms | 18.01% | 516801 | 0 | 57.05% | 1.22 | 1003.00 MB/s |
| Timsort | 100000 | 13.1676ms | 14.7983ms | 10.90% | 523232 | 0 | 57.05% | 1.22 | 463.52 MB/s |
| ARS Gen 1: Foundation | 100000 | 23.6545ms | 30.8025ms | 9.28% | 99984 | 300000 | 57.05% | 1.22 | 258.03 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 24.6980ms | 31.2644ms | 8.58% | 99984 | 300000 | 57.05% | 1.22 | 247.13 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 23.8530ms | 29.2146ms | 7.43% | 1144061 | 108703 | 57.06% | 1.22 | 255.88 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 9.7425ms | 10.4667ms | 9.34% | 99988 | 100000 | 57.06% | 1.22 | 626.48 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.4990ms | 5.2361ms | 32.70% | 99988 | 0 | 57.05% | 1.22 | 1744.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.4309ms | 4.8962ms | 16.74% | 99988 | 0 | 57.06% | 1.22 | 1778.99 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.9190ms | 5.4369ms | 22.43% | 99988 | 0 | 57.06% | 1.22 | 1557.43 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.9790ms | 6.9633ms | 15.96% | 99988 | 0 | 57.06% | 1.22 | 1225.84 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.1217ms | 6.6893ms | 25.19% | 199972 | 0 | 57.05% | 1.22 | 1955.17 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 7.0891ms | 9.1120ms | 10.77% | 199972 | 0 | 57.05% | 1.22 | 860.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 5.1426ms | 6.1755ms | 12.47% | 99988 | 0 | 57.06% | 1.22 | 1186.85 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 21.4399ms | 22.1738ms | 2.49% | 99988 | 200000 | 57.04% | 1.22 | 284.68 MB/s |
| Quicksort | 1000000 | 439.6719ms | 445.2540ms | 0.79% | 19595153 | 0 | 57.17% | 1.22 | 138.82 MB/s |
| Timsort | 1000000 | 569.3888ms | 578.5611ms | 0.82% | 20426759 | 0 | 57.10% | 1.22 | 107.19 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 248.4968ms | 253.2518ms | 1.62% | 20672327 | 1017407 | 56.99% | 1.22 | 245.62 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 85.5206ms | 88.2446ms | 2.52% | 11334517 | 1000000 | 57.07% | 1.22 | 713.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 67.7885ms | 69.8739ms | 4.06% | 11334517 | 0 | 57.09% | 1.22 | 900.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 75.2195ms | 78.0713ms | 2.21% | 11566000 | 0 | 57.06% | 1.22 | 811.43 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 84.4081ms | 86.7622ms | 2.77% | 12332487 | 0 | 57.10% | 1.22 | 723.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 96.7898ms | 103.2589ms | 3.06% | 12561492 | 0 | 57.08% | 1.22 | 630.60 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 66.8805ms | 73.4902ms | 4.56% | 8485673 | 0 | 57.11% | 1.22 | 912.60 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 90.1176ms | 92.0479ms | 1.79% | 7026293 | 0 | 57.17% | 1.22 | 677.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 75.0428ms | 76.5974ms | 2.77% | 7792026 | 0 | 57.14% | 1.22 | 813.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 371.1393ms | 481.5586ms | 9.43% | 15103346 | 2000000 | 57.23% | 1.22 | 164.45 MB/s |
| Quicksort | 10000000 | 8797.5169ms | 9097.4107ms | 1.62% | 182952038 | 0 | 59.82% | 1.16 | 69.38 MB/s |
| Timsort | 10000000 | 10710.3793ms | 10746.4885ms | 0.66% | 202525310 | 0 | 59.44% | 1.14 | 56.99 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 2926.1778ms | 2957.8580ms | 0.50% | 192878458 | 10017407 | 56.48% | 1.18 | 208.58 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 1830.4439ms | 1905.8234ms | 2.27% | 110900253 | 0 | 58.58% | 1.14 | 333.44 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 2548.6752ms | 2611.3301ms | 1.18% | 110924473 | 0 | 58.62% | 1.10 | 239.48 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 835.3839ms | 841.5747ms | 0.66% | 34176169 | 0 | 57.88% | 1.18 | 730.62 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 1137.8604ms | 1145.2580ms | 0.28% | 40132484 | 0 | 58.50% | 1.17 | 536.40 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 859.2156ms | 872.8790ms | 0.62% | 41266054 | 0 | 57.90% | 1.18 | 710.36 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 9915.0859ms | 10078.7592ms | 2.35% | 212961539 | 20000000 | 59.66% | 1.16 | 61.56 MB/s |

### Distribution: Gaussian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0669ms | 0.2139ms | 38.40% | 5530 | 0 | 61.78% | 1.10 | 912.69 MB/s |
| Timsort | 1000 | 0.0920ms | 0.2479ms | 25.00% | 6109 | 0 | 61.78% | 1.10 | 663.32 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.3143ms | 0.7481ms | 20.27% | 984 | 2000 | 61.78% | 1.10 | 194.18 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.7226ms | 0.8263ms | 25.52% | 984 | 2000 | 61.78% | 1.10 | 84.46 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.1283ms | 0.1994ms | 19.65% | 5530 | 0 | 61.78% | 1.10 | 475.70 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1938ms | 0.2036ms | 11.30% | 5530 | 0 | 61.78% | 1.10 | 314.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0773ms | 0.1938ms | 23.79% | 5530 | 0 | 61.78% | 1.10 | 789.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0715ms | 0.2719ms | 36.07% | 6109 | 0 | 61.78% | 1.10 | 853.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.1890ms | 0.2044ms | 26.15% | 5530 | 0 | 61.78% | 1.10 | 322.88 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.2345ms | 0.2929ms | 25.61% | 6109 | 0 | 61.78% | 1.10 | 260.32 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.1902ms | 0.2058ms | 35.57% | 5530 | 0 | 61.78% | 1.10 | 320.94 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.1851ms | 0.2111ms | 19.34% | 5530 | 0 | 61.78% | 1.10 | 329.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.1875ms | 0.2150ms | 9.97% | 5530 | 0 | 61.78% | 1.10 | 325.57 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5305ms | 1.7488ms | 23.80% | 5530 | 2000 | 61.78% | 1.10 | 115.04 MB/s |
| Quicksort | 10000 | 0.6600ms | 1.9430ms | 23.68% | 53207 | 0 | 61.77% | 1.10 | 924.78 MB/s |
| Timsort | 10000 | 1.2359ms | 2.4125ms | 17.87% | 53257 | 0 | 61.77% | 1.10 | 493.84 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.8216ms | 8.7458ms | 22.44% | 9984 | 30000 | 61.77% | 1.10 | 216.31 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 8.6863ms | 9.3031ms | 11.55% | 9984 | 30000 | 61.77% | 1.10 | 70.27 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 3.6129ms | 11.3214ms | 22.38% | 122576 | 14351 | 61.77% | 1.10 | 168.93 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 3.8477ms | 4.1626ms | 5.99% | 12651 | 10000 | 61.77% | 1.10 | 158.63 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.2769ms | 1.6504ms | 34.53% | 12651 | 0 | 61.77% | 1.10 | 478.00 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.3913ms | 1.4456ms | 30.67% | 12634 | 0 | 61.77% | 1.10 | 1559.95 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.7937ms | 2.2381ms | 22.76% | 9990 | 0 | 61.77% | 1.10 | 340.27 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.5241ms | 2.0496ms | 34.50% | 9990 | 0 | 61.77% | 1.10 | 1164.50 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.7163ms | 1.4345ms | 21.29% | 12651 | 0 | 61.77% | 1.10 | 852.08 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.6217ms | 1.5724ms | 24.17% | 12651 | 0 | 61.77% | 1.10 | 981.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.3945ms | 1.5112ms | 47.45% | 12651 | 0 | 61.77% | 1.10 | 1547.02 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.1975ms | 6.9666ms | 22.75% | 12651 | 20000 | 61.77% | 1.10 | 277.75 MB/s |
| Quicksort | 100000 | 10.8156ms | 12.2048ms | 10.98% | 516801 | 0 | 61.76% | 1.10 | 564.33 MB/s |
| Timsort | 100000 | 10.0308ms | 15.3319ms | 12.50% | 523232 | 0 | 61.76% | 1.10 | 608.48 MB/s |
| ARS Gen 1: Foundation | 100000 | 26.0243ms | 32.0445ms | 7.66% | 99984 | 300000 | 61.76% | 1.10 | 234.53 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 24.3105ms | 31.1687ms | 9.76% | 99984 | 300000 | 61.76% | 1.10 | 251.07 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 22.7427ms | 29.8393ms | 11.14% | 1144061 | 108703 | 61.77% | 1.10 | 268.37 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 6.9098ms | 11.0141ms | 18.42% | 99988 | 100000 | 61.77% | 1.10 | 883.31 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.0932ms | 4.9912ms | 20.16% | 99988 | 0 | 61.77% | 1.10 | 1491.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.6804ms | 5.1126ms | 21.95% | 99988 | 0 | 61.77% | 1.10 | 1658.37 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.4761ms | 6.1514ms | 23.53% | 99988 | 0 | 61.77% | 1.10 | 2464.92 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.3010ms | 6.0216ms | 19.67% | 99988 | 0 | 61.77% | 1.10 | 1419.09 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.5943ms | 6.8313ms | 16.91% | 199972 | 0 | 61.76% | 1.10 | 1328.50 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.1085ms | 6.9981ms | 14.68% | 199972 | 0 | 61.77% | 1.10 | 1194.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.7955ms | 5.3434ms | 27.26% | 99988 | 0 | 61.77% | 1.10 | 2183.32 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 14.3970ms | 24.1476ms | 20.36% | 99988 | 200000 | 61.76% | 1.10 | 423.94 MB/s |
| Quicksort | 1000000 | 282.4508ms | 288.5722ms | 1.22% | 19595153 | 0 | 61.79% | 1.10 | 216.09 MB/s |
| Timsort | 1000000 | 357.4173ms | 362.7915ms | 0.76% | 20426759 | 0 | 61.73% | 1.10 | 170.77 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 227.1779ms | 230.1603ms | 1.20% | 20672327 | 1017407 | 61.74% | 1.10 | 268.67 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 86.0248ms | 88.0644ms | 2.80% | 11334517 | 1000000 | 61.76% | 1.10 | 709.51 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 72.5288ms | 74.6511ms | 2.40% | 11334517 | 0 | 61.77% | 1.10 | 841.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 83.6822ms | 87.1606ms | 3.52% | 11566000 | 0 | 61.75% | 1.10 | 729.37 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 94.5242ms | 97.5010ms | 3.08% | 12332487 | 0 | 61.78% | 1.10 | 645.71 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 106.6237ms | 112.7279ms | 2.61% | 12561492 | 0 | 61.76% | 1.10 | 572.44 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 72.8235ms | 74.1471ms | 2.10% | 8485673 | 0 | 61.78% | 1.10 | 838.12 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 95.2962ms | 98.7925ms | 3.58% | 7026293 | 0 | 61.82% | 1.10 | 640.48 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 78.7629ms | 79.4431ms | 3.14% | 7792026 | 0 | 61.81% | 1.10 | 774.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 372.7961ms | 480.4437ms | 9.27% | 15103346 | 2000000 | 61.84% | 1.10 | 163.72 MB/s |
| Quicksort | 10000000 | 8081.6100ms | 8281.5135ms | 1.11% | 182952038 | 0 | 63.08% | 1.07 | 75.52 MB/s |
| Timsort | 10000000 | 10261.1107ms | 10370.5949ms | 0.66% | 202525310 | 0 | 62.70% | 1.06 | 59.48 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 2775.9183ms | 2843.9233ms | 1.59% | 192878458 | 10017407 | 61.13% | 1.08 | 219.87 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 1822.8826ms | 1902.4717ms | 1.97% | 110900253 | 0 | 62.39% | 1.05 | 334.83 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 2490.5494ms | 2573.0102ms | 1.63% | 110924473 | 0 | 62.37% | 1.03 | 245.07 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 842.1299ms | 850.1293ms | 0.78% | 34176169 | 0 | 62.11% | 1.08 | 724.77 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 1150.4154ms | 1159.0272ms | 1.10% | 40132484 | 0 | 62.46% | 1.07 | 530.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 873.3528ms | 879.3310ms | 0.88% | 41266054 | 0 | 62.11% | 1.08 | 698.86 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 10080.2423ms | 10235.3193ms | 2.29% | 212961532 | 20000000 | 62.73% | 1.07 | 60.55 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.1878ms | 0.2236ms | 18.75% | 5669 | 0 | 62.78% | 1.04 | 324.95 MB/s |
| Timsort | 1000 | 0.0626ms | 0.2212ms | 41.69% | 5836 | 0 | 62.78% | 1.04 | 975.38 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1845ms | 0.6250ms | 36.85% | 984 | 2000 | 62.78% | 1.04 | 330.88 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2092ms | 0.6827ms | 33.04% | 984 | 2000 | 62.78% | 1.04 | 291.76 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0741ms | 0.1767ms | 19.15% | 5669 | 0 | 62.78% | 1.04 | 823.51 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1734ms | 0.1817ms | 16.97% | 5669 | 0 | 62.78% | 1.04 | 351.91 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.1670ms | 0.1758ms | 13.42% | 5669 | 0 | 62.78% | 1.04 | 365.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.2067ms | 0.2511ms | 20.53% | 5836 | 0 | 62.78% | 1.04 | 295.34 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0616ms | 0.1802ms | 26.60% | 5669 | 0 | 62.78% | 1.04 | 991.10 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0708ms | 0.2182ms | 39.51% | 5836 | 0 | 62.78% | 1.04 | 862.41 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0520ms | 0.2077ms | 26.52% | 5669 | 0 | 62.78% | 1.04 | 1173.41 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0681ms | 0.1677ms | 23.10% | 5669 | 0 | 62.78% | 1.04 | 896.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0641ms | 0.1721ms | 23.25% | 5669 | 0 | 62.78% | 1.04 | 951.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4192ms | 1.3546ms | 34.93% | 5669 | 2000 | 62.78% | 1.04 | 145.61 MB/s |
| Quicksort | 10000 | 1.7860ms | 1.8430ms | 2.32% | 53884 | 0 | 62.78% | 1.04 | 341.73 MB/s |
| Timsort | 10000 | 2.1659ms | 2.4278ms | 12.23% | 54110 | 0 | 62.78% | 1.04 | 281.80 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.2417ms | 8.6150ms | 8.00% | 9984 | 30000 | 62.78% | 1.04 | 74.06 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 8.9539ms | 10.0314ms | 6.27% | 9984 | 30000 | 62.78% | 1.04 | 68.17 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 5.9476ms | 11.2476ms | 16.72% | 119348 | 14351 | 62.78% | 1.04 | 102.62 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 3.2082ms | 4.0436ms | 21.71% | 9988 | 10000 | 62.78% | 1.04 | 190.25 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3220ms | 1.1354ms | 34.64% | 9988 | 0 | 62.78% | 1.04 | 1895.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.4581ms | 1.3322ms | 36.63% | 9988 | 0 | 62.78% | 1.04 | 1332.33 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.3977ms | 2.0782ms | 16.47% | 12637 | 0 | 62.78% | 1.04 | 436.67 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.5889ms | 1.9507ms | 26.55% | 12601 | 0 | 62.78% | 1.04 | 1036.36 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.7895ms | 1.4026ms | 19.43% | 9988 | 0 | 62.78% | 1.04 | 773.10 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.1371ms | 1.5101ms | 28.59% | 9988 | 0 | 62.78% | 1.04 | 536.75 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.2875ms | 1.4918ms | 44.89% | 9988 | 0 | 62.78% | 1.04 | 2122.87 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.8747ms | 6.9170ms | 26.47% | 9988 | 20000 | 62.78% | 1.04 | 325.58 MB/s |
| Quicksort | 100000 | 6.8050ms | 15.3170ms | 21.01% | 523034 | 0 | 62.77% | 1.04 | 896.91 MB/s |
| Timsort | 100000 | 15.1150ms | 18.0307ms | 7.81% | 517886 | 0 | 62.77% | 1.04 | 403.81 MB/s |
| ARS Gen 1: Foundation | 100000 | 24.8017ms | 32.9194ms | 20.12% | 99984 | 300000 | 62.77% | 1.04 | 246.09 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 24.5997ms | 33.9904ms | 11.88% | 99984 | 300000 | 62.77% | 1.04 | 248.11 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 22.5440ms | 32.0828ms | 13.20% | 1137109 | 108703 | 62.78% | 1.04 | 270.74 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 11.3611ms | 13.4690ms | 19.57% | 163660 | 100000 | 62.78% | 1.04 | 537.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 5.8712ms | 8.3776ms | 13.26% | 163660 | 0 | 62.78% | 1.04 | 1039.56 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 6.0735ms | 8.0860ms | 16.23% | 163894 | 0 | 62.78% | 1.04 | 1004.95 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.7767ms | 6.5332ms | 37.92% | 99996 | 0 | 62.77% | 1.04 | 2198.11 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.6851ms | 7.2971ms | 22.79% | 99996 | 0 | 62.77% | 1.04 | 1302.76 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 5.4528ms | 8.6398ms | 20.34% | 199990 | 0 | 62.77% | 1.04 | 1119.34 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.6025ms | 7.8536ms | 21.56% | 199984 | 0 | 62.77% | 1.04 | 1089.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.3121ms | 6.7466ms | 20.57% | 100006 | 0 | 62.78% | 1.04 | 1842.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 15.3178ms | 26.6225ms | 14.37% | 163660 | 200000 | 62.77% | 1.04 | 398.46 MB/s |
| Quicksort | 1000000 | 201.0596ms | 202.7331ms | 1.93% | 20615107 | 0 | 62.74% | 1.04 | 303.57 MB/s |
| Timsort | 1000000 | 225.3852ms | 232.4949ms | 3.05% | 20372372 | 0 | 62.71% | 1.04 | 270.80 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 247.6349ms | 254.6712ms | 2.05% | 20913840 | 1017407 | 62.79% | 1.04 | 246.47 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 63.7555ms | 66.0872ms | 4.43% | 11679273 | 1000000 | 62.77% | 1.04 | 957.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 44.7789ms | 46.1769ms | 1.25% | 11679273 | 0 | 62.77% | 1.04 | 1363.03 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 39.9881ms | 41.2797ms | 3.55% | 7094571 | 0 | 62.76% | 1.04 | 1526.33 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 61.8408ms | 63.6670ms | 5.47% | 12943434 | 0 | 62.76% | 1.04 | 986.97 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 57.4851ms | 60.5118ms | 4.20% | 7863282 | 0 | 62.75% | 1.04 | 1061.76 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 44.7197ms | 45.6272ms | 1.28% | 8901000 | 0 | 62.77% | 1.04 | 1364.84 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 58.6147ms | 61.3931ms | 3.25% | 7210714 | 0 | 62.77% | 1.04 | 1041.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 46.1758ms | 50.3124ms | 4.63% | 7979979 | 0 | 62.77% | 1.04 | 1321.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 277.1999ms | 348.2551ms | 10.93% | 13334812 | 2000000 | 62.80% | 1.04 | 220.18 MB/s |
| Quicksort | 10000000 | 2876.7464ms | 2911.8257ms | 0.48% | 184895325 | 0 | 62.33% | 1.05 | 212.17 MB/s |
| Timsort | 10000000 | 4006.7488ms | 4064.5729ms | 0.80% | 204590586 | 0 | 62.24% | 1.04 | 152.33 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 3222.7151ms | 3309.0057ms | 2.08% | 192482624 | 10017407 | 62.63% | 1.04 | 189.39 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 791.8236ms | 806.5677ms | 1.20% | 110993593 | 0 | 62.56% | 1.03 | 770.82 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 990.6138ms | 1017.0427ms | 1.76% | 136377742 | 0 | 62.63% | 1.02 | 616.13 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 513.6699ms | 523.6312ms | 1.97% | 34558745 | 0 | 62.87% | 1.03 | 1188.22 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 710.8731ms | 721.4748ms | 1.20% | 40451339 | 0 | 63.02% | 1.03 | 858.59 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 524.4078ms | 529.1037ms | 0.57% | 41413397 | 0 | 62.88% | 1.03 | 1163.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 6292.1142ms | 6362.8607ms | 1.11% | 212130074 | 20000000 | 63.56% | 1.04 | 97.00 MB/s |

### Distribution: Duplicates

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0696ms | 0.1961ms | 26.77% | 5532 | 0 | 63.89% | 1.03 | 877.28 MB/s |
| Timsort | 1000 | 0.0880ms | 0.2494ms | 27.97% | 5597 | 0 | 63.89% | 1.03 | 693.72 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2026ms | 0.7338ms | 35.27% | 984 | 2000 | 63.89% | 1.03 | 301.21 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2174ms | 0.8078ms | 28.60% | 984 | 2000 | 63.89% | 1.03 | 280.72 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.1575ms | 0.2227ms | 16.10% | 5532 | 0 | 63.89% | 1.03 | 387.61 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1916ms | 0.1970ms | 19.69% | 5532 | 0 | 63.89% | 1.03 | 318.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.1848ms | 0.2117ms | 22.25% | 5532 | 0 | 63.89% | 1.03 | 330.36 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0802ms | 0.2390ms | 23.25% | 5597 | 0 | 63.89% | 1.03 | 760.77 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.1068ms | 0.2197ms | 21.85% | 5532 | 0 | 63.89% | 1.03 | 571.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0966ms | 0.2543ms | 24.58% | 5597 | 0 | 63.89% | 1.03 | 631.78 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0597ms | 0.1936ms | 37.78% | 5532 | 0 | 63.89% | 1.03 | 1022.91 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.1886ms | 0.2093ms | 9.07% | 5532 | 0 | 63.89% | 1.03 | 323.66 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.1863ms | 0.2278ms | 32.39% | 5532 | 0 | 63.89% | 1.03 | 327.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3442ms | 1.8191ms | 36.71% | 5532 | 2000 | 63.89% | 1.03 | 177.34 MB/s |
| Quicksort | 10000 | 1.8910ms | 2.0473ms | 6.27% | 54031 | 0 | 63.89% | 1.03 | 322.77 MB/s |
| Timsort | 10000 | 1.0146ms | 2.3616ms | 20.75% | 54467 | 0 | 63.89% | 1.03 | 601.55 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.9958ms | 10.3393ms | 27.23% | 9984 | 30000 | 63.89% | 1.03 | 305.81 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.6264ms | 9.7870ms | 17.90% | 9984 | 30000 | 63.89% | 1.03 | 131.93 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 10.5339ms | 10.9878ms | 3.07% | 122654 | 14351 | 63.89% | 1.03 | 57.94 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.9237ms | 3.9345ms | 17.87% | 18115 | 10000 | 63.89% | 1.03 | 317.28 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3839ms | 1.7269ms | 29.14% | 18115 | 0 | 63.89% | 1.03 | 1590.04 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.1475ms | 1.6794ms | 23.10% | 18210 | 0 | 63.89% | 1.03 | 531.88 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.8171ms | 1.9811ms | 28.68% | 12047 | 0 | 63.89% | 1.03 | 746.99 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.8149ms | 2.0888ms | 34.19% | 12077 | 0 | 63.89% | 1.03 | 749.01 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3995ms | 1.5414ms | 38.85% | 18115 | 0 | 63.89% | 1.03 | 1527.74 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.1334ms | 1.3687ms | 12.86% | 18115 | 0 | 63.89% | 1.03 | 538.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.4746ms | 1.5639ms | 29.31% | 18115 | 0 | 63.89% | 1.03 | 1286.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 5.8827ms | 6.4108ms | 8.51% | 18115 | 20000 | 63.89% | 1.03 | 103.75 MB/s |
| Quicksort | 100000 | 5.8532ms | 11.4552ms | 23.36% | 523301 | 0 | 63.88% | 1.03 | 1042.77 MB/s |
| Timsort | 100000 | 12.4617ms | 14.3602ms | 8.62% | 523890 | 0 | 63.88% | 1.03 | 489.78 MB/s |
| ARS Gen 1: Foundation | 100000 | 26.8718ms | 32.0386ms | 10.04% | 99984 | 300000 | 63.88% | 1.03 | 227.13 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 23.1898ms | 31.8639ms | 10.97% | 99984 | 300000 | 63.88% | 1.03 | 263.20 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 21.3050ms | 30.4395ms | 10.68% | 1146595 | 108703 | 63.88% | 1.03 | 286.48 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 9.0715ms | 10.3373ms | 16.57% | 150813 | 100000 | 63.89% | 1.03 | 672.83 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.6079ms | 4.8091ms | 22.72% | 150813 | 0 | 63.88% | 1.03 | 1691.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 4.6503ms | 5.7873ms | 17.37% | 150914 | 0 | 63.88% | 1.03 | 1312.51 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.6851ms | 4.9629ms | 35.08% | 125502 | 0 | 63.88% | 1.03 | 2273.11 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.2728ms | 6.6506ms | 23.17% | 125610 | 0 | 63.88% | 1.03 | 1428.47 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.4030ms | 6.6711ms | 19.61% | 199984 | 0 | 63.88% | 1.03 | 1793.57 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.2796ms | 8.4619ms | 18.41% | 199980 | 0 | 63.88% | 1.03 | 1426.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.3781ms | 5.7446ms | 16.29% | 100000 | 0 | 63.88% | 1.03 | 1806.80 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 21.7402ms | 25.2935ms | 8.57% | 150813 | 200000 | 63.88% | 1.03 | 280.75 MB/s |
| Quicksort | 1000000 | 104.5998ms | 106.9500ms | 1.58% | 5137660 | 0 | 63.92% | 1.03 | 583.51 MB/s |
| Timsort | 1000000 | 181.8580ms | 188.5397ms | 1.99% | 6204570 | 0 | 63.92% | 1.03 | 335.62 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 174.4868ms | 179.8463ms | 2.11% | 12089575 | 1017407 | 63.88% | 1.04 | 349.80 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 33.7260ms | 42.3327ms | 18.45% | 1189071 | 1000000 | 63.89% | 1.03 | 1809.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 24.4536ms | 31.0879ms | 19.05% | 1189071 | 0 | 63.89% | 1.03 | 2495.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 25.9967ms | 33.2929ms | 19.15% | 1189803 | 0 | 63.89% | 1.03 | 2347.80 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 28.1464ms | 34.2443ms | 15.31% | 1189071 | 0 | 63.89% | 1.03 | 2168.49 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 28.8345ms | 35.5816ms | 21.14% | 1189803 | 0 | 63.89% | 1.03 | 2116.74 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 32.8894ms | 43.4331ms | 13.96% | 1999984 | 0 | 63.88% | 1.03 | 1855.77 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 41.9783ms | 48.0077ms | 12.87% | 1999994 | 0 | 63.89% | 1.03 | 1453.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 38.5337ms | 44.0273ms | 10.45% | 1999980 | 0 | 63.89% | 1.03 | 1583.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 356.4737ms | 361.1519ms | 1.59% | 6332736 | 2000000 | 63.96% | 1.03 | 171.22 MB/s |
| Quicksort | 10000000 | 1141.1182ms | 1164.4167ms | 1.06% | 52542717 | 0 | 64.31% | 1.03 | 534.87 MB/s |
| Timsort | 10000000 | 2378.7713ms | 2415.2339ms | 0.93% | 66326091 | 0 | 64.57% | 1.03 | 256.58 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 1689.5542ms | 1779.5503ms | 2.78% | 120087685 | 10017407 | 63.90% | 1.04 | 361.25 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 322.4575ms | 413.8288ms | 11.77% | 9999988 | 0 | 64.00% | 1.03 | 1892.81 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 356.0204ms | 548.4879ms | 19.51% | 9999988 | 0 | 64.01% | 1.03 | 1714.37 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 509.6106ms | 538.2892ms | 2.78% | 19999972 | 0 | 64.07% | 1.03 | 1197.68 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 560.1437ms | 623.4561ms | 6.31% | 19999982 | 0 | 64.09% | 1.03 | 1089.63 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 527.6239ms | 541.9315ms | 4.02% | 19999972 | 0 | 64.07% | 1.03 | 1156.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 5694.5193ms | 5783.5003ms | 1.03% | 116949377 | 20000000 | 65.59% | 1.03 | 107.18 MB/s |

### Distribution: Zipfian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0598ms | 0.1860ms | 33.57% | 5530 | 0 | 67.55% | 1.01 | 1019.85 MB/s |
| Timsort | 1000 | 0.0715ms | 0.2333ms | 40.57% | 6109 | 0 | 67.55% | 1.01 | 853.44 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2460ms | 0.6941ms | 21.59% | 984 | 2000 | 67.55% | 1.01 | 248.15 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.7118ms | 0.7595ms | 11.31% | 984 | 2000 | 67.55% | 1.01 | 85.74 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0613ms | 0.1798ms | 22.92% | 5530 | 0 | 67.55% | 1.01 | 996.15 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0792ms | 0.1778ms | 22.65% | 5530 | 0 | 67.55% | 1.01 | 771.05 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0736ms | 0.1978ms | 23.85% | 5530 | 0 | 67.55% | 1.01 | 829.58 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0723ms | 0.2404ms | 35.76% | 6109 | 0 | 67.55% | 1.01 | 843.69 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0492ms | 0.1769ms | 35.17% | 5530 | 0 | 67.55% | 1.01 | 1241.66 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0631ms | 0.2518ms | 38.08% | 6109 | 0 | 67.55% | 1.01 | 966.88 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.1718ms | 0.1889ms | 9.63% | 5530 | 0 | 67.55% | 1.01 | 355.26 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.1680ms | 0.1748ms | 5.33% | 5530 | 0 | 67.55% | 1.01 | 363.30 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.1730ms | 0.1804ms | 9.11% | 5530 | 0 | 67.55% | 1.01 | 352.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5122ms | 1.3056ms | 31.24% | 5530 | 2000 | 67.55% | 1.01 | 119.16 MB/s |
| Quicksort | 10000 | 0.6849ms | 1.7142ms | 20.24% | 53207 | 0 | 67.55% | 1.01 | 891.17 MB/s |
| Timsort | 10000 | 0.5995ms | 2.0676ms | 29.20% | 53257 | 0 | 67.55% | 1.01 | 1018.03 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.6500ms | 9.5521ms | 7.12% | 9984 | 30000 | 67.55% | 1.01 | 70.56 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 8.9759ms | 9.6679ms | 7.72% | 9984 | 30000 | 67.55% | 1.01 | 68.00 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 4.4663ms | 10.8825ms | 19.01% | 122576 | 14351 | 67.55% | 1.01 | 136.66 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.5990ms | 4.0417ms | 20.95% | 12651 | 10000 | 67.55% | 1.01 | 381.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.4813ms | 1.5978ms | 41.79% | 12651 | 0 | 67.55% | 1.01 | 1268.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.3310ms | 1.8225ms | 23.64% | 12634 | 0 | 67.55% | 1.01 | 458.57 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6622ms | 2.1820ms | 32.11% | 9990 | 0 | 67.55% | 1.01 | 921.74 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.6251ms | 2.3159ms | 37.80% | 9990 | 0 | 67.55% | 1.01 | 375.59 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.4077ms | 1.4930ms | 25.84% | 12651 | 0 | 67.55% | 1.01 | 1497.10 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.2226ms | 2.2674ms | 41.91% | 12651 | 0 | 67.55% | 1.01 | 499.22 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.4667ms | 1.4569ms | 31.29% | 12651 | 0 | 67.55% | 1.01 | 1307.89 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.6447ms | 6.5335ms | 26.41% | 12651 | 20000 | 67.55% | 1.01 | 371.11 MB/s |
| Quicksort | 100000 | 7.7077ms | 12.7805ms | 16.23% | 516801 | 0 | 67.54% | 1.01 | 791.88 MB/s |
| Timsort | 100000 | 12.0686ms | 16.3558ms | 11.43% | 523232 | 0 | 67.54% | 1.01 | 505.74 MB/s |
| ARS Gen 1: Foundation | 100000 | 25.5444ms | 33.3475ms | 11.17% | 99984 | 300000 | 67.54% | 1.01 | 238.94 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 25.4019ms | 31.2113ms | 9.47% | 99984 | 300000 | 67.54% | 1.01 | 240.28 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 23.8955ms | 32.2984ms | 9.96% | 1144061 | 108703 | 67.55% | 1.01 | 255.43 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 5.2356ms | 9.7521ms | 26.58% | 99988 | 100000 | 67.55% | 1.01 | 1165.78 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.5592ms | 4.8628ms | 23.05% | 99988 | 0 | 67.55% | 1.01 | 1714.85 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.3543ms | 5.7339ms | 27.13% | 99988 | 0 | 67.55% | 1.01 | 1819.59 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 4.3159ms | 5.8136ms | 18.94% | 99988 | 0 | 67.55% | 1.01 | 1414.18 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.2792ms | 5.9793ms | 15.85% | 99988 | 0 | 67.55% | 1.01 | 1426.33 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 5.2322ms | 6.3479ms | 13.57% | 199972 | 0 | 67.55% | 1.01 | 1166.53 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.8098ms | 7.8434ms | 16.64% | 199972 | 0 | 67.55% | 1.01 | 1268.96 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.2616ms | 5.3043ms | 22.91% | 99988 | 0 | 67.55% | 1.01 | 1432.22 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 20.8544ms | 22.9414ms | 6.57% | 99988 | 200000 | 67.54% | 1.01 | 292.67 MB/s |
| Quicksort | 1000000 | 485.6372ms | 488.2514ms | 0.44% | 19595153 | 0 | 67.54% | 1.01 | 125.68 MB/s |
| Timsort | 1000000 | 624.5508ms | 632.4956ms | 1.80% | 20426759 | 0 | 67.51% | 1.01 | 97.73 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 246.5705ms | 250.8368ms | 1.05% | 20672327 | 1017407 | 67.50% | 1.01 | 247.54 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 81.6407ms | 83.3781ms | 4.19% | 11334517 | 1000000 | 67.53% | 1.01 | 747.61 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 72.4335ms | 76.0916ms | 2.66% | 11334517 | 0 | 67.52% | 1.01 | 842.64 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 92.0183ms | 96.1805ms | 3.40% | 11566000 | 0 | 67.51% | 1.01 | 663.29 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 92.6156ms | 94.8489ms | 3.69% | 12332487 | 0 | 67.52% | 1.01 | 659.02 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 123.1175ms | 128.8559ms | 3.03% | 12561492 | 0 | 67.51% | 1.01 | 495.75 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 69.1186ms | 71.1487ms | 1.82% | 8485673 | 0 | 67.54% | 1.01 | 883.05 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 95.4537ms | 96.8351ms | 2.58% | 7026293 | 0 | 67.56% | 1.01 | 639.42 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 71.9535ms | 75.6941ms | 3.00% | 7792026 | 0 | 67.55% | 1.01 | 848.26 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 444.8612ms | 563.8670ms | 9.15% | 15054769 | 2000000 | 67.54% | 1.01 | 137.20 MB/s |
| Quicksort | 10000000 | 10459.4692ms | 10692.1182ms | 1.68% | 182952038 | 0 | 67.94% | 0.99 | 58.35 MB/s |
| Timsort | 10000000 | 7720.0103ms | 11387.7285ms | 14.64% | 202525310 | 0 | 67.26% | 1.00 | 79.06 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 2388.8874ms | 2487.3593ms | 1.60% | 192878458 | 10017407 | 66.80% | 1.01 | 255.50 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 1402.3584ms | 1452.6141ms | 1.63% | 110900253 | 0 | 67.65% | 0.99 | 435.23 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 1812.1835ms | 1866.3974ms | 1.55% | 110924473 | 0 | 67.51% | 0.98 | 336.80 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 694.4007ms | 700.2492ms | 0.46% | 34176169 | 0 | 67.65% | 1.00 | 878.96 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 947.8572ms | 954.2095ms | 0.40% | 40132484 | 0 | 67.81% | 1.00 | 643.93 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 708.4603ms | 718.5215ms | 0.89% | 41266054 | 0 | 67.65% | 1.00 | 861.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 9221.0887ms | 9431.6808ms | 2.13% | 212961497 | 20000000 | 67.38% | 1.00 | 66.19 MB/s |

### Distribution: Skewed

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.1873ms | 0.2100ms | 13.09% | 5530 | 0 | 66.67% | 0.99 | 325.81 MB/s |
| Timsort | 1000 | 0.0893ms | 0.2354ms | 23.52% | 6109 | 0 | 66.67% | 0.99 | 683.24 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.6806ms | 0.7084ms | 15.59% | 984 | 2000 | 66.67% | 0.99 | 89.67 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2620ms | 0.7890ms | 23.31% | 984 | 2000 | 66.67% | 0.99 | 232.93 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0527ms | 0.1852ms | 24.39% | 5530 | 0 | 66.67% | 0.99 | 1159.22 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0591ms | 0.1904ms | 38.03% | 5530 | 0 | 66.67% | 0.99 | 1033.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.1688ms | 0.1741ms | 9.64% | 5530 | 0 | 66.67% | 0.99 | 361.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1122ms | 0.2212ms | 24.96% | 6109 | 0 | 66.67% | 0.99 | 544.13 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0654ms | 0.1827ms | 24.85% | 5530 | 0 | 66.67% | 0.99 | 933.07 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0947ms | 0.2640ms | 22.74% | 6109 | 0 | 66.67% | 0.99 | 644.60 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0981ms | 0.1833ms | 17.00% | 5530 | 0 | 66.67% | 0.99 | 622.33 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.1708ms | 0.1865ms | 8.95% | 5530 | 0 | 66.67% | 0.99 | 357.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0704ms | 0.1838ms | 30.15% | 5530 | 0 | 66.67% | 0.99 | 866.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 1.7259ms | 1.9646ms | 11.60% | 5530 | 2000 | 66.67% | 0.99 | 35.36 MB/s |
| Quicksort | 10000 | 0.5772ms | 1.8512ms | 26.43% | 53207 | 0 | 66.67% | 0.99 | 1057.42 MB/s |
| Timsort | 10000 | 1.0829ms | 2.2353ms | 17.17% | 53257 | 0 | 66.67% | 0.99 | 563.62 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.8094ms | 9.0876ms | 5.24% | 9984 | 30000 | 66.67% | 0.99 | 69.28 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 4.8263ms | 9.2543ms | 17.54% | 9984 | 30000 | 66.67% | 0.99 | 126.46 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 10.4136ms | 11.2098ms | 4.20% | 122576 | 14351 | 66.67% | 0.99 | 58.61 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.4485ms | 3.8113ms | 23.69% | 12651 | 10000 | 66.67% | 0.99 | 421.36 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.6351ms | 1.4225ms | 32.71% | 12651 | 0 | 66.67% | 0.99 | 961.07 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.4118ms | 1.7393ms | 14.77% | 12634 | 0 | 66.67% | 0.99 | 432.31 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.3946ms | 2.0986ms | 19.25% | 9990 | 0 | 66.67% | 0.99 | 437.66 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 2.0359ms | 2.2838ms | 12.79% | 9990 | 0 | 66.67% | 0.99 | 299.79 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.6006ms | 1.5081ms | 30.16% | 12651 | 0 | 66.67% | 0.99 | 1016.22 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.4204ms | 1.6285ms | 31.21% | 12651 | 0 | 66.67% | 0.99 | 1451.88 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.1596ms | 1.7504ms | 18.57% | 12651 | 0 | 66.67% | 0.99 | 526.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.3672ms | 7.1398ms | 28.47% | 12651 | 20000 | 66.67% | 0.99 | 257.84 MB/s |
| Quicksort | 100000 | 9.3132ms | 11.0700ms | 9.26% | 516801 | 0 | 66.66% | 0.99 | 655.36 MB/s |
| Timsort | 100000 | 6.1824ms | 11.7453ms | 20.43% | 523232 | 0 | 66.66% | 0.99 | 987.24 MB/s |
| ARS Gen 1: Foundation | 100000 | 21.9511ms | 28.9393ms | 11.32% | 99984 | 300000 | 66.66% | 0.99 | 278.05 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 20.5622ms | 29.5739ms | 14.47% | 99984 | 300000 | 66.66% | 0.99 | 296.83 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 23.1499ms | 28.1463ms | 7.86% | 1144061 | 108703 | 66.67% | 0.99 | 263.65 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 8.8072ms | 11.6375ms | 17.78% | 99988 | 100000 | 66.67% | 0.99 | 693.01 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.0777ms | 4.4615ms | 29.86% | 99988 | 0 | 66.67% | 0.99 | 2937.69 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.5557ms | 5.6858ms | 29.91% | 99988 | 0 | 66.67% | 0.99 | 2388.16 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.1152ms | 6.3364ms | 20.66% | 99988 | 0 | 66.67% | 0.99 | 1959.29 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.2990ms | 5.4873ms | 24.83% | 99988 | 0 | 66.67% | 0.99 | 1850.11 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.6775ms | 6.8450ms | 27.23% | 199972 | 0 | 66.67% | 0.99 | 1659.69 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.5967ms | 6.6644ms | 34.72% | 199972 | 0 | 66.67% | 0.99 | 2350.51 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.1354ms | 4.8481ms | 22.84% | 99988 | 0 | 66.67% | 0.99 | 1946.66 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 18.2810ms | 22.6251ms | 10.30% | 99988 | 200000 | 66.66% | 0.99 | 333.87 MB/s |
| Quicksort | 1000000 | 235.8722ms | 246.1393ms | 1.53% | 19595153 | 0 | 66.64% | 0.99 | 258.76 MB/s |
| Timsort | 1000000 | 283.1610ms | 289.0772ms | 1.43% | 20426759 | 0 | 66.60% | 0.99 | 215.55 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 187.8968ms | 189.0718ms | 1.08% | 20672327 | 1017407 | 66.64% | 0.99 | 324.83 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 66.6956ms | 68.2424ms | 5.34% | 11334517 | 1000000 | 66.64% | 0.99 | 915.13 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 53.7062ms | 56.1661ms | 4.27% | 11334517 | 0 | 66.65% | 0.99 | 1136.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 69.2583ms | 70.8755ms | 4.89% | 11566000 | 0 | 66.63% | 0.99 | 881.27 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 64.3237ms | 70.7177ms | 3.96% | 12332487 | 0 | 66.64% | 0.99 | 948.87 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 86.3744ms | 93.5248ms | 4.37% | 12561492 | 0 | 66.63% | 0.99 | 706.63 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 53.5718ms | 55.6580ms | 1.79% | 8485673 | 0 | 66.66% | 0.99 | 1139.32 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 74.9319ms | 76.2913ms | 1.64% | 7026293 | 0 | 66.67% | 0.99 | 814.54 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 56.6619ms | 58.4973ms | 3.31% | 7792026 | 0 | 66.67% | 0.99 | 1077.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 404.5570ms | 522.3632ms | 9.46% | 15054769 | 2000000 | 66.64% | 0.99 | 150.87 MB/s |
| Quicksort | 10000000 | 8172.9717ms | 8275.7306ms | 0.75% | 182952038 | 0 | 66.61% | 0.97 | 74.68 MB/s |
| Timsort | 10000000 | 9302.1716ms | 9485.9308ms | 0.91% | 202525310 | 0 | 66.39% | 0.97 | 65.61 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 2446.9450ms | 2513.6336ms | 2.77% | 192878458 | 10017407 | 66.08% | 0.99 | 249.43 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 1449.1412ms | 1486.7548ms | 1.88% | 110900253 | 0 | 66.59% | 0.97 | 421.18 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 1996.7472ms | 2080.8613ms | 2.14% | 110924473 | 0 | 66.36% | 0.96 | 305.67 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 690.7192ms | 698.2328ms | 0.75% | 34176169 | 0 | 66.68% | 0.98 | 883.65 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 958.5635ms | 963.3743ms | 0.90% | 40132484 | 0 | 66.81% | 0.98 | 636.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 711.3707ms | 716.1834ms | 0.49% | 41266054 | 0 | 66.67% | 0.98 | 857.99 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 9271.2404ms | 9416.7391ms | 1.50% | 212961525 | 20000000 | 66.55% | 0.98 | 65.83 MB/s |

### Distribution: Clustered

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0723ms | 0.1862ms | 28.76% | 5530 | 0 | 66.09% | 0.98 | 844.66 MB/s |
| Timsort | 1000 | 0.0813ms | 0.2354ms | 22.05% | 6109 | 0 | 66.09% | 0.98 | 751.12 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.6847ms | 0.7088ms | 2.93% | 984 | 2000 | 66.09% | 0.98 | 89.14 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.1847ms | 0.7700ms | 24.99% | 984 | 2000 | 66.09% | 0.98 | 330.38 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0657ms | 0.1923ms | 31.77% | 5530 | 0 | 66.09% | 0.98 | 928.46 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1771ms | 0.2063ms | 10.33% | 5530 | 0 | 66.09% | 0.98 | 344.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0586ms | 0.1865ms | 25.34% | 5530 | 0 | 66.09% | 0.98 | 1041.13 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.2315ms | 0.2453ms | 15.22% | 6109 | 0 | 66.09% | 0.98 | 263.69 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0582ms | 0.1933ms | 24.60% | 5530 | 0 | 66.09% | 0.98 | 1048.43 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.1620ms | 0.2371ms | 14.18% | 6109 | 0 | 66.09% | 0.98 | 376.70 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.1811ms | 0.1997ms | 13.05% | 5530 | 0 | 66.09% | 0.98 | 337.07 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0631ms | 0.1971ms | 25.09% | 5530 | 0 | 66.09% | 0.98 | 967.31 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.1718ms | 0.2083ms | 8.25% | 5530 | 0 | 66.09% | 0.98 | 355.34 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 1.2684ms | 1.4151ms | 8.29% | 5530 | 2000 | 66.09% | 0.98 | 48.12 MB/s |
| Quicksort | 10000 | 1.8628ms | 1.8922ms | 7.22% | 53207 | 0 | 66.09% | 0.98 | 327.65 MB/s |
| Timsort | 10000 | 2.2483ms | 2.3629ms | 15.18% | 53257 | 0 | 66.09% | 0.98 | 271.47 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.4695ms | 9.0152ms | 9.20% | 9984 | 30000 | 66.09% | 0.98 | 72.06 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 8.7355ms | 9.1553ms | 6.44% | 9984 | 30000 | 66.09% | 0.98 | 69.87 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 4.0965ms | 10.7725ms | 27.43% | 122576 | 14351 | 66.09% | 0.98 | 148.99 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.4299ms | 4.0052ms | 22.33% | 12651 | 10000 | 66.09% | 0.98 | 426.85 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.5110ms | 1.4354ms | 33.99% | 12651 | 0 | 66.09% | 0.98 | 1194.37 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.0987ms | 1.5716ms | 25.13% | 12634 | 0 | 66.09% | 0.98 | 555.53 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.8452ms | 2.1646ms | 15.33% | 9990 | 0 | 66.09% | 0.98 | 330.78 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.2395ms | 2.3060ms | 18.05% | 9990 | 0 | 66.09% | 0.98 | 492.43 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.5945ms | 1.5955ms | 25.82% | 12651 | 0 | 66.09% | 0.98 | 1026.61 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.1167ms | 1.5669ms | 20.60% | 12651 | 0 | 66.09% | 0.98 | 546.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.1515ms | 1.6481ms | 13.70% | 12651 | 0 | 66.09% | 0.98 | 530.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 6.2257ms | 6.6342ms | 7.79% | 12651 | 20000 | 66.09% | 0.98 | 98.04 MB/s |
| Quicksort | 100000 | 7.2873ms | 11.8889ms | 14.16% | 516801 | 0 | 66.08% | 0.98 | 837.56 MB/s |
| Timsort | 100000 | 6.4693ms | 13.3710ms | 24.79% | 523232 | 0 | 66.08% | 0.98 | 943.47 MB/s |
| ARS Gen 1: Foundation | 100000 | 24.0002ms | 30.4151ms | 8.49% | 99984 | 300000 | 66.08% | 0.98 | 254.31 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 23.4434ms | 30.3941ms | 11.17% | 99984 | 300000 | 66.08% | 0.98 | 260.35 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 22.9333ms | 30.7946ms | 10.01% | 1144061 | 108703 | 66.09% | 0.98 | 266.14 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 9.4718ms | 11.3719ms | 11.85% | 99988 | 100000 | 66.09% | 0.98 | 644.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.3917ms | 4.9666ms | 20.54% | 99988 | 0 | 66.09% | 0.98 | 1799.55 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.1674ms | 4.8686ms | 27.12% | 99988 | 0 | 66.09% | 0.98 | 2816.09 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 2.8790ms | 6.0730ms | 38.15% | 99988 | 0 | 66.09% | 0.98 | 2120.01 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.5967ms | 4.9241ms | 22.02% | 99988 | 0 | 66.09% | 0.98 | 2350.48 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.1191ms | 5.5472ms | 20.12% | 199972 | 0 | 66.08% | 0.98 | 1481.76 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.7679ms | 6.9177ms | 12.58% | 199972 | 0 | 66.08% | 0.98 | 1058.19 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.4742ms | 5.3120ms | 21.94% | 99988 | 0 | 66.09% | 0.98 | 2466.88 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 19.9525ms | 21.5755ms | 7.43% | 99988 | 200000 | 66.08% | 0.98 | 305.90 MB/s |
| Quicksort | 1000000 | 359.5260ms | 378.3673ms | 3.04% | 19595153 | 0 | 66.06% | 0.98 | 169.77 MB/s |
| Timsort | 1000000 | 473.3401ms | 487.1770ms | 1.98% | 20426759 | 0 | 66.03% | 0.98 | 128.95 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 219.1922ms | 223.1116ms | 1.00% | 20672327 | 1017407 | 66.05% | 0.98 | 278.46 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 66.9710ms | 69.0195ms | 4.34% | 11334517 | 1000000 | 66.07% | 0.98 | 911.37 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 60.6341ms | 61.2394ms | 2.64% | 11334517 | 0 | 66.06% | 0.98 | 1006.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 75.7120ms | 77.2816ms | 3.00% | 11566000 | 0 | 66.05% | 0.98 | 806.15 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 74.3665ms | 75.9912ms | 4.31% | 12332487 | 0 | 66.06% | 0.98 | 820.73 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 97.6451ms | 102.1251ms | 3.36% | 12561492 | 0 | 66.05% | 0.98 | 625.07 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 57.3382ms | 60.3253ms | 3.67% | 8485673 | 0 | 66.07% | 0.98 | 1064.48 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 76.2776ms | 78.9657ms | 4.95% | 7026293 | 0 | 66.09% | 0.98 | 800.17 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 63.4058ms | 65.1728ms | 1.48% | 7792026 | 0 | 66.08% | 0.98 | 962.61 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 398.8339ms | 516.6497ms | 9.81% | 15103346 | 2000000 | 66.06% | 0.98 | 153.03 MB/s |
| Quicksort | 10000000 | 8627.0957ms | 8685.6202ms | 0.50% | 182952038 | 0 | 66.03% | 0.97 | 70.75 MB/s |
| Timsort | 10000000 | 9985.3769ms | 10109.8051ms | 0.57% | 202525310 | 0 | 65.84% | 0.96 | 61.12 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 2480.6259ms | 2497.2503ms | 0.93% | 192878458 | 10017407 | 65.54% | 0.98 | 246.05 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 1447.0176ms | 1522.5582ms | 1.99% | 110900253 | 0 | 66.03% | 0.96 | 421.80 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 2033.6341ms | 2108.1593ms | 2.98% | 110924473 | 0 | 65.82% | 0.95 | 300.13 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 699.7394ms | 703.7446ms | 0.39% | 34176169 | 0 | 66.11% | 0.97 | 872.26 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 968.0068ms | 974.8824ms | 0.42% | 40132484 | 0 | 66.23% | 0.97 | 630.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 724.6462ms | 729.4506ms | 0.54% | 41266054 | 0 | 66.09% | 0.97 | 842.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 9179.0433ms | 9332.9384ms | 1.14% | 212961489 | 20000000 | 66.04% | 0.97 | 66.49 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.1795ms | 0.1957ms | 9.77% | 5530 | 0 | 65.70% | 0.96 | 339.97 MB/s |
| Timsort | 1000 | 0.0742ms | 0.2395ms | 31.59% | 6109 | 0 | 65.70% | 0.96 | 822.75 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.6679ms | 0.7236ms | 4.13% | 984 | 2000 | 65.70% | 0.96 | 91.39 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2629ms | 0.7770ms | 26.77% | 984 | 2000 | 65.70% | 0.96 | 232.15 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0584ms | 0.1776ms | 22.67% | 5530 | 0 | 65.70% | 0.96 | 1044.68 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1278ms | 0.1817ms | 12.90% | 5530 | 0 | 65.70% | 0.96 | 477.58 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0839ms | 0.1814ms | 21.13% | 5530 | 0 | 65.70% | 0.96 | 727.80 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1741ms | 0.2509ms | 11.46% | 6109 | 0 | 65.70% | 0.96 | 350.65 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0509ms | 0.1767ms | 25.22% | 5530 | 0 | 65.70% | 0.96 | 1200.01 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.2213ms | 0.2363ms | 8.73% | 6109 | 0 | 65.70% | 0.96 | 275.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0616ms | 0.1899ms | 23.97% | 5530 | 0 | 65.70% | 0.96 | 990.54 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0482ms | 0.1894ms | 27.57% | 5530 | 0 | 65.70% | 0.96 | 1266.55 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0518ms | 0.1693ms | 25.00% | 5530 | 0 | 65.70% | 0.96 | 1177.76 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 1.4035ms | 1.8772ms | 8.63% | 5530 | 2000 | 65.70% | 0.96 | 43.49 MB/s |
| Quicksort | 10000 | 0.5600ms | 1.7376ms | 22.03% | 53207 | 0 | 65.70% | 0.96 | 1089.99 MB/s |
| Timsort | 10000 | 1.6314ms | 2.1340ms | 8.12% | 53257 | 0 | 65.70% | 0.96 | 374.12 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.4816ms | 9.1118ms | 29.63% | 9984 | 30000 | 65.70% | 0.96 | 245.95 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.5480ms | 9.0642ms | 30.46% | 9984 | 30000 | 65.70% | 0.96 | 239.54 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 3.7399ms | 11.1028ms | 22.38% | 122576 | 14351 | 65.70% | 0.96 | 163.20 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.1225ms | 3.8773ms | 23.72% | 12651 | 10000 | 65.70% | 0.96 | 543.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.2309ms | 1.4329ms | 12.77% | 12651 | 0 | 65.70% | 0.96 | 495.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.6733ms | 1.3395ms | 28.79% | 12634 | 0 | 65.70% | 0.96 | 906.52 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6965ms | 1.9824ms | 33.90% | 9990 | 0 | 65.70% | 0.96 | 876.27 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.9418ms | 2.0149ms | 24.08% | 9990 | 0 | 65.70% | 0.96 | 648.06 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3847ms | 1.6724ms | 36.10% | 12651 | 0 | 65.70% | 0.96 | 1586.44 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.1280ms | 1.5428ms | 14.74% | 12651 | 0 | 65.70% | 0.96 | 541.08 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.5077ms | 1.3644ms | 44.16% | 12651 | 0 | 65.70% | 0.96 | 1202.16 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 6.2090ms | 6.6938ms | 8.16% | 12651 | 20000 | 65.70% | 0.96 | 98.30 MB/s |
| Quicksort | 100000 | 5.4135ms | 11.8294ms | 25.32% | 516801 | 0 | 65.70% | 0.96 | 1127.46 MB/s |
| Timsort | 100000 | 5.8885ms | 14.0236ms | 26.65% | 523232 | 0 | 65.69% | 0.96 | 1036.52 MB/s |
| ARS Gen 1: Foundation | 100000 | 27.7183ms | 30.3402ms | 5.36% | 99984 | 300000 | 65.70% | 0.96 | 220.20 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 22.0132ms | 30.4143ms | 10.97% | 99984 | 300000 | 65.70% | 0.96 | 277.27 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 23.8529ms | 29.9766ms | 11.13% | 1144061 | 108703 | 65.70% | 0.96 | 255.88 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.6711ms | 11.0527ms | 28.73% | 99988 | 100000 | 65.70% | 0.96 | 1306.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.8436ms | 5.4727ms | 19.85% | 99988 | 0 | 65.70% | 0.96 | 1587.97 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 3.2472ms | 4.4356ms | 20.91% | 99988 | 0 | 65.70% | 0.96 | 1879.64 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.7652ms | 5.7053ms | 22.05% | 99988 | 0 | 65.70% | 0.96 | 1621.02 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.0757ms | 4.3376ms | 42.04% | 99988 | 0 | 65.70% | 0.96 | 2940.47 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.7549ms | 6.5098ms | 24.49% | 199972 | 0 | 65.70% | 0.96 | 1625.46 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.3356ms | 8.2863ms | 29.79% | 199972 | 0 | 65.70% | 0.96 | 1829.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.6588ms | 5.2502ms | 26.07% | 99988 | 0 | 65.70% | 0.96 | 2295.56 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 20.0938ms | 22.9541ms | 7.15% | 99988 | 200000 | 65.69% | 0.96 | 303.75 MB/s |
| Quicksort | 1000000 | 400.2695ms | 402.5870ms | 0.74% | 19595153 | 0 | 65.68% | 0.96 | 152.49 MB/s |
| Timsort | 1000000 | 500.1964ms | 506.0694ms | 0.79% | 20426759 | 0 | 65.65% | 0.96 | 122.02 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 205.2667ms | 207.0363ms | 2.46% | 20672327 | 1017407 | 65.67% | 0.96 | 297.35 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 66.0150ms | 67.1645ms | 1.18% | 11334517 | 1000000 | 65.68% | 0.96 | 924.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 53.7047ms | 57.5133ms | 7.87% | 11334517 | 0 | 65.69% | 0.96 | 1136.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 69.1378ms | 70.4251ms | 1.78% | 11566000 | 0 | 65.67% | 0.96 | 882.80 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 66.4808ms | 69.6529ms | 6.10% | 12332487 | 0 | 65.68% | 0.96 | 918.09 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 89.9933ms | 92.8179ms | 2.31% | 12561492 | 0 | 65.67% | 0.96 | 678.22 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 52.6950ms | 54.1658ms | 3.28% | 8485673 | 0 | 65.69% | 0.96 | 1158.27 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 73.4656ms | 75.5031ms | 2.82% | 7026293 | 0 | 65.70% | 0.96 | 830.80 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 57.5854ms | 58.7007ms | 2.00% | 7792026 | 0 | 65.70% | 0.96 | 1059.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 408.9730ms | 517.4494ms | 8.70% | 15103346 | 2000000 | 65.68% | 0.96 | 149.24 MB/s |
| Quicksort | 10000000 | 8698.8494ms | 8763.6957ms | 0.62% | 182952038 | 0 | 65.66% | 0.95 | 70.16 MB/s |
| Timsort | 10000000 | 10025.9619ms | 10092.4450ms | 0.54% | 202525310 | 0 | 65.50% | 0.94 | 60.88 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 2350.7372ms | 2403.8243ms | 1.03% | 192878458 | 10017407 | 65.25% | 0.96 | 259.64 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 1427.5460ms | 1475.6178ms | 4.37% | 110900253 | 0 | 65.69% | 0.95 | 427.55 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 2026.7219ms | 2073.4081ms | 2.33% | 110924473 | 0 | 65.49% | 0.93 | 301.15 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 693.4469ms | 702.3324ms | 0.76% | 34176169 | 0 | 65.73% | 0.95 | 880.17 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 957.2952ms | 967.1145ms | 1.14% | 40132484 | 0 | 65.83% | 0.95 | 637.58 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 715.3453ms | 723.5131ms | 0.49% | 41266054 | 0 | 65.72% | 0.95 | 853.23 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 9316.7632ms | 9460.3076ms | 1.64% | 212961542 | 20000000 | 65.68% | 0.95 | 65.51 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0778ms | 0.2084ms | 23.44% | 5532 | 0 | 65.38% | 0.95 | 784.62 MB/s |
| Timsort | 1000 | 0.2333ms | 0.2700ms | 8.42% | 5597 | 0 | 65.38% | 0.95 | 261.66 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1931ms | 0.6984ms | 23.81% | 984 | 2000 | 65.38% | 0.95 | 316.12 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.4255ms | 0.7676ms | 15.02% | 984 | 2000 | 65.38% | 0.95 | 143.43 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.1729ms | 0.1914ms | 12.50% | 5532 | 0 | 65.38% | 0.95 | 352.96 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0761ms | 0.1818ms | 19.71% | 5532 | 0 | 65.38% | 0.95 | 801.56 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0665ms | 0.1769ms | 23.46% | 5532 | 0 | 65.38% | 0.95 | 917.48 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1118ms | 0.2315ms | 17.98% | 5597 | 0 | 65.38% | 0.95 | 546.04 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.1784ms | 0.2008ms | 23.24% | 5532 | 0 | 65.38% | 0.95 | 342.18 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0990ms | 0.2337ms | 21.56% | 5597 | 0 | 65.38% | 0.95 | 616.55 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0586ms | 0.1847ms | 29.72% | 5532 | 0 | 65.38% | 0.95 | 1041.41 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.1714ms | 0.1786ms | 7.82% | 5532 | 0 | 65.38% | 0.95 | 356.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.1517ms | 0.1770ms | 9.58% | 5532 | 0 | 65.38% | 0.95 | 402.29 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5703ms | 1.6884ms | 22.43% | 5532 | 2000 | 65.38% | 0.95 | 107.03 MB/s |
| Quicksort | 10000 | 1.0267ms | 1.9004ms | 15.41% | 54031 | 0 | 65.38% | 0.95 | 594.49 MB/s |
| Timsort | 10000 | 1.5130ms | 2.3492ms | 12.37% | 54467 | 0 | 65.38% | 0.95 | 403.39 MB/s |
| ARS Gen 1: Foundation | 10000 | 8.4592ms | 9.1009ms | 6.93% | 9984 | 30000 | 65.38% | 0.95 | 72.15 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 3.0911ms | 9.8478ms | 24.56% | 9984 | 30000 | 65.38% | 0.95 | 197.46 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 3.7287ms | 10.8160ms | 21.24% | 122654 | 14351 | 65.38% | 0.95 | 163.69 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.8290ms | 4.0743ms | 18.42% | 18115 | 10000 | 65.38% | 0.95 | 333.70 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.6412ms | 1.4779ms | 22.49% | 18115 | 0 | 65.38% | 0.95 | 951.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.4421ms | 1.7111ms | 28.41% | 18210 | 0 | 65.38% | 0.95 | 1380.70 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.0564ms | 1.9030ms | 21.81% | 12047 | 0 | 65.38% | 0.95 | 577.79 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.5603ms | 2.0066ms | 24.50% | 12077 | 0 | 65.38% | 0.95 | 1089.41 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.4898ms | 1.6524ms | 34.81% | 18115 | 0 | 65.38% | 0.95 | 1246.22 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.1753ms | 1.5358ms | 13.50% | 18115 | 0 | 65.38% | 0.95 | 519.33 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.1672ms | 1.7859ms | 18.57% | 18115 | 0 | 65.38% | 0.95 | 522.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.8063ms | 6.3350ms | 23.29% | 18115 | 20000 | 65.38% | 0.95 | 217.49 MB/s |
| Quicksort | 100000 | 4.9868ms | 11.7093ms | 23.86% | 523301 | 0 | 65.38% | 0.95 | 1223.93 MB/s |
| Timsort | 100000 | 7.0779ms | 13.9979ms | 22.29% | 523890 | 0 | 65.37% | 0.95 | 862.34 MB/s |
| ARS Gen 1: Foundation | 100000 | 20.7196ms | 30.1247ms | 12.12% | 99984 | 300000 | 65.38% | 0.95 | 294.58 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 26.4483ms | 31.2370ms | 7.97% | 99984 | 300000 | 65.38% | 0.95 | 230.77 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 24.3823ms | 29.6860ms | 10.61% | 1146595 | 108703 | 65.38% | 0.95 | 250.33 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.7757ms | 10.5180ms | 26.83% | 150813 | 100000 | 65.38% | 0.95 | 1278.02 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 3.3495ms | 4.8249ms | 28.60% | 150813 | 0 | 65.38% | 0.95 | 1822.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.0654ms | 5.4473ms | 24.73% | 150914 | 0 | 65.38% | 0.95 | 2955.14 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.7624ms | 5.8828ms | 26.57% | 125502 | 0 | 65.38% | 0.95 | 3463.18 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.9689ms | 5.4414ms | 17.42% | 125610 | 0 | 65.38% | 0.95 | 1537.82 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.6337ms | 6.3262ms | 14.45% | 199984 | 0 | 65.38% | 0.95 | 1317.21 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 6.2893ms | 7.1731ms | 18.19% | 199980 | 0 | 65.38% | 0.95 | 970.45 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.8701ms | 4.5334ms | 35.20% | 100000 | 0 | 65.38% | 0.95 | 3263.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 16.2742ms | 21.6472ms | 8.67% | 150813 | 200000 | 65.37% | 0.95 | 375.04 MB/s |
| Quicksort | 1000000 | 93.7626ms | 96.5197ms | 1.45% | 5137660 | 0 | 65.39% | 0.95 | 650.95 MB/s |
| Timsort | 1000000 | 163.2642ms | 168.6772ms | 1.31% | 6204570 | 0 | 65.39% | 0.95 | 373.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 163.4228ms | 172.6982ms | 2.60% | 12089575 | 1017407 | 65.37% | 0.95 | 373.48 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 27.1667ms | 36.8252ms | 15.21% | 1189071 | 1000000 | 65.38% | 0.95 | 2246.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.7673ms | 28.7190ms | 24.80% | 1189071 | 0 | 65.38% | 0.95 | 3252.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.0852ms | 27.0533ms | 24.66% | 1189803 | 0 | 65.38% | 0.95 | 3198.03 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 23.1843ms | 31.2367ms | 19.71% | 1189071 | 0 | 65.38% | 0.95 | 2632.60 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 23.5879ms | 30.7212ms | 24.45% | 1189803 | 0 | 65.38% | 0.95 | 2587.56 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 31.1563ms | 36.0841ms | 11.58% | 1999984 | 0 | 65.38% | 0.95 | 1959.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 35.4186ms | 39.5926ms | 16.30% | 1999994 | 0 | 65.38% | 0.95 | 1723.25 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 33.4291ms | 37.7068ms | 11.09% | 1999980 | 0 | 65.38% | 0.95 | 1825.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 313.5217ms | 320.1154ms | 1.53% | 6332736 | 2000000 | 65.40% | 0.95 | 194.68 MB/s |
| Quicksort | 10000000 | 1057.2329ms | 1069.4347ms | 0.91% | 52542717 | 0 | 65.56% | 0.95 | 577.31 MB/s |
| Timsort | 10000000 | 2175.9684ms | 2214.4015ms | 0.86% | 66326091 | 0 | 65.67% | 0.95 | 280.50 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 1671.0522ms | 1806.5806ms | 3.79% | 120087685 | 10017407 | 65.36% | 0.96 | 365.25 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 280.8651ms | 351.2368ms | 11.11% | 9999988 | 0 | 65.41% | 0.95 | 2173.11 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 275.6273ms | 460.8399ms | 23.33% | 9999988 | 0 | 65.41% | 0.95 | 2214.41 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 403.2614ms | 428.6474ms | 3.69% | 19999972 | 0 | 65.44% | 0.95 | 1513.54 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 446.4470ms | 505.3074ms | 6.15% | 19999982 | 0 | 65.44% | 0.95 | 1367.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 418.8402ms | 438.4577ms | 4.58% | 19999972 | 0 | 65.45% | 0.95 | 1457.24 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 5032.7569ms | 5107.6414ms | 1.34% | 116948525 | 20000000 | 66.05% | 0.95 | 121.28 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.1854ms | 0.1967ms | 10.09% | 5563 | 0 | 66.74% | 0.94 | 329.21 MB/s |
| Timsort | 1000 | 0.2387ms | 0.2486ms | 4.83% | 5867 | 0 | 66.74% | 0.94 | 255.70 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2123ms | 0.7476ms | 25.43% | 5563 | 2000 | 66.74% | 0.94 | 287.54 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.3388ms | 0.7697ms | 18.13% | 5563 | 2000 | 66.74% | 0.94 | 180.17 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.1863ms | 0.2015ms | 8.83% | 5563 | 0 | 66.74% | 0.94 | 327.62 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.1866ms | 0.1989ms | 5.52% | 5563 | 0 | 66.74% | 0.94 | 327.08 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.1840ms | 0.2005ms | 8.43% | 5563 | 0 | 66.74% | 0.94 | 331.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0817ms | 0.2801ms | 23.97% | 5867 | 0 | 66.74% | 0.94 | 747.06 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0577ms | 0.2014ms | 28.79% | 5563 | 0 | 66.74% | 0.94 | 1058.55 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.2425ms | 0.2595ms | 16.08% | 5867 | 0 | 66.74% | 0.94 | 251.72 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.1884ms | 0.2053ms | 20.23% | 5563 | 0 | 66.74% | 0.94 | 323.96 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0696ms | 0.2103ms | 24.05% | 5563 | 0 | 66.74% | 0.94 | 876.77 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0689ms | 0.2172ms | 28.83% | 5563 | 0 | 66.74% | 0.94 | 885.90 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 1.3380ms | 1.5014ms | 26.95% | 5563 | 2000 | 66.74% | 0.94 | 45.62 MB/s |
| Quicksort | 10000 | 2.0250ms | 2.1538ms | 9.64% | 54514 | 0 | 66.73% | 0.94 | 301.41 MB/s |
| Timsort | 10000 | 1.2402ms | 2.6961ms | 17.41% | 53540 | 0 | 66.73% | 0.94 | 492.12 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.6989ms | 11.1250ms | 20.62% | 54514 | 30000 | 66.73% | 0.95 | 129.89 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 10.7853ms | 11.5954ms | 11.30% | 54514 | 30000 | 66.73% | 0.95 | 56.59 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 4.9988ms | 13.3084ms | 26.46% | 122405 | 14351 | 66.73% | 0.94 | 122.10 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 2.1652ms | 6.4055ms | 30.82% | 54518 | 10000 | 66.73% | 0.94 | 281.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.1601ms | 3.5814ms | 24.34% | 54518 | 0 | 66.73% | 0.94 | 526.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 3.0802ms | 3.9809ms | 14.56% | 53544 | 0 | 66.73% | 0.94 | 198.15 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.0907ms | 3.5100ms | 23.80% | 54518 | 0 | 66.73% | 0.94 | 559.60 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 4.1260ms | 4.3463ms | 7.25% | 53544 | 0 | 66.73% | 0.94 | 147.93 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 2.6355ms | 2.7133ms | 6.24% | 54518 | 0 | 66.73% | 0.94 | 231.59 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9029ms | 3.3792ms | 26.74% | 54518 | 0 | 66.73% | 0.94 | 676.01 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 3.3285ms | 3.6610ms | 6.87% | 54518 | 0 | 66.73% | 0.94 | 183.37 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 8.9658ms | 9.4482ms | 11.04% | 54518 | 20000 | 66.73% | 0.94 | 68.08 MB/s |
| Quicksort | 100000 | 12.5987ms | 14.3080ms | 10.25% | 516569 | 0 | 66.73% | 0.94 | 484.46 MB/s |
| Timsort | 100000 | 16.4158ms | 18.1147ms | 6.82% | 517546 | 0 | 66.73% | 0.94 | 371.81 MB/s |
| ARS Gen 1: Foundation | 100000 | 41.6940ms | 43.1006ms | 2.76% | 516569 | 300000 | 66.73% | 0.95 | 146.39 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 40.9730ms | 43.3334ms | 2.47% | 516569 | 300000 | 66.73% | 0.95 | 148.96 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 27.5824ms | 30.9123ms | 8.39% | 1144035 | 108703 | 66.73% | 0.95 | 221.28 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 15.0536ms | 20.9012ms | 11.38% | 516575 | 100000 | 66.73% | 0.95 | 405.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 12.1723ms | 18.4550ms | 17.30% | 516575 | 0 | 66.73% | 0.94 | 501.43 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 18.0919ms | 21.4968ms | 10.59% | 517552 | 0 | 66.73% | 0.94 | 337.36 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 11.7359ms | 17.3243ms | 14.85% | 516575 | 0 | 66.73% | 0.95 | 520.07 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 15.1492ms | 21.6243ms | 10.74% | 517552 | 0 | 66.73% | 0.94 | 402.89 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 10.9958ms | 13.3864ms | 19.93% | 516575 | 0 | 66.73% | 0.94 | 555.08 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 10.5165ms | 13.5105ms | 19.03% | 516575 | 0 | 66.73% | 0.94 | 580.37 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 11.9823ms | 14.8720ms | 10.82% | 516575 | 0 | 66.73% | 0.94 | 509.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 33.3618ms | 35.3385ms | 3.18% | 516575 | 200000 | 66.73% | 0.95 | 182.95 MB/s |
| Quicksort | 1000000 | 608.8727ms | 615.8170ms | 0.94% | 19583931 | 0 | 66.72% | 0.94 | 100.24 MB/s |
| Timsort | 1000000 | 793.0756ms | 803.4860ms | 0.62% | 20441738 | 0 | 66.70% | 0.94 | 76.96 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 272.8273ms | 279.6950ms | 1.15% | 20678990 | 1017407 | 66.70% | 0.95 | 223.71 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 661.3337ms | 674.2299ms | 0.74% | 19583935 | 1000000 | 66.72% | 0.94 | 92.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 625.3726ms | 637.9317ms | 0.99% | 19583935 | 0 | 66.72% | 0.94 | 97.60 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 809.0191ms | 819.2498ms | 0.90% | 20441742 | 0 | 66.70% | 0.94 | 75.44 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 636.1503ms | 642.8608ms | 0.70% | 19583935 | 0 | 66.72% | 0.94 | 95.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 803.1746ms | 828.1970ms | 1.27% | 20441742 | 0 | 66.70% | 0.94 | 75.99 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 616.5716ms | 627.1980ms | 1.40% | 19583935 | 0 | 66.72% | 0.94 | 98.99 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 619.7281ms | 626.2806ms | 0.99% | 19583935 | 0 | 66.72% | 0.94 | 98.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 612.6201ms | 623.6134ms | 1.08% | 19583935 | 0 | 66.72% | 0.94 | 99.63 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 710.5067ms | 715.8170ms | 0.86% | 21805929 | 2000000 | 66.76% | 0.94 | 85.90 MB/s |
| Quicksort | 10000000 | 10532.8090ms | 10996.4165ms | 3.79% | 182931219 | 0 | 67.15% | 0.93 | 57.95 MB/s |
| Timsort | 10000000 | 12546.0734ms | 12666.1776ms | 0.67% | 202590997 | 0 | 66.91% | 0.92 | 48.65 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 3248.3300ms | 3365.7587ms | 1.84% | 192901248 | 10017407 | 66.54% | 0.93 | 187.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | **OOM** | **OOM** | - | - | - | - | - | - |
| ARS Gen 6: Aero Architecture | 10000000 | 11380.7352ms | 11526.5502ms | 0.67% | 182931225 | 0 | 67.21% | 0.92 | 53.63 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 13370.9988ms | 13661.9696ms | 0.84% | 202591003 | 0 | 66.95% | 0.92 | 45.65 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 10916.4657ms | 11196.8293ms | 1.49% | 182931225 | 0 | 67.17% | 0.93 | 55.91 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 11195.1166ms | 11331.4841ms | 0.73% | 182931225 | 0 | 67.17% | 0.93 | 54.52 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 11139.7398ms | 11245.5121ms | 0.66% | 182931225 | 0 | 67.16% | 0.92 | 54.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 12007.4928ms | 12090.9848ms | 0.72% | 281474124 | 20000000 | 67.36% | 0.93 | 50.83 MB/s |

## Category: Custom

### Distribution: Random

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0431ms | 0.0973ms | 28.16% | 10385 | 0 | 67.95% | 0.91 | 1061.80 MB/s |
| Timsort | 1000 | 0.0488ms | 0.1940ms | 26.40% | 10594 | 0 | 67.95% | 0.91 | 937.10 MB/s |
| ARS Gen 1: Foundation | 1000 | 1.1965ms | 1.2517ms | 5.68% | 0 | 2000 | 67.95% | 0.91 | 38.26 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.6472ms | 1.4723ms | 19.14% | 0 | 2000 | 67.95% | 0.91 | 70.73 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0323ms | 0.0964ms | 23.95% | 10385 | 0 | 67.95% | 0.91 | 1415.34 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0765ms | 0.0963ms | 13.51% | 10385 | 0 | 67.95% | 0.91 | 598.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0955ms | 0.0972ms | 9.83% | 10385 | 0 | 67.95% | 0.91 | 479.26 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1524ms | 0.1847ms | 15.19% | 10594 | 0 | 67.95% | 0.91 | 300.32 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0426ms | 0.0986ms | 21.13% | 10385 | 0 | 67.95% | 0.91 | 1075.55 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0820ms | 0.1739ms | 28.02% | 10594 | 0 | 67.95% | 0.91 | 558.17 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0944ms | 0.0967ms | 11.41% | 10385 | 0 | 67.95% | 0.91 | 484.99 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0322ms | 0.0963ms | 29.49% | 10385 | 0 | 67.95% | 0.91 | 1423.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0950ms | 0.0965ms | 18.03% | 10385 | 0 | 67.95% | 0.91 | 481.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5740ms | 1.4412ms | 21.59% | 10385 | 2000 | 67.95% | 0.91 | 79.76 MB/s |
| Quicksort | 10000 | 1.1742ms | 1.2737ms | 6.18% | 137508 | 0 | 67.95% | 0.91 | 389.86 MB/s |
| Timsort | 10000 | 1.9349ms | 1.9943ms | 15.68% | 142586 | 0 | 67.95% | 0.91 | 236.58 MB/s |
| ARS Gen 1: Foundation | 10000 | 11.3186ms | 26.7985ms | 25.98% | 0 | 30000 | 67.95% | 0.91 | 40.44 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 27.8623ms | 29.1348ms | 2.72% | 0 | 30000 | 67.95% | 0.91 | 16.43 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.7822ms | 2.8993ms | 13.03% | 193983 | 14351 | 67.95% | 0.91 | 164.53 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.6898ms | 1.9303ms | 9.94% | 53118 | 10000 | 67.95% | 0.91 | 270.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3809ms | 1.3750ms | 29.50% | 53118 | 0 | 67.95% | 0.91 | 1201.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.1214ms | 1.4685ms | 17.61% | 58251 | 0 | 67.95% | 0.91 | 408.19 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.4925ms | 1.9622ms | 26.66% | 60899 | 0 | 67.95% | 0.91 | 929.39 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.3804ms | 2.1245ms | 28.06% | 63941 | 0 | 67.95% | 0.91 | 331.61 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.2124ms | 1.3253ms | 17.71% | 53118 | 0 | 67.95% | 0.91 | 377.58 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.4059ms | 1.4942ms | 37.23% | 53118 | 0 | 67.95% | 0.91 | 1127.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.1290ms | 1.3218ms | 16.46% | 53118 | 0 | 67.95% | 0.91 | 405.46 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.0953ms | 3.1879ms | 22.36% | 53118 | 20000 | 67.95% | 0.91 | 417.93 MB/s |
| Quicksort | 100000 | 4.7356ms | 14.7401ms | 22.78% | 1712659 | 0 | 67.95% | 0.91 | 966.64 MB/s |
| Timsort | 100000 | 7.6081ms | 23.1872ms | 31.17% | 1755445 | 0 | 67.94% | 0.91 | 601.68 MB/s |
| ARS Gen 1: Foundation | 100000 | 50.1657ms | 68.4448ms | 8.53% | 0 | 300000 | 67.89% | 0.91 | 91.25 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 44.3925ms | 68.1824ms | 14.48% | 0 | 300000 | 67.88% | 0.91 | 103.12 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 17.7208ms | 19.0191ms | 3.35% | 1896422 | 108703 | 67.94% | 0.91 | 258.32 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 6.4491ms | 7.4082ms | 7.09% | 891444 | 100000 | 67.95% | 0.91 | 709.81 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.8237ms | 5.6548ms | 23.44% | 891444 | 0 | 67.95% | 0.91 | 2510.08 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 5.7266ms | 6.1952ms | 14.06% | 929862 | 0 | 67.95% | 0.91 | 799.37 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 6.1864ms | 6.6642ms | 8.83% | 952336 | 0 | 67.94% | 0.91 | 739.95 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.5994ms | 7.7028ms | 22.46% | 989613 | 0 | 67.94% | 0.91 | 1761.03 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 5.4847ms | 6.0106ms | 12.98% | 891444 | 0 | 67.94% | 0.91 | 834.63 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.1646ms | 6.3479ms | 33.68% | 781111 | 0 | 67.95% | 0.91 | 2114.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 1.7434ms | 6.7442ms | 25.32% | 891444 | 0 | 67.95% | 0.91 | 2625.74 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 4.8839ms | 10.4415ms | 18.55% | 891444 | 200000 | 67.94% | 0.91 | 937.29 MB/s |
| Quicksort | 1000000 | 46.7492ms | 48.5257ms | 3.14% | 20511100 | 0 | 67.93% | 0.91 | 979.19 MB/s |
| Timsort | 1000000 | 76.7177ms | 78.7028ms | 2.49% | 20889747 | 0 | 67.92% | 0.91 | 596.69 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 45.9638ms | 49.0953ms | 5.15% | 21592913 | 1017407 | 67.94% | 0.91 | 995.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 24.2319ms | 25.9082ms | 3.69% | 10312686 | 1000000 | 67.95% | 0.91 | 1889.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.8540ms | 23.7883ms | 3.40% | 10312686 | 0 | 67.95% | 0.91 | 2002.99 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 23.8703ms | 25.8886ms | 3.51% | 10715019 | 0 | 67.94% | 0.91 | 1917.71 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 22.5701ms | 23.5577ms | 5.89% | 13219998 | 0 | 67.94% | 0.91 | 2028.19 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 24.9670ms | 26.8954ms | 6.75% | 13636241 | 0 | 67.94% | 0.91 | 1833.47 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 25.0971ms | 26.3037ms | 30.18% | 10312686 | 0 | 67.94% | 0.91 | 1823.97 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.5026ms | 26.6723ms | 6.40% | 11365129 | 0 | 67.95% | 0.91 | 1794.97 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 26.5220ms | 27.0002ms | 3.60% | 12417106 | 0 | 67.95% | 0.91 | 1725.98 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 46.8886ms | 48.3281ms | 3.45% | 12264258 | 2000000 | 67.95% | 0.91 | 976.28 MB/s |
| Quicksort | 10000000 | 598.8228ms | 602.9982ms | 0.51% | 238498488 | 0 | 67.83% | 0.92 | 764.44 MB/s |
| Timsort | 10000000 | 1077.2993ms | 1082.8114ms | 0.72% | 242343619 | 0 | 67.69% | 0.92 | 424.92 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 467.2895ms | 558.2142ms | 35.96% | 248249406 | 10017407 | 67.83% | 0.92 | 979.61 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 252.2013ms | 257.2584ms | 11.43% | 137656336 | 10000000 | 67.91% | 0.91 | 1815.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 230.6349ms | 235.1050ms | 1.67% | 137656336 | 0 | 67.91% | 0.91 | 1984.80 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 254.0673ms | 257.3957ms | 1.18% | 141707686 | 0 | 67.83% | 0.91 | 1801.74 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 227.9785ms | 233.8170ms | 13.05% | 164725544 | 0 | 67.88% | 0.91 | 2007.92 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 293.1168ms | 355.9825ms | 7.43% | 168735751 | 0 | 67.79% | 0.91 | 1561.71 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 268.4299ms | 271.3091ms | 1.38% | 46888036 | 0 | 67.86% | 0.91 | 1705.34 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 308.4879ms | 314.1280ms | 1.08% | 52539700 | 0 | 67.93% | 0.91 | 1483.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 274.8699ms | 279.6552ms | 1.81% | 52577969 | 0 | 67.89% | 0.91 | 1665.38 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 761.3938ms | 797.8471ms | 3.79% | 162224971 | 20000000 | 67.97% | 0.91 | 601.22 MB/s |

### Distribution: Gaussian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0387ms | 0.0955ms | 19.22% | 10072 | 0 | 67.96% | 0.91 | 1182.18 MB/s |
| Timsort | 1000 | 0.0698ms | 0.1923ms | 22.32% | 10668 | 0 | 67.96% | 0.91 | 655.67 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.3246ms | 1.0047ms | 32.53% | 415 | 2000 | 67.96% | 0.91 | 141.03 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.7858ms | 1.0784ms | 11.01% | 415 | 2000 | 67.96% | 0.91 | 58.26 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0954ms | 0.0982ms | 21.41% | 10072 | 0 | 67.96% | 0.91 | 480.00 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0955ms | 0.0971ms | 11.66% | 10072 | 0 | 67.96% | 0.91 | 479.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0280ms | 0.0973ms | 24.37% | 10072 | 0 | 67.96% | 0.91 | 1636.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1704ms | 0.1875ms | 23.88% | 10668 | 0 | 67.96% | 0.91 | 268.66 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0691ms | 0.0982ms | 22.49% | 10072 | 0 | 67.96% | 0.91 | 662.93 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0813ms | 0.1805ms | 19.01% | 10668 | 0 | 67.96% | 0.91 | 563.10 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0519ms | 0.0959ms | 14.64% | 10072 | 0 | 67.96% | 0.91 | 881.35 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0330ms | 0.0964ms | 22.52% | 10072 | 0 | 67.96% | 0.91 | 1386.83 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0954ms | 0.0973ms | 16.28% | 10072 | 0 | 67.96% | 0.91 | 480.00 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4050ms | 1.4586ms | 23.60% | 10072 | 2000 | 67.96% | 0.91 | 113.04 MB/s |
| Quicksort | 10000 | 0.3404ms | 1.1924ms | 30.30% | 137379 | 0 | 67.96% | 0.91 | 1344.94 MB/s |
| Timsort | 10000 | 1.4640ms | 2.0173ms | 13.34% | 139385 | 0 | 67.96% | 0.91 | 312.68 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.9530ms | 8.0087ms | 27.61% | 53782 | 30000 | 67.96% | 0.91 | 234.39 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 7.9648ms | 8.3236ms | 4.19% | 53776 | 30000 | 67.96% | 0.91 | 57.47 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.5092ms | 2.8667ms | 17.27% | 191502 | 14351 | 67.96% | 0.91 | 303.32 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.7833ms | 1.9654ms | 24.90% | 60924 | 10000 | 67.96% | 0.91 | 584.42 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.4372ms | 1.4099ms | 28.46% | 60924 | 0 | 67.96% | 0.91 | 1047.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.1743ms | 1.5741ms | 14.88% | 64183 | 0 | 67.96% | 0.91 | 389.81 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6921ms | 2.0040ms | 22.12% | 58018 | 0 | 67.96% | 0.91 | 661.39 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.7420ms | 2.0923ms | 31.13% | 61360 | 0 | 67.96% | 0.91 | 616.90 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.3822ms | 1.3074ms | 37.17% | 60924 | 0 | 67.96% | 0.91 | 1197.80 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.0294ms | 1.4696ms | 17.94% | 60924 | 0 | 67.96% | 0.91 | 444.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.4625ms | 1.2877ms | 25.04% | 60924 | 0 | 67.96% | 0.91 | 989.77 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.0610ms | 3.3201ms | 24.87% | 60924 | 20000 | 67.96% | 0.91 | 431.46 MB/s |
| Quicksort | 100000 | 3.6374ms | 10.9713ms | 21.55% | 1425231 | 0 | 67.96% | 0.91 | 1258.49 MB/s |
| Timsort | 100000 | 17.7485ms | 18.5866ms | 3.13% | 1429721 | 0 | 67.95% | 0.91 | 257.92 MB/s |
| ARS Gen 1: Foundation | 100000 | 14.1163ms | 34.2935ms | 31.67% | 1357174 | 300000 | 67.95% | 0.91 | 324.28 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 32.8241ms | 36.1538ms | 6.55% | 1357228 | 300000 | 67.95% | 0.91 | 139.46 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 6.0255ms | 16.5084ms | 28.50% | 1617850 | 108703 | 67.95% | 0.91 | 759.71 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.1283ms | 6.4528ms | 28.40% | 706001 | 100000 | 67.96% | 0.91 | 2150.86 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.0985ms | 4.5606ms | 12.04% | 706001 | 0 | 67.96% | 0.91 | 1116.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.9238ms | 5.5465ms | 28.51% | 713292 | 0 | 67.96% | 0.91 | 2379.49 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.8990ms | 6.1263ms | 16.32% | 674072 | 0 | 67.96% | 0.91 | 1174.06 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 2.0246ms | 6.0773ms | 29.70% | 681861 | 0 | 67.95% | 0.91 | 2261.05 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.4734ms | 5.9666ms | 20.07% | 706001 | 0 | 67.96% | 0.91 | 1317.93 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.9553ms | 6.3274ms | 14.19% | 602446 | 0 | 67.96% | 0.91 | 923.79 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.0947ms | 5.4025ms | 19.00% | 706001 | 0 | 67.96% | 0.91 | 1479.20 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 4.3005ms | 9.9088ms | 19.05% | 706001 | 200000 | 67.96% | 0.91 | 1064.44 MB/s |
| Quicksort | 1000000 | 28.8727ms | 29.9801ms | 4.20% | 13509237 | 0 | 67.94% | 0.91 | 1585.45 MB/s |
| Timsort | 1000000 | 54.0414ms | 56.3294ms | 3.17% | 14657477 | 0 | 67.93% | 0.91 | 847.06 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 40.2519ms | 41.6366ms | 2.38% | 14935986 | 1017407 | 67.95% | 0.91 | 1137.25 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 20.1499ms | 20.9504ms | 5.16% | 4726150 | 1000000 | 67.96% | 0.91 | 2271.79 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 19.7015ms | 21.2908ms | 3.62% | 4726150 | 0 | 67.96% | 0.91 | 2323.50 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 20.0585ms | 22.3735ms | 5.42% | 4750059 | 0 | 67.95% | 0.91 | 2282.14 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.3108ms | 20.0189ms | 4.34% | 6313227 | 0 | 67.95% | 0.91 | 2499.97 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 22.0222ms | 22.5204ms | 6.26% | 6334846 | 0 | 67.95% | 0.91 | 2078.65 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 20.4629ms | 22.5866ms | 7.49% | 4696239 | 0 | 67.96% | 0.91 | 2237.04 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.7228ms | 26.7946ms | 2.42% | 2303700 | 0 | 67.95% | 0.91 | 1779.60 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.7073ms | 27.7926ms | 4.83% | 2597927 | 0 | 67.95% | 0.91 | 1780.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 45.7778ms | 47.4874ms | 2.19% | 11435042 | 2000000 | 67.95% | 0.91 | 999.97 MB/s |
| Quicksort | 10000000 | 409.5825ms | 416.8533ms | 0.95% | 132718769 | 0 | 67.84% | 0.91 | 1117.63 MB/s |
| Timsort | 10000000 | 818.0263ms | 827.6162ms | 1.04% | 148730332 | 0 | 67.69% | 0.91 | 559.60 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 385.0308ms | 388.5834ms | 1.39% | 145964994 | 10017407 | 67.84% | 0.91 | 1188.90 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 212.3009ms | 215.1244ms | 1.12% | 47306515 | 10000000 | 67.93% | 0.91 | 2156.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 208.8102ms | 214.1836ms | 1.48% | 47306515 | 0 | 67.93% | 0.91 | 2192.25 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 280.3020ms | 285.2865ms | 2.08% | 47373283 | 0 | 67.86% | 0.91 | 1633.11 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 203.5985ms | 207.6393ms | 2.58% | 60307375 | 0 | 67.90% | 0.91 | 2248.36 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 337.1904ms | 346.1879ms | 2.05% | 60414728 | 0 | 67.85% | 0.91 | 1357.58 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 272.5836ms | 280.6227ms | 1.26% | 12103155 | 0 | 67.93% | 0.91 | 1679.35 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 298.0960ms | 302.3749ms | 0.81% | 10738780 | 0 | 67.97% | 0.91 | 1535.62 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 276.8473ms | 280.2032ms | 1.51% | 12704704 | 0 | 67.95% | 0.91 | 1653.49 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1165.0143ms | 1177.7791ms | 0.71% | 177038151 | 20000000 | 67.96% | 0.91 | 392.93 MB/s |

### Distribution: NearlySorted

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0310ms | 0.0888ms | 22.39% | 9584 | 0 | 67.96% | 0.91 | 1476.47 MB/s |
| Timsort | 1000 | 0.1364ms | 0.1411ms | 15.91% | 9396 | 0 | 67.96% | 0.91 | 335.55 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.4202ms | 0.6342ms | 20.09% | 9525 | 2000 | 67.96% | 0.91 | 108.93 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.6171ms | 0.6730ms | 14.85% | 9577 | 2000 | 67.96% | 0.91 | 74.18 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0299ms | 0.0924ms | 25.86% | 9584 | 0 | 67.96% | 0.91 | 1530.47 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0332ms | 0.0902ms | 23.26% | 9584 | 0 | 67.96% | 0.91 | 1377.19 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0667ms | 0.0897ms | 16.58% | 9584 | 0 | 67.96% | 0.91 | 685.79 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0770ms | 0.1641ms | 28.96% | 9396 | 0 | 67.96% | 0.91 | 594.15 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0603ms | 0.0921ms | 21.69% | 9584 | 0 | 67.96% | 0.91 | 759.52 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.1231ms | 0.1692ms | 22.92% | 9396 | 0 | 67.96% | 0.91 | 371.87 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0804ms | 0.0977ms | 17.11% | 9584 | 0 | 67.96% | 0.91 | 569.38 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0592ms | 0.1024ms | 16.72% | 9584 | 0 | 67.96% | 0.91 | 772.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0456ms | 0.0894ms | 17.88% | 9584 | 0 | 67.96% | 0.91 | 1002.92 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4294ms | 1.4107ms | 25.23% | 9584 | 2000 | 67.96% | 0.91 | 106.60 MB/s |
| Quicksort | 10000 | 0.4928ms | 1.1404ms | 23.85% | 132340 | 0 | 67.96% | 0.91 | 928.85 MB/s |
| Timsort | 10000 | 0.5208ms | 1.6349ms | 24.94% | 127962 | 0 | 67.96% | 0.91 | 878.99 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.6389ms | 7.5443ms | 16.41% | 126689 | 30000 | 67.96% | 0.91 | 98.68 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.0373ms | 7.0198ms | 26.08% | 126634 | 30000 | 67.96% | 0.91 | 224.70 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.9176ms | 2.8248ms | 22.72% | 183481 | 14351 | 67.96% | 0.91 | 498.85 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.6934ms | 1.8703ms | 13.11% | 42063 | 10000 | 67.96% | 0.91 | 270.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.6378ms | 1.2331ms | 27.58% | 42063 | 0 | 67.96% | 0.91 | 717.72 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.5741ms | 1.2505ms | 28.93% | 34337 | 0 | 67.96% | 0.91 | 797.35 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6516ms | 1.8638ms | 26.61% | 48574 | 0 | 67.96% | 0.91 | 702.50 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.5953ms | 1.9953ms | 10.55% | 43350 | 0 | 67.96% | 0.91 | 286.95 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.1437ms | 1.5216ms | 13.44% | 42063 | 0 | 67.96% | 0.91 | 400.25 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.7791ms | 1.3215ms | 29.12% | 42063 | 0 | 67.96% | 0.91 | 587.53 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.6011ms | 1.1558ms | 17.88% | 42063 | 0 | 67.96% | 0.91 | 761.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.8768ms | 3.1067ms | 28.27% | 42063 | 20000 | 67.96% | 0.91 | 522.10 MB/s |
| Quicksort | 100000 | 6.0911ms | 14.2290ms | 20.18% | 1683670 | 0 | 67.96% | 0.91 | 751.53 MB/s |
| Timsort | 100000 | 11.9366ms | 18.4172ms | 20.77% | 1626828 | 0 | 67.95% | 0.91 | 383.50 MB/s |
| ARS Gen 1: Foundation | 100000 | 34.1286ms | 37.3579ms | 8.13% | 1609521 | 300000 | 67.95% | 0.91 | 134.13 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 17.9600ms | 38.4307ms | 31.69% | 1609162 | 300000 | 67.95% | 0.91 | 254.88 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 16.2417ms | 16.7368ms | 2.20% | 1798710 | 108703 | 67.95% | 0.91 | 281.85 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 5.7552ms | 6.9665ms | 11.05% | 800253 | 100000 | 67.95% | 0.91 | 795.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.3872ms | 4.5180ms | 27.73% | 800253 | 0 | 67.95% | 0.91 | 3299.94 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.7802ms | 4.0024ms | 24.50% | 405566 | 0 | 67.95% | 0.91 | 2571.47 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 6.0103ms | 7.0995ms | 11.53% | 871884 | 0 | 67.95% | 0.91 | 761.63 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.6612ms | 5.6702ms | 18.40% | 434645 | 0 | 67.95% | 0.91 | 982.06 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 2.3394ms | 5.3076ms | 27.92% | 800253 | 0 | 67.95% | 0.91 | 1956.73 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.0614ms | 6.5506ms | 15.09% | 691231 | 0 | 67.95% | 0.91 | 904.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.8798ms | 6.2234ms | 14.23% | 800253 | 0 | 67.95% | 0.91 | 938.08 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 9.9749ms | 10.6752ms | 5.60% | 800253 | 200000 | 67.95% | 0.91 | 458.91 MB/s |
| Quicksort | 1000000 | 47.5688ms | 50.4241ms | 3.41% | 20534409 | 0 | 67.95% | 0.91 | 962.32 MB/s |
| Timsort | 1000000 | 72.1478ms | 74.5588ms | 2.18% | 19242442 | 0 | 67.93% | 0.91 | 634.48 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 45.5368ms | 47.8139ms | 3.01% | 20724738 | 1017407 | 67.95% | 0.91 | 1005.26 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 23.8135ms | 24.9022ms | 10.05% | 9495241 | 1000000 | 67.96% | 0.91 | 1922.29 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.1770ms | 23.6020ms | 4.04% | 9495241 | 0 | 67.96% | 0.91 | 2064.14 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 20.1761ms | 20.5950ms | 4.42% | 4133458 | 0 | 67.96% | 0.91 | 2268.85 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 20.3839ms | 23.0427ms | 5.87% | 12338615 | 0 | 67.95% | 0.91 | 2245.71 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 18.4223ms | 21.5589ms | 6.20% | 5639882 | 0 | 67.95% | 0.91 | 2484.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 25.1105ms | 26.1037ms | 5.07% | 9495241 | 0 | 67.96% | 0.91 | 1823.00 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.4832ms | 26.6937ms | 2.31% | 10586609 | 0 | 67.95% | 0.91 | 1796.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.3058ms | 27.2293ms | 3.05% | 11698023 | 0 | 67.96% | 0.91 | 1808.93 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 49.8043ms | 52.8462ms | 3.28% | 14647039 | 2000000 | 67.95% | 0.91 | 919.13 MB/s |
| Quicksort | 10000000 | 586.2839ms | 601.5686ms | 1.71% | 242094386 | 0 | 67.83% | 0.92 | 780.79 MB/s |
| Timsort | 10000000 | 1003.6029ms | 1007.8772ms | 0.45% | 225241414 | 0 | 67.69% | 0.91 | 456.12 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 383.0095ms | 409.4825ms | 3.24% | 241839376 | 10017407 | 67.90% | 0.91 | 1195.18 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 209.5920ms | 214.8006ms | 1.65% | 131502249 | 10000000 | 67.92% | 0.91 | 2184.07 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 196.0585ms | 201.5001ms | 1.67% | 131502249 | 0 | 67.93% | 0.91 | 2334.83 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 177.3859ms | 179.2075ms | 1.23% | 48686386 | 0 | 67.92% | 0.91 | 2580.61 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 209.9838ms | 217.9164ms | 1.89% | 159739203 | 0 | 67.91% | 0.91 | 2180.00 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 195.1125ms | 237.7555ms | 7.99% | 91790712 | 0 | 67.86% | 0.91 | 2346.15 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 226.0231ms | 231.2484ms | 1.32% | 23888954 | 0 | 67.90% | 0.91 | 2025.30 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 285.0187ms | 288.3021ms | 0.62% | 41658392 | 0 | 67.93% | 0.91 | 1606.08 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 241.1763ms | 247.2756ms | 1.30% | 41677150 | 0 | 67.91% | 0.91 | 1898.05 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1221.9275ms | 1231.3870ms | 0.51% | 237087205 | 20000000 | 67.92% | 0.91 | 374.62 MB/s |

### Distribution: Duplicates

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0310ms | 0.0323ms | 15.42% | 3786 | 0 | 67.86% | 0.92 | 1476.09 MB/s |
| Timsort | 1000 | 0.0708ms | 0.0758ms | 13.37% | 3800 | 0 | 67.86% | 0.92 | 646.90 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.1043ms | 0.2558ms | 19.84% | 995 | 2000 | 67.86% | 0.92 | 438.76 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2714ms | 0.3287ms | 9.55% | 995 | 2000 | 67.86% | 0.92 | 168.70 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0276ms | 0.0322ms | 21.40% | 3786 | 0 | 67.86% | 0.92 | 1658.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0301ms | 0.0315ms | 4.02% | 3786 | 0 | 67.86% | 0.92 | 1523.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0134ms | 0.0321ms | 19.23% | 3786 | 0 | 67.86% | 0.92 | 3413.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0687ms | 0.0796ms | 28.20% | 3800 | 0 | 67.86% | 0.92 | 666.37 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0120ms | 0.0335ms | 32.71% | 3786 | 0 | 67.86% | 0.92 | 3827.14 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0251ms | 0.0741ms | 24.28% | 3800 | 0 | 67.86% | 0.92 | 1826.16 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0295ms | 0.0324ms | 14.01% | 3786 | 0 | 67.86% | 0.92 | 1551.27 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0100ms | 0.0324ms | 28.96% | 3786 | 0 | 67.86% | 0.92 | 4597.87 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0102ms | 0.0317ms | 30.33% | 3786 | 0 | 67.86% | 0.92 | 4474.28 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4389ms | 1.2693ms | 21.99% | 3786 | 2000 | 67.86% | 0.92 | 104.29 MB/s |
| Quicksort | 10000 | 0.2573ms | 0.3027ms | 24.84% | 38638 | 0 | 67.86% | 0.92 | 1778.90 MB/s |
| Timsort | 10000 | 0.6730ms | 0.6995ms | 7.86% | 38754 | 0 | 67.86% | 0.92 | 680.22 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.5425ms | 2.8370ms | 8.48% | 9995 | 30000 | 67.86% | 0.92 | 180.04 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 0.8663ms | 2.9145ms | 24.56% | 9995 | 30000 | 67.86% | 0.92 | 528.40 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.3658ms | 2.7566ms | 17.05% | 115245 | 14351 | 67.86% | 0.92 | 335.15 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.3903ms | 1.6435ms | 46.16% | 10003 | 10000 | 67.86% | 0.92 | 1172.90 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3985ms | 1.0435ms | 45.82% | 10003 | 0 | 67.86% | 0.92 | 1148.74 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.2904ms | 1.0987ms | 34.05% | 10003 | 0 | 67.86% | 0.92 | 1576.45 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.5706ms | 1.6261ms | 29.73% | 10003 | 0 | 67.86% | 0.92 | 802.19 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.1310ms | 1.8765ms | 15.21% | 10003 | 0 | 67.86% | 0.92 | 404.76 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.9330ms | 1.0811ms | 15.85% | 10003 | 0 | 67.86% | 0.92 | 490.66 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.3477ms | 1.1051ms | 33.68% | 10003 | 0 | 67.86% | 0.92 | 1316.68 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.8756ms | 1.2622ms | 34.52% | 10003 | 0 | 67.86% | 0.92 | 522.81 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.7898ms | 2.8240ms | 25.79% | 10003 | 20000 | 67.86% | 0.92 | 579.61 MB/s |
| Quicksort | 100000 | 2.8757ms | 3.1196ms | 2.73% | 381839 | 0 | 67.86% | 0.92 | 1591.85 MB/s |
| Timsort | 100000 | 2.8232ms | 7.7474ms | 21.32% | 382404 | 0 | 67.85% | 0.92 | 1621.43 MB/s |
| ARS Gen 1: Foundation | 100000 | 17.6827ms | 20.5315ms | 5.37% | 99995 | 300000 | 67.86% | 0.92 | 258.88 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 19.0596ms | 20.7509ms | 6.09% | 99995 | 300000 | 67.86% | 0.92 | 240.17 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 12.0355ms | 17.6967ms | 11.72% | 1130007 | 108703 | 67.86% | 0.92 | 380.34 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 4.7182ms | 4.9411ms | 3.85% | 100001 | 100000 | 67.86% | 0.92 | 970.21 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 0.9489ms | 3.4831ms | 27.05% | 100001 | 0 | 67.86% | 0.92 | 4823.98 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 1.5397ms | 3.6121ms | 22.49% | 100001 | 0 | 67.86% | 0.92 | 2973.03 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.9724ms | 4.1994ms | 9.83% | 100001 | 0 | 67.86% | 0.92 | 1152.36 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.9176ms | 4.4001ms | 24.02% | 100001 | 0 | 67.86% | 0.92 | 2387.20 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 1.3576ms | 4.8198ms | 29.27% | 199996 | 0 | 67.86% | 0.92 | 3371.74 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.9085ms | 4.3097ms | 18.36% | 199996 | 0 | 67.86% | 0.92 | 1573.89 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.0498ms | 5.1594ms | 19.03% | 199996 | 0 | 67.86% | 0.92 | 1130.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.4992ms | 8.9057ms | 25.13% | 100001 | 200000 | 67.86% | 0.92 | 1831.61 MB/s |
| Quicksort | 1000000 | 11.1721ms | 12.8849ms | 6.24% | 3606185 | 0 | 67.85% | 0.92 | 4097.38 MB/s |
| Timsort | 1000000 | 30.8536ms | 32.4304ms | 6.45% | 4711109 | 0 | 67.85% | 0.92 | 1483.66 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 32.0016ms | 33.6994ms | 4.46% | 12063052 | 1017407 | 67.85% | 0.92 | 1430.44 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.2701ms | 18.3815ms | 5.94% | 999999 | 1000000 | 67.86% | 0.92 | 2650.61 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 17.7598ms | 18.0784ms | 7.38% | 999999 | 0 | 67.86% | 0.92 | 2577.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 17.8647ms | 19.0540ms | 3.75% | 999999 | 0 | 67.86% | 0.92 | 2562.39 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 14.9581ms | 15.4530ms | 5.89% | 999999 | 0 | 67.86% | 0.92 | 3060.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 14.8807ms | 15.9284ms | 5.02% | 999999 | 0 | 67.86% | 0.92 | 3076.23 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 20.9499ms | 22.0772ms | 2.82% | 1999994 | 0 | 67.86% | 0.92 | 2185.03 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 19.7672ms | 20.9243ms | 3.15% | 1999994 | 0 | 67.86% | 0.92 | 2315.78 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.8755ms | 22.1929ms | 5.21% | 1999994 | 0 | 67.86% | 0.92 | 2192.83 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 45.1239ms | 46.3122ms | 3.72% | 5365259 | 2000000 | 67.85% | 0.92 | 1014.46 MB/s |
| Quicksort | 10000000 | 147.4962ms | 154.7501ms | 2.62% | 38019843 | 0 | 67.85% | 0.92 | 3103.56 MB/s |
| Timsort | 10000000 | 429.8732ms | 445.6145ms | 1.54% | 51536944 | 0 | 67.84% | 0.92 | 1064.88 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 342.9872ms | 352.9056ms | 1.30% | 120066201 | 10017407 | 67.84% | 0.92 | 1334.64 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 138.3500ms | 140.8840ms | 2.68% | 10000007 | 10000000 | 67.85% | 0.91 | 3308.74 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 138.6102ms | 141.0425ms | 1.39% | 10000007 | 0 | 67.85% | 0.91 | 3302.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 138.1657ms | 141.4056ms | 1.65% | 10000007 | 0 | 67.85% | 0.91 | 3313.15 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 118.8667ms | 121.0463ms | 3.09% | 10000007 | 0 | 67.85% | 0.91 | 3851.07 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 118.8536ms | 119.4338ms | 1.46% | 10000007 | 0 | 67.85% | 0.91 | 3851.49 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 169.3115ms | 174.8847ms | 3.44% | 20000002 | 0 | 67.86% | 0.91 | 2703.68 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 159.4768ms | 162.3815ms | 1.84% | 20000002 | 0 | 67.86% | 0.91 | 2870.41 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 170.6762ms | 172.3240ms | 1.96% | 20000002 | 0 | 67.86% | 0.91 | 2682.06 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1169.6077ms | 1179.6118ms | 0.56% | 109681151 | 20000000 | 67.86% | 0.91 | 391.38 MB/s |

### Distribution: Zipfian

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0454ms | 0.0474ms | 26.17% | 5543 | 0 | 67.85% | 0.91 | 1007.49 MB/s |
| Timsort | 1000 | 0.0470ms | 0.0968ms | 27.18% | 5717 | 0 | 67.85% | 0.91 | 973.47 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2403ms | 0.2693ms | 8.59% | 4495 | 2000 | 67.85% | 0.91 | 190.49 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.2859ms | 0.3174ms | 26.45% | 4495 | 2000 | 67.85% | 0.91 | 160.13 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0457ms | 0.0532ms | 72.77% | 5543 | 0 | 67.85% | 0.91 | 1002.57 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0457ms | 0.0506ms | 18.29% | 5543 | 0 | 67.85% | 0.91 | 1002.00 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0474ms | 0.0525ms | 21.90% | 5543 | 0 | 67.85% | 0.91 | 965.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0840ms | 0.1110ms | 17.88% | 5717 | 0 | 67.85% | 0.91 | 544.74 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0143ms | 0.0491ms | 29.24% | 5543 | 0 | 67.85% | 0.91 | 3200.25 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0878ms | 0.1213ms | 17.86% | 5717 | 0 | 67.85% | 0.91 | 521.45 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0465ms | 0.0487ms | 22.09% | 5543 | 0 | 67.85% | 0.91 | 984.29 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0192ms | 0.0512ms | 23.02% | 5543 | 0 | 67.85% | 0.91 | 2383.94 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0469ms | 0.0506ms | 25.73% | 5543 | 0 | 67.85% | 0.91 | 976.96 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 1.2438ms | 1.3726ms | 7.43% | 5543 | 2000 | 67.85% | 0.91 | 36.80 MB/s |
| Quicksort | 10000 | 0.1706ms | 0.4223ms | 19.46% | 53161 | 0 | 67.85% | 0.91 | 2683.84 MB/s |
| Timsort | 10000 | 0.2374ms | 0.8687ms | 32.11% | 54050 | 0 | 67.85% | 0.91 | 1928.15 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.7549ms | 3.4483ms | 7.74% | 48280 | 30000 | 67.85% | 0.91 | 166.17 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.3212ms | 3.9038ms | 22.87% | 48280 | 30000 | 67.85% | 0.91 | 346.46 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.8574ms | 3.0024ms | 13.43% | 124997 | 14351 | 67.85% | 0.91 | 246.45 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 2.3217ms | 2.5117ms | 7.06% | 52916 | 10000 | 67.85% | 0.91 | 197.16 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 1.3485ms | 1.6096ms | 11.00% | 52916 | 0 | 67.85% | 0.91 | 339.46 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.6030ms | 2.0422ms | 27.86% | 50927 | 0 | 67.85% | 0.91 | 759.09 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.6390ms | 2.5384ms | 25.87% | 46974 | 0 | 67.85% | 0.91 | 716.36 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.8168ms | 2.8643ms | 16.22% | 45111 | 0 | 67.85% | 0.91 | 251.96 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.5478ms | 1.7294ms | 24.90% | 12797 | 0 | 67.85% | 0.91 | 835.59 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.5359ms | 1.7034ms | 26.63% | 52916 | 0 | 67.85% | 0.91 | 854.26 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.3930ms | 1.5260ms | 38.22% | 52916 | 0 | 67.85% | 0.91 | 328.62 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.8573ms | 3.2037ms | 30.86% | 52916 | 20000 | 67.85% | 0.91 | 533.93 MB/s |
| Quicksort | 100000 | 3.9354ms | 4.2697ms | 4.49% | 532153 | 0 | 67.85% | 0.91 | 1163.21 MB/s |
| Timsort | 100000 | 5.5508ms | 8.5382ms | 13.57% | 535821 | 0 | 67.85% | 0.91 | 824.69 MB/s |
| ARS Gen 1: Foundation | 100000 | 13.0155ms | 15.9307ms | 14.77% | 513174 | 300000 | 67.85% | 0.91 | 351.71 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 6.7006ms | 16.8582ms | 31.27% | 513072 | 300000 | 67.85% | 0.91 | 683.16 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 4.2158ms | 11.6294ms | 37.63% | 1173331 | 108703 | 67.85% | 0.91 | 1085.83 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.1410ms | 8.9313ms | 30.67% | 516174 | 100000 | 67.85% | 0.91 | 1457.37 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.7453ms | 8.1546ms | 21.83% | 516174 | 0 | 67.85% | 0.91 | 964.66 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 4.8215ms | 10.6607ms | 20.13% | 519053 | 0 | 67.84% | 0.91 | 949.43 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.3283ms | 8.5170ms | 25.60% | 503503 | 0 | 67.85% | 0.91 | 1375.36 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.9395ms | 7.0744ms | 28.14% | 505145 | 0 | 67.84% | 0.91 | 926.73 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.0769ms | 8.6060ms | 23.16% | 202261 | 0 | 67.84% | 0.91 | 1122.83 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.9076ms | 8.6060ms | 19.01% | 181345 | 0 | 67.85% | 0.91 | 932.76 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 6.3782ms | 11.1335ms | 19.07% | 207416 | 0 | 67.84% | 0.91 | 717.70 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 5.7253ms | 9.3121ms | 24.32% | 516174 | 200000 | 67.85% | 0.91 | 799.54 MB/s |
| Quicksort | 1000000 | 12.8116ms | 13.3932ms | 9.19% | 5295042 | 0 | 67.84% | 0.91 | 3573.04 MB/s |
| Timsort | 1000000 | 33.7824ms | 34.9811ms | 7.25% | 6316098 | 0 | 67.83% | 0.91 | 1355.04 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 29.6110ms | 31.1603ms | 4.15% | 12311175 | 1017407 | 67.85% | 0.91 | 1545.92 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 29.9672ms | 31.0561ms | 2.37% | 5214844 | 1000000 | 67.84% | 0.91 | 1527.55 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 27.7286ms | 29.3791ms | 4.96% | 5214844 | 0 | 67.84% | 0.91 | 1650.87 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 50.7810ms | 51.7640ms | 5.91% | 6532668 | 0 | 67.83% | 0.91 | 901.45 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 25.8125ms | 27.2846ms | 4.02% | 5227326 | 0 | 67.84% | 0.91 | 1773.42 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 46.3637ms | 48.5449ms | 2.61% | 6531874 | 0 | 67.83% | 0.91 | 987.33 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 34.2406ms | 35.3389ms | 2.70% | 1936223 | 0 | 67.85% | 0.91 | 1336.90 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 45.7391ms | 47.1150ms | 4.75% | 2060175 | 0 | 67.85% | 0.91 | 1000.81 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 47.2319ms | 48.4005ms | 2.07% | 2061077 | 0 | 67.85% | 0.91 | 969.18 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 53.1221ms | 53.7801ms | 2.28% | 9738869 | 2000000 | 67.84% | 0.91 | 861.72 MB/s |
| Quicksort | 10000000 | 200.2161ms | 203.4526ms | 0.67% | 52712737 | 0 | 67.82% | 0.91 | 2286.35 MB/s |
| Timsort | 10000000 | 508.4328ms | 515.4000ms | 0.68% | 65495865 | 0 | 67.78% | 0.91 | 900.34 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 375.2091ms | 384.3434ms | 2.51% | 122376530 | 10017407 | 67.83% | 0.91 | 1220.02 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 332.0383ms | 340.5308ms | 1.21% | 52560326 | 10000000 | 67.83% | 0.91 | 1378.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 323.9249ms | 329.3656ms | 1.29% | 52560326 | 0 | 67.83% | 0.91 | 1413.18 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 648.7321ms | 657.5296ms | 0.71% | 65582835 | 0 | 67.79% | 0.91 | 705.63 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 305.7284ms | 309.8888ms | 1.37% | 52502047 | 0 | 67.83% | 0.91 | 1497.29 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 633.3395ms | 639.2032ms | 0.82% | 65820976 | 0 | 67.79% | 0.91 | 722.78 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 445.0141ms | 448.7715ms | 1.04% | 20247233 | 0 | 67.86% | 0.91 | 1028.65 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 511.8184ms | 516.1624ms | 0.68% | 20125002 | 0 | 67.87% | 0.91 | 894.39 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 469.7228ms | 479.2360ms | 1.43% | 20284933 | 0 | 67.86% | 0.91 | 974.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1237.8893ms | 1248.1414ms | 0.54% | 160388438 | 20000000 | 67.81% | 0.91 | 369.79 MB/s |

### Distribution: Skewed

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0326ms | 0.0970ms | 28.67% | 10266 | 0 | 67.74% | 0.92 | 1402.33 MB/s |
| Timsort | 1000 | 0.1669ms | 0.1749ms | 7.99% | 10653 | 0 | 67.74% | 0.92 | 274.33 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.4112ms | 0.9278ms | 23.02% | 758 | 2000 | 67.74% | 0.92 | 111.34 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.4775ms | 0.9857ms | 18.32% | 758 | 2000 | 67.74% | 0.92 | 95.86 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0408ms | 0.1041ms | 33.50% | 10266 | 0 | 67.74% | 0.92 | 1122.05 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0317ms | 0.0972ms | 31.19% | 10266 | 0 | 67.74% | 0.92 | 1443.23 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0365ms | 0.0967ms | 28.51% | 10266 | 0 | 67.74% | 0.92 | 1255.70 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1663ms | 0.1942ms | 29.17% | 10653 | 0 | 67.74% | 0.92 | 275.30 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0282ms | 0.1128ms | 38.40% | 10266 | 0 | 67.74% | 0.92 | 1623.68 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0693ms | 0.1811ms | 24.90% | 10653 | 0 | 67.74% | 0.92 | 660.28 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0372ms | 0.0964ms | 20.66% | 10266 | 0 | 67.74% | 0.92 | 1229.59 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0352ms | 0.0970ms | 27.49% | 10266 | 0 | 67.74% | 0.92 | 1300.91 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0509ms | 0.1029ms | 28.29% | 10266 | 0 | 67.74% | 0.92 | 899.04 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.4375ms | 1.3532ms | 29.05% | 10266 | 2000 | 67.74% | 0.92 | 104.64 MB/s |
| Quicksort | 10000 | 0.4023ms | 1.1814ms | 26.25% | 134210 | 0 | 67.74% | 0.92 | 1137.95 MB/s |
| Timsort | 10000 | 1.5996ms | 1.9319ms | 18.26% | 136312 | 0 | 67.74% | 0.92 | 286.17 MB/s |
| ARS Gen 1: Foundation | 10000 | 2.2666ms | 7.0029ms | 25.90% | 82054 | 30000 | 67.74% | 0.92 | 201.96 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.3538ms | 8.6103ms | 25.98% | 82008 | 30000 | 67.74% | 0.92 | 194.48 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.7677ms | 2.8526ms | 5.27% | 189657 | 14351 | 67.74% | 0.92 | 165.39 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.5270ms | 2.0358ms | 25.51% | 68718 | 10000 | 67.74% | 0.92 | 868.60 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.6178ms | 1.4389ms | 25.50% | 68718 | 0 | 67.74% | 0.92 | 741.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.2811ms | 1.6390ms | 13.02% | 71066 | 0 | 67.74% | 0.92 | 357.32 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.3381ms | 2.2890ms | 15.57% | 59309 | 0 | 67.74% | 0.92 | 342.11 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.8439ms | 2.1743ms | 36.02% | 62468 | 0 | 67.74% | 0.92 | 542.41 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.3124ms | 1.5409ms | 14.77% | 68718 | 0 | 67.74% | 0.92 | 348.81 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.4335ms | 1.3219ms | 44.12% | 68718 | 0 | 67.74% | 0.92 | 1055.90 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.4977ms | 1.5378ms | 39.15% | 68718 | 0 | 67.74% | 0.92 | 919.68 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.1873ms | 3.4106ms | 24.37% | 68718 | 20000 | 67.74% | 0.92 | 385.56 MB/s |
| Quicksort | 100000 | 3.4322ms | 10.3251ms | 21.71% | 1336542 | 0 | 67.74% | 0.92 | 1333.71 MB/s |
| Timsort | 100000 | 16.9960ms | 17.3336ms | 2.86% | 1344225 | 0 | 67.73% | 0.92 | 269.34 MB/s |
| ARS Gen 1: Foundation | 100000 | 19.7988ms | 35.2153ms | 16.48% | 1270914 | 300000 | 67.73% | 0.92 | 231.21 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 16.9708ms | 33.9132ms | 21.35% | 1270940 | 300000 | 67.73% | 0.92 | 269.74 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 5.2832ms | 17.2692ms | 33.19% | 1545581 | 108703 | 67.74% | 0.92 | 866.45 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 5.4193ms | 6.2787ms | 15.69% | 739518 | 100000 | 67.74% | 0.92 | 844.69 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.3874ms | 4.9450ms | 7.41% | 739518 | 0 | 67.74% | 0.92 | 1043.35 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 5.0149ms | 5.4198ms | 5.51% | 746122 | 0 | 67.74% | 0.92 | 912.81 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.3577ms | 6.3151ms | 16.58% | 629866 | 0 | 67.74% | 0.92 | 1363.31 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.6919ms | 6.5995ms | 28.32% | 636677 | 0 | 67.74% | 0.92 | 2705.68 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.3537ms | 5.6326ms | 16.40% | 630569 | 0 | 67.74% | 0.92 | 1364.95 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.7823ms | 6.1315ms | 9.50% | 636332 | 0 | 67.74% | 0.92 | 957.20 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.3505ms | 6.2360ms | 23.16% | 739518 | 0 | 67.74% | 0.92 | 1947.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.9139ms | 10.1693ms | 28.85% | 739518 | 200000 | 67.74% | 0.92 | 1570.99 MB/s |
| Quicksort | 1000000 | 28.1784ms | 29.8572ms | 4.13% | 12872031 | 0 | 67.73% | 0.92 | 1624.52 MB/s |
| Timsort | 1000000 | 53.7037ms | 55.4057ms | 2.66% | 13977939 | 0 | 67.71% | 0.92 | 852.39 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 38.5233ms | 41.3795ms | 2.77% | 14243405 | 1017407 | 67.73% | 0.92 | 1188.28 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 22.3809ms | 22.8716ms | 3.11% | 5181387 | 1000000 | 67.74% | 0.92 | 2045.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 20.9525ms | 22.3491ms | 4.05% | 5181387 | 0 | 67.74% | 0.92 | 2184.77 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 22.6805ms | 25.3284ms | 5.26% | 5211192 | 0 | 67.73% | 0.92 | 2018.32 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 18.5273ms | 20.2763ms | 6.84% | 6086657 | 0 | 67.73% | 0.92 | 2470.76 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 21.0315ms | 23.0315ms | 3.11% | 6107374 | 0 | 67.73% | 0.92 | 2176.56 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 25.9202ms | 27.9105ms | 3.23% | 2319879 | 0 | 67.73% | 0.92 | 1766.05 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 28.0351ms | 30.5441ms | 3.99% | 1843064 | 0 | 67.73% | 0.92 | 1632.82 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 26.5611ms | 29.3978ms | 5.79% | 2005681 | 0 | 67.74% | 0.92 | 1723.43 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 46.3630ms | 47.8452ms | 3.54% | 11789864 | 2000000 | 67.73% | 0.92 | 987.35 MB/s |
| Quicksort | 10000000 | 399.4156ms | 408.0456ms | 4.85% | 126626573 | 0 | 67.63% | 0.92 | 1146.08 MB/s |
| Timsort | 10000000 | 809.5045ms | 812.9948ms | 0.59% | 142418090 | 0 | 67.49% | 0.92 | 565.49 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 380.3954ms | 383.2404ms | 0.82% | 139730996 | 10017407 | 67.64% | 0.92 | 1203.39 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 238.2536ms | 242.6044ms | 1.66% | 53775253 | 10000000 | 67.71% | 0.91 | 1921.33 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 237.1365ms | 240.5108ms | 1.25% | 53775253 | 0 | 67.70% | 0.91 | 1930.38 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 357.7422ms | 364.8923ms | 0.80% | 53750048 | 0 | 67.65% | 0.91 | 1279.59 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 216.4554ms | 219.7915ms | 1.09% | 60102666 | 0 | 67.69% | 0.91 | 2114.82 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 378.5994ms | 384.4702ms | 1.33% | 60633916 | 0 | 67.66% | 0.91 | 1209.10 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 278.3848ms | 282.8571ms | 1.42% | 17635072 | 0 | 67.72% | 0.91 | 1644.36 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 290.9590ms | 295.6993ms | 1.20% | 16619301 | 0 | 67.75% | 0.91 | 1573.29 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 275.0934ms | 279.4232ms | 1.17% | 14063410 | 0 | 67.74% | 0.91 | 1664.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1159.9270ms | 1172.7703ms | 0.65% | 180564040 | 20000000 | 67.74% | 0.92 | 394.65 MB/s |

### Distribution: Clustered

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0269ms | 0.0955ms | 27.51% | 10001 | 0 | 67.72% | 0.92 | 1701.41 MB/s |
| Timsort | 1000 | 0.1684ms | 0.1786ms | 6.99% | 10525 | 0 | 67.72% | 0.92 | 271.89 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.6166ms | 0.6563ms | 15.00% | 5255 | 2000 | 67.72% | 0.92 | 74.24 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.7075ms | 0.7614ms | 13.79% | 5222 | 2000 | 67.72% | 0.92 | 64.70 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0422ms | 0.0971ms | 22.33% | 10001 | 0 | 67.72% | 0.92 | 1084.13 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0959ms | 0.0979ms | 15.86% | 10001 | 0 | 67.72% | 0.92 | 477.28 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0943ms | 0.0970ms | 13.14% | 10001 | 0 | 67.72% | 0.92 | 485.24 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1589ms | 0.1654ms | 10.74% | 10525 | 0 | 67.72% | 0.92 | 288.13 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0954ms | 0.0982ms | 15.68% | 10001 | 0 | 67.72% | 0.92 | 479.59 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0480ms | 0.1683ms | 34.06% | 10525 | 0 | 67.72% | 0.92 | 954.41 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0943ms | 0.0976ms | 14.12% | 10001 | 0 | 67.72% | 0.92 | 485.21 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0331ms | 0.0975ms | 27.29% | 10001 | 0 | 67.72% | 0.92 | 1381.47 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0961ms | 0.1036ms | 19.24% | 10001 | 0 | 67.72% | 0.92 | 476.35 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.3504ms | 1.5032ms | 26.91% | 10001 | 2000 | 67.72% | 0.92 | 130.64 MB/s |
| Quicksort | 10000 | 0.8099ms | 0.8274ms | 6.46% | 108556 | 0 | 67.72% | 0.92 | 565.21 MB/s |
| Timsort | 10000 | 0.4546ms | 1.3883ms | 28.16% | 108474 | 0 | 67.72% | 0.92 | 1006.85 MB/s |
| ARS Gen 1: Foundation | 10000 | 4.2899ms | 4.8782ms | 7.54% | 72991 | 30000 | 67.72% | 0.92 | 106.71 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 2.0338ms | 4.5922ms | 19.31% | 73783 | 30000 | 67.72% | 0.92 | 225.08 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.1206ms | 2.7434ms | 19.48% | 161230 | 14351 | 67.72% | 0.92 | 408.51 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.7435ms | 2.0074ms | 22.82% | 69853 | 10000 | 67.72% | 0.92 | 615.66 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.4730ms | 1.5049ms | 27.24% | 69853 | 0 | 67.72% | 0.92 | 967.82 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.3884ms | 1.6445ms | 18.18% | 70272 | 0 | 67.72% | 0.92 | 329.70 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 0.5736ms | 2.1882ms | 34.03% | 60786 | 0 | 67.72% | 0.92 | 798.06 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 2.0310ms | 2.1788ms | 5.00% | 61503 | 0 | 67.72% | 0.92 | 225.39 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.2157ms | 1.6346ms | 23.92% | 69853 | 0 | 67.72% | 0.92 | 376.53 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.1641ms | 1.4436ms | 14.90% | 69853 | 0 | 67.72% | 0.92 | 393.24 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.3754ms | 1.5205ms | 10.56% | 69853 | 0 | 67.72% | 0.92 | 332.82 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.9437ms | 3.1075ms | 26.76% | 69853 | 20000 | 67.72% | 0.92 | 485.06 MB/s |
| Quicksort | 100000 | 2.7168ms | 7.9370ms | 21.56% | 1013442 | 0 | 67.72% | 0.92 | 1684.96 MB/s |
| Timsort | 100000 | 12.1506ms | 13.6111ms | 6.25% | 1015758 | 0 | 67.72% | 0.92 | 376.74 MB/s |
| ARS Gen 1: Foundation | 100000 | 7.1232ms | 16.7171ms | 36.13% | 680001 | 300000 | 67.72% | 0.92 | 642.64 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 15.0217ms | 23.9172ms | 19.14% | 678692 | 300000 | 67.72% | 0.92 | 304.74 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 7.7764ms | 16.3239ms | 16.90% | 1240691 | 108703 | 67.72% | 0.92 | 588.66 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 2.3609ms | 5.0505ms | 39.49% | 651314 | 100000 | 67.72% | 0.92 | 1938.95 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 2.1869ms | 4.7784ms | 28.82% | 651314 | 0 | 67.72% | 0.92 | 2093.22 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.7350ms | 5.4376ms | 27.01% | 655373 | 0 | 67.72% | 0.92 | 1673.74 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 3.0353ms | 5.5238ms | 23.82% | 562972 | 0 | 67.72% | 0.92 | 1508.14 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 3.0408ms | 6.3087ms | 28.70% | 567476 | 0 | 67.72% | 0.92 | 1505.39 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 4.6830ms | 5.7069ms | 12.16% | 125097 | 0 | 67.72% | 0.92 | 977.50 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 5.3936ms | 6.3133ms | 18.81% | 174430 | 0 | 67.72% | 0.92 | 848.71 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.7313ms | 5.5719ms | 11.77% | 125097 | 0 | 67.72% | 0.92 | 967.53 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 3.1858ms | 9.9327ms | 23.46% | 651314 | 200000 | 67.72% | 0.92 | 1436.89 MB/s |
| Quicksort | 1000000 | 20.7760ms | 22.2653ms | 3.69% | 9897537 | 0 | 67.71% | 0.92 | 2203.33 MB/s |
| Timsort | 1000000 | 46.5891ms | 47.4310ms | 1.68% | 10922052 | 0 | 67.70% | 0.92 | 982.56 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 33.5841ms | 34.5120ms | 4.19% | 12334533 | 1017407 | 67.72% | 0.92 | 1363.04 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 23.3393ms | 24.8568ms | 4.64% | 5157342 | 1000000 | 67.72% | 0.92 | 1961.34 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 22.3064ms | 24.8705ms | 6.60% | 5157342 | 0 | 67.72% | 0.92 | 2052.17 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 31.9724ms | 33.5800ms | 2.71% | 5154964 | 0 | 67.72% | 0.92 | 1431.75 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 20.3508ms | 20.9683ms | 5.09% | 5184602 | 0 | 67.72% | 0.92 | 2249.36 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 28.8658ms | 31.3002ms | 6.32% | 5199112 | 0 | 67.72% | 0.92 | 1585.84 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 30.2312ms | 31.1567ms | 3.93% | 1057644 | 0 | 67.72% | 0.92 | 1514.21 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 32.1072ms | 33.6540ms | 2.41% | 1056782 | 0 | 67.73% | 0.92 | 1425.74 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 30.5640ms | 31.5947ms | 1.86% | 1036481 | 0 | 67.73% | 0.92 | 1497.72 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.8685ms | 47.3368ms | 3.80% | 10902242 | 2000000 | 67.72% | 0.92 | 1020.23 MB/s |
| Quicksort | 10000000 | 343.6980ms | 353.2947ms | 1.29% | 98662687 | 0 | 67.64% | 0.92 | 1331.88 MB/s |
| Timsort | 10000000 | 734.2574ms | 741.8607ms | 0.82% | 114119813 | 0 | 67.50% | 0.92 | 623.44 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 394.7099ms | 399.8678ms | 1.18% | 121977250 | 10017407 | 67.65% | 0.92 | 1159.75 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 289.6398ms | 301.9884ms | 2.36% | 50791752 | 10000000 | 67.71% | 0.92 | 1580.46 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 283.0043ms | 298.6681ms | 2.37% | 50791752 | 0 | 67.71% | 0.91 | 1617.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 520.1519ms | 532.7914ms | 1.28% | 60557938 | 0 | 67.72% | 0.91 | 880.06 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 266.2813ms | 277.2355ms | 1.83% | 51642229 | 0 | 67.71% | 0.91 | 1719.10 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 493.7907ms | 526.5847ms | 2.31% | 61325447 | 0 | 67.71% | 0.91 | 927.04 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 286.9283ms | 293.3530ms | 1.71% | 19749758 | 0 | 67.73% | 0.92 | 1595.39 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 296.0705ms | 300.2500ms | 1.08% | 19686053 | 0 | 67.74% | 0.91 | 1546.13 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 286.8301ms | 288.7918ms | 1.39% | 19351940 | 0 | 67.73% | 0.92 | 1595.94 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1170.6851ms | 1187.5993ms | 0.67% | 171597557 | 20000000 | 67.69% | 0.92 | 391.02 MB/s |

### Distribution: BucketCollapse

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0391ms | 0.0958ms | 27.46% | 10390 | 0 | 67.63% | 0.92 | 1169.58 MB/s |
| Timsort | 1000 | 0.1128ms | 0.1762ms | 15.44% | 10831 | 0 | 67.63% | 0.92 | 405.75 MB/s |
| ARS Gen 1: Foundation | 1000 | 1.1516ms | 1.1826ms | 1.70% | 0 | 2000 | 67.63% | 0.92 | 39.75 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.7120ms | 1.4139ms | 16.01% | 0 | 2000 | 67.63% | 0.92 | 64.30 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0616ms | 0.0976ms | 18.91% | 10390 | 0 | 67.63% | 0.92 | 743.67 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0365ms | 0.0964ms | 24.28% | 10390 | 0 | 67.63% | 0.92 | 1253.87 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0469ms | 0.0964ms | 24.73% | 10390 | 0 | 67.63% | 0.92 | 975.75 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0920ms | 0.1788ms | 25.09% | 10831 | 0 | 67.63% | 0.92 | 497.53 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0853ms | 0.0973ms | 11.12% | 10390 | 0 | 67.63% | 0.92 | 536.74 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0817ms | 0.1967ms | 21.81% | 10831 | 0 | 67.63% | 0.92 | 560.61 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0320ms | 0.0959ms | 23.59% | 10390 | 0 | 67.63% | 0.92 | 1430.60 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0951ms | 0.0985ms | 16.84% | 10390 | 0 | 67.63% | 0.92 | 481.16 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0356ms | 0.0961ms | 22.13% | 10390 | 0 | 67.63% | 0.92 | 1286.03 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5096ms | 1.4282ms | 23.32% | 10390 | 2000 | 67.63% | 0.92 | 89.83 MB/s |
| Quicksort | 10000 | 1.1788ms | 1.2123ms | 5.00% | 138013 | 0 | 67.63% | 0.92 | 388.34 MB/s |
| Timsort | 10000 | 0.7747ms | 2.0318ms | 20.83% | 141912 | 0 | 67.63% | 0.92 | 590.88 MB/s |
| ARS Gen 1: Foundation | 10000 | 26.2560ms | 28.5024ms | 3.49% | 0 | 30000 | 67.63% | 0.92 | 17.43 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 26.8819ms | 29.8498ms | 3.66% | 0 | 30000 | 67.63% | 0.92 | 17.03 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 0.9331ms | 3.0875ms | 31.43% | 193921 | 14351 | 67.63% | 0.92 | 490.59 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 1.7376ms | 2.0274ms | 8.94% | 52981 | 10000 | 67.63% | 0.92 | 263.45 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3461ms | 1.2412ms | 31.11% | 52981 | 0 | 67.63% | 0.92 | 1322.53 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 0.4769ms | 1.4956ms | 25.56% | 58392 | 0 | 67.63% | 0.92 | 959.94 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.4027ms | 2.1286ms | 28.03% | 59769 | 0 | 67.63% | 0.92 | 326.35 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.6824ms | 2.0864ms | 28.39% | 61996 | 0 | 67.63% | 0.92 | 670.84 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.8979ms | 1.5049ms | 30.17% | 52981 | 0 | 67.63% | 0.92 | 509.83 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 1.0180ms | 1.4434ms | 20.43% | 52981 | 0 | 67.63% | 0.92 | 449.65 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 1.0068ms | 1.5330ms | 31.29% | 52981 | 0 | 67.63% | 0.92 | 454.67 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 0.9652ms | 3.0476ms | 35.83% | 52981 | 20000 | 67.63% | 0.92 | 474.29 MB/s |
| Quicksort | 100000 | 4.9745ms | 14.4376ms | 35.52% | 1719256 | 0 | 67.63% | 0.92 | 920.23 MB/s |
| Timsort | 100000 | 8.8663ms | 23.8355ms | 28.28% | 1754991 | 0 | 67.63% | 0.92 | 516.30 MB/s |
| ARS Gen 1: Foundation | 100000 | 48.0774ms | 70.6622ms | 13.62% | 6 | 300000 | 67.58% | 0.92 | 95.21 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 48.7940ms | 68.9421ms | 11.22% | 6 | 300000 | 67.58% | 0.92 | 93.82 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 5.8870ms | 18.7996ms | 27.73% | 1892508 | 108703 | 67.63% | 0.92 | 777.58 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 6.3433ms | 7.1983ms | 7.63% | 892718 | 100000 | 67.63% | 0.92 | 721.65 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.5525ms | 5.3823ms | 11.53% | 892718 | 0 | 67.63% | 0.92 | 1005.52 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 5.1735ms | 6.2468ms | 10.64% | 927777 | 0 | 67.63% | 0.92 | 884.82 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 5.9745ms | 6.4610ms | 8.42% | 945821 | 0 | 67.63% | 0.92 | 766.19 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 4.4464ms | 7.9585ms | 18.18% | 983099 | 0 | 67.63% | 0.92 | 1029.52 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 5.8328ms | 6.6111ms | 7.40% | 892718 | 0 | 67.63% | 0.92 | 784.81 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 2.8480ms | 5.8926ms | 32.05% | 779688 | 0 | 67.63% | 0.92 | 1607.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 2.3736ms | 6.3247ms | 22.78% | 892718 | 0 | 67.63% | 0.92 | 1928.59 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 4.4656ms | 10.6468ms | 28.02% | 892718 | 200000 | 67.63% | 0.92 | 1025.08 MB/s |
| Quicksort | 1000000 | 42.4199ms | 49.2682ms | 5.42% | 20527770 | 0 | 67.62% | 0.92 | 1079.12 MB/s |
| Timsort | 1000000 | 78.3400ms | 81.1766ms | 2.35% | 20882507 | 0 | 67.61% | 0.92 | 584.33 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 49.3299ms | 51.5605ms | 2.73% | 21591493 | 1017407 | 67.62% | 0.92 | 927.96 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 25.2490ms | 26.2782ms | 25.36% | 10305414 | 1000000 | 67.63% | 0.92 | 1812.99 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 23.6764ms | 24.0627ms | 2.42% | 10305414 | 0 | 67.63% | 0.92 | 1933.42 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 25.0653ms | 25.9424ms | 2.10% | 10716853 | 0 | 67.63% | 0.92 | 1826.28 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 20.4615ms | 24.6330ms | 7.56% | 12980752 | 0 | 67.63% | 0.92 | 2237.19 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 23.3625ms | 27.6685ms | 12.53% | 13386815 | 0 | 67.63% | 0.92 | 1959.39 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 25.6271ms | 27.1783ms | 4.69% | 10305414 | 0 | 67.63% | 0.92 | 1786.25 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.7089ms | 26.9888ms | 3.46% | 11357538 | 0 | 67.63% | 0.92 | 1780.57 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.7937ms | 27.4623ms | 5.40% | 12411927 | 0 | 67.63% | 0.92 | 1774.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 51.9718ms | 55.6997ms | 3.42% | 13803392 | 2000000 | 67.63% | 0.92 | 880.79 MB/s |
| Quicksort | 10000000 | 592.3386ms | 600.8700ms | 1.24% | 238474145 | 0 | 67.53% | 0.92 | 772.81 MB/s |
| Timsort | 10000000 | 1072.2154ms | 1083.1686ms | 0.63% | 242253110 | 0 | 67.40% | 0.92 | 426.93 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 429.3050ms | 437.2091ms | 2.84% | 248315969 | 10017407 | 67.53% | 0.92 | 1066.29 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 241.0960ms | 245.0216ms | 1.39% | 137652965 | 10000000 | 67.60% | 0.92 | 1898.68 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 227.4370ms | 233.2041ms | 1.39% | 137652965 | 0 | 67.60% | 0.92 | 2012.71 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 250.3939ms | 252.9905ms | 1.06% | 141728262 | 0 | 67.53% | 0.92 | 1828.17 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 219.7920ms | 225.6294ms | 1.66% | 165297293 | 0 | 67.57% | 0.92 | 2082.71 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 345.9494ms | 357.5185ms | 1.49% | 169305451 | 0 | 67.51% | 0.92 | 1323.21 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 269.0828ms | 274.3905ms | 1.20% | 46892234 | 0 | 67.56% | 0.92 | 1701.20 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 309.3557ms | 313.5734ms | 0.65% | 52527879 | 0 | 67.62% | 0.92 | 1479.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 275.2319ms | 277.8395ms | 1.36% | 52546730 | 0 | 67.59% | 0.92 | 1663.19 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1245.4534ms | 1255.7293ms | 0.49% | 200024039 | 20000000 | 67.64% | 0.92 | 367.55 MB/s |

### Distribution: LowCardinality

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0158ms | 0.0437ms | 31.48% | 5400 | 0 | 67.66% | 0.92 | 2893.76 MB/s |
| Timsort | 1000 | 0.0890ms | 0.0913ms | 16.56% | 5570 | 0 | 67.66% | 0.92 | 514.08 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.2895ms | 0.3344ms | 8.25% | 984 | 2000 | 67.66% | 0.92 | 158.12 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 0.3240ms | 0.3713ms | 8.07% | 984 | 2000 | 67.66% | 0.92 | 141.29 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0428ms | 0.0454ms | 17.98% | 5400 | 0 | 67.66% | 0.92 | 1070.17 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0139ms | 0.0434ms | 35.23% | 5400 | 0 | 67.66% | 0.92 | 3300.39 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0179ms | 0.0451ms | 30.45% | 5400 | 0 | 67.66% | 0.92 | 2562.21 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.0461ms | 0.0890ms | 24.80% | 5570 | 0 | 67.66% | 0.92 | 992.48 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0156ms | 0.0476ms | 29.99% | 5400 | 0 | 67.66% | 0.92 | 2925.94 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0262ms | 0.1069ms | 32.91% | 5570 | 0 | 67.66% | 0.92 | 1750.26 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0425ms | 0.0461ms | 14.33% | 5400 | 0 | 67.66% | 0.92 | 1076.48 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0226ms | 0.0443ms | 28.39% | 5400 | 0 | 67.66% | 0.92 | 2024.34 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0235ms | 0.0452ms | 24.23% | 5400 | 0 | 67.66% | 0.92 | 1944.54 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 0.5333ms | 1.3794ms | 26.65% | 5400 | 2000 | 67.66% | 0.92 | 85.83 MB/s |
| Quicksort | 10000 | 0.1084ms | 0.3773ms | 32.37% | 54500 | 0 | 67.66% | 0.92 | 4221.74 MB/s |
| Timsort | 10000 | 0.3021ms | 0.9316ms | 24.89% | 54502 | 0 | 67.66% | 0.92 | 1515.20 MB/s |
| ARS Gen 1: Foundation | 10000 | 1.3527ms | 3.4429ms | 22.15% | 9984 | 30000 | 67.66% | 0.92 | 338.42 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 1.3102ms | 3.6456ms | 21.32% | 9984 | 30000 | 67.66% | 0.92 | 349.39 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 2.7403ms | 2.9185ms | 5.97% | 122929 | 14351 | 67.66% | 0.92 | 167.05 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.7021ms | 1.8492ms | 24.19% | 9990 | 10000 | 67.66% | 0.92 | 652.02 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.3534ms | 1.2728ms | 26.05% | 9990 | 0 | 67.66% | 0.92 | 1295.34 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.1132ms | 1.4409ms | 10.10% | 9990 | 0 | 67.66% | 0.92 | 411.21 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.5815ms | 1.8362ms | 12.02% | 9990 | 0 | 67.66% | 0.92 | 289.45 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 1.6697ms | 1.8263ms | 8.24% | 9990 | 0 | 67.66% | 0.92 | 274.16 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 1.0691ms | 1.3669ms | 16.96% | 9990 | 0 | 67.66% | 0.92 | 428.16 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.9427ms | 1.0729ms | 21.38% | 9990 | 0 | 67.66% | 0.92 | 485.61 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.9201ms | 1.1637ms | 34.12% | 9990 | 0 | 67.66% | 0.92 | 497.52 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 1.8943ms | 2.7877ms | 13.73% | 9990 | 20000 | 67.66% | 0.92 | 241.66 MB/s |
| Quicksort | 100000 | 2.7081ms | 4.2880ms | 12.24% | 529144 | 0 | 67.66% | 0.92 | 1690.35 MB/s |
| Timsort | 100000 | 2.5512ms | 9.7483ms | 23.97% | 522900 | 0 | 67.66% | 0.92 | 1794.31 MB/s |
| ARS Gen 1: Foundation | 100000 | 13.0802ms | 17.7931ms | 16.31% | 99984 | 300000 | 67.66% | 0.92 | 349.97 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 7.9708ms | 18.4599ms | 29.92% | 99984 | 300000 | 67.66% | 0.92 | 574.30 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 5.7688ms | 13.1811ms | 30.23% | 1143611 | 108703 | 67.66% | 0.92 | 793.52 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 3.8236ms | 4.7914ms | 11.77% | 119187 | 100000 | 67.66% | 0.92 | 1197.20 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 1.2416ms | 3.5956ms | 31.40% | 119187 | 0 | 67.66% | 0.92 | 3686.90 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 2.8009ms | 4.0415ms | 19.23% | 119445 | 0 | 67.66% | 0.92 | 1634.34 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 1.5817ms | 4.3183ms | 31.85% | 119187 | 0 | 67.66% | 0.92 | 2894.11 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 1.2280ms | 4.5935ms | 31.64% | 119445 | 0 | 67.66% | 0.92 | 3727.58 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 3.2583ms | 4.8284ms | 23.77% | 199982 | 0 | 67.66% | 0.92 | 1404.90 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 3.7684ms | 4.5313ms | 17.07% | 199982 | 0 | 67.66% | 0.92 | 1214.73 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 3.1145ms | 4.2398ms | 15.05% | 99998 | 0 | 67.66% | 0.92 | 1469.79 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 2.4469ms | 8.6252ms | 24.86% | 119187 | 200000 | 67.66% | 0.92 | 1870.77 MB/s |
| Quicksort | 1000000 | 13.3445ms | 13.9532ms | 4.90% | 5201262 | 0 | 67.65% | 0.92 | 3430.37 MB/s |
| Timsort | 1000000 | 34.5263ms | 36.0479ms | 5.57% | 6205056 | 0 | 67.64% | 0.92 | 1325.84 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 29.4431ms | 30.5295ms | 7.25% | 12086377 | 1017407 | 67.66% | 0.92 | 1554.74 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 17.8805ms | 20.3295ms | 5.91% | 999988 | 1000000 | 67.66% | 0.92 | 2560.13 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 18.5355ms | 19.9446ms | 4.38% | 999988 | 0 | 67.66% | 0.92 | 2469.65 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 19.0449ms | 19.4877ms | 8.19% | 999988 | 0 | 67.66% | 0.92 | 2403.60 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 15.5118ms | 17.0389ms | 4.57% | 999988 | 0 | 67.66% | 0.92 | 2951.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 15.5183ms | 16.9066ms | 8.37% | 999988 | 0 | 67.66% | 0.92 | 2949.83 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 21.2803ms | 22.6761ms | 3.04% | 1999972 | 0 | 67.66% | 0.92 | 2151.11 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 20.6639ms | 21.9794ms | 5.38% | 1999972 | 0 | 67.66% | 0.92 | 2215.28 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 20.6060ms | 23.1582ms | 5.56% | 1999972 | 0 | 67.66% | 0.92 | 2221.51 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 44.2638ms | 46.1618ms | 1.78% | 5484518 | 2000000 | 67.65% | 0.92 | 1034.17 MB/s |
| Quicksort | 10000000 | 216.2495ms | 219.2959ms | 1.72% | 51923610 | 0 | 67.64% | 0.92 | 2116.83 MB/s |
| Timsort | 10000000 | 550.2949ms | 559.4813ms | 1.28% | 66958021 | 0 | 67.62% | 0.92 | 831.85 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 334.3384ms | 337.0127ms | 0.55% | 120087440 | 10017407 | 67.65% | 0.92 | 1369.16 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 138.8810ms | 142.0222ms | 2.48% | 9999988 | 10000000 | 67.66% | 0.92 | 3296.09 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 139.8791ms | 141.6357ms | 1.07% | 9999988 | 0 | 67.66% | 0.92 | 3272.57 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 139.6334ms | 142.7096ms | 1.60% | 9999988 | 0 | 67.66% | 0.92 | 3278.32 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 118.8560ms | 120.5460ms | 4.16% | 9999988 | 0 | 67.66% | 0.92 | 3851.41 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 118.0102ms | 121.8861ms | 12.09% | 9999988 | 0 | 67.66% | 0.92 | 3879.02 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 170.9483ms | 174.9453ms | 2.24% | 19999972 | 0 | 67.66% | 0.92 | 2677.79 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 161.1453ms | 165.0526ms | 4.69% | 19999982 | 0 | 67.66% | 0.92 | 2840.69 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 170.4442ms | 173.2773ms | 1.64% | 19999972 | 0 | 67.66% | 0.92 | 2685.71 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1156.4252ms | 1168.8318ms | 0.45% | 116785052 | 20000000 | 67.66% | 0.92 | 395.84 MB/s |

### Distribution: PrefixCollision

| Algorithm | N | Min Time | Median Time | CoV | Comparisons | Moves | LLC Miss Rate | IPC | Throughput |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Quicksort | 1000 | 0.0382ms | 0.0955ms | 24.11% | 10390 | 0 | 67.65% | 0.92 | 1196.96 MB/s |
| Timsort | 1000 | 0.0872ms | 0.1742ms | 28.88% | 10831 | 0 | 67.65% | 0.92 | 524.83 MB/s |
| ARS Gen 1: Foundation | 1000 | 0.5128ms | 1.2141ms | 24.92% | 0 | 2000 | 67.65% | 0.92 | 89.27 MB/s |
| ARS Gen 2: Grid Mapping | 1000 | 1.4077ms | 1.5018ms | 8.20% | 0 | 2000 | 67.65% | 0.92 | 32.52 MB/s |
| ARS Gen 3: Apex Baseline | 1000 | 0.0600ms | 0.1048ms | 25.74% | 10390 | 0 | 67.65% | 0.92 | 762.52 MB/s |
| ARS Gen 4: Parallel Apex | 1000 | 0.0280ms | 0.1048ms | 32.91% | 10390 | 0 | 67.65% | 0.92 | 1633.76 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000 | 0.0953ms | 0.1037ms | 15.53% | 10390 | 0 | 67.65% | 0.92 | 480.39 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000 | 0.1542ms | 0.1865ms | 19.61% | 10831 | 0 | 67.65% | 0.92 | 296.77 MB/s |
| ARS Gen 6: Aero Architecture | 1000 | 0.0321ms | 0.1057ms | 29.76% | 10390 | 0 | 67.65% | 0.92 | 1425.08 MB/s |
| ARS Gen 6: Aero (Stable) | 1000 | 0.0549ms | 0.1847ms | 27.05% | 10831 | 0 | 67.65% | 0.92 | 833.94 MB/s |
| ARS Exp A: Recursive Parallel | 1000 | 0.0647ms | 0.1030ms | 26.54% | 10390 | 0 | 67.65% | 0.92 | 707.92 MB/s |
| ARS Exp B: Hierarchical Staging | 1000 | 0.0323ms | 0.1117ms | 27.66% | 10390 | 0 | 67.65% | 0.92 | 1417.49 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000 | 0.0963ms | 0.1122ms | 33.22% | 10390 | 0 | 67.65% | 0.92 | 475.50 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000 | 1.3154ms | 1.4648ms | 8.85% | 10390 | 2000 | 67.65% | 0.92 | 34.80 MB/s |
| Quicksort | 10000 | 1.1679ms | 1.3420ms | 12.63% | 138013 | 0 | 67.65% | 0.92 | 391.94 MB/s |
| Timsort | 10000 | 1.9371ms | 2.0325ms | 8.88% | 141912 | 0 | 67.65% | 0.92 | 236.32 MB/s |
| ARS Gen 1: Foundation | 10000 | 20.8318ms | 28.2526ms | 11.11% | 0 | 30000 | 67.65% | 0.92 | 21.97 MB/s |
| ARS Gen 2: Grid Mapping | 10000 | 27.9711ms | 29.5754ms | 5.15% | 0 | 30000 | 67.65% | 0.92 | 16.37 MB/s |
| ARS Gen 3: Apex Baseline | 10000 | 1.7207ms | 3.0679ms | 15.86% | 193921 | 14351 | 67.65% | 0.92 | 266.03 MB/s |
| ARS Gen 4: Parallel Apex | 10000 | 0.5964ms | 2.0013ms | 25.65% | 52981 | 10000 | 67.65% | 0.92 | 767.50 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000 | 0.5030ms | 1.3571ms | 23.53% | 52981 | 0 | 67.65% | 0.92 | 910.01 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000 | 1.2052ms | 1.5368ms | 24.13% | 58392 | 0 | 67.65% | 0.92 | 379.83 MB/s |
| ARS Gen 6: Aero Architecture | 10000 | 1.5173ms | 2.1014ms | 16.44% | 59769 | 0 | 67.65% | 0.92 | 301.69 MB/s |
| ARS Gen 6: Aero (Stable) | 10000 | 0.6628ms | 2.0996ms | 35.10% | 61996 | 0 | 67.65% | 0.92 | 690.68 MB/s |
| ARS Exp A: Recursive Parallel | 10000 | 0.7281ms | 1.5516ms | 23.41% | 52981 | 0 | 67.65% | 0.92 | 628.70 MB/s |
| ARS Exp B: Hierarchical Staging | 10000 | 0.6825ms | 1.2500ms | 25.69% | 52981 | 0 | 67.65% | 0.92 | 670.72 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000 | 0.5919ms | 1.1808ms | 20.77% | 52981 | 0 | 67.65% | 0.92 | 773.44 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000 | 2.7171ms | 3.2828ms | 13.24% | 52981 | 20000 | 67.65% | 0.92 | 168.47 MB/s |
| Quicksort | 100000 | 14.0933ms | 14.9855ms | 4.20% | 1719256 | 0 | 67.65% | 0.92 | 324.81 MB/s |
| Timsort | 100000 | 7.9036ms | 23.8100ms | 21.60% | 1754991 | 0 | 67.65% | 0.92 | 579.18 MB/s |
| ARS Gen 1: Foundation | 100000 | 55.7647ms | 69.7440ms | 6.82% | 6 | 300000 | 67.60% | 0.92 | 82.09 MB/s |
| ARS Gen 2: Grid Mapping | 100000 | 50.8702ms | 70.2700ms | 11.18% | 6 | 300000 | 67.59% | 0.92 | 89.99 MB/s |
| ARS Gen 3: Apex Baseline | 100000 | 5.9867ms | 18.4809ms | 22.15% | 1892508 | 108703 | 67.65% | 0.92 | 764.64 MB/s |
| ARS Gen 4: Parallel Apex | 100000 | 6.4254ms | 7.2179ms | 9.85% | 892718 | 100000 | 67.65% | 0.92 | 712.43 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 100000 | 4.5406ms | 5.4831ms | 13.89% | 892718 | 0 | 67.65% | 0.92 | 1008.16 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 100000 | 5.2038ms | 5.8164ms | 10.62% | 927777 | 0 | 67.65% | 0.92 | 879.67 MB/s |
| ARS Gen 6: Aero Architecture | 100000 | 5.3400ms | 6.7318ms | 14.11% | 945821 | 0 | 67.65% | 0.92 | 857.24 MB/s |
| ARS Gen 6: Aero (Stable) | 100000 | 6.3411ms | 7.6683ms | 17.60% | 983099 | 0 | 67.65% | 0.92 | 721.90 MB/s |
| ARS Exp A: Recursive Parallel | 100000 | 5.1220ms | 5.4151ms | 5.00% | 892718 | 0 | 67.65% | 0.92 | 893.71 MB/s |
| ARS Exp B: Hierarchical Staging | 100000 | 4.1951ms | 5.7083ms | 11.37% | 779688 | 0 | 67.65% | 0.92 | 1091.18 MB/s |
| ARS Exp C: Adaptive Hierarchical | 100000 | 4.8807ms | 6.1846ms | 11.34% | 892718 | 0 | 67.65% | 0.92 | 937.91 MB/s |
| ARS Exp D: Stream Micro-Batch | 100000 | 4.1157ms | 11.1361ms | 22.57% | 892718 | 200000 | 67.65% | 0.92 | 1112.24 MB/s |
| Quicksort | 1000000 | 47.1400ms | 49.4878ms | 9.10% | 20527770 | 0 | 67.64% | 0.92 | 971.07 MB/s |
| Timsort | 1000000 | 79.0955ms | 80.9271ms | 1.32% | 20882507 | 0 | 67.63% | 0.92 | 578.75 MB/s |
| ARS Gen 3: Apex Baseline | 1000000 | 46.5280ms | 49.9083ms | 5.68% | 21591493 | 1017407 | 67.64% | 0.92 | 983.85 MB/s |
| ARS Gen 4: Parallel Apex | 1000000 | 24.7477ms | 26.5814ms | 4.21% | 10305414 | 1000000 | 67.65% | 0.92 | 1849.72 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 1000000 | 23.5765ms | 24.8775ms | 4.54% | 10305414 | 0 | 67.65% | 0.92 | 1941.61 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 1000000 | 25.4760ms | 26.1232ms | 3.63% | 10716853 | 0 | 67.65% | 0.92 | 1796.84 MB/s |
| ARS Gen 6: Aero Architecture | 1000000 | 23.4353ms | 24.8874ms | 2.86% | 12980752 | 0 | 67.65% | 0.92 | 1953.30 MB/s |
| ARS Gen 6: Aero (Stable) | 1000000 | 25.8922ms | 27.7029ms | 3.18% | 13386815 | 0 | 67.65% | 0.92 | 1767.96 MB/s |
| ARS Exp A: Recursive Parallel | 1000000 | 25.5874ms | 27.2512ms | 4.70% | 10305414 | 0 | 67.65% | 0.92 | 1789.02 MB/s |
| ARS Exp B: Hierarchical Staging | 1000000 | 25.4788ms | 26.8980ms | 3.03% | 11357538 | 0 | 67.65% | 0.92 | 1796.64 MB/s |
| ARS Exp C: Adaptive Hierarchical | 1000000 | 25.2692ms | 27.9812ms | 4.05% | 12411927 | 0 | 67.65% | 0.92 | 1811.55 MB/s |
| ARS Exp D: Stream Micro-Batch | 1000000 | 51.8842ms | 54.3421ms | 4.07% | 13609096 | 2000000 | 67.65% | 0.92 | 882.28 MB/s |
| Quicksort | 10000000 | 597.8947ms | 603.9750ms | 0.98% | 238474145 | 0 | 67.55% | 0.92 | 765.63 MB/s |
| Timsort | 10000000 | 1065.8473ms | 1085.3536ms | 1.22% | 242253110 | 0 | 67.43% | 0.92 | 429.48 MB/s |
| ARS Gen 3: Apex Baseline | 10000000 | 427.3045ms | 491.1806ms | 12.94% | 248315969 | 10017407 | 67.55% | 0.92 | 1071.28 MB/s |
| ARS Gen 4: Parallel Apex | 10000000 | 240.8116ms | 243.4039ms | 1.18% | 137652965 | 10000000 | 67.62% | 0.91 | 1900.92 MB/s |
| ARS Gen 5: Optimized Apex (MAIN) | 10000000 | 228.7729ms | 235.2405ms | 1.43% | 137652965 | 0 | 67.62% | 0.92 | 2000.95 MB/s |
| ARS Gen 5: Optimized Apex (Stable) | 10000000 | 245.4304ms | 249.2911ms | 0.88% | 141728262 | 0 | 67.55% | 0.91 | 1865.15 MB/s |
| ARS Gen 6: Aero Architecture | 10000000 | 219.9813ms | 226.5184ms | 1.96% | 165297293 | 0 | 67.59% | 0.92 | 2080.92 MB/s |
| ARS Gen 6: Aero (Stable) | 10000000 | 344.8121ms | 355.4130ms | 1.90% | 169305451 | 0 | 67.53% | 0.91 | 1327.57 MB/s |
| ARS Exp A: Recursive Parallel | 10000000 | 268.6916ms | 274.1670ms | 1.35% | 46892234 | 0 | 67.58% | 0.91 | 1703.68 MB/s |
| ARS Exp B: Hierarchical Staging | 10000000 | 310.0400ms | 311.5084ms | 1.48% | 52527879 | 0 | 67.63% | 0.91 | 1476.47 MB/s |
| ARS Exp C: Adaptive Hierarchical | 10000000 | 275.0817ms | 281.0070ms | 1.03% | 52546730 | 0 | 67.61% | 0.91 | 1664.10 MB/s |
| ARS Exp D: Stream Micro-Batch | 10000000 | 1241.7227ms | 1251.1381ms | 0.65% | 200024008 | 20000000 | 67.66% | 0.91 | 368.65 MB/s |
