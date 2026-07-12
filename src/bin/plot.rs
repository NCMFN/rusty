use std::process::Command;

fn main() {
    println!("Generating plots using Python (matplotlib)...");
    let script = r#"
import pandas as pd
import matplotlib.pyplot as plt
import os

if not os.path.exists('bench_results_latency_vs_dim.csv'):
    print('No data found to plot')
    exit(0)

# Latency vs n plot
df_lat = pd.read_csv('bench_results_latency_vs_dim.csv')
agg = df_lat.groupby(['scheme', 'n']).mean().reset_index()

plt.figure(figsize=(8, 6))
for scheme in ['hash_only', 'hybrid', 'lattice_only']:
    subset = agg[agg['scheme'] == scheme]
    plt.plot(subset['n'], subset['latency_us'] / 1000.0, marker='o', label=scheme)

plt.axhline(y=1.0, color='r', linestyle='--', label='1ms SLA')
plt.xlabel('Dimension (n)')
plt.ylabel('Mean Latency (ms)')
plt.title('Mean Latency vs n')
plt.legend()
plt.grid(True)
plt.savefig('latency_vs_n.png')
print('Saved latency_vs_n.png')

# Throughput vs n plot
df_thru = pd.read_csv('throughput.csv')
plt.figure(figsize=(8, 6))
for scheme in ['hash_only', 'hybrid', 'lattice_only']:
    subset = df_thru[df_thru['scheme'] == scheme]
    if not subset.empty:
        plt.plot(subset['n'], subset['throughput_ops'], marker='o', label=scheme)

plt.xlabel('Dimension (n)')
plt.ylabel('Throughput (ops per sec)')
plt.title('Throughput vs n')
plt.legend()
plt.grid(True)
plt.savefig('throughput_vs_n.png')
print('Saved throughput_vs_n.png')
"#;

    std::fs::write("plot.py", script).unwrap();

    let status = Command::new("python3")
        .arg("-c")
        .arg("import pandas, matplotlib")
        .status();

    if status.is_ok() && status.unwrap().success() {
        let _ = Command::new("python3").arg("plot.py").status();
    } else {
        println!("Please run: pip install pandas matplotlib && python3 plot.py");
    }
}
