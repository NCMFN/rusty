# Empirical Benchmarks for HLCS-HFT

This report presents empirical benchmark results for the HLCS-HFT protocol, comparing it to Hash-Only and Lattice-Only baseline schemes.

## 1. Dimension Sweep
Measured latency across different dimensions for hybrid, hash-only, and lattice-only schemes.

| n | Scheme | Mean Latency (us) |
|---|---|---|
| 128 | hash_only | 1.10 |
| 128 | hybrid | 47.45 |
| 128 | lattice_only | 44.39 |
| 192 | hash_only | 1.09 |
| 192 | hybrid | 93.51 |
| 192 | lattice_only | 88.58 |
| 256 | hash_only | 1.11 |
| 256 | hybrid | 155.33 |
| 256 | lattice_only | 150.04 |
| 384 | hash_only | 1.16 |
| 384 | hybrid | 331.27 |
| 384 | lattice_only | 321.22 |
| 512 | hash_only | 1.27 |
| 512 | hybrid | 562.64 |
| 512 | lattice_only | 549.87 |
| 640 | hash_only | 1.77 |
| 640 | hybrid | 863.04 |
| 640 | lattice_only | 847.05 |
| 768 | hash_only | 1.41 |
| 768 | hybrid | 447.59 |
| 768 | lattice_only | 422.33 |
| 896 | hash_only | 1.70 |
| 896 | hybrid | 509.89 |
| 896 | lattice_only | 483.13 |
| 1024 | hash_only | 2.04 |
| 1024 | hybrid | 587.48 |
| 1024 | lattice_only | 558.07 |

## 2. Statistical Rigor
Mean, p50, p95, p99 latencies and jitter across 10 independent trials of 5000 repetitions each.

