#!/bin/bash

cargo fmt
find . -name "*.h" -o -name "*.cpp" | xargs clang-format -i
