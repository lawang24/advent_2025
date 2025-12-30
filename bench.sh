#!/bin/bash

# 1. Check if a folder name was provided
if [ -z "$1" ]; then
    echo "Usage: $0 <folder_name>"
    echo "Example: $0 my_rust_project"
    exit 1
fi

# 2. Change into the provided directory
cd "$1" || { echo "Error: Could not enter directory '$1'"; exit 1; }

# 3. Define the path to binaries relative to the project root
RELEASE_DIR="./target/release"
OUTPUT_FILE="benchmarks.md"

BINS=( "$RELEASE_DIR/p1" "$RELEASE_DIR/p2" )

# Check if they actually exist before running
for bin in "${BINS[@]}"; do
    if [ ! -x "$bin" ]; then
        echo "Error: Binary $bin not found or not executable."
        exit 1
    fi
done

cargo build --release

# 6. Execute hyperfine
# All binaries are passed at once to generate a single comparison table
hyperfine \
    --warmup 15 \
    --export-markdown "$OUTPUT_FILE" \
    "${BINS[@]}"

echo "----------------------------------------------------"
echo "Benchmarks complete. Results saved to $1/$OUTPUT_FILE"