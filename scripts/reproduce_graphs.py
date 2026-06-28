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

if __name__ == "__main__":
    print("Generating graphs...")
    plot_distribution()
    plot_entropy()
    plot_scalability()
    plot_strings()
    print("Graphs saved to assets/graphs/")
