import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import os

# --- Academic Formatting Configuration ---
plt.rcParams.update({
    "text.usetex": False,
    "font.family": "serif",
    "font.serif": ["DejaVu Serif", "Times New Roman", "Palatino"],
    "axes.labelsize": 14,
    "axes.titlesize": 16,
    "xtick.labelsize": 12,
    "ytick.labelsize": 12,
    "legend.fontsize": 12,
    "figure.titlesize": 18,
    "figure.dpi": 300,
    "savefig.bbox": "tight",
    "axes.grid": True,
    "grid.alpha": 0.3,
    "grid.linestyle": "--"
})

CSV_PATH = "../benchmarks/Scientific_Research_Report.csv"
SCALABILITY_CSV = "../benchmarks/reports/scalability_results.csv"
OUTPUT_DIR = "."

# Updated Algorithm Mapping for Research Narrative
RESEARCH_ALGS = {
    "Quicksort": "Quicksort (Unstable)",
    "Timsort": "Timsort (Stable)",
    "ARS Gen 5: Optimized Apex (MAIN)": "ARSApex (Unstable)",
    "ARS Gen 5: Optimized Apex (Stable)": "ARSApex (Stable)",
    "ARS Gen 6: Aero Architecture": "ARSAero (Unstable)",
    "ARS Gen 6: Aero (Stable)": "ARSAero (Stable)",
    "ARS Exp C: Adaptive Hierarchical": "ARS Adaptive (Exp)"
}

# Distinct Professional Palette
COLOR_MAP = {
    "Quicksort (Unstable)": "#7f8c8d",
    "Timsort (Stable)": "#2c3e50",
    "ARSApex (Unstable)": "#c0392b",
    "ARSApex (Stable)": "#2980b9",
    "ARSAero (Unstable)": "#8e44ad",
    "ARSAero (Stable)": "#27ae60",
    "ARS Adaptive (Exp)": "#f1c40f",
    "Rayon ParSort": "#d35400"
}

def save_fig(name):
    plt.savefig(os.path.join(OUTPUT_DIR, f"Figure_{name}.png"), dpi=300)
    plt.close()

def main():
    if not os.path.exists(CSV_PATH):
        print(f"Error: {CSV_PATH} not found.")
        return

    raw_df = pd.read_csv(CSV_PATH)
    
    # Filter and Rename
    df = raw_df[raw_df["Algorithm"].isin(RESEARCH_ALGS.keys())].copy()
    df["Algorithm"] = df["Algorithm"].map(RESEARCH_ALGS)
    df_i64 = df[df["Category"] == "i64"].copy()
    n_max = 1000000

    # --- FIGURE A: Scaling Analysis ---
    plt.figure(figsize=(10, 6))
    data_a = df_i64[df_i64["Distribution"] == "Random"]
    sns.lineplot(data=data_a, x="N", y="Time (ms)", hue="Algorithm", 
                 palette=COLOR_MAP, marker="o", markersize=8, linewidth=2, errorbar=None)
    plt.xscale("log")
    plt.yscale("log")
    plt.xlabel("Input Size ($N$)")
    plt.ylabel("Execution Time (ms)")
    plt.title("Computational Scaling on Random Distribution")
    plt.legend(frameon=True)
    save_fig("A_Scaling")

    # --- FIGURE B: Distribution Robustness (N=1M) ---
    plt.figure(figsize=(12, 6))
    data_b = df_i64[df_i64["N"] == n_max]
    sns.barplot(data=data_b, x="Distribution", y="Time (ms)", hue="Algorithm", palette=COLOR_MAP)
    plt.ylabel("Time (ms)")
    plt.xlabel("Dataset Distribution")
    plt.title(f"Algorithm Robustness ($N=10^6$)")
    plt.legend(title=None, loc='upper center', bbox_to_anchor=(0.5, -0.15), ncol=3)
    save_fig("B_Robustness")

    # --- FIGURE E: Hardware Efficiency (IPC & LLC) ---
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 7))
    data_e = df_i64[df_i64["N"] == n_max]
    
    sns.barplot(data=data_e, x="Algorithm", y="LLC Miss Rate", palette=COLOR_MAP, ax=ax1)
    ax1.set_title("LLC Miss Rate ($N=10^6$)")
    ax1.set_xticklabels(ax1.get_xticklabels(), rotation=45, ha='right')
    
    sns.barplot(data=data_e, x="Algorithm", y="IPC", palette=COLOR_MAP, ax=ax2)
    ax2.set_title("Instructions Per Cycle (IPC) ($N=10^6$)")
    ax2.set_xticklabels(ax2.get_xticklabels(), rotation=45, ha='right')
    save_fig("E_Hardware")

    # --- FIGURE J: Achieved Bandwidth ---
    plt.figure(figsize=(10, 6))
    sns.barplot(data=data_e, x="Algorithm", y="Bandwidth (MB/s)", palette=COLOR_MAP)
    plt.title(f"Achieved Memory Bandwidth ($N=10^6$)")
    plt.xticks(rotation=45, ha='right')
    plt.ylabel("MB/s")
    save_fig("J_Bandwidth")

    # --- FIGURE K: Scalability Curve ---
    if os.path.exists(SCALABILITY_CSV):
        scal_df = pd.read_csv(SCALABILITY_CSV)
        plt.figure(figsize=(10, 6))
        # Filter for Random distribution for clarity
        data_k = scal_df[scal_df["Distribution"] == "Random"]
        sns.lineplot(data=data_k, x="Threads", y="Time (ms)", hue="Algorithm", 
                     palette=COLOR_MAP, marker="o", markersize=8, linewidth=2)
        plt.xscale("log", base=2)
        plt.yscale("log")
        plt.xticks(sorted(scal_df["Threads"].unique()), sorted(scal_df["Threads"].unique()))
        plt.title("Strong Scaling: Thread Count vs. Runtime ($N=10^7$)")
        plt.ylabel("Time (ms)")
        plt.xlabel("Thread Count")
        plt.legend(frameon=True)
        save_fig("K_Scalability")

    print(f"Updated publication-ready figures (Figure_*.png) generated in {os.path.abspath(OUTPUT_DIR)}")

if __name__ == "__main__":
    main()