| n | Scheme | Repetition | Mean (us) | p50 (us) | p95 (us) | p99 (us) | Jitter |
|---|---|---|---|---|---|---|---|
| 128 | hybrid | 0 | 46.93 | 46.00 | 64.00 | 78.00 | 0.1430 |
| 128 | hash_only | 0 | 1.06 | 1.00 | 1.00 | 1.00 | 1.3701 |
| 128 | lattice_only | 0 | 43.90 | 43.00 | 61.00 | 74.00 | 0.1468 |
| 128 | hybrid | 1 | 47.16 | 46.00 | 55.00 | 99.00 | 0.1698 |
| 128 | hash_only | 1 | 1.04 | 1.00 | 1.00 | 1.00 | 0.8947 |
| 128 | lattice_only | 1 | 44.03 | 43.00 | 52.00 | 84.00 | 0.1746 |
| 128 | hybrid | 2 | 47.11 | 46.00 | 64.00 | 79.00 | 0.1547 |
| 128 | hash_only | 2 | 1.06 | 1.00 | 1.00 | 1.00 | 1.1614 |
| 128 | lattice_only | 2 | 44.08 | 43.00 | 59.00 | 76.00 | 0.1784 |
| 128 | hybrid | 3 | 46.98 | 46.00 | 65.00 | 77.00 | 0.1283 |
| 128 | hash_only | 3 | 1.09 | 1.00 | 1.00 | 1.00 | 1.3035 |
| 128 | lattice_only | 3 | 43.86 | 43.00 | 62.00 | 73.00 | 0.1337 |
| 128 | hybrid | 4 | 47.23 | 46.00 | 66.00 | 78.00 | 0.1369 |
| 128 | hash_only | 4 | 1.05 | 1.00 | 1.00 | 1.00 | 1.0170 |
| 128 | lattice_only | 4 | 44.07 | 43.00 | 62.00 | 74.00 | 0.1463 |
| 128 | hybrid | 5 | 47.18 | 46.00 | 66.00 | 78.00 | 0.1365 |
| 128 | hash_only | 5 | 1.05 | 1.00 | 1.00 | 1.00 | 0.8841 |
| 128 | lattice_only | 5 | 44.10 | 43.00 | 62.00 | 75.00 | 0.1506 |
| 128 | hybrid | 6 | 47.21 | 46.00 | 66.00 | 78.00 | 0.1449 |
| 128 | hash_only | 6 | 1.09 | 1.00 | 1.00 | 1.00 | 1.2822 |
| 128 | lattice_only | 6 | 44.11 | 43.00 | 62.00 | 75.00 | 0.1464 |
| 128 | hybrid | 7 | 47.20 | 46.00 | 66.00 | 78.00 | 0.1433 |
| 128 | hash_only | 7 | 1.06 | 1.00 | 1.00 | 1.00 | 0.9805 |
| 128 | lattice_only | 7 | 43.97 | 43.00 | 62.00 | 74.00 | 0.1409 |
| 128 | hybrid | 8 | 46.98 | 46.00 | 65.00 | 77.00 | 0.1297 |
| 128 | hash_only | 8 | 1.05 | 1.00 | 1.00 | 1.00 | 0.9536 |
| 128 | lattice_only | 8 | 43.96 | 43.00 | 62.00 | 74.00 | 0.1357 |
| 128 | hybrid | 9 | 47.13 | 46.00 | 66.00 | 78.00 | 0.1349 |
| 128 | hash_only | 9 | 1.05 | 1.00 | 1.00 | 1.00 | 0.9368 |
| 128 | lattice_only | 9 | 44.06 | 43.00 | 62.00 | 75.00 | 0.1446 |
| 512 | hybrid | 0 | 561.17 | 557.00 | 603.00 | 624.00 | 0.0470 |
| 512 | hash_only | 0 | 1.14 | 1.00 | 2.00 | 2.00 | 0.9876 |
| 512 | lattice_only | 0 | 548.14 | 545.00 | 590.00 | 612.00 | 0.0420 |
| 512 | hybrid | 1 | 562.44 | 558.00 | 608.00 | 637.00 | 0.0503 |
| 512 | hash_only | 1 | 1.19 | 1.00 | 2.00 | 2.00 | 1.2693 |
| 512 | lattice_only | 1 | 550.02 | 545.00 | 596.00 | 625.00 | 0.0534 |
| 512 | hybrid | 2 | 564.21 | 559.00 | 612.00 | 647.00 | 0.0479 |
| 512 | hash_only | 2 | 1.21 | 1.00 | 2.00 | 2.00 | 1.1399 |
| 512 | lattice_only | 2 | 550.87 | 546.00 | 599.00 | 634.00 | 0.0483 |
| 512 | hybrid | 3 | 566.69 | 563.00 | 616.00 | 641.00 | 0.0470 |
| 512 | hash_only | 3 | 1.34 | 1.00 | 2.00 | 4.00 | 1.5966 |
| 512 | lattice_only | 3 | 553.06 | 549.00 | 602.00 | 628.00 | 0.0463 |
| 512 | hybrid | 4 | 564.43 | 560.00 | 615.00 | 641.00 | 0.0473 |
| 512 | hash_only | 4 | 1.21 | 1.00 | 2.00 | 2.00 | 1.2013 |
| 512 | lattice_only | 4 | 551.47 | 547.00 | 602.00 | 628.00 | 0.0477 |
| 512 | hybrid | 5 | 565.60 | 561.00 | 612.00 | 648.00 | 0.0693 |
| 512 | hash_only | 5 | 1.23 | 1.00 | 2.00 | 2.00 | 1.3599 |
| 512 | lattice_only | 5 | 551.95 | 548.00 | 598.00 | 639.00 | 0.0494 |
| 512 | hybrid | 6 | 646.11 | 577.00 | 878.00 | 951.00 | 0.1905 |
| 512 | hash_only | 6 | 1.81 | 1.00 | 5.00 | 8.00 | 1.4179 |
| 512 | lattice_only | 6 | 630.94 | 565.00 | 858.00 | 935.00 | 0.1905 |
| 512 | hybrid | 7 | 591.56 | 564.00 | 783.00 | 1123.00 | 0.1753 |
| 512 | hash_only | 7 | 1.37 | 1.00 | 2.00 | 8.00 | 1.5606 |
| 512 | lattice_only | 7 | 576.65 | 551.00 | 763.00 | 1043.00 | 0.1741 |
| 512 | hybrid | 8 | 562.69 | 557.00 | 614.00 | 641.00 | 0.0489 |
| 512 | hash_only | 8 | 1.18 | 1.00 | 2.00 | 2.00 | 1.2729 |
| 512 | lattice_only | 8 | 549.61 | 545.00 | 599.00 | 625.00 | 0.0472 |
| 512 | hybrid | 9 | 562.82 | 560.00 | 606.00 | 632.00 | 0.0424 |
| 512 | hash_only | 9 | 1.23 | 1.00 | 2.00 | 2.00 | 1.5375 |
| 512 | lattice_only | 9 | 550.23 | 547.00 | 594.00 | 623.00 | 0.0443 |
| 1024 | hybrid | 0 | 589.96 | 583.00 | 647.00 | 731.00 | 0.0911 |
| 1024 | hash_only | 0 | 1.77 | 2.00 | 2.00 | 3.00 | 1.4599 |
| 1024 | lattice_only | 0 | 541.42 | 535.00 | 603.00 | 688.00 | 0.1125 |
| 1024 | hybrid | 1 | 596.56 | 589.00 | 659.00 | 759.00 | 0.0936 |
| 1024 | hash_only | 1 | 1.78 | 2.00 | 2.00 | 3.00 | 1.2616 |
| 1024 | lattice_only | 1 | 545.43 | 541.00 | 611.00 | 716.00 | 0.0999 |
| 1024 | hybrid | 2 | 595.80 | 592.00 | 643.00 | 689.00 | 0.0860 |
| 1024 | hash_only | 2 | 1.86 | 2.00 | 2.00 | 3.00 | 1.1311 |
| 1024 | lattice_only | 2 | 550.53 | 547.00 | 602.00 | 648.00 | 0.0951 |
| 1024 | hybrid | 3 | 624.50 | 620.00 | 683.00 | 801.00 | 0.1151 |
| 1024 | hash_only | 3 | 1.75 | 2.00 | 2.00 | 2.00 | 0.7375 |
| 1024 | lattice_only | 3 | 575.49 | 574.00 | 634.00 | 787.00 | 0.1265 |
| 1024 | hybrid | 4 | 599.47 | 594.00 | 652.00 | 718.00 | 0.0825 |
| 1024 | hash_only | 4 | 1.79 | 2.00 | 2.00 | 3.00 | 1.0080 |
| 1024 | lattice_only | 4 | 551.66 | 549.00 | 609.00 | 674.00 | 0.0993 |
| 1024 | hybrid | 5 | 587.04 | 578.00 | 653.00 | 697.00 | 0.0813 |
| 1024 | hash_only | 5 | 1.65 | 2.00 | 2.00 | 2.00 | 0.8644 |
| 1024 | lattice_only | 5 | 541.59 | 536.00 | 609.00 | 659.00 | 0.0984 |
| 1024 | hybrid | 6 | 581.97 | 576.00 | 631.00 | 742.00 | 0.0953 |
| 1024 | hash_only | 6 | 1.71 | 2.00 | 2.00 | 3.00 | 1.1245 |
| 1024 | lattice_only | 6 | 537.12 | 533.00 | 587.00 | 705.00 | 0.1229 |
| 1024 | hybrid | 7 | 589.95 | 583.00 | 649.00 | 700.00 | 0.1112 |
| 1024 | hash_only | 7 | 1.50 | 1.00 | 2.00 | 2.00 | 0.9361 |
| 1024 | lattice_only | 7 | 537.54 | 532.00 | 600.00 | 672.00 | 0.1144 |
| 1024 | hybrid | 8 | 592.93 | 585.00 | 649.00 | 755.00 | 0.1488 |
| 1024 | hash_only | 8 | 1.80 | 2.00 | 2.00 | 3.00 | 1.3542 |
| 1024 | lattice_only | 8 | 543.65 | 541.00 | 604.00 | 680.00 | 0.1287 |
| 1024 | hybrid | 9 | 598.08 | 590.00 | 651.00 | 749.00 | 0.1081 |
| 1024 | hash_only | 9 | 1.69 | 2.00 | 2.00 | 3.00 | 0.8782 |
| 1024 | lattice_only | 9 | 549.16 | 545.00 | 606.00 | 667.00 | 0.1112 |

