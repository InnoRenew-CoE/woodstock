#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 <question> [<question2> ...]"
    echo ""
    echo "If only one question is given, interactive mode starts."
    echo "Each turn appends the previous Q&A to history sent via POST."
    exit 1
}

port=${SERVER_PORT:-6970}
base_url="https://observatory.innorenew.eu/chat/search"

history='[]'

send_query() {
    local q="$1"
    local method="$2"
    local reply=""

    if [ "$method" = "post" ]; then
        local tmp
        tmp=$(jq -n -c --arg q "$q" --argjson h "$history" '{query: $q, history: $h}')
        while IFS= read -r line; do
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
                        val=$(echo "$line" | jq -r '.value')
                        printf '%s' "$val"
                        reply="${reply}${val}"
                    fi
                    ;;
                done) echo ""; echo "--- END ---" ;;
                error) echo "--- ERROR ---"; echo "$line" | jq -r '.value' ;;
                *) echo "$line" ;;
            esac
        done < <(curl -sN -X POST -H "Content-Type: application/json" -d "$tmp" "$base_url")
    else
        local query_enc
        query_enc=$(printf '%s' "$q" | jq -sRr @uri)
        while IFS= read -r line; do
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
                        val=$(echo "$line" | jq -r '.value')
                        printf '%s' "$val"
                        reply="${reply}${val}"
                    fi
                    ;;
                done) echo ""; echo "--- END ---" ;;
                error) echo "--- ERROR ---"; echo "$line" | jq -r '.value' ;;
                *) echo "$line" ;;
            esac
        done < <(curl -sN "${base_url}?query=${query_enc}")
    fi

    history=$(echo "$history" | jq -c \
        --arg q "$q" \
        --arg r "$reply" \
        '. + [{"role": "user", "content": $q}, {"role": "assistant", "content": $r}]')
}

if [ $# -eq 0 ]; then
    usage
fi

first="$1"
shift

echo "--- Question ---"
echo "$first"
echo ""

send_query "$first" "get"

for q in "$@"; do
    echo ""
    echo "--- Question ---"
    echo "$q"
    echo ""
    send_query "$q" "post"
done

# interactive mode — if only one question was given on the CLI
if [ $# -eq 0 ]; then
    while true; do
        echo ""
        read -r -p "> " q
        [ -z "$q" ] && continue
        [ "$q" = "exit" ] || [ "$q" = "quit" ] && break
        echo ""
        send_query "$q" "post"
    done
fi
