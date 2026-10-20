#!/bin/bash
set -euo pipefail

API_URL="https://git.sr.ht/query"

USERNAME="nikolaeu"
REPO_NAME="numi"
REV_SPEC=$1
FILE_PATH=$2
TOKEN="$(vault kv get -field=token kv/numi/sh)"
QUERY="query GetRepositoryId(\$username: String!, \$repoName: String!) { user(username: \$username) { repository(name: \$repoName) { id name description } } }"

JSON_PAYLOAD=$(jq -n \
                  --arg query "$QUERY" \
                  --arg username "$USERNAME" \
                  --arg repoName "$REPO_NAME" \
                  '{query: $query, variables: {username: $username, repoName: $repoName}}')

RESPONSE=$(curl -X POST "$API_URL" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "$JSON_PAYLOAD" \
    -s)

REPO_ID=$(echo $RESPONSE | jq '.data.user.repository.id')
# echo "Repository ID: $REPO_ID"

# UPLOAD

QUERY='mutation ($repoId: Int!, $revspec: String!, $file: Upload!) { uploadArtifact(repoId: $repoId, revspec: $revspec, file: $file) { id filename checksum size url } }'

JSON_OPERATIONS=$(jq -n \
                    --arg query "$QUERY" \
                    --argjson repoId $REPO_ID \
                    --arg revspec "$REV_SPEC" \
                    '{query: $query, variables: {repoId: $repoId, revspec: $revspec, file: null}}')
JSON_MAP=$(jq -n '{"0": ["variables.file"]}')

curl -X POST "$API_URL" \
    -H "Authorization: Bearer $TOKEN" \
    -F "operations=$JSON_OPERATIONS" \
    -F "map=$JSON_MAP" \
    -F "0=@$FILE_PATH"
