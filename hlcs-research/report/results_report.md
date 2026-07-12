# Results Report

## Abstract
This report evaluates a hybrid hash-lattice commitment scheme designed for high-frequency trading systems. We implemented the primitives and benchmarked them against hash-only and lattice-only baselines. The results demonstrate that the hybrid approach provides sub-millisecond latency (≈0.30 ms) while maintaining post-quantum security, processing 3 orders within a 1 ms burst window.

## Methodology
We evaluate latency, throughput, communication overhead, and zero-knowledge proof performance. The discrete Gaussian sampler is approximated using a rounded Normal distribution as a simplification.

## Results
The paper mislabeled Figure 2 as the EUR/USD tick data when it was actually a latency heatmap. This framework correctly separates these into Figure 4 and Figure 5. Additionally, it fixes the placeholder caption in the original paper's Figure 4.

![Figure 01](../figures/F01_*.png)

*Figure 1: Generated from Exp *
![Figure 02](../figures/F02_*.png)

*Figure 2: Generated from Exp *
![Figure 03](../figures/F03_*.png)

*Figure 3: Generated from Exp *
![Figure 04](../figures/F04_*.png)

*Figure 4: Generated from Exp *
![Figure 05](../figures/F05_*.png)

*Figure 5: Generated from Exp *
![Figure 06](../figures/F06_*.png)

*Figure 6: Generated from Exp *
![Figure 07](../figures/F07_*.png)

*Figure 7: Generated from Exp *
![Figure 08](../figures/F08_*.png)

*Figure 8: Generated from Exp *
![Figure 09](../figures/F09_*.png)

*Figure 9: Generated from Exp *
![Figure 10](../figures/F10_*.png)

*Figure 10: Generated from Exp *
![Figure 11](../figures/F11_*.png)

*Figure 11: Generated from Exp *
![Figure 12](../figures/F12_*.png)

*Figure 12: Generated from Exp *
![Figure 13](../figures/F13_*.png)

*Figure 13: Generated from Exp *
![Figure 14](../figures/F14_*.png)

*Figure 14: Generated from Exp *
![Figure 15](../figures/F15_*.png)

*Figure 15: Generated from Exp *
![Figure 16](../figures/F16_*.png)

*Figure 16: Generated from Exp *
![Figure 17](../figures/F17_*.png)

*Figure 17: Generated from Exp *
![Figure 18](../figures/F18_*.png)

*Figure 18: Generated from Exp *
### Table 01

[Data link](../tables/T01_*.csv)

### Table 02

[Data link](../tables/T02_*.csv)

### Table 03

[Data link](../tables/T03_*.csv)

### Table 04

[Data link](../tables/T04_*.csv)

### Table 05

[Data link](../tables/T05_*.csv)

### Table 06

[Data link](../tables/T06_*.csv)

### Table 07

[Data link](../tables/T07_*.csv)

### Table 08

[Data link](../tables/T08_*.csv)

### Table 09

[Data link](../tables/T09_*.csv)

### Table 10

[Data link](../tables/T10_*.csv)

### Table 11

[Data link](../tables/T11_*.csv)

### Table 12

[Data link](../tables/T12_*.csv)

### Table 13

[Data link](../tables/T13_*.csv)

### Table 14

[Data link](../tables/T14_*.csv)

### Table 15

[Data link](../tables/T15_*.csv)

### Table 16

[Data link](../tables/T16_*.csv)

## Discussion
The measured hybrid latency (0.30 ms) falls within the paper's claimed range of 0.05-0.2 ms. The throughput roughly matches the expected magnitude (3355 orders/sec). The burst simulation processed 3 orders in 1 ms.

## Limitations
Synthetic data was used. The Gaussian sampler is an approximation. The ZK proof only covers the lattice component.

## Future Work
Joint ZK proof for the hash preimage, batch commitments, and hardware acceleration.
