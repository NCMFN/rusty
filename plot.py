import pandas as pd
import matplotlib.pyplot as plt
import subprocess
import os

# Create plot directory
os.makedirs("hlcs-hft/plots", exist_ok=True)

# 1. Latency & Throughput
df_lt = pd.read_csv("hlcs-hft/bench_results/latency_throughput.csv")

plt.figure(figsize=(10, 6))
for scheme in df_lt['scheme'].unique():
    subset = df_lt[df_lt['scheme'] == scheme]
    plt.plot(subset['n'], subset['mean_latency_ms'], marker='o', label=scheme)
plt.yscale('log')
plt.xlabel('n (Lattice Dimension)')
plt.ylabel('Mean Latency (ms)')
plt.title('Commit + Verify Latency vs. Lattice Dimension')
plt.legend()
plt.grid(True, which="both", ls="--", alpha=0.5)
plt.savefig("hlcs-hft/plots/latency.png")
plt.close()

plt.figure(figsize=(10, 6))
for scheme in df_lt['scheme'].unique():
    subset = df_lt[df_lt['scheme'] == scheme]
    plt.plot(subset['n'], subset['ops_per_sec'], marker='o', label=scheme)
plt.yscale('log')
plt.xlabel('n (Lattice Dimension)')
plt.ylabel('Throughput (ops/sec)')
plt.title('Throughput vs. Lattice Dimension')
plt.legend()
plt.grid(True, which="both", ls="--", alpha=0.5)
plt.savefig("hlcs-hft/plots/throughput.png")
plt.close()

# 2. Bandwidth
df_bw = pd.read_csv("hlcs-hft/bench_results/bandwidth.csv")

plt.figure(figsize=(10, 6))
for scheme in df_bw['scheme'].unique():
    subset = df_bw[df_bw['scheme'] == scheme]
    plt.plot(subset['n'], subset['commit_size_bytes'], marker='o', label=scheme)
plt.xlabel('n (Lattice Dimension)')
plt.ylabel('Commitment Size (bytes)')
plt.title('Commitment Size vs. Lattice Dimension')
plt.legend()
plt.grid(True, ls="--", alpha=0.5)
plt.savefig("hlcs-hft/plots/bandwidth.png")
plt.close()

# 3. SLA Breach Heatmap
df_overload = pd.read_csv("hlcs-hft/bench_results/overload_p99.csv")
heatmap_data = df_overload.pivot(index='n', columns='load_rate', values='p99_latency_ms')

plt.figure(figsize=(10, 8))
plt.imshow(heatmap_data, cmap='coolwarm', aspect='auto')
plt.colorbar(label='p99 Latency (ms)')
plt.xticks(range(len(heatmap_data.columns)), heatmap_data.columns)
plt.yticks(range(len(heatmap_data.index)), heatmap_data.index)
plt.xlabel('Load Rate (ops/sec)')
plt.ylabel('n (Lattice Dimension)')
plt.title('p99 Latency under Simulated Load')

# Annotate breaches
for i in range(len(heatmap_data.index)):
    for j in range(len(heatmap_data.columns)):
        val = heatmap_data.iloc[i, j]
        color = 'white' if val < 1.0 else 'black'
        plt.text(j, i, f"{val:.2f}", ha="center", va="center", color=color, fontsize=8)

plt.savefig("hlcs-hft/plots/sla_heatmap.png")
plt.close()

# 4. ZK Proof
df_zk = pd.read_csv("hlcs-hft/bench_results/zk_proof.csv")


# Generate RESULTS.md
try:
    cpu_info = subprocess.check_output("lscpu | grep 'Model name'", shell=True).decode('utf-8').strip().split(':')[1].strip()
except Exception:
    cpu_info = "Unknown"

with open("hlcs-hft/RESULTS.md", "w") as f:
    f.write("# HLCS-HFT Benchmarking Results\n\n")
    f.write(f"**Hardware Details:** CPU: {cpu_info}\n\n")

    f.write("## 1. Latency and Throughput\n\n")
    f.write(df_lt.to_markdown(index=False) + "\n\n")
    f.write("![Latency](plots/latency.png)\n")
    f.write("![Throughput](plots/throughput.png)\n\n")

    f.write("## 2. Bandwidth (Commitment Size)\n\n")
    f.write(df_bw.to_markdown(index=False) + "\n\n")
    f.write("![Bandwidth](plots/bandwidth.png)\n\n")

    f.write("## 3. P99 Latency & Jitter (Simulated Overload)\n\n")
    f.write(df_overload.to_markdown(index=False) + "\n\n")
    f.write("![SLA Heatmap](plots/sla_heatmap.png)\n\n")

    f.write("## 4. ZK Proof Timing & Size\n\n")
    f.write(df_zk.to_markdown(index=False) + "\n\n")

    f.write("## Discussion / Deviations\n\n")
    f.write("This report contains real measurements taken within the benchmark sandbox environment. Values may differ from theoretically projected or anticipated values based on ideal conditions (such as perfect parallelization scaling or extremely high-end hardware). For instance:\n")
    f.write("- **n=1024 hybrid latency:** See table above, might differ from '0.92ms' depending on this sandbox's CPU speed.\n")
    f.write("- **Hybrid vs Lattice-only throughput:** Parallelism via Rayon is implemented for n>=768. The speedup ratio in this environment might be constrained by the number of physical/virtual cores available compared to a dedicated HFT rig.\n")
    f.write("- **Bandwidth:** Calculated using actual vector lengths * element size, rather than just abstract theoretical bounds.\n")
    f.write("- **ZK Proofs:** Measured over actual operations rather than projections.\n")
    f.write("\nNote: Allocations per commit were not profiled here via a custom allocator hook but the throughput constraints reflect standard Rust standard library allocations on the hot path.\n")
