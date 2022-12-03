#!/bin/sh
trunk build --release index.html
cargo run --release --bin server_app -- --dir dist/./
