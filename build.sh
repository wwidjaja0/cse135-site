#!/bin/bash

# Navigate to the cgi-bin directory
cd "$(dirname "$0")/cgi-bin" || exit 1

echo "🔧 Building C files..."
for cfile in *.c; do
    [[ -f "$cfile" ]] || continue
    outfile="${cfile%.c}.cgi"
    echo "  🔹 Compiling $cfile -> $outfile"
    gcc -o "$outfile" "$cfile"
done

echo "🦀 Building Rust files..."
for rsfile in *.rs; do
    [[ -f "$rsfile" ]] || continue
    outfile="${rsfile%.rs}.cgi"
    echo "  🔹 Compiling $rsfile -> $outfile"
    rustc -o "$outfile" "$rsfile"
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
