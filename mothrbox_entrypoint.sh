#!/bin/bash
set -e

# From outside, this looks like just the Rust CLI.
# Inside, walrus.rs can still call Deno as a child process.

exec mothrbox_rs "$@"
