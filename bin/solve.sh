#!/bin/sh
set -eu

cd -- "$(git rev-parse --show-toplevel)"
make build
exec ./target/debug/solve "$@"
