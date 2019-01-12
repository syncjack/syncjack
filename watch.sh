#!/usr/bin/env bash
#docker-compose run --rm rusted-tested /bin/bash -c 'RUSTFLAGS="" cargo watch -c -q -x "kcov --verbose -- --exclude-line='#'"'
cargo watch -c -x "clippy" -x "test"