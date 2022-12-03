#!/bin/sh
trunk build index.html
cargo run --bin server_app -- --dir dist/

