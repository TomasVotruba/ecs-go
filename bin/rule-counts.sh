#!/usr/bin/env bash
# Rule coverage counter. Counts the rules each tool implements and renders them
# into the README between the rule-counts markers. Run with --check in CI to fail
# when the README is stale.
#
#   bin/rule-counts.sh          update README in place
#   bin/rule-counts.sh --check  fail (exit 1) if README is out of date
#
# Sources:
#   ECS      - tracked baseline from ecs-go/ecs-rules.txt ("N rules configured")
#   ecs-go   - `ecs-go list-checkers`
#   ecs-rust - `ecs-rust list-rules`
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
README="$ROOT/README.md"
RULES_TXT="$ROOT/ecs-go/ecs-rules.txt"
START="<!-- rule-counts:start -->"
END="<!-- rule-counts:end -->"

ecs=$(grep -oE '[0-9]+ rules configured in ECS' "$RULES_TXT" | grep -oE '^[0-9]+')

go build -C "$ROOT/ecs-go" -o /tmp/ecs-go-count . >&2
go_count=$(/tmp/ecs-go-count list-checkers | grep -cE '^[[:space:]]*- ')

cargo build --release --manifest-path "$ROOT/ecs-rust/Cargo.toml" >&2 2>/dev/null
rust_count=$("$ROOT/ecs-rust/target/release/ecs-rust" list-rules | grep -cE 'Fixer$')

go_pct=$(( go_count * 100 / ecs ))
rust_pct=$(( rust_count * 100 / ecs ))

block=$(cat <<EOF
$START
| tool | rules | of ECS |
|---|---:|---:|
| [ECS](https://github.com/symplify/easy-coding-standard) (baseline) | $ecs | 100% |
| ecs-go | $go_count | ${go_pct}% |
| ecs-rust | $rust_count | ${rust_pct}% |
$END
EOF
)

# Write the block back into README between the markers.
new=$(awk -v start="$START" -v end="$END" -v block="$block" '
  $0 ~ start {print block; skip=1; next}
  $0 ~ end {skip=0; next}
  !skip {print}
' "$README")

# Emit to the CI job summary so every PR shows the counters.
if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
    {
        echo "### Rule coverage"
        echo "| tool | rules | of ECS |"
        echo "|---|---:|---:|"
        echo "| ECS (baseline) | $ecs | 100% |"
        echo "| ecs-go | $go_count | ${go_pct}% |"
        echo "| ecs-rust | $rust_count | ${rust_pct}% |"
    } >> "$GITHUB_STEP_SUMMARY"
fi

if [ "${1:-}" = "--check" ]; then
    if ! diff <(printf '%s\n' "$new") "$README" >/dev/null; then
        echo "README rule counts are stale. Run: bin/rule-counts.sh" >&2
        diff "$README" <(printf '%s\n' "$new") || true
        exit 1
    fi
    echo "Rule counts up to date: ECS $ecs, ecs-go $go_count, ecs-rust $rust_count"
else
    printf '%s\n' "$new" > "$README"
    echo "Updated README: ECS $ecs, ecs-go $go_count, ecs-rust $rust_count"
fi
