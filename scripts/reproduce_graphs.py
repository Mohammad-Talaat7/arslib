import matplotlib.pyplot as plt
import numpy as np
import os

# Create assets directory if it doesn't exist
os.makedirs("assets/graphs", exist_ok=True)

# Graph 1: Distribution Performance
def plot_distribution():
    labels = ['Random', 'Gaussian', 'Duplicates']
    pdq = [262.47, 157.36, 56.57]
    ips4o = [114.43, 90.93, 35.91]
    ars = [109.90, 88.30, 60.89]

    x = np.arange(len(labels))
    width = 0.25

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.bar(x - width, pdq, width, label='PDQsort', color='#e74c3c')
    ax.bar(x, ips4o, width, label='IPS4o', color='#3498db')
    ax.bar(x + width, ars, width, label='ARS Aero', color='#1abc9c')

    ax.set_ylabel('Execution Time (ms)')
    ax.set_title('Distribution Performance (N=10^7)')
    ax.set_xticks(x)
    ax.set_xticklabels(labels)
    ax.legend()
    
    plt.tight_layout()
    plt.savefig('assets/graphs/distribution_performance.png', dpi=300)
    plt.close()

# Graph 2: Entropy Response
def plot_entropy():
    labels = ['Random', 'Gaussian', 'NearlySorted', 'Duplicates', 'Zipfian']
    pdq = [262.47, 157.36, 271.92, 56.57, 76.62]
    ips4o = [114.43, 90.93, 91.63, 35.91, 82.25]
    ars = [109.90, 88.30, 110.85, 60.89, 131.60]

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.plot(labels, pdq, marker='s', linewidth=2, label='PDQsort', color='#e74c3c')
    ax.plot(labels, ips4o, marker='s', linewidth=2, label='IPS4o', color='#3498db')
    ax.plot(labels, ars, marker='o', linewidth=2, label='ARS Aero', color='#1abc9c')

    ax.set_ylabel('Execution Time (ms)')
    ax.set_title('Entropy Response (N=10^7)')
    ax.legend()
    ax.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig('assets/graphs/entropy_response.png', dpi=300)
    plt.close()

# Graph 3: Parallel Scalability
def plot_scalability():
    threads = [1, 2, 4, 8]
    ips4o = [373.7, 200.5, 124.6, 114.4]
    ars = [248.1, 154.0, 100.7, 89.4]

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.plot(threads, ips4o, marker='s', linewidth=2, label='IPS4o', color='#3498db')
    ax.plot(threads, ars, marker='o', linewidth=2, label='ARS Aero', color='#1abc9c')

    ax.set_xlabel('Thread Count')
    ax.set_ylabel('Execution Time (ms)')
    ax.set_title('Parallel Scalability (N=10^7)')
    ax.set_xticks(threads)
    ax.legend()
    ax.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig('assets/graphs/parallel_scalability.png', dpi=300)
    plt.close()

# Graph 4: Strings Comparison
def plot_strings():
    labels = ['PDQsort', 'Timsort', 'ARS Aero']
    times = [9097.41, 10746.49, 1905.82]
    colors = ['#9b59b6', '#e67e22', '#1abc9c']

    fig, ax = plt.subplots(figsize=(8, 5))
    bars = ax.bar(labels, times, color=colors, width=0.5)

    ax.set_ylabel('Execution Time (ms)')
    ax.set_title('Sorting 10,000,000 Random Strings')
    
    for bar in bars:
        yval = bar.get_height()
        ax.text(bar.get_x() + bar.get_width()/2, yval + 100, f"{yval:.2f}", ha='center', va='bottom', fontsize=9)

    plt.tight_layout()
    plt.savefig('assets/graphs/strings_comparison.png', dpi=300)
    plt.close()

