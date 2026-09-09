#!/usr/bin/env bash
# Head-to-head wall-time benchmark: ecs-go vs ecs-rust (and, when configured, the
# original PHP ECS) running the *same* PSR-12 rule subset over a real codebase.
# The Go side is pinned to the ported rules via a --config file, so all binaries
# do identical work. Each binary fixes a fresh copy RUNS times; the mean wall
# time is reported.
#
# Env:
#   GO_BIN    path to the ecs-go binary
#   RUST_BIN  path to the ecs-rust binary
#   CONFIG    ecs-go.json listing the ported rules (Go side)
#   ECS_CMD   optional command to run the PHP ECS --fix on a dir passed as the
#             last argument (e.g. "php vendor/bin/ecs check --fix
#             --no-progress-bar --config /tmp/ecs.php"). When set, an "ecs (PHP)"
#             wall-time column is added.
#   RUNS      timed repetitions per binary (default 10)
#
# Args: <label> <source-dir> [<label> <source-dir> ...]
set -euo pipefail

: "${GO_BIN:?set GO_BIN}"
: "${RUST_BIN:?set RUST_BIN}"
: "${CONFIG:?set CONFIG}"
ECS_CMD=${ECS_CMD:-}
RUNS=${RUNS:-10}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

# best <out-var> <src> <binary> [args...]  -> mean wall-clock nanoseconds over RUNS
best() {
    local __out=$1 src=$2
    shift 2
    local total=0
    for _ in $(seq "$RUNS"); do
        rm -rf "$WORK/run"
        cp -r "$src" "$WORK/run"
        local t0 t1
        t0=$(date +%s%N)
        "$@" "$WORK/run" >/dev/null 2>&1 || true
        t1=$(date +%s%N)
        total=$((total + t1 - t0))
    done
    printf -v "$__out" '%s' "$((total / RUNS))"
}

sec() { awk -v ns="$1" 'BEGIN { printf "%.3f", ns/1e9 }'; }

echo "## ecs-go vs ecs-rust (same PSR-12 rule subset, mean of $RUNS runs)"
echo ""
if [ -n "$ECS_CMD" ]; then
    echo "| codebase | .php files | ecs (PHP) | ecs-go | ecs-rust |"
    echo "|---|---:|---:|---:|---:|"
else
    echo "| codebase | .php files | ecs-go | ecs-rust |"
    echo "|---|---:|---:|---:|"
fi

while [ $# -ge 2 ]; do
    label=$1
    src=$2
    shift 2
    files=$(find "$src" -name '*.php' | wc -l | tr -d ' ')

    best go_ns "$src" "$GO_BIN" --fix --config "$CONFIG"
    best rust_ns "$src" "$RUST_BIN" --fix

    if [ -n "$ECS_CMD" ]; then
        # shellcheck disable=SC2086
        best ecs_ns "$src" $ECS_CMD
        echo "| $label | $files | $(sec "$ecs_ns")s | $(sec "$go_ns")s | $(sec "$rust_ns")s |"
    else
        echo "| $label | $files | $(sec "$go_ns")s | $(sec "$rust_ns")s |"
    fi
done
