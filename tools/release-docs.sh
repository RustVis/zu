#!/bin/bash

# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by Lesser General Public License that can be found
# in the LICENSE file.

# Build and release documentation.

set -xe

cd crates/zu-docs
trunk build --release
wasm-opt dist/zu-docs-*_bg.wasm -Os -o dist/opt.wasm
mv -f dist/opt.wasm dist/zu-docs-*_bg.wasm
rsync --delete -ave "ssh -p ${ZU_SERVER_PORT}" dist/ ${ZU_SERVER}/zu
