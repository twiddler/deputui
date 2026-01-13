#!/usr/bin/env bash

echo '{"foo": {"current": "1.0.0", "wanted": "1.1.0"}, "bar": {"current": "2.0.0", "wanted": "2.2.2"}, "baz": {"current": "12.0.0", "wanted": "13.13.13"}}' | cargo run
