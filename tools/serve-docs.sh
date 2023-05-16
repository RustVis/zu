#!/bin/bash

# Build, serve zu-docs and watch for file changes

cargo watch -s 'cd crates/zu-docs && trunk serve'
