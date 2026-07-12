import os
import csv
import matplotlib.pyplot as plt

def main():
    results = []
    with open("results.csv", "r") as f:
        reader = csv.DictReader(f)
        for row in reader:
            results.append({
                "size": int(row["size"]),
                "dist": row["dist"],
                "ars_ms": float(row["ars_ms"]),
                "learned_ms": float(row["learned_ms"]),
                "speedup": float(row["speedup"])
            })

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
