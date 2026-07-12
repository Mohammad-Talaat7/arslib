#include <iostream>
#include <fstream>
#include <vector>
#include <chrono>
#include <stdexcept>
#include "LearnedSort/include/learned_sort.h"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: learned_harness <dataset_file.bin>\n";
        return 1;
    }
    
    std::string filename = argv[1];
    
    // Load data
    std::ifstream file(filename, std::ios::binary | std::ios::ate);
    if (!file) {
        std::cerr << "Cannot open file " << filename << "\n";
        return 1;
    }
    
    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);
    
    size_t num_elements = size / sizeof(double);
    std::vector<double> data(num_elements);
    
    if (!file.read(reinterpret_cast<char*>(data.data()), size)) {
        std::cerr << "Error reading file\n";
        return 1;
    }
    
    // Time the sort
    auto start = std::chrono::high_resolution_clock::now();
    learned_sort::sort(data.begin(), data.end());
    auto end = std::chrono::high_resolution_clock::now();
    
    // Verify
    for (size_t i = 1; i < data.size(); ++i) {
        if (data[i] < data[i-1]) {
            std::cerr << "Verification failed at index " << i << "\n";
            return 1;
        }
    }
    
    std::chrono::duration<double> diff = end - start;
    std::cout << diff.count() << "\n";
    
    return 0;
}
