#!/bin/bash

# Debug commands (x), stop on error (e), catch uninitialized (u), pass error code above (o pipefail)
set -euo pipefail

about() {
cat <<EOF

Create public release, upload files from private release, updates homebrew cask.

Usage: $0 <tag>

EOF
}

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
	about && exit 0
fi

RELEASE_TAG=$1
RELEASE_TITLE="numi-cli ${RELEASE_TAG}"

PUBLIC_REPO="https://github.com/nikolaeu/numi"
HOMEBREW_REPO="nikolaeu/homebrew-numi"
ROOT_PATH=$(cd "$(dirname "${BASH_SOURCE[0]}")/../" && pwd -P)
WORKDIR="$ROOT_PATH/_work"

DARWIN_ARCH64="numi-cli-$RELEASE_TAG-aarch64-apple-darwin.tar.gz"
DARWIN_X86_64="numi-cli-$RELEASE_TAG-x86_64-apple-darwin.tar.gz"
LINUX_GNU_X86_64="numi-cli-$RELEASE_TAG-x86_64-unknown-linux-gnu.tar.gz"
LINUX_GNU_ARCH64="numi-cli-$RELEASE_TAG-aarch64-unknown-linux-gnu.tar.gz"
LINUX_MUSL_ARCH64="numi-cli-$RELEASE_TAG-aarch64-unknown-linux-musl.tar.gz"
LINUX_MUSL_X86_64="numi-cli-$RELEASE_TAG-x86_64-unknown-linux-musl.tar.gz"
WIN_X86="numi-cli-$RELEASE_TAG-x86_64-pc-windows-msvc.tar.gz"
WIN_ARCH="numi-cli-$RELEASE_TAG-aarch64-pc-windows-msvc.tar.gz"

echo "WORKDIR: $WORKDIR"
rm -rf "$WORKDIR"
mkdir -p "$WORKDIR"

# Right now passing --prerelease, in the future should be removed
RELEASE_FLAGS=""
FORMULA_FILE="numi-cli.rb"
CLASS_NAME="NumiCli"
if [[ "$RELEASE_TAG" == *pre* ]]; then
	RELEASE_FLAGS="--prerelease"
	FORMULA_FILE="numi-cli-pre.rb"
	CLASS_NAME="NumiCliPre"
fi

PUBLIC_RELEASE_TAG="cli-$RELEASE_TAG"
NOTES_FILE="${RELEASE_TAG%-pre}.md"

export GITHUB_TOKEN="$(vault kv get -field=release_token kv/numi/github)"

## Downloading from S3
export AWS_ACCESS_KEY_ID="$(vault kv get -field=key_id kv/numi/s3)"
export AWS_SECRET_ACCESS_KEY="$(vault kv get -field=secret kv/numi/s3)"
export AWS_ENDPOINT_URL="$(vault kv get -field=endpoint kv/numi/s3)"
export RELEASE_BUCKET="$(vault kv get -field=bucket kv/numi/s3)"
export BASE_URL="$(vault kv get -field=base_url kv/numi/s3)"
S3_PATH="s3://${RELEASE_BUCKET}/cli/${RELEASE_TAG}"

aws s3 cp "${S3_PATH}/$DARWIN_ARCH64" "$WORKDIR/$DARWIN_ARCH64" --endpoint $AWS_ENDPOINT_URL
aws s3 cp "${S3_PATH}/$DARWIN_X86_64" "$WORKDIR/$DARWIN_X86_64" --endpoint $AWS_ENDPOINT_URL

aws s3 cp "${S3_PATH}/$LINUX_GNU_X86_64" "$WORKDIR/$LINUX_GNU_X86_64" --endpoint $AWS_ENDPOINT_URL
aws s3 cp "${S3_PATH}/$LINUX_GNU_ARCH64" "$WORKDIR/$LINUX_GNU_ARCH64" --endpoint $AWS_ENDPOINT_URL

