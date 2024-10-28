#!/bin/bash

# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by Lesser General Public License that can be found
# in the LICENSE file.

# Build, serve zu-docs and watch for file changes

cargo watch -s 'cd crates/zu-docs && trunk serve'
