#!/bin/bash

# Navigate to the cgi-bin directory
cd "$(dirname "$0")/cgi-bin" || exit 1

echo "🔧 Building C files..."
for cfile in c/*.c; do
    [[ -f "$cfile" ]] || continue
    base=$(basename "$cfile" .c)
    outfile="${base}.cgi"
    echo "  🔹 Compiling $cfile -> $outfile"
    gcc -o "$outfile" "$cfile"
done

echo "🦀 Building Rust files..."
find . -mindepth 2 -maxdepth 2 -name Cargo.toml | while read -r cargo_file; do
    project_dir=$(dirname "$cargo_file")
    project_name=$(basename "$project_dir")
    output_binary="${project_name}.cgi"

    echo "🔧 Building $project_name..."

    # Build the Cargo project
    cargo build --release --manifest-path "$cargo_file"

    # Copy the compiled binary up to cgi-bin/
    cp "$project_dir/target/release/$project_name" "$output_binary"
    chmod +x "$output_binary"
    echo "✅ Built $output_binary"
done

echo "🐹 Building Go files..."
for gofile in go-*/*.go; do
    [[ -f "$gofile" ]] || continue
    base=$(basename "$gofile" .go)
    outfile="${base}.cgi"
    echo "  🔹 Compiling $gofile -> $outfile"
    srcdir=$(dirname "$gofile")
    (cd "$srcdir" && go build -o "../$outfile" "$(basename "$gofile")")
done

echo "✅ All .cgi files built."
