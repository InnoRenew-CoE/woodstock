#!/usr/bin/env bash
# Counts the submissions that did not pass ingestion.
#
# A submission that fails a stage ends up in {STAGING_FOLDER}/{stage}/failed/{uuid}
# (see src/worker/stage_io.rs), so every uuid folder there is one submission that
# stopped at that stage.
set -euo pipefail

known_stages=(new loaded chunked hyped embedded)

usage() {
    cat <<'EOF'
Usage: count_failed.sh [--list] [stage ...]

Counts the uuid folders under {STAGING_FOLDER}/{stage}/failed/ and prints one
line per stage plus the total.

  -l, --list   Print each failed uuid with the original file name and internal
               id read from its metadata.json.
  -h, --help   Print this help.

Without stage arguments the script reports the five worker stages, plus any
other stage folder that holds a failed folder. With stage arguments it reports
only those stages.

The staging root comes from STAGING_FOLDER, then from STAGING_FOLDER in the
.env file next to this script, then from ./staging. A relative path is resolved
against the script directory.

Exit status is 0 when the counts were read, and 1 when the staging root is
missing or an argument is wrong.
EOF
}

list_entries=false
requested=()
for arg in "$@"; do
    case "$arg" in
        -l | --list) list_entries=true ;;
        -h | --help)
            usage
            exit 0
            ;;
        -*)
            echo "count_failed.sh: unknown option '$arg'" >&2
            usage >&2
            exit 1
            ;;
        *) requested+=("$arg") ;;
    esac
done

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)

staging=${STAGING_FOLDER:-}
if [ -z "$staging" ] && [ -f "$script_dir/.env" ]; then
    staging=$(sed -n 's/^[[:space:]]*STAGING_FOLDER[[:space:]]*=[[:space:]]*//p' "$script_dir/.env" | tail -n 1)
    staging=${staging%\"}
    staging=${staging#\"}
    staging=${staging%\'}
    staging=${staging#\'}
fi
staging=${staging:-./staging}

case "$staging" in
    /*) ;;
    *) staging="$script_dir/${staging#./}" ;;
esac

if [ ! -d "$staging" ]; then
    echo "count_failed.sh: staging folder not found: $staging" >&2
    echo "Point STAGING_FOLDER at the folder the worker writes to." >&2
    exit 1
fi

stages=()
if [ ${#requested[@]} -gt 0 ]; then
    stages=("${requested[@]}")
    for stage in "${stages[@]}"; do
        if [ ! -d "$staging/$stage" ]; then
            echo "count_failed.sh: no stage folder for '$stage': $staging/$stage" >&2
        fi
    done
else
    stages=("${known_stages[@]}")
    # A stage added to the worker after this script was written still shows up.
    while IFS= read -r failed_dir; do
        name=${failed_dir%/failed}
        name=${name##*/}
        known=false
        for known_name in "${known_stages[@]}"; do
            if [ "$known_name" = "$name" ]; then
                known=true
            fi
        done
        if [ "$known" = false ]; then
            stages+=("$name")
        fi
    done < <(find "$staging" -mindepth 2 -maxdepth 2 -type d -name failed 2>/dev/null)
fi

printf 'staging: %s\n\n' "$staging"
printf '%-12s %8s\n' "stage" "failed"

total=0
for stage in "${stages[@]}"; do
    failed_dir="$staging/$stage/failed"
    count=0
    if [ -d "$failed_dir" ]; then
        count=$(find "$failed_dir" -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d '[:space:]')
    fi
    total=$((total + count))
    printf '%-12s %8d\n' "$stage" "$count"
done
printf '%-12s %8d\n' "total" "$total"

if [ "$list_entries" = false ] || [ "$total" -eq 0 ]; then
    exit 0
fi

have_jq=false
if command -v jq >/dev/null 2>&1; then
    have_jq=true
fi

echo ""
printf '%-12s %-38s %s\n' "stage" "uuid" "original name, internal id"
for stage in "${stages[@]}"; do
    failed_dir="$staging/$stage/failed"
    if [ ! -d "$failed_dir" ]; then
        continue
    fi
    while IFS= read -r -d '' entry; do
        uuid=${entry##*/}
        detail="metadata missing or unreadable"
        metadata_file="$entry/metadata.json"
        if [ "$have_jq" = true ] && [ -f "$metadata_file" ]; then
            original_name=$(jq -r '.original_name // empty' "$metadata_file" 2>/dev/null || true)
            internal_id=$(jq -r '.internal_id // empty' "$metadata_file" 2>/dev/null || true)
            if [ -n "$original_name" ] || [ -n "$internal_id" ]; then
                detail="${original_name:-unknown name}, id ${internal_id:-unknown}"
            fi
        fi
        printf '%-12s %-38s %s\n' "$stage" "$uuid" "$detail"
    done < <(find "$failed_dir" -mindepth 1 -maxdepth 1 -type d -print0)
done