aws s3 cp "${S3_PATH}/$LINUX_MUSL_X86_64" "$WORKDIR/$LINUX_MUSL_X86_64" --endpoint $AWS_ENDPOINT_URL
aws s3 cp "${S3_PATH}/$LINUX_MUSL_ARCH64" "$WORKDIR/$LINUX_MUSL_ARCH64" --endpoint $AWS_ENDPOINT_URL

aws s3 cp "${S3_PATH}/$WIN_X86" "$WORKDIR/$WIN_X86" --endpoint $AWS_ENDPOINT_URL
aws s3 cp "${S3_PATH}/$WIN_ARCH" "$WORKDIR/$WIN_ARCH" --endpoint $AWS_ENDPOINT_URL || true

gh release delete $PUBLIC_RELEASE_TAG --repo "$PUBLIC_REPO" --yes || true
gh release create $PUBLIC_RELEASE_TAG $RELEASE_FLAGS --repo "$PUBLIC_REPO" --title "$RELEASE_TITLE" --notes-file "./release_notes/${NOTES_FILE}"
gh release upload $PUBLIC_RELEASE_TAG --repo "$PUBLIC_REPO" --clobber "$WORKDIR/$DARWIN_ARCH64" "$WORKDIR/$DARWIN_X86_64" "$WORKDIR/$LINUX_GNU_X86_64" "$WORKDIR/$LINUX_GNU_ARCH64"
gh release upload $PUBLIC_RELEASE_TAG --repo "$PUBLIC_REPO" --clobber "$WORKDIR/$LINUX_MUSL_X86_64" "$WORKDIR/$LINUX_MUSL_ARCH64"
gh release upload $PUBLIC_RELEASE_TAG --repo "$PUBLIC_REPO" --clobber "$WORKDIR/$WIN_X86" "$WORKDIR/$WIN_ARCH" || true
gh release upload $PUBLIC_RELEASE_TAG --repo "$PUBLIC_REPO" --clobber ./assets/numi.alfredworkflow

## Update Homebrew
# DOWNLOAD_BASE="$PUBLIC_REPO/releases/download/$PUBLIC_RELEASE_TAG"
DOWNLOAD_BASE="${BASE_URL}/cli/${RELEASE_TAG}"

cp "$ROOT_PATH/assets/numi-cli.template.rb" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{class}|$CLASS_NAME|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{mac_arm_url}|$DOWNLOAD_BASE/$DARWIN_ARCH64|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{mac_arm_sha}|$(shasum -a 256 "$WORKDIR/$DARWIN_ARCH64" | awk '{print $1}')|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{mac_intel_url}|$DOWNLOAD_BASE/$DARWIN_X86_64|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{mac_intel_sha}|$(shasum -a 256 "$WORKDIR/$DARWIN_X86_64" | awk '{print $1}')|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{linux_intel_url}|$DOWNLOAD_BASE/$LINUX_GNU_X86_64|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{linux_intel_sha}|$(shasum -a 256 "$WORKDIR/$LINUX_GNU_X86_64"| awk '{print $1}')|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{linux_arm_url}|$DOWNLOAD_BASE/$LINUX_GNU_ARCH64|" "$WORKDIR/$FORMULA_FILE"
sed -i -e "s|{linux_arm_sha}|$(shasum -a 256 "$WORKDIR/$LINUX_GNU_ARCH64"| awk '{print $1}')|" "$WORKDIR/$FORMULA_FILE"

HOMEBREW_REPO_PATH=$WORKDIR/homebrew-numi
HOMEBREW_REPO_URL="https://oauth2:$GITHUB_TOKEN@github.com/$HOMEBREW_REPO.git"

git config --global user.email "release@numi.app"
git config --global user.name "Release workflow"

git clone $HOMEBREW_REPO_URL --depth 1 $HOMEBREW_REPO_PATH
cp -f $WORKDIR/$FORMULA_FILE $HOMEBREW_REPO_PATH/Formula/
cd $HOMEBREW_REPO_PATH
git add ./Formula/$FORMULA_FILE
git commit -m "$FORMULA_FILE $RELEASE_TAG" || true
git push $HOMEBREW_REPO_URL
rm -rf $HOMEBREW_REPO_PATH
