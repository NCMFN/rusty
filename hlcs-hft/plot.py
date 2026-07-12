import csv
import os
import matplotlib.pyplot as plt
from collections import defaultdict
import numpy as np

def read_csv(path):
    with open(path, 'r') as f:
        reader = csv.DictReader(f)
        return list(reader)

def plot_base_sweep():
    data = read_csv('bench_results/base_sweep.csv')
    schemes = defaultdict(lambda: {'n': [], 'mean': [], 'p99': [], 'ci95': []})
    for row in data:
        s = row['scheme']
        schemes[s]['n'].append(int(row['n']))
        schemes[s]['mean'].append(float(row['mean_ms']))
        schemes[s]['p99'].append(float(row['p99_ms']))
        schemes[s]['ci95'].append(float(row['ci95_ms']))

    plt.figure(figsize=(8, 5))
    for s, vals in schemes.items():
        plt.errorbar(vals['n'], vals['mean'], yerr=vals['ci95'], label=s, marker='o', capsize=5)
    plt.xlabel('n (LWE dimension)')
    plt.ylabel('Mean Latency (ms)')
    plt.title('Latency vs Dimension')
    plt.legend()
    plt.grid(True)
    plt.savefig('bench_results/base_sweep_latency.png')
    plt.close()

def plot_param_sensitivity():
    data = read_csv('bench_results/param_sensitivity.csv')
    sigmas = [float(r['sigma']) for r in data]
    means = [float(r['mean_ms']) for r in data]

    plt.figure(figsize=(8, 5))
    plt.plot(sigmas, means, marker='o', linestyle='-')
    plt.xlabel('Sigma (Gaussian Std Dev)')
    plt.ylabel('Mean Latency (ms)')
    plt.title('Latency vs Sigma (n=512)')
    plt.grid(True)
    plt.savefig('bench_results/param_sensitivity.png')
    plt.close()

def plot_hardware_ablation():
    data = read_csv('bench_results/hardware_ablation.csv')
    modes = defaultdict(lambda: {'n': [], 'mean': []})
    for row in data:
        modes[row['mode']]['n'].append(int(row['n']))
        modes[row['mode']]['mean'].append(float(row['mean_ms']))

    plt.figure(figsize=(8, 5))
    for m, vals in modes.items():
        plt.plot(vals['n'], vals['mean'], label=m, marker='o')
    plt.xlabel('n (LWE dimension)')
    plt.ylabel('Mean Latency (ms)')
    plt.title('Hardware Ablation (Cold vs Warm Cache)')
    plt.legend()
    plt.grid(True)
    plt.savefig('bench_results/hardware_ablation.png')
    plt.close()

def plot_zk_sweep():
    data = read_csv('bench_results/zk_sweep.csv')
    ns = sorted(list(set(int(r['n']) for r in data)))

    plt.figure(figsize=(8, 5))
    for n in ns:
        rounds = [int(r['rounds']) for r in data if int(r['n']) == n]
        prove_time = [float(r['prove_ms']) for r in data if int(r['n']) == n]
        plt.plot(rounds, prove_time, label=f'n={n}', marker='o')

    plt.xlabel('ZK Rounds')
    plt.ylabel('Prover Time (ms)')
    plt.title('ZK Prover Cost vs Rounds')
    plt.legend()
    plt.grid(True)
    plt.savefig('bench_results/zk_sweep_prover.png')
    plt.close()

def generate_markdown():
    with open('RESULTS.md', 'w') as f:
        f.write("# HLCS-HFT Benchmark Results\n\n")
        f.write("## 1. Base Sweep (Latency vs Dimension)\n")
        f.write("![Base Sweep](bench_results/base_sweep_latency.png)\n\n")

        f.write("## 2. Parameter Sensitivity (Sigma)\n")
        f.write("![Parameter Sensitivity](bench_results/param_sensitivity.png)\n\n")

        f.write("## 3. Hardware / Caching Ablation\n")
        f.write("![Hardware Ablation](bench_results/hardware_ablation.png)\n\n")

        f.write("## 4. Zero-Knowledge Proof Cost vs Rounds\n")
        f.write("![ZK Sweep](bench_results/zk_sweep_prover.png)\n\n")

        f.write("## 5. Environment\n")
        f.write("Results generated via standard CPU executing Rust `hlcs-hft` reference implementation.\n")

if __name__ == '__main__':
    plot_base_sweep()
    plot_param_sensitivity()
    plot_hardware_ablation()
    plot_zk_sweep()
    generate_markdown()
