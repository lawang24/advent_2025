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

# Discover binaries from src/bin/*.rs files
BINS=()
for src in src/bin/*.rs; do
    if [ -f "$src" ]; then
        name=$(basename "$src" .rs)
        bin="$RELEASE_DIR/$name"
        if [ -x "$bin" ]; then
            BINS+=("$bin")
        else
            echo "Warning: Binary $bin not found or not executable, skipping."
        fi
    fi
done

# Verify that at least one binary was found
if [ ${#BINS[@]} -eq 0 ]; then
    echo "Error: No binaries found in $RELEASE_DIR"
    exit 1
fi

# Run hyperfine benchmark on all binaries
# All binaries are passed at once to generate a single comparison table
hyperfine \
    --warmup 15 \
    --export-markdown "$OUTPUT_FILE" \
    "${BINS[@]}"

# Display completion message
echo "----------------------------------------------------"
echo "Benchmarks complete. Results saved to $1/$OUTPUT_FILE"