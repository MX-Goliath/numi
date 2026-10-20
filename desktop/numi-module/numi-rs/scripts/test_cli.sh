#!/bin/bash

# Debug commands (x), stop on error (e), catch uninitialized (u), pass error code above (o pipefail)
set -euo pipefail

about() {
cat <<EOF

Test cli

Usage: $0 [cli path]

EOF
}

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
	about && exit 0
fi

CLI_PATH="${1:-./cli/target/debug/numi-cli}"

test_match () {
	#RES=$(cargo run -p numi-cli -- "$1")
	RES=$($CLI_PATH "$1")
	[[ "$RES" =~ $2 ]] && echo "Test '$1' passed: $RES" || (echo "Test '$1' failed: expected '$2', got '$RES'" && exit 1)
}

test_not_match () {
	#RES=$(cargo run -p numi-cli -- "$1")
	RES=$($CLI_PATH "$1")
	[[ ! "$RES" =~ $2 ]] && echo "Test '$1' passed: $RES" || (echo "Test '$1' failed: got '$RES'" && exit 1)
}

test_match_localized () {
	#RES=$(cargo run -p numi-cli -- "$1")
	RES=$($CLI_PATH --locale=ru "$1")
	[[ "$RES" =~ $2 ]] && echo "Test '$1' passed: $RES" || (echo "Test '$1' failed: expected '$2', got '$RES'" && exit 1)
}

printf "### Testing cli $CLI_PATH\n"

# reset cache
$CLI_PATH --no-cache 0

test_match "1+2" "3"
test_match "2 m in cm" "200 cm"
test_match "2 m 4 cm" "204 cm"
test_match "2 USD in BYN" "[[:digit:]\.]+ BYN"
test_match "\$2 in BYN" "[[:digit:]\.]+ BYN"
test_not_match "2 USD in BYN" "^2 BYN"
test_match "2 USD in IOT" "[[:digit:]\.]+ IOT"
test_not_match "2 USD in IOT" "^2 IOT"
test_match_localized "2 метра в см" "200 см"
