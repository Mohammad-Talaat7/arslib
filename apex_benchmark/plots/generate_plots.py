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
    "legend.fontsize": 10,
    "figure.titlesize": 18,
    "figure.dpi": 300,
    "savefig.bbox": "tight",
    "axes.grid": True,
    "grid.alpha": 0.3,
    "grid.linestyle": "--"
})

CSV_PATH = "../benchmarks/Scientific_Research_Report.csv"
OUTPUT_DIR = "."

# Updated Algorithm Mapping for Apex vs Others
RESEARCH_ALGS = {
    "Timsort": "Timsort (Stable)",
    "PDQsort": "PDQsort (Unstable)",
    "Mergesort": "Mergesort",
    "Heapsort": "Heapsort",
    "Introsort": "Introsort",
    "Spreadsort / Radix Hybrid": "Spreadsort",
    "IPS4o": "IPS4o (Proxy)",
    "Fluxsort": "Fluxsort (Proxy)",
    "ARS Gen 5: Optimized Apex (MAIN)": "ARSApex (unstable)",
    "ARS Gen 5: Optimized Apex (Stable)": "ARSApex (stable)",
    "ARS Exp E: Quantile Adaptive": "ARS Quantile (Exp E)"
}

# Distinct Professional Palette
COLOR_MAP = {
    "Timsort (Stable)": "#2c3e50",
    "PDQsort (Unstable)": "#7f8c8d",
    "Mergesort": "#34495e",
    "Heapsort": "#95a5a6",
    "Introsort": "#16a085",
    "Spreadsort": "#27ae60",
    "IPS4o (Proxy)": "#8e44ad",
    "Fluxsort (Proxy)": "#2980b9",
    "ARSApex (unstable)": "#c0392b",
    "ARSApex (stable)": "#e67e22",
    "ARS Quantile (Exp E)": "#f1c40f"
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
    plt.legend(frameon=True, loc='best')
    save_fig("A_Scaling")

    # --- FIGURE B: Distribution Robustness (N=1M) ---
    plt.figure(figsize=(12, 7))
    n_max = 1000000
    data_b = df_i64[df_i64["N"] == n_max]
    sns.barplot(data=data_b, x="Distribution", y="Time (ms)", hue="Algorithm", palette=COLOR_MAP)
    plt.ylabel("Time (ms)")
    plt.xlabel("Dataset Distribution")
    plt.title(f"Algorithm Robustness ($N=10^6$)")
    plt.legend(title=None, loc='upper center', bbox_to_anchor=(0.5, -0.15), ncol=3)
    save_fig("B_Robustness")

    # --- FIGURE C: Information Theoretic Efficiency ---
    plt.figure(figsize=(10, 6))
    sns.lineplot(data=df_i64, x="N", y="Comparisons", hue="Algorithm", 
                 palette=COLOR_MAP, marker="s", markersize=7, errorbar=None)
    plt.xscale("log")
    plt.yscale("log")
    plt.ylabel("Comparisons ($C$)")
    plt.xlabel("Input Size ($N$)")
    plt.title("Growth of Comparison Complexity")
    save_fig("C_Complexity")

    # --- FIGURE D: Efficiency Ratio (Scatter) ---
    plt.figure(figsize=(9, 7))
    sns.scatterplot(data=df_i64[df_i64["N"] >= 10000], x="Comparisons", y="Time (ms)", 
                    hue="Algorithm", palette=COLOR_MAP, style="Algorithm", s=150, alpha=0.8)
    plt.xscale("log")
    plt.yscale("log")
    plt.title("Throughput Efficiency: Comparisons vs. Runtime")
    plt.legend(loc='lower right', fontsize=8)
    save_fig("D_Efficiency")

    # --- FIGURE E: Cache & Hardware Locality ---
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 7))
    data_e = df_i64[df_i64["N"] == n_max]
    
    sns.barplot(data=data_e, x="Algorithm", y="Cache Misses", palette=COLOR_MAP, ax=ax1)
    ax1.set_title("L3 Cache Pressure ($N=10^6$)")
    ax1.set_xticklabels(ax1.get_xticklabels(), rotation=45, ha='right')
    
    sns.barplot(data=data_e, x="Algorithm", y="Branch Misses", palette=COLOR_MAP, ax=ax2)
    ax2.set_title("Branch Predictor Accuracy ($N=10^6$)")
    ax2.set_xticklabels(ax2.get_xticklabels(), rotation=45, ha='right')
    save_fig("E_Hardware")

    # --- FIGURE F: Normalized Performance Heatmap ---
    pivot_df = df_i64[df_i64["N"] == n_max].pivot(index="Algorithm", columns="Distribution", values="Time (ms)")
    if "Timsort (Stable)" in pivot_df.index:
        norm_pivot = pivot_df.div(pivot_df.loc["Timsort (Stable)"], axis=1)
        plt.figure(figsize=(10, 7))
        sns.heatmap(norm_pivot, annot=True, fmt=".2f", cmap="RdYlGn_r", center=1.0, 
                    cbar_kws={'label': 'Slowdown relative to Timsort'})
        plt.title(f"Relative Performance Heatmap ($N=10^6$)")
        save_fig("F_Heatmap")

    # --- FIGURE G: Effective Constant Factor (T/NlogN) ---
    df_i64["Constant"] = df_i64["Time (ms)"] / (df_i64["N"] * np.log2(df_i64["N"]))
    plt.figure(figsize=(10, 6))
    sns.lineplot(data=df_i64, x="N", y="Constant", hue="Algorithm", palette=COLOR_MAP, marker="D", errorbar=None)
    plt.xscale("log")
    plt.ylabel(r"$\kappa = T(N) / (N \log N)$")
    plt.title("Effective Constant Factor Stability")
    save_fig("G_ConstantFactor")

    print(f"Updated publication-ready figures (Figure_*.png) generated in {os.path.abspath(OUTPUT_DIR)}")

if __name__ == "__main__":
    main()
