#!/usr/bin/env python3
# Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

# Convert color schemes from material ui.

import os
import re
import subprocess
import sys


def to_lower(name):
    name_pattern = re.compile(r"(?<!^)(?=[A-Z])")
    return name_pattern.sub("_", name).lower()

def to_upper(name):
    name_pattern = re.compile(r"(?<!^)(?=[A-Z])")
    return name_pattern.sub("_", name).upper()

def rustfmt(filename):
    subprocess.call(["rustfmt", filename])

def convert_color(input_file):
    filename = os.path.split(input_file)[1]
    output_file = to_lower(filename)
    output_file = output_file.replace(".js", ".rs")
    
    basename = os.path.splitext(filename)[0]
    color_name = to_upper(basename)

    lines = [
        "// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.\n",
        "// Use of this source is governed by Apache-2.0 License that can be found\n",
        "// in the LICENSE file.\n",
        "\n",
        "use super::Color;\n",
        "\n",
        F"pub const {color_name}: &Color = &Color",
        "{\n",
    ]


    name_map = {
        " 50": " a50",
        " 100": " a100",
        " 200": " a200",
        " 300": " a300",
        " 400": " a400",
        " 500": " a500",
        " 600": " a600",
        " 700": " a700",
        " 800": " a800",
        " 900": " a900",
        " A100": " a1000",
        " A200": " a2000",
        " A400": " a4000",
        " A700": " a7000",
    }
    
    with open(input_file) as input_fh:
        for line in input_fh:
            if line == ";":
                break
            if ":" not in line:
                continue
            line = line.replace("'", '"')
            for (key, val) in name_map.items():
                line = line.replace(key, val)
            lines.append(line)

    lines.append("};\n")

    with open(output_file, "w") as output_fh:
        output_fh.writelines(lines)

    rustfmt(output_file)

def convert_all_colors(input_file):
    parent_dir = os.path.dirname(input_file)
    js_files = [
        "amber.js",
        "blue.js",
        "blueGrey.js",
        "brown.js",
        "cyan.js",
        "deepOrange.js",
        "deepPurple.js",
        "green.js",
        "grey.js",
        "indigo.js",
        "lightBlue.js",
        "lightGreen.js",
        "lime.js",
        "orange.js",
        "pink.js",
        "purple.js",
        "red.js",
        "teal.js",
        "yellow.js",
    ]

    for js_file in js_files:
        input_file = os.path.join(parent_dir, js_file)
        convert_color(input_file)


def main():
    if len(sys.argv) != 2:
        print("Usage: %s some-color.js" % sys.argv[0])
        sys.exit(1)

    input_file = sys.argv[1]
    convert_all_colors(input_file)

if __name__ == "__main__":
    main()
