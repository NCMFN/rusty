import os
import csv
from collections import defaultdict

def main():
    bench_dir = "bench_results"
    md_path = "RESULTS.md"

    with open(md_path, "w") as f:
        f.write("# Empirical Benchmarks for HLCS-HFT\n\n")
        f.write("This report presents empirical benchmark results for the HLCS-HFT protocol, comparing it to Hash-Only and Lattice-Only baseline schemes.\n\n")

        # 1. Dimension Sweep (Exp A)
        f.write("## 1. Dimension Sweep\n")
        if os.path.exists(f"{bench_dir}/exp_a_latency.csv"):
            f.write("Measured latency across different dimensions for hybrid, hash-only, and lattice-only schemes.\n\n")
            f.write("| n | Scheme | Mean Latency (us) |\n")
            f.write("|---|---|---|\n")
            data = defaultdict(list)
            with open(f"{bench_dir}/exp_a_latency.csv") as csvfile:
                reader = csv.DictReader(csvfile)
                for row in reader:
                    data[(row['n'], row['scheme'])].append(int(row['latency_us']))

            for (n, scheme), lats in sorted(data.items(), key=lambda x: (int(x[0][0]), x[0][1])):
                mean_lat = sum(lats) / len(lats)
                f.write(f"| {n} | {scheme} | {mean_lat:.2f} |\n")
        f.write("\n")

        # 2. Statistical Rigor (Exp C)
        f.write("## 2. Statistical Rigor\n")
        if os.path.exists(f"{bench_dir}/exp_c_stats.csv"):
            f.write("Mean, p50, p95, p99 latencies and jitter across 10 independent trials of 5000 repetitions each.\n\n")
            f.write("| n | Scheme | Repetition | Mean (us) | p50 (us) | p95 (us) | p99 (us) | Jitter |\n")
            f.write("|---|---|---|---|---|---|---|---|\n")
            with open(f"{bench_dir}/exp_c_stats.csv") as csvfile:
                reader = csv.DictReader(csvfile)
                for row in reader:
                    f.write(f"| {row['n']} | {row['scheme']} | {row['rep']} | {float(row['mean']):.2f} | {float(row['p50']):.2f} | {float(row['p95']):.2f} | {float(row['p99']):.2f} | {float(row['jitter']):.4f} |\n")
        f.write("\n")

        # 3. ZK Rounds (Exp G)
        f.write("## 3. ZK Proofs\n")
        if os.path.exists(f"{bench_dir}/exp_g_zk.csv"):
            f.write("Proof and verification generation times for different proof rounds.\n\n")
            f.write("| n | Rounds | Mean Prove (us) | Mean Verify (us) |\n")
            f.write("|---|---|---|---|\n")
            zk_data = defaultdict(lambda: {"prove": [], "verify": []})
            with open(f"{bench_dir}/exp_g_zk.csv") as csvfile:
                reader = csv.DictReader(csvfile)
                for row in reader:
                    zk_data[(row['n'], row['rounds'])]['prove'].append(int(row['prove_us']))
                    zk_data[(row['n'], row['rounds'])]['verify'].append(int(row['verify_us']))

            for (n, r), times in sorted(zk_data.items(), key=lambda x: (int(x[0][0]), int(x[0][1]))):
                m_prove = sum(times['prove']) / len(times['prove'])
                m_verify = sum(times['verify']) / len(times['verify'])
                f.write(f"| {n} | {r} | {m_prove:.2f} | {m_verify:.2f} |\n")
        f.write("\n")

        # Callouts
        f.write("## 4. Observations & Deviations\n")
        f.write("- All latencies reported are empirical distributions based on real independent implementations of lattice-only, hash-only, and hybrid protocols.\n")
        f.write("- **Important Deviation:** Previous narrative claims (e.g., exactly 0.92ms latency at n=1024 or a rigid 20.1x speedup over lattice-only) were static projections and multipliers. The numbers in the tables above represent genuine measurements of real CPU time.\n")
        f.write("- **Sandbox Limitations:** These benchmarks were executed in an isolated container without kernel-bypass networking, NUMA topology guarantees, or fixed clock frequencies. Absolute microsecond values are indicative of relative algorithmic costs rather than exact real-world hardware limits.\n")

if __name__ == "__main__":
    main()