# Graph 5: Effective Constant Factor
def plot_constant_factor():
    n_sizes = [1000, 10000, 100000, 1000000, 10000000]
    quicksort = [19.0, 21.8, 31.8, 33.4, 25.9]
    ips4o = [22.5, 14.8, 7.0, 8.2, 11.4]
    apex_base = [24.5, 78.6, 37.8, 38.6, 27.6]
    apex_opt = [57.8, 50.2, 34.4, 15.3, 12.9]
    aero = [16.5, 105.8, 11.0, 14.1, 10.3]

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.plot(n_sizes, quicksort, marker='s', linewidth=2, label='Quicksort', color='#e74c3c')
    ax.plot(n_sizes, ips4o, marker='p', linewidth=2, label='IPS4o', color='#9b59b6')
    ax.plot(n_sizes, apex_base, marker='^', linewidth=2, label='ARS Apex Baseline', color='#3498db')
    ax.plot(n_sizes, apex_opt, marker='d', linewidth=2, label='ARS Apex Optimized', color='#e67e22')
    ax.plot(n_sizes, aero, marker='o', linewidth=2, label='ARS Aero', color='#1abc9c')

    ax.set_xscale('log')
    ax.set_xlabel('Dataset Size (N)')
    ax.set_ylabel('Cost per Element (ns)')
    ax.set_title('Effective Constant Factor (T/N)')
    ax.legend()
    ax.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig('assets/graphs/constant_factor.png', dpi=300)
    plt.close()

# Graph 6: Hardware Utilization Trends
def plot_hardware_trends():
    n_sizes = [10000, 100000, 1000000, 10000000]
    llc_miss = [19.59, 17.11, 28.73, 40.49]
    branch_miss = [0.017, 0.096, 0.64, 6.26]

    fig, ax1 = plt.subplots(figsize=(8, 5))

    color = '#3498db'
    ax1.set_xscale('log')
    ax1.set_xlabel('Dataset Size (N)')
    ax1.set_ylabel('LLC Miss Rate (%)', color=color)
    ax1.plot(n_sizes, llc_miss, marker='^', linewidth=2, color=color, label='LLC Miss Rate')
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.grid(True, linestyle='--', alpha=0.7)

    ax2 = ax1.twinx()
    color = '#e74c3c'
    ax2.set_ylabel('Branch Misses (10^6)', color=color)
    ax2.plot(n_sizes, branch_miss, marker='s', linewidth=2, linestyle='--', color=color, label='Branch Misses')
    ax2.tick_params(axis='y', labelcolor=color)

    fig.tight_layout()
    plt.title('Hardware Utilization Trends')
    fig.legend(loc="upper left", bbox_to_anchor=(0.1, 0.9))
    
    plt.savefig('assets/graphs/hardware_trends.png', dpi=300)
    plt.close()

# Graph 7: Hardware Efficiency Comparison
def plot_hardware_efficiency():
    labels = ['PDQsort', 'IPS4o', 'ARS Aero']
    ipc = [2.24, 1.38, 1.27]
    branch_misses = [6.26, 7.12, 7.65]

    x = np.arange(len(labels))
    width = 0.35

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.bar(x - width/2, ipc, width, label='IPC', color='#3498db')
    ax.bar(x + width/2, branch_misses, width, label='Branch Misses (10^7)', color='#e74c3c')

    ax.set_ylabel('Metric Value')
    ax.set_title('Hardware Efficiency Comparison')
    ax.set_xticks(x)
    ax.set_xticklabels(labels)
    ax.legend()
    
    plt.tight_layout()
    plt.savefig('assets/graphs/hardware_efficiency.png', dpi=300)
    plt.close()

# Graph 8: Streaming Pareto Frontier
def plot_streaming_pareto():
    throughput = [24.29, 203.99, 9.31]
    latency = [22575, 424, 133759]
    labels = ['32k', 'Bursty', 'Zipfian']

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.scatter(throughput, latency, color='#1abc9c', s=100)

    for i, label in enumerate(labels):
        ax.annotate(label, (throughput[i], latency[i]), xytext=(5, 5), textcoords='offset points', fontsize=10)

    ax.set_xlabel('Throughput (M ops/s)')
    ax.set_ylabel('P99 Latency (μs)')
    ax.set_title('Streaming Pareto Frontier (N=10^8)')
    ax.grid(True, linestyle='--', alpha=0.7)
    
    plt.tight_layout()
    plt.savefig('assets/graphs/streaming_pareto.png', dpi=300)
    plt.close()

if __name__ == "__main__":
    print("Generating graphs...")
    plot_distribution()
    plot_entropy()
    plot_scalability()
    plot_strings()
    plot_constant_factor()
    plot_hardware_trends()
    plot_hardware_efficiency()
    plot_streaming_pareto()
    print("Graphs saved to assets/graphs/")
