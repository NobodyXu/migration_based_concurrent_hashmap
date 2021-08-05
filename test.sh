#!/bin/sh -ex

cd $(dirname $(realpath $0))

cargo test

export RUSTFLAGS='-Zsanitizer=address'

exec cargo +nightly test