## 3. ZK Proofs
Proof and verification generation times for different proof rounds.

| n | Rounds | Mean Prove (us) | Mean Verify (us) |
|---|---|---|---|
| 128 | 1 | 28.50 | 23.48 |
| 128 | 2 | 56.85 | 46.19 |
| 128 | 3 | 85.97 | 68.15 |
| 128 | 5 | 141.48 | 115.21 |
| 128 | 7 | 202.58 | 164.69 |
| 128 | 10 | 285.32 | 233.25 |
| 128 | 15 | 426.14 | 347.38 |
| 128 | 20 | 566.13 | 463.71 |
| 512 | 1 | 306.75 | 273.19 |
| 512 | 2 | 612.14 | 544.17 |
| 512 | 3 | 920.15 | 817.47 |
| 512 | 5 | 1540.04 | 1372.60 |
| 512 | 7 | 2148.09 | 1908.70 |
| 512 | 10 | 3058.83 | 2727.31 |
| 512 | 15 | 4631.15 | 4113.71 |
| 512 | 20 | 6228.93 | 5481.25 |
| 1024 | 1 | 308.23 | 301.70 |
| 1024 | 2 | 639.14 | 544.11 |
| 1024 | 3 | 958.73 | 836.23 |
| 1024 | 5 | 1652.42 | 1423.74 |
| 1024 | 7 | 2352.06 | 2002.38 |
| 1024 | 10 | 3363.29 | 2786.29 |
| 1024 | 15 | 5167.01 | 4302.42 |
| 1024 | 20 | 6837.44 | 5565.62 |

## 4. Observations & Deviations
- All latencies reported are empirical distributions based on real independent implementations of lattice-only, hash-only, and hybrid protocols.
- **Important Deviation:** Previous narrative claims (e.g., exactly 0.92ms latency at n=1024 or a rigid 20.1x speedup over lattice-only) were static projections and multipliers. The numbers in the tables above represent genuine measurements of real CPU time.
- **Sandbox Limitations:** These benchmarks were executed in an isolated container without kernel-bypass networking, NUMA topology guarantees, or fixed clock frequencies. Absolute microsecond values are indicative of relative algorithmic costs rather than exact real-world hardware limits.
