#!/bin/bash

# Check if a folder name argument was provided
if [ -z "$1" ]; then
    echo "Usage: $0 <folder_name>"
    echo "Example: $0 my_rust_project"
    exit 1
fi

# Change into the provided directory
cd "$1" || { echo "Error: Could not enter directory '$1'"; exit 1; }

# Build the project in release mode
cargo build --release

# Configuration: paths and output file
RELEASE_DIR="./target/release"
OUTPUT_FILE="benchmarks.md"

# List of binaries to benchmark
BINS=( "$RELEASE_DIR/p1" "$RELEASE_DIR/p2" )

# Verify that all binaries exist and are executable before benchmarking
for bin in "${BINS[@]}"; do
    if [ ! -x "$bin" ]; then
        echo "Error: Binary $bin not found or not executable."
        exit 1
    fi
done

# Run hyperfine benchmark on all binaries
# All binaries are passed at once to generate a single comparison table
hyperfine \
    --warmup 15 \
    --export-markdown "$OUTPUT_FILE" \
    "${BINS[@]}"

# Display completion message
echo "----------------------------------------------------"
echo "Benchmarks complete. Results saved to $1/$OUTPUT_FILE"