import os
import subprocess
import sys
import csv
import matplotlib.pyplot as plt

def run_cmd(cmd, cwd=None):
    subprocess.run(cmd, check=True, cwd=cwd, stdout=subprocess.DEVNULL)

def run_bench(cmd):
    res = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return float(res.stdout.strip())

def main():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    
    print("=== Building Binaries ===")
    run_cmd(["cargo", "build", "--release", "-p", "dataset_generator", "-p", "ars_harness"], cwd=os.path.join(base_dir, "..", ".."))
    
    run_cmd(["clang++", "-O3", "-march=native", "-std=c++17", "learned_harness.cpp", "-o", "learned_harness"], cwd=base_dir)
    
    sizes = [10_000, 100_000, 1_000_000, 10_000_000]
    distributions = [
        "Random", "Gaussian", "NearlySorted", "Duplicates", 
        "Zipfian", "Skewed", "Clustered", "BucketCollapse", 
        "LowCardinality", "PrefixCollision"
    ]
    
    gen_exe = os.path.join(base_dir, "..", "..", "target", "release", "dataset_generator")
    ars_exe = os.path.join(base_dir, "..", "..", "target", "release", "ars_harness")
    learned_exe = os.path.join(base_dir, "learned_harness")
    
    results = []
    
    print(f"{'Size':<10} | {'Distribution':<18} | {'ARS (ms)':<10} | {'Learned (ms)':<12} | {'Speedup':<8}")
    print("-" * 65)
    
    for size in sizes:
        for dist in distributions:
            ds_path = os.path.join(base_dir, "temp.bin")
            # Generate
            run_cmd([gen_exe, str(size), dist, ds_path])
            
            try:
                ars_time = run_bench([ars_exe, ds_path]) * 1000.0 # to ms
                learned_time = run_bench([learned_exe, ds_path]) * 1000.0 # to ms
                speedup = learned_time / ars_time if ars_time > 0 else 0
                
                print(f"{size:<10} | {dist:<18} | {ars_time:<10.2f} | {learned_time:<12.2f} | {speedup:<8.2f}x")
                
                results.append({
                    "size": size,
                    "dist": dist,
                    "ars_ms": ars_time,
                    "learned_ms": learned_time,
                    "speedup": speedup
                })
            finally:
                if os.path.exists(ds_path):
                    os.remove(ds_path)
    
    # Write CSV
    with open("results.csv", "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["size", "dist", "ars_ms", "learned_ms", "speedup"])
        writer.writeheader()
        writer.writerows(results)
        
    # Generate Plots
    os.makedirs("plots", exist_ok=True)
    
    # 1. Throughput scaling on Random (Uniform)
    random_res = [r for r in results if r["dist"] == "Random"]
    ars_tp = [r["size"] / r["ars_ms"] / 1000 for r in random_res]
    learned_tp = [r["size"] / r["learned_ms"] / 1000 for r in random_res]
    
    plt.figure(figsize=(8, 6))
    plt.plot([r["size"] for r in random_res], ars_tp, marker='o', label='ARS Aero')
    plt.plot([r["size"] for r in random_res], learned_tp, marker='s', label='Learned Sort 2.0')
    plt.xscale('log')
    plt.xlabel('Dataset Size (N)')
    plt.ylabel('Throughput (Millions of Elements / sec)')
    plt.title('Throughput Scaling (Random Uniform Distribution)')
    plt.legend()
    plt.grid(True, which="both", ls="--", alpha=0.5)
    plt.savefig("plots/scaling.png")
    
    # 2. Distribution Resilience at N=10,000,000
    res_10m = [r for r in results if r["size"] == 10_000_000]
    labels = [r["dist"] for r in res_10m]
    ars_tps = [r["size"] / r["ars_ms"] / 1000 for r in res_10m]
    learned_tps = [r["size"] / r["learned_ms"] / 1000 for r in res_10m]
    
    x = range(len(labels))
    width = 0.35
    
    fig, ax = plt.subplots(figsize=(12, 6))
    rects1 = ax.bar([pos - width/2 for pos in x], ars_tps, width, label='ARS Aero', color='#1f77b4')
    rects2 = ax.bar([pos + width/2 for pos in x], learned_tps, width, label='Learned Sort 2.0', color='#ff7f0e')
    
    ax.set_ylabel('Throughput (Millions of Elements / sec)')
    ax.set_title('Throughput Across Distributions (N = 10,000,000)')
    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=45, ha="right")
    ax.legend()
    
    fig.tight_layout()
    plt.savefig("plots/distribution_resilience.png")

if __name__ == "__main__":
    main()
