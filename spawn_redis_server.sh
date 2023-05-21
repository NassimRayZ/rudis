#!/bin/sh
exec cargo run \
	--quiet \
	--release \
	--target-dir=/tmp/redis-target \
	--manifest-path $(dirname $0)/Cargo.toml "$@"
