#!/usr/bin/env bash

echo '{"lodash": {"current": "4.14.0", "latest": "4.17.21"}}' | cargo run --bin deputui-pnpm | cargo run --bin deputui-review
