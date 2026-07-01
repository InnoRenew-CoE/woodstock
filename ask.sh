#!/usr/bin/env bash
set -euo pipefail

if [ $# -eq 0 ]; then
    echo "Usage: $0 <question>"
    exit 1
fi

query=$(printf '%s' "$*" | jq -sRr @uri)

port=${SERVER_PORT:-6970}

echo "--- Query ---"
echo "$*"
echo ""
echo "--- Response ---"

curl -sN "https://observatory.innorenew.eu/chat/search?query=${query}" | while IFS= read -r line; do
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
        done)
            echo ""
            echo "--- END ---"
            ;;
        error)
            echo "--- ERROR ---"
            echo "$line" | jq -r '.value'
            ;;
        *)
            echo "$line"
            ;;
    esac
done

echo ""
