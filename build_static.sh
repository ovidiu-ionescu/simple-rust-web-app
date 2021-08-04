#!/bin/bash

echo "Build a statically linked executable"
docker run  --rm -t \
	-v $PWD:/volume \
	clux/muslrust \
	cargo build --release

NAME=$(grep name Cargo.toml | awk -F '"' '{ print $2 }')
cp target/x86_64-unknown-linux-musl/release/$NAME Kubernetes/Docker/
strip Kubernetes/Docker/$NAME

