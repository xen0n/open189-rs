#!/bin/sh

set -e
set -x


die() {
	local message=$1

	printf "\033[1;31m *** $message\033[m\n" >&2
	exit 1
}


die_example() {
	die "failed cargo build --example"
}


cargo build || die "failed cargo build"
cargo build --example get_access_token_cc || die_example
cargo build --example sms_get_token || die_example
cargo build --example sms_send_verification_code || die_example

cargo test
