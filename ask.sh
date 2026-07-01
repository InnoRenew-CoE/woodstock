#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 [-p] <question>"
    echo "  -p    Use POST (send with empty history)"
    exit 1
}

use_post=false
while getopts "p" opt; do
    case "$opt" in
        p) use_post=true ;;
        *) usage ;;
    esac
done
shift $((OPTIND - 1))

if [ $# -eq 0 ]; then
    usage
fi

port=${SERVER_PORT:-6970}
base_url="http://localhost:${port}/chat/search"

echo "--- Query ---"
echo "$*"
echo ""

if [ "$use_post" = true ]; then
    body=$(jq -n --arg q "$*" '{query: $q, history: []}')
    echo "--- POST ---"
    echo "curl -sN -X POST -H 'Content-Type: application/json' -d '$body' $base_url"
    echo ""
    curl -sN -X POST -H "Content-Type: application/json" -d "$body" "$base_url" | while IFS= read -r line; do
        [ -z "$line" ] && continue
        type=$(echo "$line" | jq -r '.type // empty' 2>/dev/null)
        display=$(echo "$line" | jq -r '.display // false' 2>/dev/null)
        case "$type" in
            chunks)
                count=$(echo "$line" | jq '.value | length')
                echo "--- Chunks (${count} retrieved) ---"
                echo "$line" | jq -r '.value[] | "  [\(.score | tostring)] doc_id=\(.doc_id) seq=\(.doc_seq_num) \(.content[0:120])..."'
                echo ""
                ;;
            token)
                if [ "$display" = "true" ]; then
                    printf '%s' "$(echo "$line" | jq -r '.value')"
                fi
                ;;
            done) echo ""; echo "--- END ---" ;;
            error) echo "--- ERROR ---"; echo "$line" | jq -r '.value' ;;
            *) echo "$line" ;;
        esac
    done
    echo ""
else
    query=$(printf '%s' "$*" | jq -sRr @uri)
    curl -sN "${base_url}?query=${query}" | while IFS= read -r line; do
        [ -z "$line" ] && continue
        type=$(echo "$line" | jq -r '.type // empty' 2>/dev/null)
        display=$(echo "$line" | jq -r '.display // false' 2>/dev/null)
        case "$type" in
            chunks)
                count=$(echo "$line" | jq '.value | length')
                echo "--- Chunks (${count} retrieved) ---"
                echo "$line" | jq -r '.value[] | "  [\(.score | tostring)] doc_id=\(.doc_id) seq=\(.doc_seq_num) \(.content[0:120])..."'
                echo ""
                ;;
            token)
                if [ "$display" = "true" ]; then
                    printf '%s' "$(echo "$line" | jq -r '.value')"
                fi
                ;;
            done) echo ""; echo "--- END ---" ;;
            error) echo "--- ERROR ---"; echo "$line" | jq -r '.value' ;;
            *) echo "$line" ;;
        esac
    done
    echo ""
fi
