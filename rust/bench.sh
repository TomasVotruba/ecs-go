#!/usr/bin/env bash
# Head-to-head wall-time benchmark: ecs-go vs ecs-rust running the *same* PSR-12
# rule subset over a real codebase. The Go side is pinned to the ported rules via
# a --config file, so both binaries do identical work. Each binary fixes a fresh
# copy RUNS times; the best (min) wall time is reported.
#
# Env:
#   GO_BIN    path to the ecs-go binary
#   RUST_BIN  path to the ecs-rust binary
#   CONFIG    ecs-go.json listing the ported rules (Go side)
#   RUNS      timed repetitions per binary (default 5)
#
# Args: <label> <source-dir> [<label> <source-dir> ...]
set -euo pipefail

: "${GO_BIN:?set GO_BIN}"
: "${RUST_BIN:?set RUST_BIN}"
: "${CONFIG:?set CONFIG}"
RUNS=${RUNS:-5}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

# best <out-var> <src> <binary> [args...]  -> min wall-clock seconds over RUNS
best() {
    local __out=$1 src=$2
    shift 2
    local min=""
    for _ in $(seq "$RUNS"); do
        rm -rf "$WORK/run"
        cp -r "$src" "$WORK/run"
        local t0 t1 d
        t0=$(date +%s%N)
        "$@" "$WORK/run" >/dev/null 2>&1 || true
        t1=$(date +%s%N)
        d=$((t1 - t0))
        if [ -z "$min" ] || [ "$d" -lt "$min" ]; then min=$d; fi
    done
    printf -v "$__out" '%s' "$min"
}

sec() { awk -v ns="$1" 'BEGIN { printf "%.3f", ns/1e9 }'; }
speedup() { awk -v g="$1" -v r="$2" 'BEGIN { printf "%.2f", g/r }'; }

echo "## ecs-go vs ecs-rust (same PSR-12 rule subset, best of $RUNS runs)"
echo ""
echo "| codebase | .php files | ecs-go | ecs-rust | speedup |"
echo "|---|---:|---:|---:|---:|"

while [ $# -ge 2 ]; do
    label=$1
    src=$2
    shift 2
    files=$(find "$src" -name '*.php' | wc -l | tr -d ' ')

    best go_ns "$src" "$GO_BIN" --fix --config "$CONFIG"
    best rust_ns "$src" "$RUST_BIN" --fix

    echo "| $label | $files | $(sec "$go_ns")s | $(sec "$rust_ns")s | $(speedup "$go_ns" "$rust_ns")x |"
done
