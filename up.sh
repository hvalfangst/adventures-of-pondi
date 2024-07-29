#!/bin/sh

# Exits immediately if a command exits with a non-zero status
set -e

# Compiles the synthesizer application
cargo build

# Runs the synthesizer application
cargo run