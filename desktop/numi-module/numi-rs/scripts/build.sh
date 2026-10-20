#!/bin/bash

# Debug commands (x), stop on error (e), catch uninitialized (u), pass error code above (o pipefail)
set -eo pipefail

about() {
cat <<EOF

Test (unit tests, cli), archive and upload to release tool.

Usage: $0 <tag> [target] [options]

Options:
  skiptest  Skip unit tests and cli tests
  internal  Upload to releases
	public		Upload to releases

EOF
}


# Required on Windows so forward slash does not resolve to path
export MSYS_NO_PATHCONV=1

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
	about && exit 0
fi

RELEASE_TAG=$1
DEFAULT_TARGET=$(rustc -vV | sed -n 's|host: ||p')
TARGET=${2:-$DEFAULT_TARGET}
OPTS="${3:-$''}"
echo "Options: $OPTS"

export AWS_ACCESS_KEY_ID="$(vault kv get -field=key_id kv/numi/s3)"
export AWS_SECRET_ACCESS_KEY="$(vault kv get -field=secret kv/numi/s3)"
export AWS_ENDPOINT_URL="$(vault kv get -field=endpoint kv/numi/s3)"
export RELEASE_BUCKET="$(vault kv get -field=bucket kv/numi/s3)"
S3_PATH="s3://${RELEASE_BUCKET}/cli"

if [ -z "$RELEASE_TAG" ]; then
	echo "Release tag is missing" && exit 1
fi

if [[ $TARGET == *musl* ]]; then
    FEATURES="rustls"
else
    FEATURES="default-tls"
fi

FILENAME="numi-cli-$RELEASE_TAG-$TARGET.tar.gz"

# rustup target add "$TARGET"

# Prepare artifacts dir
rm -rf ./artifacts || true
mkdir -p ./artifacts

if [[ "$OPTS" != *"skiptest"* ]]; then
	echo "--- Test"
	cargo test --target $TARGET
fi

echo "--- Archive"
cd cli
sed -i -e "s|(dev)|$RELEASE_TAG|" ./src/main.rs
cargo build --target $TARGET --release -p numi-cli --features $FEATURES
git checkout ./src/main.rs
tar -czf $FILENAME -C "./target/$TARGET/release/" numi-cli
cd ..

if [[ "$OPTS" != *"skiptest"* ]]; then
	echo "--- Test cli"
	./scripts/test_cli.sh ./cli/target/$TARGET/release/numi-cli
fi

cp "cli/$FILENAME" ./artifacts/

if [[ "$OPTS" == *"internal"* || "$OPTS" == *"public"* ]]; then
	echo "--- Uploading"
	FILENAME="numi-cli-$RELEASE_TAG-$TARGET.tar.gz"
	aws s3 cp  "cli/$FILENAME" "${S3_PATH}/$RELEASE_TAG/$FILENAME" --endpoint $AWS_ENDPOINT_URL --acl public-read
fi
