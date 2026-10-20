#!/bin/bash

# Debug commands (x), stop on error (e), catch uninitialized (u), pass error code above (o pipefail)
set -euo pipefail

about() {
cat <<EOF

Create latest release in S3

Usage: $0 <tag>

EOF
}

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
	about && exit 0
fi

RELEASE_TAG=$1

export AWS_ACCESS_KEY_ID="$(vault kv get -field=key_id kv/numi/s3)"
export AWS_SECRET_ACCESS_KEY="$(vault kv get -field=secret kv/numi/s3)"
export AWS_ENDPOINT_URL="$(vault kv get -field=endpoint kv/numi/s3)"
export RELEASE_BUCKET="$(vault kv get -field=bucket kv/numi/s3)"
export BASE_URL="$(vault kv get -field=base_url kv/numi/s3)"

SOURCE_S3_PATH="s3://${RELEASE_BUCKET}/cli/${RELEASE_TAG}"
DEST_S3_PATH="s3://${RELEASE_BUCKET}/cli/latest"

# List all files in the source S3 directory
s3_files=$(aws s3 ls "${SOURCE_S3_PATH}/" --recursive --endpoint $AWS_ENDPOINT_URL | awk '{print $4}')

# Loop through each file and copy it to the destination S3 directory
for file in $s3_files; do
    filename=$(basename "$file")
    new_filename=${filename//-${RELEASE_TAG}-/-}
    aws s3 cp "${SOURCE_S3_PATH}/${filename}" "${DEST_S3_PATH}/${new_filename}" --endpoint $AWS_ENDPOINT_URL
done
