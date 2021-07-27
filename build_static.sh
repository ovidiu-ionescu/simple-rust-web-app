#!/bin/bash

echo "Build a statically linked executable"
docker run  --rm -t \
	-v $PWD:/volume \
	clux/muslrust \
	cargo build --release

