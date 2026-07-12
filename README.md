# HLCS-HFT: A Sub-Millisecond Hybrid Hash-Lattice Commitment Scheme

This repository contains the reproduction of the research on a Hybrid Hash-Lattice Commitment Scheme for High-Frequency Trading (HFT) environments.

## Overview
The codebase implements the hybrid commitment scheme described in the paper, optimized with:
- Cache-aware memory layout using `u16` matrix storage
- Parallelization using `rayon` for matrix-vector multiplication
- Single modular reduction strategy in the hot loop
- Zero-allocation hot paths

## Reproducing Results

To generate the evaluation metrics (latency, throughput, bandwidth, overload, and zero-knowledge benchmarks):

1. **Run the core benchmarks**
   ```bash
   cargo run --release --bin benchmark
   ```
   This generates `bench_results_latency_vs_dim.csv`, `throughput.csv`, `bandwidth.csv`, and `overload.csv`.

2. **Run the zero-knowledge benchmarks**
   ```bash
   cargo run --release --bin zk_benchmark
   ```
   This generates `zk_benchmark.csv`.

3. **Plot the results** (requires python with `pandas` and `matplotlib`)
   ```bash
   cargo run --release --bin plot
   ```
