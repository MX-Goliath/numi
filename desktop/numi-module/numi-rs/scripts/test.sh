#!/bin/bash

# Debug commands (x), stop on error (e), catch uninitialized (u), pass error code above (o pipefail)
set -eo pipefail

about() {
cat <<EOF

Test, lint, build and run cli tests.

Usage: $0 [target]

EOF
}

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
	about && exit 0
fi

DEFAULT_TARGET=$(rustc -vV | sed -n 's|host: ||p')
TARGET=${1:-$DEFAULT_TARGET}

if [[ $TARGET == *musl* ]]; then
    FEATURES="rustls"
else
    FEATURES="default-tls"
fi

cargo --version
cargo clippy --version
git --version

cargo clippy -- --deny=warnings

echo "--- Test"
cargo test --target $TARGET
cd ./cli && cargo build --target $TARGET --features $FEATURES && cd .. && ./scripts/test_cli.sh "./cli/target/$TARGET/debug/numi-cli"
