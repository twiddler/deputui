#!/usr/bin/env bash

echo '{"lodash": {"current": "4.14.0", "latest": "4.17.21"}}' | cargo run --bin outdat-list | cargo run --bin outdatui
