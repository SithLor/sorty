#!/bin/bash

mkdir -p asm

# ...existing code...

# Generate assembly code
cargo rustc --profile dev-asm -- -C debuginfo=2 --emit asm -C llvm-args=--x86-asm-syntax=intel --crate-type rlib -o ./asm/_math.s

# Disassemble the binary
objdump -d ./asm/_math.s > ./asm/disassembled_math.s

echo "Disassembled output saved to ./asm/disassembled_math.s"