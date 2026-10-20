#!/bin/bash

# Debug commands (x), stop on error (e), catch uninitialized (u), pass error code above (o pipefail)
set -euo pipefail

about() {
cat <<EOF

Triggers remote command on CI

Usage: $0 <command> [options]

Supported commands: test, release, public-release.

EOF
}

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
	about && exit 0
fi

COMMAND=$1
ARG="${2:-$''}"
if [ -f ".env" ]; then source .env; export VAULT_ADDR VAULT_TOKEN;fi

ORG="nk"
BRANCH=$(git rev-parse --abbrev-ref HEAD)
COMMIT=$(git rev-parse HEAD)
BUILDKITE_TOKEN="$(vault kv get -field=api-token kv/buildkite)"

if [[ "$COMMAND" == "test" ]]; then
	echo "Test branch"
	BUILD_URL=$(curl -sSf -H "Authorization: Bearer $BUILDKITE_TOKEN" "https://api.buildkite.com/v2/organizations/$ORG/pipelines/numi-rs/builds" \
		-X "POST" \
		-d "{
			\"commit\": \"$COMMIT\",
			\"branch\": \"$BRANCH\",
			\"message\": \"Test Release\",
			\"env\": {
				\"RELEASE_TAG\": \"v0.0.1\",
				\"OPTS\": \"none\"
				}
			}" | jq -r '.web_url')
fi

if [[ "$COMMAND" == "release" || "$COMMAND" == "public-release" ]]; then
	if [[ -z "$ARG" ]]; then
		echo "Specify tag for release" >&2; exit 1
	fi
	TAG="$ARG"
	if ! [[ $TAG =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
			echo "TAG is invalid, should be in vX.X.X form"
			exit 1
	fi
	if [[ "$COMMAND" == "public-release" ]]; then
		read -p "Creating public release. Continue? " -n 1 -r
		echo    # (optional) move to a new line
		if [[ ! $REPLY =~ ^[Yy]$ ]]; then
			exit 0
		fi
	fi

	git tag -d $TAG || true
	git tag -a $TAG -m "$TAG"
	git push origin :$TAG || true
	git push origin $TAG

fi

if [[ "$COMMAND" == "release" ]]; then
	echo "Release"
	BUILD_URL=$(curl -sSf -H "Authorization: Bearer $BUILDKITE_TOKEN" "https://api.buildkite.com/v2/organizations/$ORG/pipelines/numi-rs/builds" \
		-X "POST" \
		-d "{
			\"commit\": \"$COMMIT\",
			\"branch\": \"$BRANCH\",
			\"message\": \"Release $TAG\",
			\"env\": {
				\"RELEASE_TAG\": \"$TAG\",
				\"OPTS\": \"internal\"
				}
			}" | jq -r '.web_url')
fi

if [[ "$COMMAND" == "public-release" ]]; then
	echo "Public release"
	BUILD_URL=$(curl -sSf -H "Authorization: Bearer $BUILDKITE_TOKEN" "https://api.buildkite.com/v2/organizations/$ORG/pipelines/numi-rs/builds" \
		-X "POST" \
		-d "{
			\"commit\": \"$COMMIT\",
			\"branch\": \"$BRANCH\",
			\"message\": \"Public release $TAG\",
			\"env\": {
				\"RELEASE_TAG\": \"$TAG\",
				\"OPTS\": \"public\"
				}
			}" | jq -r '.web_url')
fi

if [ -n "$BUILD_URL" ]; then
	open "$BUILD_URL"
fi
