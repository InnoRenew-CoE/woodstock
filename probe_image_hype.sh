#!/usr/bin/env bash
# Replays one image hype request the way the ingestion worker sends it, so the model
# response and the latency can be checked without running a whole document.
#
# Nothing about the request is logged at runtime, so this script rebuilds it from the
# files the worker leaves behind:
#
#   images    $RAG_IMAGE_FOLDER/<document_id>/<image id>.<ext>, default files/rag-images
#             served at /rag/images/<document_id>/<image id>.<ext>
#   image list {STAGING_FOLDER}/{chunked,hyped}/<uuid>/chunked.json
#             .chunks[].images[] holds path, route and alt_text for every image
#   prompt    only in the code, see src/rag/processing/hype.rs (IMAGE_PROMPT below)
#
# Usage:
#   ./probe_image_hype.sh <uuid> [chunk index] [image index]
#   ./probe_image_hype.sh --image <path> [--text <chunk text>]
#
# Options:
#   --list             show the chunks of the submission that have images
#   --dry-run          build the request body and stop before sending it
#
# The endpoint and model are read from the same variables the worker uses, with the
# usual fallbacks, so a probe matches what production sends.
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$script_dir"

IMAGE_PROMPT='/no_think Generate retrieval questions that can be answered from the image and its surrounding document context. Only answer with questions, one per line, with no numbering or prefixes.'
IMAGE_SYSTEM='You generate concise search questions from document images.'

usage() {
    cat <<'EOF'
Replays one image hype request the way the ingestion worker sends it.

Usage:
   ./probe_image_hype.sh <uuid> [chunk index] [image index]
   ./probe_image_hype.sh --image <path> [--text <chunk text>]

Options:
   --list             show the chunks of the submission that have images
   --dry-run          build the request body and stop before sending it
   --chunk <index>    pick the chunk instead of the first one with an image
   --text <text>      chunk text to send with --image

The uuid must be a submission that is still staged, because the staging folder is
removed once a submission finishes. See src/worker/stage_io.rs.
EOF
    exit 1
}

env_value() {
    local key=$1
    local value=${!key:-}
    if [ -n "$value" ]; then
        printf '%s' "$value"
        return
    fi
    local env_file="$script_dir/.env"
    if [ -f "$env_file" ]; then
        value=$(grep -E "^${key}=" "$env_file" | tail -n 1 | cut -d= -f2- | sed -e 's/^"//' -e 's/"$//' -e "s/^'//" -e "s/'$//")
        if [ -n "$value" ]; then
            printf '%s' "$value"
            return
        fi
    fi
    printf ''
}

first_env() {
    local key value
    for key in "$@"; do
        value=$(env_value "$key")
        if [ -n "$value" ]; then
            printf '%s' "$value"
            return
        fi
    done
}

mime_for() {
    case "${1,,}" in
        *.png) printf 'image/png' ;;
        *.jpg | *.jpeg) printf 'image/jpeg' ;;
        *.gif) printf 'image/gif' ;;
        *.webp) printf 'image/webp' ;;
        *.bmp) printf 'image/bmp' ;;
        *) printf 'application/octet-stream' ;;
    esac
}

if ! command -v jq > /dev/null; then
    echo "jq is required" >&2
    exit 1
fi

base_url=$(first_env INGESTION_COMPATIBLE_BASE_URL OPENAI_COMPATIBLE_BASE_URL OPENAI_BASE_URL)
base_url=${base_url:-http://localhost:11434/v1}
api_key=$(first_env INGESTION_COMPATIBLE_API_KEY OPENAI_COMPATIBLE_API_KEY OPENAI_API_KEY API_KEY)
model=$(first_env INGESTION_MODEL CHAT_MODEL)
model=${model:-hf.co/unsloth/Qwen3-30B-A3B-Instruct-2507-GGUF:UD-Q4_K_XL}
staging=$(first_env STAGING_FOLDER)
staging=${staging:-./staging}

uuid=""
image_path=""
chunk_text=""
chunk_index=""
image_index=""
list_only=false
dry_run=false
positionals=()

while [ $# -gt 0 ]; do
    case "$1" in
        --image)
            image_path=${2:?--image needs a path}
            shift 2
            ;;
        --text)
            chunk_text=${2:?--text needs text}
            shift 2
            ;;
        --chunk)
            chunk_index=${2:?--chunk needs an index}
            shift 2
            ;;
        --list)
            list_only=true
            shift
            ;;
        --dry-run)
            dry_run=true
            shift
            ;;
        -h | --help)
            usage
            ;;
        *)
            positionals+=("$1")
            shift
            ;;
    esac
done

if [ -n "$image_path" ]; then
    if [ ! -f "$image_path" ]; then
        echo "no image at $image_path" >&2
        exit 1
    fi
    route=$image_path
    alt=""
else
    uuid=${positionals[0]:-}
    if [ -z "$uuid" ]; then
        usage
    fi
    chunk_index=${chunk_index:-${positionals[1]:-}}
    image_index=${image_index:-${positionals[2]:-0}}

    artifact=""
    for candidate in \
        "$staging/chunked/$uuid/chunked.json" \
        "$staging/chunked/failed/$uuid/chunked.json" \
        "$staging/hyped/$uuid/chunked.json" \
        "$staging/hyped/failed/$uuid/chunked.json" \
        "$staging/hyped/$uuid/hyped.json" \
        "$staging/embedded/$uuid/hyped.json"; do
        if [ -f "$candidate" ]; then
            artifact=$candidate
            break
        fi
    done
    if [ -z "$artifact" ]; then
        echo "no staged artifact for $uuid under $staging." >&2
        echo "Finished submissions are deleted, so pick one that is still pending, or use --image." >&2
        exit 1
    fi

    if [ "$list_only" = true ]; then
        echo "artifact $artifact"
        jq -r '.chunks | to_entries[] | select(.value.images | length > 0) | "chunk \(.key): \(.value.images | length) image(s)"' "$artifact"
        exit 0
    fi

    if [ -z "$chunk_index" ]; then
        chunk_index=$(jq -r '[.chunks | to_entries[] | select(.value.images | length > 0) | .key] | first // empty' "$artifact")
        if [ -z "$chunk_index" ]; then
            echo "submission $uuid has no images" >&2
            exit 1
        fi
    fi

    image_count=$(jq -r --argjson c "$chunk_index" '.chunks[$c].images | length' "$artifact")
    if [ "$image_count" = "0" ] || [ -z "$image_count" ] || [ "$image_count" = "null" ]; then
        echo "chunk $chunk_index has no images" >&2
        exit 1
    fi
    if [ "$image_index" -ge "$image_count" ]; then
        echo "chunk $chunk_index has $image_count image(s), index $image_index does not exist" >&2
        exit 1
    fi

    image_path=$(jq -r --argjson c "$chunk_index" --argjson i "$image_index" '.chunks[$c].images[$i].path' "$artifact")
    route=$(jq -r --argjson c "$chunk_index" --argjson i "$image_index" '.chunks[$c].images[$i].route' "$artifact")
    alt=$(jq -r --argjson c "$chunk_index" --argjson i "$image_index" '.chunks[$c].images[$i].alt_text // empty' "$artifact")
    chunk_text=$(jq -r --argjson c "$chunk_index" '.chunks[$c].text' "$artifact")

    if [ ! -f "$image_path" ] && [ -f "$script_dir/$image_path" ]; then
        image_path="$script_dir/$image_path"
    fi
    if [ ! -f "$image_path" ]; then
        echo "the artifact points at $image_path but that file is not there" >&2
        exit 1
    fi
fi

mime=$(mime_for "$image_path")
context=$(printf 'IMAGE ROUTE: %s\nALT TEXT: %s\nDOCUMENT CHUNK CONTEXT:\n%s' "$route" "$alt" "$chunk_text")

work_dir=$(mktemp -d)
trap 'rm -rf "$work_dir"' EXIT

base64 -w0 "$image_path" > "$work_dir/image.b64"

jq -n \
    --arg model "$model" \
    --arg system "$IMAGE_SYSTEM" \
    --arg prompt "$IMAGE_PROMPT" \
    --arg context "$context" \
    --arg prefix "data:$mime;base64," \
    --rawfile image "$work_dir/image.b64" \
    '{
        model: $model,
        stream: false,
        messages: [
            {role: "system", content: $system},
            {
                role: "user",
                content: [
                    {type: "text", text: ($prompt + "\n" + $context)},
                    {type: "image_url", image_url: {url: ($prefix + $image)}}
                ]
            }
        ]
    }' > "$work_dir/body.json"

echo "image      $image_path ($(stat -c %s "$image_path") bytes, $mime)"
if [ -n "$uuid" ]; then
    echo "artifact   $artifact"
    echo "chunk      $chunk_index, image $image_index of $image_count, chunk text $(printf '%s' "$chunk_text" | wc -c) chars"
    echo "route      $route"
fi
echo "endpoint   $base_url/chat/completions"
echo "model      $model"
echo "body       $(stat -c %s "$work_dir/body.json") bytes"
echo "system     $IMAGE_SYSTEM"
echo "user       $IMAGE_PROMPT"
if [ -n "$chunk_text" ]; then
    echo "context    $(printf '%s' "$chunk_text" | head -c 300 | tr '\n' ' ')"
    if [ "$(printf '%s' "$chunk_text" | wc -c)" -gt 300 ]; then
        echo "           ..."
    fi
fi
echo ""

if [ "$dry_run" = true ]; then
    echo "dry run, body left at $work_dir/body.json"
    trap - EXIT
    exit 0
fi

auth=()
if [ -n "$api_key" ]; then
    auth=(-H "Authorization: Bearer $api_key")
fi

start=$(date +%s%N)
result=$(curl -sS -o "$work_dir/response.json" -w '%{http_code} %{time_total}' \
    --max-time "${PROBE_TIMEOUT:-900}" \
    -H 'Content-Type: application/json' \
    "${auth[@]}" \
    --data-binary @"$work_dir/body.json" \
    "$base_url/chat/completions") || result="000 0"
status=${result%% *}
elapsed=${result##* }
end=$(date +%s%N)

echo "status     $status"
echo "elapsed    ${elapsed}s (curl), $(( (end - start) / 1000000 ))ms (wall)"
echo ""

if [ "$status" != "200" ]; then
    head -c 2000 "$work_dir/response.json"
    echo ""
    exit 1
fi

jq -r '.choices[0].message.content // empty' "$work_dir/response.json" > "$work_dir/content.txt"
echo "questions  $(grep -c '[^[:space:]]' "$work_dir/content.txt" || true) answer line(s), $(wc -c < "$work_dir/content.txt") chars"
echo "----"
cat "$work_dir/content.txt"
